use parity_codec::{Decode, Encode};
use rstd::vec::Vec;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;

const ERR_DID_ALREADY_CLAIMED: &str = "This DID has already been claimed.";
const ERR_UN_ALREADY_CLAIMED: &str = "This unique name has already been claimed.";
const ERR_OVERFLOW: &str = "Overflow adding new metadata";

const ERR_BYTEARRAY_LIMIT_DID: &str = "DID bytearray is too large";
const ERR_BYTEARRAY_LIMIT_NAME: &str = "Name bytearray is too large";

const BYTEARRAY_LIMIT_DID: usize = 100;
const BYTEARRAY_LIMIT_NAME: usize = 50;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Metalog {
    pub did: Vec<u8>,
    pub unique_name: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        /// Query for unique names
        UnMeta get(meta_of_un): map Vec<u8> => Metalog;
        UnOwner get(owner_of_un): map Vec<u8> => Option<T::AccountId>;

        /// Query by DIDs
        DidMeta get(meta_of_did): map Vec<u8> => Metalog;
        DidOwner get(owner_of_did): map Vec<u8> => Option<T::AccountId>;

        /// Array of personal owned Metalog data
        OwnedMetaArray get(metadata_of_owner_by_index): map (T::AccountId, u64) => Metalog;

        /// Number of stored Metalogs per account
        OwnedMetaCount get(owner_meta_count): map T::AccountId => u64;

        /// Index of DID
        OwnedMetaIndex: map Vec<u8> => u64;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        fn deposit_event<T>() = default;

        fn create_metalog(
			origin,
			did: Vec<u8>,
			unique_name: Vec<u8>) -> Result {

			let sender = ensure_signed(origin)?;

			// Verify
			ensure!(did.len() <= BYTEARRAY_LIMIT_DID, ERR_BYTEARRAY_LIMIT_DID);
			ensure!(unique_name.len() <= BYTEARRAY_LIMIT_NAME, ERR_BYTEARRAY_LIMIT_NAME);
			ensure!(!<DidOwner<T>>::exists(&did), ERR_DID_ALREADY_CLAIMED);
            ensure!(!<UnOwner<T>>::exists(&unique_name), ERR_UN_ALREADY_CLAIMED);

			let count = Self::owner_meta_count(&sender);
			let updated_count = count.checked_add(1).ok_or(ERR_OVERFLOW)?;

			let metalog = Metalog {
				did,
				unique_name,
			};

			// Store
			<OwnedMetaArray<T>>::insert((sender.clone(), count), &metalog);
			<OwnedMetaCount<T>>::insert(&sender, updated_count);
			<OwnedMetaIndex<T>>::insert(&metalog.did, updated_count);
			<DidMeta<T>>::insert(&metalog.did, &metalog);
			<DidOwner<T>>::insert(&metalog.did, &sender);
			<UnMeta<T>>::insert(&metalog.unique_name, &metalog);
			<UnOwner<T>>::insert(&metalog.unique_name, &sender);

			Self::deposit_event(RawEvent::Stored(sender, metalog.did, metalog.unique_name));
			Ok(())
		}
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Stored(AccountId, Vec<u8>, Vec<u8>),
    }
);

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_noop, assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type Log = DigestItem;
    }
    impl Trait for Test {
        type Event = ();
    }
    type TemplateModule = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn create_metalog_works() {
        with_externalities(&mut new_test_ext(), || {
            let did = vec![1, 2];
            let un = vec![1];
            let mut did_too_long = did.clone();
            let mut un_too_long = un.clone();
            for _i in 1..100 {
                did_too_long.push(2);
                un_too_long.push(1);
            }
            assert_noop!(
                TemplateModule::create_metalog(
                    Origin::signed(20),
                    did_too_long.clone(),
                    un.clone()
                ),
                ERR_BYTEARRAY_LIMIT_DID
            );
            assert_noop!(
                TemplateModule::create_metalog(
                    Origin::signed(20),
                    did.clone(),
                    un_too_long.clone()
                ),
                ERR_BYTEARRAY_LIMIT_NAME
            );
            assert_ok!(TemplateModule::create_metalog(
                Origin::signed(20),
                did.clone(),
                un.clone()
            ));
            assert_eq!(TemplateModule::owner_of_did(did), Some(20));
        });
    }
}

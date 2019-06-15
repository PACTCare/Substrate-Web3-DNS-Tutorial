# 4. Runtime

Now let’s start to create your own runtime module. All runtime modules are registered in the [lib.rs file]( ../node/runtime/src/lib.rs). You find in this file also the registration of some of the previously mentioned runtime modules, like Consensus or Balances. If you want, you can change the default modules or integrate other modules from the Substrate Runtime Module Library (SRML). But for now, let’s leave everything like it is and just add an additional module. 

Luckily, the [lib.rs file]( ../node/runtime/src/lib.rs) and the folder already contains a [template.rs file]( ../node/runtime/src/template.rs), which we can use. If you want, you can change the name of the file. If you do this, make sure you also change the three appearances of this file inside [lib.rs file]( ../node/runtime/src/lib.rs), which are:

* Import of the file ("mod template;")
* Implementation of the traits ("impl template::Trait for Runtime{...")
* Integration of the module into the construct_runtime! macro ("TemplateModule: template::{Module,...")

Next, we are going to add a storage item, function as well as an event to the [template.rs file]( ../node/runtime/src/template.rs). Therefore, we are going to edit the “Something” in the decl_storage, decl_module as well as decl_event part. 

## Storage

The “Something get(something)” element in the template file stores just a single value. For the next generation of DNS, we probably want to store a more complex structure. That’s why we first create a struct, which we call Metalog:

```
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Metalog {
    pub did: Vec<u8>,
    pub unique_name: Vec<u8>,
}
```
DID stands for [decentralized identifier](https://w3c-ccg.github.io/did-spec/) and it points to a document containing all the information about the data as well as a storage location. The unique name is the equivalent of the current domain names and is a simple, human-readable name. To define this custom struct, we also need to import additional traits at the top of the document:
```
use parity_codec::{Encode, Decode};
use rstd::vec::Vec;
```
The import of rstd::vec::Vec is necessary to support byte vectors, which will essentially be used to handle strings in the module. 

We probably also want to map the individual domains names to specific accounts instead of storing them across all accounts. That’s why we replace the StorageValue import with the StorageMap import. Additionally, we add the ensure trait. This is used to test certain requirements before the variable is stored. 
```
use support::{decl_module, decl_storage, decl_event, ensure, StorageMap, dispatch::Result};
```
Finally, we create the storage mapping inside the decl_storage! module. 
```
trait Store for Module<T: Trait> as TemplateModule {
    /// Query for unique names
    UnMeta get(meta_of_un): map Vec<u8> => Metalog;
    UnOwner get(owner_of_un): map Vec<u8> => Option<T::AccountId>;

    /// Query by DIDs
    DidMeta get(meta_of_did): map Vec<u8> => Metalog;
    DidOwner get(owner_of_did): map Vec<u8> => Option<T::AccountId>;

    /// Personal owned Metalog data referenced by number
    OwnedMetaArray get(metadata_of_owner_by_index): map (T::AccountId, u64) => Metalog;

    /// Number of stored Metalogs per account
    OwnedMetaCount get(owner_meta_count): map T::AccountId => u64;

    /// Index of DID
    OwnedMetaIndex: map Vec<u8> => u64;
}    
```
Especially the last three elements are interesting since they basically represent an array based on the combination of tuples and maps. This enables every user to own multiple metalog entries. 

## Function
Now, we actually going to implement a function to create a metalog entry. Therefore we put the following function inside “pub struct Module<T: Trait> for enum Call where origin: T::Origin {“.
```
/// Store initial metalog
fn create_metalog(
    origin,
    did: Vec<u8>,
    unique_name: Vec<u8>) -> Result {
    // Verify
    let sender = ensure_signed(origin)?;
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
```
It’s very important to always stick to the pattern **"Verify First, Store Last"**, because your Runtime should never panic and also be safe against potential attacks. Panics can completely destroy you blockchain storage. That’s why at the beginning of the function we use multiple ensure! checks. Typical verifications are:

* Verifying Signed Messages
* Overflows/Underflows Checks
* Collision Checks
* Storage Limit

After the checks we store the values. 

## Events

You might have noticed that at the end of the above function we already implemented a call to our event. This ensures that we tell the world that the function executed successfully. So all that is left to do is to declare the actual event in the decl_event! module. 

```
pub enum Event<T>
where
    AccountId = <T as system::Trait>::AccountId,
    {
        Stored(AccountId, Vec<u8>, Vec<u8>),
    }
```

Obviously, the runtime module is only a starting point for a Web3 Domain Name/Metadata System. You probably want to at least at functions to update the entry as well as at the smart contract module, so that people can create additional services on top of your blockchain, like for example marketplaces. Feel free to take a look at [Stars Network Whitepaper](https://github.com/PACTCare/Stars-Network/blob/master/WHITEPAPER.md) and specifically the [Starlog runtime](https://github.com/PACTCare/Starlog).

---
\
**-> [Next: 5. Testing](./5_testing.md)**
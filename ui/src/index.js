import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const provider = new WsProvider('ws://127.0.0.1:9944');

async function substrate() {
    console.log('connect...');
    const api = await ApiPromise.create({
        provider: provider,
        types: {
            // Register custom type
            Metalog: {
                did: "Vec<u8>",
                unique_name: "Vec<u8>"
            },
        },
    });

    // For console testing 
    window.substrateApi = api;

    document.getElementById("button").addEventListener("click", async function () {
        let did = document.getElementById("did").value;
        let unique_name = document.getElementById("name").value;

        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');

        const transfer = await api.tx.template.createMetalog(did, unique_name);
        const hash = await transfer.signAndSend(alice);
        console.log('Transfer sent with hash', hash);
    });
}



window.onload = substrate();
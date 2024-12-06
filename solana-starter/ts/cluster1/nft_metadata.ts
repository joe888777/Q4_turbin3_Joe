import wallet from "./wallet/wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = 
        const metadata = {
            name: "rugPro",
            symbol: "RPO",
            description: "We will we will rug you.",
            image: "https://devnet.irys.xyz/FGeJdEizUR9qQY5CQgNozHQNakFpZsNNg4h7K5MJbwaq",
            attributes: [
                {trait_type: 'background', value: 'purple-pink'},
                {trait_type: 'movement', value: 'roll'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://devnet.irys.xyz/FGeJdEizUR9qQY5CQgNozHQNakFpZsNNg4h7K5MJbwaq"
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

const rugNFTAddress = "CFGfxiVJpV4FnAYH8UY5Boa1TnFrkFPz5wRPbXYHJhmf";

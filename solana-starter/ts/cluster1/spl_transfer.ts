import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wallet/wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("7ypZufFhrepLZMqMdmr6bcHEUv3v9oC2Ymw3CtnDP9XH");

// Recipient address
const to = new PublicKey("F8XJLRvgPt7WcRQdqEYx2Y9KPdoPRyjTMR5bgfdpotT7");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );
        // Get the token account of the toWallet address, and if it does not exist, create it
        const tx = await transfer(
            connection,
            keypair,
            fromWallet.address,
            toTokenAccount.address,
            keypair,
            1_000_000
        );
        console.log(`you can view your tx in https://explorer.solana.com/tx/${tx}?cluster=devnet`);
        // Transfer the new token to the "toTokenAccount" we just created
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
pub mod programs;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signer::Signer,
    system_program,
    transaction::Transaction,
    pubkey::Pubkey,
};
use solana_sdk::signature::read_keypair_file;
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};

fn main() {
    // 1. Create a Solana devnet connection
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(RPC_URL);

    // 2. Define our Turbin3 wallet
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

    // 3. Get the recent blockhash
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // 4. Derive the PDA for our prereq account
    let prereq = Turbin3PrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref(),
    ]);

    // 5. Define the instruction data
    let args = CompleteArgs {
        github: b"datmedevil17".to_vec(),
    };

    // 6. Populate the `complete` function
    let transaction = Turbin3PrereqProgram::complete(
        &[
            &signer.pubkey(),  // The signer
            &prereq,           // The derived PDA
            &system_program::id(), // System program
        ],
        &args,                // Instruction data
        Some(&signer.pubkey()), // Fee payer
        &[&signer],           // Signers
        blockhash,            // Recent blockhash
    );

    // 7. Send and confirm the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // 8. Print the transaction result
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

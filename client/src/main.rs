use std::collections::HashMap;
use anchor_client::solana_sdk::signature::{Signer, read_keypair_file};
use anchor_client::{Client, Cluster};
use anchor_lang::declare_id;
use sha2::{Sha256, Digest};
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::rc::Rc;

declare_id!("8mWjeFrmowFrSKTAaWrstBcbXeMJUGpLTX71pAhKGsqP");

pub fn comput_state_root(state: &HashMap<&str, u64>) -> [u8; 32] {
    let mut hasher = Sha256::new();

    for(k, v) in state {
        hasher.update(format!("{k}:{v}"));
    };

    hasher.finalize().into()
}

pub fn main() -> Result<()> {
    //Load local wallet keypair
    let payer = read_keypair_file(
        shellexpand::tilde("~/.config/solana/id.json").to_string()
    ).unwrap();

    let wallet = Rc::new(payer);

    //Anchor client
    let client = Client::new(Cluster::Devnet, wallet.clone());
    let program = client.program(id()).unwrap();

    //Simulated state
    let mut state = HashMap::new();
    state.insert("kavyam", 10);
    state.insert("rahul", 20);

    //Simulated transfers
    *state.get_mut("kavyam").unwrap() -= 3;
    *state.get_mut("rahul").unwrap() += 3;

    //Hash state
    let root = comput_state_root(&state);
    println!("Computed root: {:?}", root);

    //Derive PDA
    let (pda, _bump) = Pubkey::find_program_address(&[b"rollup", wallet.pubkey().as_ref()], &program.id());
    println!("Derived PDA: {:?}", pda);

    //Submit root to on-chain program
    let tx = program
    .request()
    .accounts(rollup_stub::accounts::SubmitRoot {
        signer: wallet.pubkey(),
        state_root: pda,
        system_program: solana_sdk::system_program::ID,
    })
    .args(rollup_stub::instruction::SubmitRoot { root }) // pass as struct
    .send()?;


    println!("Submitted rollup root to chain, signature: {:?}", tx);
    Ok(())
}


use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster, EventContext,Program};

use decentralized_election::{accounts as de_accounts, instruction as de_instructions};
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    // let pid = Pubkey::new(pid.as_bytes());
    let pid = Pubkey::from_str("EHTP9uMcGGMzXbUkShrSJ7e114gMnkW7nTWgvCQGmE2M").expect("Couldn't parse Pubkey");
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).expect("Example requires a keypair file");



    let url = Cluster::Custom("https://api.devnet.solana.com".to_string(), "ws://https://api.devnet.solana.com".to_string());

    println!("url is {}", url);


    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    println!("pid is {}", pid);

    let program = client.program(pid);



}

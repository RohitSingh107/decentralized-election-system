use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::system_program;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
// use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster};
use rand::rngs::OsRng;
use std::rc::Rc;
use std::str::FromStr;

use decentralized_election::{accounts as de_accounts, instruction as de_instructions};
use decentralized_election::initiate_election::ElectionAccount;

fn main() {
    println!("Hello, world!");

    // let pid = Pubkey::new(pid.as_bytes());
    let pid = Pubkey::from_str("EHTP9uMcGGMzXbUkShrSJ7e114gMnkW7nTWgvCQGmE2M")
        .expect("Couldn't parse Pubkey");
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");

    let url = Cluster::Custom(
        "https://api.devnet.solana.com".to_string(),
        "ws://https://api.devnet.solana.com".to_string(),
    );

    println!("url is {}", url);

    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    println!("pid is {}", pid);

    let program = client.program(pid);

    let program_pair = Keypair::generate(&mut OsRng);

    let resp = program
        .request()
        .signer(&program_pair)
        .accounts(de_accounts::InitiateElection {
            election_account: program_pair.pubkey(),
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(de_instructions::InitiateElection { winners_count: 1 })
        .send();

    match resp {
        Ok(signature) => println!("Successfully executed, sig: {}", signature.to_string()),
        Err(e) => {
            panic!("Program failed to send transaction!, {:?}", e);
        } 

    }

    let election_account : ElectionAccount = program.account(program_pair.pubkey()).expect("Failed to fetch election account");

    assert_eq!(election_account.candidates_count, 0);
    assert_eq!(election_account.winners_count, 1);

    let candidate_user = Keypair::generate(&mut OsRng);

    let candidate_id_pda = Pubkey::find_program_address(&[
        "candidate".as_bytes(),
        &candidate_user.pubkey().to_bytes(),
        &program_pair.pubkey().to_bytes()

    ], &program.id());





}

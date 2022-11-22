use anchor_client::solana_client::client_error::ClientError;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signature};
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_program;
use anchor_client::{Client, Cluster, Program};
use rand::rngs::OsRng;
use std::rc::Rc;
use std::str::FromStr;

use decentralized_election::apply::CandidateID;
use decentralized_election::initiate_election::ElectionAccount;
use decentralized_election::register::CandidateElectionData;
use decentralized_election::{accounts as de_accounts, instruction as de_instructions};

pub fn request_air_drop(
    program: &Program,
    pubkey: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, ClientError> {
    let sig: Signature = program
        .rpc()
        .request_airdrop(pubkey, (amount_sol * LAMPORTS_PER_SOL as f64) as u64)?;

    loop {
        let confirmed: bool = program.rpc().confirm_transaction(&sig)?;

        if confirmed {
            break;
        }
    }

    Ok(sig)
}

fn main() {
    println!("Hello, world!");

    // let pid = Pubkey::new(pid.as_bytes());
    let pid = Pubkey::from_str("EHTP9uMcGGMzXbUkShrSJ7e114gMnkW7nTWgvCQGmE2M")
        .expect("Couldn't parse Pubkey");
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");

    // let url = Cluster::Custom(
    //     "https://api.devnet.solana.com".to_string(),
    //     "wss://api.devnet.solana.com".to_string(),
    // );

    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    println!("url is {}", url);

    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    println!("pid is {}", pid);

    let program = client.program(pid);

    let election_account_keypair = Keypair::generate(&mut OsRng);

    let resp = program
        .request()
        .signer(&election_account_keypair)
        .accounts(de_accounts::InitiateElection {
            election_account: election_account_keypair.pubkey(),
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(de_instructions::InitiateElection { winners_count: 1 })
        .send();

    match resp {
        Ok(signature) => println!(
            "Successfully initiated election, sig: {}",
            signature.to_string()
        ),
        Err(e) => {
            panic!("Program failed to send transaction!, {:?}", e);
        }
    }

    let election_account: ElectionAccount = program
        .account(election_account_keypair.pubkey())
        .expect("Failed to fetch election account");

    assert_eq!(election_account.candidates_count, 0);
    assert_eq!(election_account.winners_count, 1);
    println!("Asserted Creation of election account Successfully!");

    let candidate_user = Keypair::generate(&mut OsRng);
    let resp = request_air_drop(&program, &candidate_user.pubkey(), 1.0);
    match resp {
        Ok(sig) => println!("Airdrop Successfully! with sig: {}", sig.to_string()),
        Err(e) => panic!("Airdrop failed, {}", e),
    }

    let candidate_id_pda_tuple = Pubkey::find_program_address(
        &[
            "candidate".as_bytes(),
            &candidate_user.pubkey().to_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    );

    let candidate_id_pda = candidate_id_pda_tuple.0;
    println!("candidate_id_pda {:#?}", candidate_id_pda.to_string());

    let resp = program
        .request()
        .signer(&candidate_user)
        .accounts(de_accounts::Apply {
            candidate_id: candidate_id_pda,
            election_account: election_account_keypair.pubkey(),
            signer: candidate_user.pubkey(),
            system_program: system_program::ID,
        })
        .args(de_instructions::Apply {})
        .send();

    match resp {
        Ok(sig) => println!("Appied Successfully with signature {}", sig.to_string()),
        Err(e) => {
            println!("Couldn't apply");
            panic!("Program failed to send transaction!, {:?}", e);
        }
    }

    let candidate_details: CandidateID = program
        .account(candidate_id_pda)
        .expect("Failed to fetch candidate id account");

    let election_details: ElectionAccount = program
        .account(election_account_keypair.pubkey())
        .expect("Failed to fetch election account");

    assert_eq!(election_details.candidates_count, 1);
    assert_eq!(candidate_details.id, 1);
    assert_eq!(
        candidate_details.pubkey.to_string(),
        candidate_user.pubkey().to_string()
    );
    println!("Asserted Apply Successfully!");

    let candidate_data_pda = (Pubkey::find_program_address(
        &[
            &candidate_details.id.to_be_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    ))
    .0;

    let resp = program
        .request()
        .signer(&candidate_user)
        .accounts(de_accounts::Register {
            candidate_election_data: candidate_data_pda,
            election_account: election_account_keypair.pubkey(),
            candidate_id: candidate_id_pda,
            signer: candidate_user.pubkey(),
            system_program: system_program::ID,
        })
        .args(de_instructions::Register {})
        .send();
    match resp {
        Ok(sig) => println!("Registered Successfully with signature {}", sig.to_string()),
        Err(e) => panic!("Program failed to send transaction!, {:?}", e),
    }

    let candidate_election_data: CandidateElectionData = program
        .account(candidate_data_pda)
        .expect("Failed to fetch candidate election data account");

    assert_eq!(candidate_details.id, candidate_election_data.id);
    assert_eq!(candidate_details.pubkey, candidate_election_data.pubkey);
    assert_eq!(candidate_election_data.votes, 0);
    println!("Asserted Register Successfully!");
}

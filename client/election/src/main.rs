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
use decentralized_election::vote::VotedTo;
use decentralized_election::election_enums::ElectionPhase;
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

    println!("\n\nSetting up...");

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


    println!("\n\nInitiating Elections...");

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

    println!("\n\nApplying Candiate 1");

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


    println!("\n\nRegistering Candiate 1...");
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


    println!("\n\nApplying Candiate 2...");
    let candidate_user2 = Keypair::generate(&mut OsRng);
    let resp = request_air_drop(&program, &candidate_user2.pubkey(), 1.0);
    match resp {
        Ok(sig) => println!("Airdrop Successfully! with sig: {}", sig.to_string()),
        Err(e) => panic!("Airdrop failed, {}", e),
    }

    let candidate2_id_pda: Pubkey = (Pubkey::find_program_address(
        &[
            "candidate".as_bytes(),
            &candidate_user2.pubkey().to_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    ))
    .0;

    let resp = program
        .request()
        .signer(&candidate_user2)
        .accounts(de_accounts::Apply {
            candidate_id: candidate2_id_pda,
            election_account: election_account_keypair.pubkey(),
            signer: candidate_user2.pubkey(),
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

    let candidate2_details: CandidateID = program
        .account(candidate2_id_pda)
        .expect("Failed to fetch candidate id account");

    let election_details: ElectionAccount = program
        .account(election_account_keypair.pubkey())
        .expect("Failed to fetch election account");

    assert_eq!(election_details.candidates_count, 2);
    assert_eq!(candidate2_details.id, 2);
    assert_eq!(
        candidate2_details.pubkey.to_string(),
        candidate_user2.pubkey().to_string()
    );
    println!("Asserted Apply Successfully for user 2!");


    println!("\n\nRegistering Candiate 2...");
    let candidate2_data_pda = (Pubkey::find_program_address(
        &[
            &candidate2_details.id.to_be_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    ))
    .0;

    let resp = program
        .request()
        .signer(&candidate_user2)
        .accounts(de_accounts::Register {
            candidate_election_data: candidate2_data_pda,
            election_account: election_account_keypair.pubkey(),
            candidate_id: candidate2_id_pda,
            signer: candidate_user2.pubkey(),
            system_program: system_program::ID,
        })
        .args(de_instructions::Register {})
        .send();
    match resp {
        Ok(sig) => println!(
            "Registered Successfully user2 with signature {}",
            sig.to_string()
        ),
        Err(e) => panic!("Program failed to send transaction!, {:?}", e),
    }

    let candidate2_election_data: CandidateElectionData = program
        .account(candidate2_data_pda)
        .expect("Failed to fetch candidate election data account");

    assert_eq!(candidate2_details.id, candidate2_election_data.id);
    assert_eq!(candidate2_details.pubkey, candidate2_election_data.pubkey);
    assert_eq!(candidate2_election_data.votes, 0);

    println!("Asserted Register Successfully for use2!");

    println!("\n\nChange Phase to voting ---------------------------------------------");
    let resp = program
        .request()
        .accounts(de_accounts::ChangePhase {
            election_account: election_account_keypair.pubkey(),
            signer: program.payer(),
        })
        .args(de_instructions::ChangePhase {
            new_phase: ElectionPhase::Voting,
        })
        .send();

    match resp {
        Ok(sig) => println!("Changed phase to voting with signature {}", sig.to_string()),
        Err(e) => panic!("Program failed to send transaction!, {:?}", e),
    }

    let election_details: ElectionAccount = program
        .account(election_account_keypair.pubkey())
        .expect("Failed to fetch election account");

    assert_eq!(election_details.phase, ElectionPhase::Voting);
    println!("Asserted Voting Phase change!");

    println!("\n\nvotng for user 1 from user 1");

    let to_vote_id: u64 = 1;

    let to_vote_candidiate_pda: Pubkey = (Pubkey::find_program_address(
        &[
            &to_vote_id.to_be_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    ))
    .0;

    let user_vote_pda : Pubkey = (Pubkey::find_program_address(&[
            "voter".as_bytes(),
            &candidate_user.pubkey().to_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
    ], &program.id())).0;

    let resp = program.request().signer(&candidate_user).accounts(de_accounts::Vote{
        voted_to: user_vote_pda,
        candidate_election_data: to_vote_candidiate_pda,
        election_account: election_account_keypair.pubkey(),
        signer: candidate_user.pubkey(),
        system_program: system_program::ID,
    }).args(de_instructions::Vote{}).send();
    match resp {
        Ok(sig) => println!("Voted successfully with signature {}", sig.to_string()),
        Err(e) => panic!("failed to vote!, {:?}", e),
    }
    let user_voted_data : VotedTo = program.account(user_vote_pda).expect("Failed to fetch user vote data");

    let candidate_election_data: CandidateElectionData = program
        .account(to_vote_candidiate_pda)
        .expect("Failed to fetch candidate election data account");

    assert_eq!(user_voted_data.id, to_vote_id);
    assert_eq!(candidate_election_data.votes, 1);
    println!("Asserted user vote successfully!");


    println!("\n\nvotng for user 1 from user 2");

    let to_vote_id: u64 = 1;

    let to_vote_candidiate_pda: Pubkey = (Pubkey::find_program_address(
        &[
            &to_vote_id.to_be_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
        ],
        &program.id(),
    ))
    .0;

    let user2_vote_pda : Pubkey = (Pubkey::find_program_address(&[
            "voter".as_bytes(),
            &candidate_user2.pubkey().to_bytes(),
            &election_account_keypair.pubkey().to_bytes(),
    ], &program.id())).0;

    let resp = program.request().signer(&candidate_user2).accounts(de_accounts::Vote{
        voted_to: user2_vote_pda,
        candidate_election_data: to_vote_candidiate_pda,
        election_account: election_account_keypair.pubkey(),
        signer: candidate_user2.pubkey(),
        system_program: system_program::ID,
    }).args(de_instructions::Vote{}).send();
    match resp {
        Ok(sig) => println!("Voted successfully by user2 with signature {}", sig.to_string()),
        Err(e) => panic!("failed to vote!, {:?}", e),
    }
    let user2_voted_data : VotedTo = program.account(user2_vote_pda).expect("Failed to fetch user vote data");

    let candidate_election_data: CandidateElectionData = program
        .account(to_vote_candidiate_pda)
        .expect("Failed to fetch candidate election data account");

    assert_eq!(user2_voted_data.id, to_vote_id);
    assert_eq!(candidate_election_data.votes, 2);
    println!("Asserted user2 vote successfully!");


    println!("\n\nClose election and declare final winners");

    let resp = program.request().accounts(de_accounts::ChangePhase{
        election_account: election_account_keypair.pubkey(),
        signer: program.payer(),
    }).args(de_instructions::ChangePhase{
        new_phase: ElectionPhase::Closed
    }).send();
    match resp {
        Ok(sig) => println!("Closed election with signature {}", sig.to_string()),
        Err(e) => panic!("failed to vote!, {:?}", e),
    }


    let election_details: ElectionAccount = program
        .account(election_account_keypair.pubkey())
        .expect("Failed to fetch election account");

    println!("winner is id: {} , with {} votes", election_details.winners_id[0], election_details.winners_votes_count[0]);
    println!("{:#?}", election_details.winners_votes_count);

    assert_eq!(election_details.winners_id[0], 1);
    assert_eq!(election_details.phase, ElectionPhase::Closed);
    println!("Asserted winning successfully!");




}

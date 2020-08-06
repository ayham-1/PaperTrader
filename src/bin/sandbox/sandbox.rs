 #[macro_use] extern crate log;
extern crate simplelog;

pub mod tls_sandbox;
use crate::tls_sandbox::tls_main;

use libtrader::initializer::libtrader_init;
use libtrader::db::cmd::create_company::create_company;
use libtrader::db::cmd::get_company::get_company_from_db;
use libtrader::db::cmd::create_stock::create_stock;
use libtrader::ds::generic::company::Company;
use libtrader::ds::server::global_state::GlobalState;
use libtrader::account::hash_pwd::{hash_pwd_client, hash_pwd_server};
use libtrader::account::hash_email::{hash_email_client, hash_email_server};
use libtrader::account::hash_usr::{hash_usr_client, hash_usr_server};
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, rand};

static RUN_TLS_SERVER_SANDBOX: bool = true;
static RUN_DB_SANDBOX: bool = true;
static RUN_HASH_SANDBOX: bool = true;

fn main() {
    if RUN_DB_SANDBOX {
        let mut state: GlobalState = match libtrader_init() {
            Ok(state) => {info!("inited state: {:?}\n", state); state},
            Err(err) => panic!("Failed with error: {}", err),
        };

        let mut company: Company = Company::default();
        company.id = 1;
        company.symbol = "TEST".to_string();
        company.isin = "TEST".to_string();
        company.company_name = "TEST".to_string();
        company.primary_exchange = "TEST".to_string();
        match create_company(&mut state, company) {
            Ok(()) => info!("created company"),
            Err(err) => error!("Failed to create company with error: {}", err),
        }

        match get_company_from_db(&mut state, "TEST".to_string()) {
            Ok(found_company) => info!("we found it! {:?}", found_company),
            Err(err) => error!("we must found the sacred company! err: {}", err),
        }

        match create_stock(&mut state, "test") {
            Ok(()) => info!("created stock table"),
            Err(err) => error!("failed to create stock table {}", err),
        }

        info!("state: {:?}\n", state);
    }

    if RUN_HASH_SANDBOX {
        let rng = rand::SystemRandom::new();
        let mut server_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt).unwrap();
        let enc = hash_pwd_client("this is my real password", 
                                  server_salt).unwrap();

        println!("Client Pass Hash: {}", HEXUPPER.encode(&enc.0));
        println!("Client Pass Salt: {}", HEXUPPER.encode(&enc.1));

        let enc1 = hash_pwd_server(HEXUPPER.encode(&enc.0).as_str()).unwrap();

        println!("Server Pass Hash: {}", HEXUPPER.encode(&enc1.0));
        println!("Server Pass Salt: {}", HEXUPPER.encode(&enc1.1));

        rng.fill(&mut server_salt).unwrap();
        let enc2 = hash_email_client("totallyrealemail@anemail.c0m",
                                     server_salt).unwrap();

        println!("Client Email Hash: {}", HEXUPPER.encode(&enc2.0));
        println!("Client Email Salt: {}", HEXUPPER.encode(&enc2.1));

        let enc3 = hash_email_server(HEXUPPER.encode(&enc2.0).as_str()).unwrap();
        println!("Server Email Hash: {}", HEXUPPER.encode(&enc3.0));
        println!("Server Email Salt: {}", HEXUPPER.encode(&enc3.1));

        rng.fill(&mut server_salt).unwrap();
        let enc4 = hash_usr_client("n1ckn8me",
                                   server_salt).unwrap();

        println!("Client Username Hash: {}", HEXUPPER.encode(&enc4.0));
        println!("Client Username Salt: {}", HEXUPPER.encode(&enc4.1));

        let enc5 = hash_usr_server(HEXUPPER.encode(&enc2.0).as_str()).unwrap();
        println!("Server Username Hash: {}", HEXUPPER.encode(&enc5.0));
        println!("Server Username Salt: {}", HEXUPPER.encode(&enc5.1));
    }

    if RUN_TLS_SERVER_SANDBOX {
        tls_main();
    }
}


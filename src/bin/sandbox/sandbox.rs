use libtrader::initializer::libtrader_init;
use libtrader::db::cmd::create_company::create_company;
use libtrader::ds::generic::company::Company;
use libtrader::ds::server::global_state::GlobalState;

fn main() {
    let mut state: GlobalState = match libtrader_init() {
        Ok(state) => {println!("inited state: {:?}\n", state); state},
        Err(err) => panic!("Failed with error: {}", err),
    };
    
    let company: Company = Company::default();
    match create_company(&mut state, company) {
        Ok(()) => println!("created company"),
        Err(err) => panic!("Failed to create company with error: {}", err),
    }
   
    println!("Hello World!");
    println!("state: {:?}\n", state);
}


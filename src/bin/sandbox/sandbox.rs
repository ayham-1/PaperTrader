use libtrader::initializer::libtrader_init;
use libtrader::db::cmd::create_company::create_company;
use libtrader::db::cmd::get_company::get_company_from_db;
use libtrader::db::cmd::create_stock::create_stock;
use libtrader::ds::generic::company::Company;
use libtrader::ds::server::global_state::GlobalState;

fn main() {
    let mut state: GlobalState = match libtrader_init() {
        Ok(state) => {println!("inited state: {:?}\n", state); state},
        Err(err) => panic!("Failed with error: {}", err),
    };
    
    let mut company: Company = Company::default();
    company.id = 1;
    company.symbol = "TEST".to_string();
    company.isin = "TEST".to_string();
    company.company_name = "TEST".to_string();
    company.primary_exchange = "TEST".to_string();
    match create_company(&mut state, company) {
        Ok(()) => println!("created company"),
        Err(err) => panic!("Failed to create company with error: {}", err),
    }

    match get_company_from_db(&mut state, "TEST".to_string()) {
        Ok(found_company) => println!("we found it! {:?}", found_company),
        Err(err) => panic!("we must found the sacred company! err: {}", err),
    }

    match create_stock(&mut state, "test".to_string()) {
        Ok(()) => println!("created stock table"),
        Err(err) => panic!("failed to create stock table {}", err),
    }
   
    println!("state: {:?}\n", state);
}


use libtrader::initializer::libtrader_init;

fn main() {
    match libtrader_init() {
        Ok(state) => println!("inited state: {:?}", state),
        Err(err) => println!("Failed with error: {}", err),
    }
   
    println!("Hello World!");
}


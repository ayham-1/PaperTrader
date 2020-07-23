use libtrader::initializer::libtrader_init;

fn main() {
    match libtrader_init() {
        Ok(()) => println!("connected successfully"),
        Err(err) => println!("Failed with error: {}", err),
    }
   
    println!("Hello World!");
}


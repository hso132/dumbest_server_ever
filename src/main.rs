extern crate rust_server;

use rust_server::server;
fn main() 
{
    let args: Vec<_> = std::env::args().collect();
    if args.len()!=2
    {
        println!("Usage: {} <port>", args[0]);
        return;
    }
    match args[1].parse::<u32>()
    {
        Ok(port) => server::launch(port).unwrap(),
        Err(_) => println!("Couldn't parse second argument (integer needed)")
    }
}

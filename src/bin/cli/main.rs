use hexa::{config, domain, ports};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = config::init_repo();

    loop {
        print!("Enter stock ID: ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Read the line");
        let id: usize = buf.trim().parse().expect("Expected a number");

        let req = domain::StockReq { id, symbol: None };
        let res = ports::get_stock(req, &db);
        println!("{:#?}\n", res);
    }
}

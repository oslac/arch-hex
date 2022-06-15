use hexa::domain;
use hexa::services::db;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::mk_lite();

    loop {
        print!("Enter stock ID: ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Read the line");
        let id: usize = buf.trim().parse().expect("Expected a number");

        let req = domain::StockReq { id, symbol: None };
        let res = domain::get_stock(req, &db);
        println!("{:#?}\n", res);
    }
}

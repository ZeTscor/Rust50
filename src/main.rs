use std::error::Error;
use std::io::Read;


fn main() {
    if let Err(e) = get_request("http://httpbin.org/get"){
        eprintln!("{}",e);
        }
}

fn get_request(path: &str)-> Result<(), Box<dyn Error>>{
    let mut res = reqwest::blocking::get(path)?;
    let mut body = String::new();
    res.read_to_string(&mut body);
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body: \n{}", body);

    Ok(())
}

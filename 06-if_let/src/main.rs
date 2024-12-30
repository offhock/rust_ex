fn main() {
    let aaa = Some(3u8);
    match aaa {
        Some(3) => println!("three"),
        _ => (),
    }

    let bbb = Some(3u8);
    if let Some(3) = bbb { println!("three");}
    
}

#[derive(Debug)]
enum IpAddrKind  {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrKind1  {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum IpAdd1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },        
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}   

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home1 = IpAddrKind1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddrKind1::V6(String::from("::1"));

    let home2 = IpAdd1::V4(127, 0, 0, 1);
    let loopback2 = IpAdd1::V6(String::from("::1"));


    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);

    println!("home1 is {:?}", home1);
    println!("loopback1 is {:?}", loopback1);

    println!("home2 is {:?}", home2);
    println!("loopback2 is {:?}", loopback2);    
  
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // 메소드 내용은 여기 정의할 수 있습니다.
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    enum Option<T> {
    Some(T),
    None,
    }


    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;    

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}




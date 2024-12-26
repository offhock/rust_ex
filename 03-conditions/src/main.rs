fn main() {
    println!("Hello, world!");
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let문 뒤에 if문을 사용한 정의
    let condition = true;
    let number = if condition {1} else {2};
    println!("The value of number is: {}", number);

    // for문을 이용한 elements 반복자
    println!("for문을 사용");
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    // range문 사용
    println!("range문 사용");
    for element in (10..=50).rev().step_by(10) {
        println!("The value is: {}", element);
    }

    // range 개수
    println!("The count (10..50) is: {}", (10..50).count());
    println!("The count (10..=50) is: {}", (10..=50).count());
}

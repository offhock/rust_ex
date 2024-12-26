fn main1() {

// 소유권 규칙
//      1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
//      2. 한번에 딱 하나의 오너만 존재할 수 있다.
//      3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).    

// 변수의 스코프
// String 타입 메모리와 할당
//      1. 런타임에 운영체제로부터 메모리가 요청되어야 한다.
//      2. String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다. 
{
    let mut s = String::from("hello"); // s는 여기서부터 유효합니다

    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

    println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
}   // 이 스코프는 끝났고, s는 더 이상 유효하지 않습니다. s는 여기에서 drop을 호출한다. 
    
// 변수와 데이터가 상호작용하는 방법: 복사(copy)
// 러스트는 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달수 있는 Copy 트레잇이라고 불리우는 
// 특별한 어노테이션(annotation)을 가지고 있습니다 
//    * u32와 같은 모든 정수형 타입들
//    * true와 false값을 갖는 부울린 타입 bool
//    * f64와 같은 모든 부동 소수점 타입들
//    * Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.
{
    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);
}
// 변수와 데이터가 상호작용하는 방법: 이동(move)
// String 선언이 메모리에 저장되는 형태
// stack 저장: s1의 string type이 갖는 ptr, len, capacity에 과련된 데이터
// heap  저장: "hello"문자열은 heap에 저장되고 stack에 저장된 ptr이 heap 메모리에 있는 'h'를 가리킴
// 그림참조: 4-4, 4-5 (https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html)
{
    let s1 = String::from("hello");
    let s2 = s1;                    // 러스트에서는 s1이 s2로 move된 것으로 동작하고
    // println!("{}, world!", s1);          // s1은 더이상 유효하지 않음 (그래서 에러가 발생함.)
    println!("{}, world!", s2);
}

{
    let s1 = String::from("hello");
    let s2 = s1.clone();            // 러스트에서는 s1이 s2로 copy하기 위해서는 clone을 사용하고
    println!("{}, world!", s1);             // s1은 여전히 유효함. 
    println!("{}, world!", s2);
}

}

// 소유권과 함수
// 함수에게 값을 넘기는 의미론(semantics)은 값을 변수에 대입하는 것과 유사합니다. 
// 함수에게 변수를 넘기는 것은 대입과 마찬가지로 이동하거나 복사될 것입니다.
fn main2() {
    
    let s: String = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);         // s의 값이 함수 안으로 이동했습니다...
                                            // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                         // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);             // x가 함수 안으로 이동했습니다만,
                                            // i32는 Copy가 되므로, x를 이후에 계속
                                            // 사용해도 됩니다.
    // println!("{}",s);   // 에러 발생
    println!("{}",x);   // 정상

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.


// 튜플을 이용하여 여러 값을 돌려받는 식
fn main() {
    let s1 = gives_ownership();     // gives_ownership은 반환값을 s1에게
    // 이동시킵니다.
    
    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.
    
    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
    // 이동되었고, 이 함수가 반환값을 s3으로도
    // 이동시켰습니다.
    println!("{}",s1);   // 정상
    // println!("{}",s2);   // 에러
    println!("{}",s3);   // 정상   

}
     
fn gives_ownership() -> String {   // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.
    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.
    some_string          // some_string이 반환되고, 호출한 쪽의
                        // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어왔습니다.
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
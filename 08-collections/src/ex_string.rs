pub fn collection_string() {
    new_string();
    update_string();
    concatenate_string();
    indexing_string();
    slicing_string();
    iter_string();
}

// 새로운 문자열 생성하기
fn new_string() {
    println!("새로운 문자열 생성하기");
    // 예제 8-11: 비어있는 새로운 String 생성하기
    let mut s = String::new();

    // 예제 8-12: to_string 메서드를 사용하여 문자열 리터럴로부터 String 생성하기
    let data = "initial contents";
    let s = data.to_string();
    // 이 메서드는 리터럴에서도 바로 작동합니다:
    let s = "initial contents".to_string();    

    // 예제 8-13: String::from 함수를 사용하여 문자열 리터럴로부터 String 생성하기
    let s = String::from("initial contents");

    // 예제 8-14: 문자열에 다양한 언어로 인사말 저장하기
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");    
}

// 문자열 업데이트하기
fn update_string() {
    println!("문자열 업데이트하기");
    // 예제 8-15: push_str 메서드를 사용하여 String에 문자열 슬라이스 추가하기
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    // 예제 8-16: 문자열 슬라이스를 String에 붙인 이후에 문자열 슬라이스를 사용하기
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");    

    // 예제 8-17: push를 사용하여 String 값에 한 글자 추가하기
    let mut s = String::from("lo");
    s.push('l');    

}

// 연산자나 format! 매크로를 이용한 접합
fn concatenate_string() {
    println!("연산자나 format! 매크로를 이용한 접합");
    // 예제 8-18: + 연산자를 사용하여 두 String 값을 하나의 새로운 String 값으로 조합하기
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기로 이동되어 더 이상 사용할 수 없음을 주의하세요
    println!("s3 is {s3}");    

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");    

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");    
    println!("s is {s}");    
}

// 문자열 내부의 인덱싱
fn indexing_string() {
    println!("문자열 내부의 인덱싱");    
    let s1 = String::from("hello");
    let h = &s1[..0];
    println!("h is {h}"); 
    let hello = "안녕하세요";
    let answer = &hello[..0];
    println!("answer is {answer}"); 
}

// 문자열 슬라이싱하기
fn  slicing_string() {
    println!("문자열 슬라이싱하기");
    let hello = "안녕하세요";
    let s = &hello[0..3]; // 0..4를 설정하면 다음 에러 발생
    //byte index 4 is not a char boundary; it is inside '녕' (bytes 3..6) of `안녕하세요`
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}

// 문자열에 대한 반복을 위한 메서드
fn iter_string() {
    println!("문자열에 대한 반복을 위한 메서드");
    for c in "안녕하세요".chars() {
        println!("{c}");
    }

    for b in "안녕하세요".bytes() {
        println!("{b}");
    }    
}
// https://doc.rust-kr.org/ch09-01-unrecoverable-errors-with-panic.html

// panic!으로 복구 불가능한 에러 처리하기
pub fn test_panic(flag: bool ) {
    println!("panic! test");
    // panic!("crash and burn");
    // 예제 9-1: panic!을 일으키는 벡터의 끝을 넘어서는 요소에 대한 접근 시도
    let v = vec![1, 2, 3];
    
    if flag == true {
        v[99];
    }    
}

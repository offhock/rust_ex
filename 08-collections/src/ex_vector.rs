pub fn collection_vector() 
{
    new_vector();
    update_vector();
    read_vector();
    iter_vector();
    diff_type_vector();    
}

// 새 벡터 만들기
fn new_vector() {
    println!("새 벡터 만들기");
    // 예제 8-1: i32 타입의 값을 가질 수 있는 비어있는 새 벡터 생성
    let v: Vec<i32> = Vec::new();
    // 예제 8-2: 값을 저장하고 있는 새로운 벡터 생성하기
    let v = vec![1, 2, 3];
}

// 벡터 업데이트하기
fn update_vector() {    
    println!("벡터 업데이트하기");
    // 예제 8-3: push 메서드를 사용하여 벡터에 값을 추가하기
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);        
}

// 벡터 요소 읽기
fn read_vector() {
    println!("벡터 요소 읽기");
    // 예제 8-4: 인덱스 문법 혹은 get 메서드를 사용하여 벡터 내의 아이템에 접근하기
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // 예제 8-5: 5개의 요소를 가진 벡터에 100 인덱스에 있는 요소에 접근하기
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // 실행시간 fatal error 메시지 발생: index out of bounds: the len is 5 but the index is 100
    let does_not_exist = v.get(100);

    // 예제 8-6: 아이템의 참조자를 가지고 있는 상태에서 벡터에 새로운 요소 추가 시도하기
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);      // error :  mutable borrow occurs here
    println!("The first element is: {first}");        
    println!("The first element is: {}", first);    
}

// 벡터 값에 대해 반복하기
fn iter_vector() {
    println!("벡터 값에 대해 반복하기");
    // 예제 8-7: for 루프로 벡터의 요소들에 대해 반복하여 각 요소를 출력하기
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }    

    // 예제 8-8: 벡터의 요소에 대한 가변 참조자로 반복하기
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }    
    for i in &v {
        println!("{i}");
    }        
}

// 열거형을 이용해 여러 타입 저장하기
fn diff_type_vector() {
    println!("열거형을 이용해 여러 타입 저장하기");
    // 예제 8-9: 열거형을 정의하여 벡터 내에 다른 타입의 데이터를 담을 수 있도록 하기
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("The first element is: {:?}", row);    
}


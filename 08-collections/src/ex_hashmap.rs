pub fn collection_hashmap() {
    new_hashmap();
    access_hashmap();
    ownership_hashmap();
    overwrite_hashmap();
    append_key_hashmap();
    update_hashmap();
}

// 새로운 해시맵 생성하기
fn new_hashmap(){
    println!("새로운 해시맵 생성하기");
    // 예제 8-20: 새로운 해시맵을 생성하여 몇 개의 키와 값을 집어넣기    
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);    
}

// 해시맵의 값 접근하기
fn access_hashmap() {
    println!("해시맵의 값 접근하기");
    // 예제 8-21: 해시맵 내에 저장된 블루 팀의 점수 접근하기
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

// 해시맵과 소유권
fn ownership_hashmap(){
    println!("해시맵과 소유권");

    // 예제 8-22: 키와 값이 삽입되는 순간 이들이 해시맵의 소유가 되는 것을 보여주는 예
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점부터 유효하지 않습니다.
    // 사용을 시도해보고 무슨 컴파일러 에러가 발생하는 알아보세요!   
    // println!("{field_name}: {field_value}");
    println!("{:?}",map);
}

// 값을 덮어쓰기
fn overwrite_hashmap() {
    println!("해시맵 값을 덮어쓰기");
    // 예제 8-23: 특정한 키로 저장된 값을 덮어쓰기
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);    
}

// 키가 없을 때만 키와 값 추가하기
fn append_key_hashmap() {
    println!("키가 없을 때만 키와 값 추가하기");
    
    // 예제 8-24: entry 메서드를 이용하여 어떤 키가 값을 이미 갖고 있지 않을 경우에만 추가하기
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Blue는 이미 키 값이 정의되었으므로 삽입하지 않음

    println!("{:?}", scores);
}

// 예전 값에 기초하여 값을 업데이트하기
fn update_hashmap() {
    println!("예전 값에 기초하여 값을 업데이트하기");
    // 예제 8-25: 단어와 횟수를 저장하는 해시맵을 사용하여 단어의 등장 횟수 세기
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 초기값은 0으로 설정정
        *count += 1;
    }

    println!("{:?}", map);    
}
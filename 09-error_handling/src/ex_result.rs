use std::{fs::File, io::Write};

// Result로 복구 가능한 에러 처리하기
pub fn test_error_handling(){
    // ex9_4_test_result();
    ex9_5_result();
    ex9_5_result_using_unwrap_or_else();
    // ex9_result_unwrap();
    // ex9_result_expect();
}

// 에러 발생시 panic 발생하기
fn ex9_4_result() {
    // 예제 9-4: match 표현식을 사용하여 반환 가능한 Result 배리언트들을 처리하기
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// 서로 다른 에러에 대해 매칭하기
fn ex9_5_result() {
    // 예제 9-5: 다른 종류의 에러를 다른 방식으로 처리하기
    use std::fs::File;
    use std::io::ErrorKind;    
    
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// Result<T, E>와 match 사용에 대한 대안
fn ex9_5_test_result_using_unwrap_or_else() {
    use std::fs::File;
    use std::io::ErrorKind;
    
    let greeting_file = File::open("hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// 에러 발생시 패닉을 위한 숏컷: unwrap과 expect
fn ex9_result_unwrap() {
    use std::fs::File;
    let greeting_file = File::open("hell2.txt").unwrap(); // error 발생
}
// 에러 발생시, 사용자 메시지 출력과 프로그램 정지: expect 사용
fn ex9_result_expect() {
    use std::fs::File;
    let greeting_file = File::open("hello2.txt")
        .expect("hello2.txt should be included in this project");
}

// 예제 9-6: match를 이용하여 호출 코드 쪽으로 에러를 반환하는 함수
fn ex_9_6_result_using_match_with_returned_code() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }    
}

// 에러를 전파하기 위한 숏컷: ?
fn ex9_7_result_using_question_mark_with_returned_code() {
    // 예제 9-7: ? 연산자를 이용하여 에러를 호출 코드 쪽으로 반환하는 함수
    // 예제 9-6과의 차이점:  '? 연산자'를 사용할 때의 에러 값들은 from 함수를 거친다.
    //   1. from 함수는 표준 라이브러리 내의 From 트레이트에 정의되어 있으며 
    //      어떤 값의 타입을 다른 타입으로 변환하는 데에 사용한다.
    //   2. ? 연산자가 from 함수를 호출하면, ? 연산자가 얻게 되는 에러를 
    //      ? 연산자가 사용된 현재 함수의 반환 타입에 정의된 에러 타입으로 변환한다.
    //   3. 어떤 함수가 다양한 종류의 에러로 인해 실패할 수 있지만, 
    //      모든 에러를 하나의 에러 타입으로 반환할 때 유용하다.
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;   // using ?
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}

fn ex9_8_result_using_question_mark() {
    // 예제 9-8: ? 연산자 뒤에 메서드 호출을 연결하기
    use std::fs::File;
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        // ? 연산자를 사용시 에러 처리를 간단하게 작성가능
        File::open("hello.txt")?.read_to_string(&mut username)?; 
    
        Ok(username)
    }    
}

fn ex9_9_result_using_question_mark() {
    // 예제 9-9: 파일을 열고, 읽는 대신 fs::read_to_string을 사용하기
    use std::fs;
    use std::io;
    
    // read_username_from_file의 리턴타입이 Result이므로 OK
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

// 예제 9-10: ()를 반환하는 main에서의 ? 사용 시도는 컴파일되지 않습니다
fn ex_9_10_result_with_diff_retun_type() {
    use std::fs::File;

    // read_username_from_file의 리턴타입이 () 이므로 에러 발생
    fn read_username_from_file() {
        let greeting_file = File::open("hello.txt")?;
    }    
}

// 예제 9-11: Option<T> 값에 대한 ? 연산자의 사용
fn ex_9_11_result_with_option() {
    // 예제 9-11: Option<T> 값에 대한 ? 연산자의 사용
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()        
    }
    // 이 함수는 문자가 있을 수도, 없을 수도 있기 때문에 Option<char>를 반환합니다. 
    // 이 코드는 text 문자열 슬라이스 인수를 가져와서 lines 메서드를 호출하는데, 
    // 이는 해당 문자열의 라인에 대한 반복자를 반환합니다. 첫 번째 줄의 검사가 필요하므로, 
    // 반복자의 next를 호출하여 첫 번째 값을 얻어옵니다.     
    // 여기서 ?를 사용하여 
    // 만일 text가 빈 문자열이라면 
    //     next 호출은 None을 반환하고, last_char_of_first_line의 실행을 멈추고 None을 반환합니다. 
    // 만약 text가 빈 문자열이 아니라면 
    //     next는 text의 첫 번째 줄의 문자열 슬라이스를 담고 있는 Some의 값을 반환합니다.

    // ?이 문자열 슬라이스를 추출하고, 
    // 이 문자열 슬라이스의 chars를 호출하여 문자들에 대한 반복자를 얻어올 수 있습니다. 
    // 이 첫 번째 라인의 마지막 문자에 관심이 있으므로, last를 호출하여 이 반복자의 마지막 아이템을 얻어옵니다. 
    // "\nhi"처럼 빈 줄로 시작하지만 다른 줄에는 문자가 담겨있는 경우처럼, 
    // 첫 번째 라인이 빈 문자열일 수 있으므로 반복자의 결과는 Option입니다. 
    // 만약 첫 번째 라인에 마지막 문자가 있다면 Some 배리언트를 반환할 것입니다. 
    // 가운데의 ? 연산자가 이러한 로직을 표현할 간단한 방식을 제공하여 이 함수를 한 줄로 작성할 수 있도록 해 줍니다. 
    // 만일 Option에 대하여 ? 연산자를 이용할 수 없었다면 
    // 더 많은 메서드 호출 혹은 match 표현식을 사용하여 이 로직을 구현했어야 할 것입니다.
}

// 예제 9-12: main이 Result<(), E>를 반환하도록 하여 Result 값에 대한 ? 사용 가능하게 하기
fn ex9_12_result(){
    use std::error::Error;
    use std::fs::File;
    
    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
    
        Ok(())
    }    
}
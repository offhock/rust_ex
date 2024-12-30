mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use의 스코프는 해당 모듈만 적용 가능함
use front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();      // error: use의 스코프는 해당 모듈만 적용 가능
        super::hosting::add_to_waitlist();  // ok: super를 통해서 use를 선언한 스코프에서 호출

    }
}

pub fn eat_at_restaurant() {

    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다; 식사와 함께
    // 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;    

    // 예제 7-11: use 키워드로 모듈을 스코프 안으로 가져오기
    hosting::add_to_waitlist();
}

// // 표준 크레이트(crate) 사용
// use std::io;
// use std::io::Write;

// // 위와 동일한 사용
use std::io::{self, Write};

// // glob 연산자 사용  ( * 사용)
use std::collections::*;


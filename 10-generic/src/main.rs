
// 예제 10-1: 숫자 리스트에서 가장 큰 수 찾기
fn ex10_1() {
    println!("ex10_1 숫자 리스트에서 가장 큰 수 찾기");
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

fn ex10_2() {
    println!("ex10_2 두 개의 숫자 리스트에서 가장 큰 숫자를 찾는 코드");
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    
}

fn ex10_3_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn ex10_3() {
    println!("ex10_3 두 리스트에서 가장 큰 수를 찾는 추상화된 코드");
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = ex10_3_largest(&number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = ex10_3_largest(&number_list);
    println!("The largest number is {}", largest);
    
}


fn ex10_4_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn ex10_4_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn ex10_4()
{
    println!("ex10_4 이름과 타입 시그니처만 다른 두 함수");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = ex10_4_largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = ex10_4_largest_char(&char_list);
    println!("The largest char is {}", result);    
}

fn ex10_5_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn ex10_5()
{
    println!("ex10_5 제네릭 타입 매개변수를 이용한 largest 함수; 아직 컴파일되지는 않습니다");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = ex10_5_largest::<i32>(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = ex10_5_largest::<char>(&char_list);
    println!("The largest char is {}", result);    
}

struct Point<T> {
    x: T,
    y: T,
}
fn ex10_6()
{
    println!("ex10_6 T 타입의 값 x, y를 갖는 Point<T> 구조체");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

fn ex10_7()
{
    println!("ex10_7 x와 y 필드는 둘 다 동일한 제네릭 데이터 타입 T이므로 같은 타입이어야 합니다");
    // let wont_work  = Point { x: 5.0, y: 4 }; // 에러 발생
}

struct Point_d<T,U> {
    x: T,
    y: U,
}
fn ex10_8()
{
    println!("ex10_8 두 타입의 제네릭을 사용하여, x와 y가 서로 다른 타입의 값이 될 수 있는 Point<T, U>");
    let wont_work  = Point_d { x: 5.0, y: 4 };
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn ex10_9()
{
    println!("ex10_9 T 타입의 x 필드에 대한 참조자를 반환하는 x 메서드를 Point<T>에 정의");
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()        
    }
}

fn ex10_10() {
    println!("ex10_10 구조체의 제네릭 타입 매개변수 T가 특정 구체적 타입인 경우에만 적용되는 impl 블록 ");
    let p = Point { x: 5.1, y: 10.0 };
    println!("p.x = {}", p.x());
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}

struct Point_ex10_11<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> Point_ex10_11<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point_ex10_11<X2, Y2>) -> Point_ex10_11<X1, Y2> {
        Point_ex10_11 {
            x: self.x,
            y: other.y,
        }
    }
}
fn ex10_11() {
    println!("ex10_11 구조체 정의와 다른 제네릭 타입을 사용하는 메서드");
    let p1 = Point_ex10_11 { x: 5, y: 10.4 };
    let p2 = Point_ex10_11 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn main() {
    ex10_1();
    ex10_2();
    ex10_3();
    ex10_4();
    ex10_5();
    ex10_6();
    ex10_7();
    ex10_8();
    ex10_9();
    ex10_10();
    ex10_11();
}

mod ex_panic;
mod ex_result;

fn main() {
    ex_panic::test_panic(false);
    ex_result::test_error_handling();
}

// panic!이냐, panic!이 아니냐, 그것이 문제로다
// https://doc.rust-kr.org/ch09-03-to-panic-or-not-to-panic.html#panic%EC%9D%B4%EB%83%90-panic%EC%9D%B4-%EC%95%84%EB%8B%88%EB%83%90-%EA%B7%B8%EA%B2%83%EC%9D%B4-%EB%AC%B8%EC%A0%9C%EB%A1%9C%EB%8B%A4

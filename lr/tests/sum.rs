use lb::movies::sum;
mod commons;
use commons::{setup,teardown};

#[test]
fn sum_test1() {
    assert_eq!(sum(2,2),4);
}

/**
加上下面的参数可以显示print宏
cargo test sum_1_test -- --nocapture
*/
#[test]
fn sum_1_test() {
    setup();
    assert_eq!(sum(2,2),4);
    teardown();
}
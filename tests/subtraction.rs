use rand::Rng;

/// 更相减损术
#[test]
fn subtraction() {
    println!("更相减损术");
    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let x: u64 = rng.gen();
        let y: u64 = rng.gen();
        let _divisor = greatest_common_divisor::subtraction(x, y);
        // println!("x： {} , y： {} , 最大公约数： {}", x, y, _divisor);
    }
}

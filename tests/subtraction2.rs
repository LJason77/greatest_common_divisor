use rand::Rng;

/// 更相减损术 除2
#[test]
fn subtraction2() {
    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let x: u64 = rng.gen();
        let y: u64 = rng.gen();
        let _divisor = greatest_common_divisor::subtraction2(x, y);
        // println!("x： {} , y： {} , 最大公约数： {}", x, y, _divisor);
    }
}

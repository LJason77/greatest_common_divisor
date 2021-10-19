use rand::Rng;

/// Stein 算法
#[test]
fn stein() {
    println!("Stein 算法");
    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let x: u128 = rng.gen();
        let y: u128 = rng.gen();
        let _divisor = greatest_common_divisor::stein(x, y);
        // println!("x： {} , y： {} , 最大公约数： {}", x, y, _divisor);
    }
}

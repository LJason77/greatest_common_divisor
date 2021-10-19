use rand::Rng;

/// 辗转相除法
#[test]
fn euclidean() {
    println!("辗转相除法");
    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let x: u64 = rng.gen();
        let y: u64 = rng.gen();
        let _divisor = greatest_common_divisor::euclidean(x, y);
        // println!("x： {} , y： {} , 最大公约数： {}", x, y, _divisor);
    }
}

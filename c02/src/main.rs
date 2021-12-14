use std::time::Instant;

fn main() {
    println!("Hello, world!");
    q02_2(10);
    q02_2(100);
    //q02_2(1000);

    println!("{}", q02_3(100));
    println!("{}", q02_3(59));
    println!("{}", q02_3(998244353));
}

#[allow(unused_variables)]
fn q02_2(n: i32) {
    // O(N^3) の計算量なはず
    let start = Instant::now();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let x = 1;
            }
        }
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
}

fn q02_3(n: i32) -> bool {
    // p^2 を 2~Nまで回すため, p は 2~root(N)まで
    if n <= 1 {
        return false
    }
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            return false
        }
        p += 1;
    }
    return true
}

use std::time::Instant;
use itertools::Itertools;

fn main() {
    println!("question 4.1");
    let istart = Instant::now();
    println!("10 -> {}", q04_1(10));
    println!("{:?}", istart.elapsed());

    let istart = Instant::now();
    println!("20 -> {}", q04_1(20));
    println!("{:?}", istart.elapsed());

    println!("question 4.2.cache");
    let istart = Instant::now();
    let mut cache: Vec<u32> = Vec::new();
    println!("10 -> {}", q04_2_cached(10, &mut cache));
    println!("{:?}", istart.elapsed());

    let istart = Instant::now();
    println!("20 -> {}", q04_2_cached(20, &mut cache));
    println!("{:?}", istart.elapsed());

    println!("\nquestion 4.5");
    println!("{}", q04_5(375));
    println!("{}", q04_5(1000));
    println!("{}", q04_5(10000));

}

fn q04_1(n: u32) -> u32 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    }
    return q04_1(n - 1) + q04_1(n - 2) + q04_1(n - 3)
}

fn q04_2_cached(n: u32, cache: &mut Vec<u32>) -> u32 {
    if cache.len() >= n as usize {
        return cache[n as usize]
    }
    if n == 0 {
        cache.push(0);
        return 0
    } else if n == 1 {
        cache.push(0);
        return 0
    } else if n == 2 {
        cache.push(1);
        return 1
    }
    let x = q04_1(n - 1) + q04_1(n - 2) + q04_1(n - 3);
    cache.push(x);
    return x
}

fn q04_5(k: u32) -> u32 {
    if k < 357 {
        return 0
    }
    // kの桁数
    let mut digit: u32 = 1;
    let mut x: u32 = 10u32.pow(digit);
    while k >= x {
        digit += 1;
        x = 10u32.pow(digit);
    }
    let additional_digit = digit - 3;
    println!("k={}, d={}", k, digit);
    // k以下の753数のカウント
    let mut count = 0;

    // 4桁以上の時, 3, 5, 7から数字の組み合わせをとる
    for i in 0..=additional_digit {
        for j in [3, 5, 7].iter().combinations_with_replacement(i as usize) {
            // 順列を作る数字の選び元を作る
            let mut numbers_to_pick: Vec<u32> = vec![3, 5, 7];
            for y in j.into_iter() {
                numbers_to_pick.push(*y);
            }
            println!("{:?}", numbers_to_pick);

            // 数字を並び変えて, 数値をつくる
            for y in numbers_to_pick.iter().permutations(3 + i as usize) {
                let mut str_number = String::new();
                for z in y.into_iter() {
                    str_number.push(std::char::from_digit(*z, 10).unwrap());
                }
                let number: u32 = str_number.parse().unwrap();
                if number < k {
                    count += 1;
                }
            }

        }

    }

    return count
}

fn q04_6(i: usize, w: u32, a: &[u32]) -> bool {
    if i == 0 {
        if w == 0 {
            return true
        } else {
            return false
        }
    }
    if q04_6(i - 1, w, a) {
        return true
    }
    if q04_6(i - 1, w - a[i as usize - 1], a) {
        return true
    }
    return false
}

fn main() {
    let a = vec![1, 2, 3, 4, -1, 3];
    assert_eq!(2, q03_2(&a, 3));

    println!("question 3-3");
    let b = vec![100, 1000, 9999_9998, 55555, 9998, 10101010, 90];
    println!("{:?}", q03_3(&a));
    println!("{:?}", q03_3(&b));

    println!("question 3-4");
    println!("{}", q03_4(&a));
    println!("{}", q03_4(&b));

    println!("question 3-5");
    let a = vec![4, 12, 100, 32, 124];
    println!("{}", q03_5(&a));
    let b = vec![32, 96, 160, 64, 32 * 17];
    println!("{}", q03_5(&b));

    println!("question 3-5 with bit operation");
    let a: Vec<u32> = vec![4, 12, 100, 32, 99];
    println!("{}", q03_5b(&a));
    let a: Vec<u32> = vec![4, 12, 100, 32, 124];
    println!("{}", q03_5b(&a));
    let a: Vec<u32> = vec![32, 96, 160, 64, 32 * 17];
    println!("{}", q03_5b(&a));

    println!("question 3-6");
    println!("{}", q03_6(2, 6));
    println!("{}", q03_6(2, 7));
    println!("{}", q03_6(50, 100));

    println!("question 3-7");
    println!("{}", q03_7("1000"));
    println!("{}", q03_7("99999999"));
}

fn q03_2(a: &Vec<i32>, v: i32) -> i32 {
    let mut count = 0;
    for x in a.iter() {
        if x == &v {
            count += 1;
        }
    }
    return count
}

fn q03_3(a: &Vec<i32>) -> (i32, i32) {
    let mut first_smallest = 9999_9998;
    let mut second_smallest = 9999_9999;
    for x in a.iter() {
        if x < &first_smallest {
            second_smallest = first_smallest;
            first_smallest = *x;
        } else if x < &second_smallest {
            second_smallest = *x;
        }
    }
    return (first_smallest, second_smallest)
}

fn q03_4(a: &Vec<i32>) -> i32 {
    let mut smallest = 9999_9999;
    let mut largest = -9999_9999;
    for x in a.iter() {
        if x < &smallest {
            smallest = *x;
        } else if x > &largest {
            largest = *x;
        }
    }
    return largest - smallest
}

fn q03_5(a: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut b: Vec<i32> = a.to_vec();
    while b.iter().all(|&x| x % 2 == 0) {
        count += 1;
        b = b.iter().map(|x| x / 2).collect();
    }
    return count
}

fn q03_5b(a: &Vec<u32>) -> u32 {
    // どれかが0
    if a.iter().any(|&x| x == 0) {
        return 0
    }
    // 2^0 が1 -> 奇数
    let b: Vec<u32> = a.iter().map(|x| x & 0b0001u32).collect();
    println!("{:?}", b);
    if b.iter().any(|&x| x == 1) {
        return 0
    }

    // すべての数の論理和ビット
    let mut bit_or: u32 = 0;
    for x in a.iter() {
        bit_or = bit_or | x;
    }
    println!("{:b}", bit_or);

    // ビットが0だったら, より大きい桁があり, 多く2で割れる
    // ビットが1だったら, そこまででしか割れない
    let mut count = 1;
    let mut shifted_right: u32 = bit_or >> 1;
    while shifted_right & 0b0001 == 0 {
        count += 1;
        println!("count: {}, num: {:b}", count, shifted_right);
        shifted_right = shifted_right >> 1;
    }
    return count
}

fn q03_6(k: u32, n: u32) -> u32 {
    if k * 3 < n {
        return 0
    }
    let mut count = 0;
    for x in 0..k+1 {
        for y in 0..k+1 {
            let z = n - x - y;
            if z <= k {
                count += 1;
            }
        }
    }
    return count
}

fn q03_7(str_number: &str) -> u32 {
    let numbers: Vec<char> = str_number.chars().collect();
    let n = numbers.len() - 1;
    let mut is_plus: u32 = 0;
    let mut sum: u32 = 0;
    while is_plus < (1 << n) {
        let mut start_pos = 0;
        for i in 0..n {
            let current_digit = 1 << i;
            if (is_plus & current_digit) == current_digit {
                let temp: String = numbers[start_pos..i+1].iter().collect();
                let num: u32 = temp.parse().unwrap();
                sum += num;
                println!("is_plus={:0b}, start={}, i={}, temp={}, num={}, sum={}",
                         is_plus, start_pos, i, temp, num, sum);
                start_pos = i + 1;
            }
        }
        let temp: String = numbers[start_pos..n+1].iter().collect();
        let num: u32 = temp.parse().unwrap();
        sum += num;
        println!("is_plus={:0b}, start={}, n={}, temp={}, num={}, sum={}",
                 is_plus, start_pos, n, temp, num, sum);
        is_plus += 1;
    }
    return sum
}

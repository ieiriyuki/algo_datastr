fn main() {

    let h: Vec<f64> = vec![0.1, 1.1, 2.0, 1.5, 0.5, 0.4, 1.6];
    println!("{}", code5_1(&h));
    println!("{}", 1.0 + 0.4 + 1.0 + 1.1);
    let h: Vec<f64> = vec![2., 9., 4., 5., 1., 6., 10.];
    println!("{}", code5_1(&h));

    println!("{}", &code5_8("hello".to_string(), "world".to_string()));
    println!("{}", &code5_8("hello".to_string(), "hello".to_string()));
    println!("{}", &code5_8("atgc".to_string(), "tgct".to_string()));

    println!("question 5-1");
    let ans: i32 = q05_1(2, &[10, 20, 30], &[40, 50, 60], &[70, 1000, 90]);
    println!("{}", ans);

    let ans: i32 = q05_1(1, &[100], &[10], &[1]);
    println!("{}", ans);

    let ans: i32 = q05_1(
        7,
        &[6, 8, 2, 7, 4, 2, 7],
        &[7, 8, 5, 8, 6, 3, 5],
        &[8, 3, 2, 6, 8, 4, 1],
    );
    println!("{}", ans);

    println!("question 5-2");
    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}", q05_2(10, &a));
    let a: Vec<i32> = vec![3, 3, 3, 3, 3];
    println!("{}", q05_2(11, &a));
    let a: Vec<i32> = vec![1, 2, 3, 5, 8, 13];
    println!("{}", q05_2(11, &a));
    println!("{}", q05_2(31, &a));

    println!("question 5-3");
    let a: Vec<i32> = vec![2, 3, 5];
    assert_eq!(3, q05_3(5, &a));
    let a: Vec<i32> = vec![2, 4, 5];
    assert_eq!(3, q05_3(5, &a));
    let a: Vec<i32> = vec![2, 4, 5];
    assert_eq!(4, q05_3(6, &a));
}

fn code5_1(h: &Vec<f64>) -> f64 {
    let inf: f64 = 2f64.powf(30.);
    let n: usize = h.len();
    let mut dp: Vec<f64> = vec![inf; n];
    dp[0] = 0.;
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs();
        } else {
            dp[i] = (
                dp[i - 1] + (h[i] - h[i - 1]).abs()
            ).min(
                dp[i - 2] + (h[i] - h[i - 2]).abs()
            );
        }
    }
    return dp[n - 1]
}

fn smaller(a: &i32, b: &i32) -> i32{
    if *a > *b {
        return *b
    } else {
        return *a
    }
}

fn code5_8(s: String, t: String) -> i32 {
    let _inf: i32 = 1 << 30;
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();
    let mut dp: Vec<Vec<i32>> = vec![vec![_inf; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;

    for i in 0..=s_vec.len() {
        for j in 0..=t_vec.len() {
            if i > 0 && j > 0 {
                if s_vec[i - 1] == t_vec[j -1] {
                    dp[i][j] = smaller(&dp[i][j], &dp[i - 1][j - 1]);
                }
                else {
                    dp[i][j] = smaller(&dp[i][j], &(dp[i - 1][j - 1] + 1));
                }
            }
            if i > 0 {
                dp[i][j] = smaller(&dp[i][j], &(dp[i - 1][j] + 1));
            }
            if j > 0 {
                dp[i][j] = smaller(&dp[i][j], &(dp[i][j - 1] + 1));
            }
        }
    }
    println!("{:?}", dp);
    return dp[s_vec.len()][t_vec.len()]
}

fn q05_1(n: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32{
    /* this code is imperfect */
    let mut gain: Vec<Vec<i32>> = vec![vec![0; 3]; n];
    if a.len() > b.len() && c.len() < 100 { () }
    gain[0][0] = 0;
    for i in 0..n {
        for j in 0..3 {
            if i > j { () }
        }
    }
    1
}

fn q05_2(w: i32, a: &Vec<i32>) -> bool {
    let w_usize = (w + 1) as usize;
    let mut dp: Vec<Vec<bool>> = vec![vec![false; w_usize]; a.len()];

    dp[0][0] = true;  // nothing is added
    for j in 1..w_usize {
        if j == a[0] as usize {
            dp[0][j] = true;
        } else {
            dp[0][j] = false;
        }
    }
    for i in 1..a.len() {
        dp[i][0] = true;  // still nothing is added
        for j in 1..w_usize {
            if a[i] < j as i32 {
                if dp[i - 1][j - a[i] as usize] {
                    dp[i][j] = true;  // addition makes j
                } else {
                    dp[i][j] = false;
                }
            } else if a[i] == j as i32 {
                dp[i][j] = true;
            } else {
                if dp[i - 1][j] {
                    dp[i][j] = true;  // already j
                } else {
                    dp[i][j] = false;
                }
            }
        }
    }
    dp[a.len() - 1][w_usize - 1]
}

// incomplete
fn q05_3(w: i32, a: &Vec<i32>) -> i32 {
    let w_usize = w as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; w_usize]; a.len()];

    for j in 1..=w_usize {
        if a[0] as usize <= j {
            dp[0][j - 1] += 1;
        }
    }
    for i in 1..a.len() {
        for j in 1..=w_usize {
            // a[i]を足さない場合
            dp[i][j - 1] += dp[i - 1][j - 1];

            // a[i]を足す場合
            if a[i] as usize <= j {
                dp[i][j - 1] += 1;
                if a[i] as usize + j <= w_usize {
                    dp[i][j - 1] += 1;
                }
            }
        }
    }
    println!("{:?}", dp);
    dp[a.len() - 1][w_usize - 1]
}

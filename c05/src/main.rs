fn main() {
    println!("Hello, world!");

    let h: Vec<f64> = vec![0.1, 1.1, 2.0, 1.5, 0.5, 0.4, 1.6];
    println!("{}", code5_1(&h));
    println!("{}", 1.0 + 0.4 + 1.0 + 1.1);
    let h: Vec<f64> = vec![2., 9., 4., 5., 1., 6., 10.];
    println!("{}", code5_1(&h));

    println!("{}", &code5_8("hello".to_string(), "world".to_string()));
    println!("{}", &code5_8("hello".to_string(), "hello".to_string()));
    println!("{}", &code5_8("atgc".to_string(), "tgct".to_string()));
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

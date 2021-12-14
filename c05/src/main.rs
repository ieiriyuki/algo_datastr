fn main() {

    let h: Vec<f64> = vec![0.1, 1.1, 2.0, 1.5, 0.5, 0.4, 1.6];
    println!("{}", code5_1(&h));
    println!("{}", 1.0 + 0.4 + 1.0 + 1.1);
    let h: Vec<f64> = vec![2., 9., 4., 5., 1., 6., 10.];
    println!("{}", code5_1(&h));

    println!("{}", &code5_8("hello".to_string(), "world".to_string()));
    println!("{}", &code5_8("hello".to_string(), "hello".to_string()));
    println!("{}", &code5_8("atgc".to_string(), "tgct".to_string()));

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

fn q05_1(N: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32{
    let mut gain: Vec<Vec<i32>> = vec![vec![0; 3]; N];
    for i in 0..N {
        for j in 0..3 {
            gain_i[i][j] = temp;
            gain_i[i][1] = activity;
        }
        else {
            let mut temp = gain_i[i -1][0] + a[i];
            let mut activity = 0;
            if (temp < gain_i[i -1][0] + b[i])
                && (gain_i[i -1][1] != 1) {
                temp = gain_i[i -1][0] + b[i];
                activity = 1;
            }
            if (temp < gain_i[i -1][0] + c[i])
                && (gain_i[i -1][1] != 2) {
                temp = gain_i[i -1][0] + c[i];
                activity = 2;
            }
            gain_i[i][0] = temp;
            gain_i[i][1] = activity;
        }
    }
    println!("{:?}", gain_i);
    gain_i[N-1][0]
}

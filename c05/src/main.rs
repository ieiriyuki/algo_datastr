fn main() {
    println!("Hello, world!");

    let h: Vec<f64> = vec![0.1, 1.1, 2.0, 1.5, 0.5, 0.4, 1.6];
    println!("{}", code5_1(&h));
    println!("{}", 1.0 + 0.4 + 1.0 + 1.1);
    let h: Vec<f64> = vec![2., 9., 4., 5., 1., 6., 10.];
    println!("{}", code5_1(&h));
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

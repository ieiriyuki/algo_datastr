fn main() {
    println!("Question 01_3");
    q01_3();
    q01_5();
}

fn q01_3() {
    fn get_ten_plc(num: i32) -> i32 {
        if num > 9999 {
            panic!("We do not assume an integer greator than 9999: {}", num)
        }
        return num / 10 - 10 * (num / 100) - 100 * (num / 1000)
    }
    for a in 20..29 {
        for b in 11..99 {
            let one_plc = b % 10;
            let res_1 = a * one_plc;
            let ten_plc = get_ten_plc(res_1);
            if ten_plc != 3 {
                continue;
            }
            let total = a * b;
            let ten_plc = get_ten_plc(total);
            if ten_plc != 4 || total > 949 {
                continue;
            }
            println!("multip of one place is: {}, total is: {}",
                     res_1, total);
        }
    }
}

fn q01_5() {
    // 途中
    let ary = [0; 8];
    let route = [&ary; 8];
    for i in route.iter() {
        for j in i.iter() {
            print!("{}", j);
        }
        println!("");
    }
}

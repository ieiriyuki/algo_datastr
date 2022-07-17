use regex::Regex;

pub fn word2vec(r: usize, c: usize, _s: &str) -> Vec<Vec<char>> {
    let re = Regex::new(r" ").unwrap();
    let s: &str = &re.replace_all(_s, "");

    let re = Regex::new(r"\n").unwrap();
    let mut data: Vec<Vec<char>> = vec![vec!['c'; c]; r];
    for (row, s_r) in re.split(s).enumerate() {
        let mut col: usize = 0;
        for ch in s_r.chars() {
            data[row][col] = ch;
            col += 1;
        }
    }
    return data
}

pub fn show_field(data: &Vec<Vec<char>>) {
    for row in data.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
    for i in 1..=data.len() {
        for j in 1..=data[0].len() {
            print!("{}", j + (i - 1) * data.len());
        }
        println!();
    }
}

pub fn odds_evens(data: &Vec<Vec<char>>) -> Vec<(usize, usize, u32)> {
    let r: usize = data.len();
    let c: usize = data[0].len();

    let s: usize = 0;
    let t: usize = r * c + 1;

    let mut wedges: Vec<(usize, usize, u32)> = Vec::new();

    for i in 0..r {
        for j in 0..c {
            if data[i][j] == '*' { continue }
            wedges.append(&mut make_edge(i, j, &data));
        }
    }
    return wedges
}

fn make_edge(
    r: usize, c: usize, data: &Vec<Vec<char>>
) -> Vec<(usize, usize, u32)> {
    let r_max: usize = data.len();
    let c_max: usize = data[0].len();
    let directions: Vec<(i32, i32)> = vec![
        (0, 1), (1, 0), (0, -1), (-1, 0),
    ];
    let mut temp: Vec<(usize, usize, u32)> = Vec::new();

    let idx: usize = r * c_max + c + 1;
    if idx % 2 == 1 {
        temp.push((0, idx, 1));
    } else {
        temp.push((idx, r_max * c_max + 1, 1));
    }
    for (dr, dc) in directions.iter() {
        let _r = (r as i32) + dr;
        let _c = (c as i32) + dc;
        if _r < 0 || (r_max as i32) <= _r || _c < 0 || (c_max as i32) <= _c {
            continue
        }
        if data[_r as usize][_c as usize] == '*' { continue }
        let idx_n: usize = (_r as usize) * c_max + (_c as usize) + 1;
        temp.push((idx, idx_n, 1));
    }
    return temp
}

fn get_vec(file: &str) -> Vec<Vec<char>> {
    let col = file.lines().next().unwrap().len();
    let mut row = 0;
    for _ in file.lines() {
        row += 1;
    }

    //println!("{} x {}", row, col);

    let mut vec = vec![vec![' '; col]; row];

    let mut a: usize = 0;
    let mut b: usize = 0;

    for x in file.lines() {
        // println!("{}", y);
        for y in x.chars() {
            // println!("{}", x);
            vec[a][b] = y;
            b += 1;
        }
        a += 1;
        b = 0;
    }

    vec
}

fn get_neighbours_count(vec: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    //let x = 2;
    //let y = 0;
    //println!("[xy][{}{}]", x, y);
    let x_limit = vec[0].len() -1;
    let y_limit = vec.len() -1;

    let mut count = 0;
    let mut a: usize;
    let mut b: usize;

    let c = vec[y][x];
    if c == '.' {
        return 9;
    }

    fn check_char(vec: &Vec<Vec<char>>, a: usize, b: usize) -> i32 {
        //print!("ch: {} {} ", a, b);
        //println!("[{}]", vec[b][a]);
        if vec[b][a] == '@' {
            return 1;
        } else {
            return 0;
        }
    }

    // 1
    if x > 0 && y > 0 {
        a = x - 1;
        b = y - 1;
        count += check_char(vec, a, b);
    }

    // 2
    if y > 0 {
        a = x;
        b = y - 1;
        count += check_char(vec, a, b);
    }

    // 3
    if x < x_limit && y > 0 {
        a = x + 1;
        b = y - 1;
        count += check_char(vec, a, b);
    }

    // 4
    if x > 0 {
        a = x - 1;
        b = y;
        count += check_char(vec, a, b);
    }

    // 6
    if x < x_limit {
        a = x + 1;
        b = y;
        count += check_char(vec, a, b);
    }

    // 7
    if x > 0 && y < y_limit {
        a = x - 1;
        b = y + 1;
        count += check_char(vec, a, b);
    }

    // 8
    if y < y_limit {
        a = x;
        b = y + 1;
        count += check_char(vec, a, b);
    }

    // 9
    if x < x_limit && y < y_limit {
        a = x + 1;
        b = y + 1;
        count += check_char(vec, a, b);
    }

    //print!("[{}]", count);

    count
}

fn get_sum(vec: &Vec<Vec<char>>) -> i32 {
    for n in vec.iter() {
        for m in n.iter() {
            print!("{}", m);
        }
            println!("");
    }

    println!("{} x {}", vec.len(), vec[0].len());
    let mut nec = vec![vec!['.'; vec.len()]; vec[0].len()];

    let mut sum = 0;
    for a in 0..vec[0].len() {
        for b in 0..vec.len() {
            //println!("in gb: {} {}", a, b);
            let count = get_neighbours_count(&vec, a, b);
            nec[b][a] = char::from_digit(count as u32, 10).unwrap();
            if count < 4  {
                println!("sum+ {} {}", sum, count); 
                sum += 1;
                //nec[a][b] = char::from_digit(count as u32, 10).unwrap();
            } else {
                //nec[a][b] = '0';
            }
        }
    }

    for n in nec.iter() {
        for m in n.iter() {
            print!("{}", m);
        }
            println!("");
    }

    sum
}

fn run_it(get_sum: fn(&Vec<Vec<char>>) -> i32, vec: &Vec<Vec<char>>) -> (i32, f64) {
    use std::time::Instant;

    let time = Instant::now();
    let sum  = get_sum(&vec);
    let took = time.elapsed().as_secs_f64();
    (sum, took)
}

fn print_it(filename: &str, sum: i32, took: f64) {
    println!("{}   sum:{:>7}   took:{:>5.2}s", filename, sum, took);
}

fn main() -> std::io::Result<()> {
    let filename_1st = "1st.txt";
    let filename_2nd = "2nd.txt";

    let file_1st = std::fs::read_to_string(filename_1st)?;
    let file_2nd = std::fs::read_to_string(filename_2nd)?;

    println!("--- PART I -----------------------------------------------");
    let vec = get_vec(&file_1st);

    let (sum, took) = run_it(get_sum, &vec);
    print_it(filename_1st, sum, took);

    let vec = get_vec(&file_2nd);

    let (sum, took) = run_it(get_sum, &vec);
    print_it(filename_2nd, sum, took);

    //println!("----PART II ----------------------------------------------");
    //let (sum, took) = run_it(get_sum, &file_1st, cells);
    //print_it(filename_1st, sum, cells, took);

    //let (sum, took) = run_it(get_sum, &file_2nd, cells);
    //print_it(filename_2nd, sum, cells, took);
    //println!("----------------------------------------------------------");

    Ok(())
}


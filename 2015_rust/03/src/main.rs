fn read_file(filename: &str) -> std::io::Result<String> {
    Ok(std::fs::read_to_string(filename)?.trim().to_string())
}

fn step(pos: &mut (i64, i64), c: char) {
    let (row, col) = pos;
    match c {
        '^' => *row -= 1,
        'v' => *row += 1,
        '>' => *col += 1,
        '<' => *col -= 1,
        other => panic!("unexpected char: {other:?}"),
    }

}

fn calc_santa(file: &str) -> usize {
    use std::collections::HashMap;

    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos = (0, 0);

    houses.insert(pos, 1);

    for c in file.chars() {
        step(&mut pos, c);
        *houses.entry(pos).or_insert(0) += 1;
    }
    // println!("{:?}", houses);
    houses.len()
}

fn calc_santa_robot(file: &str) -> usize {
    use std::collections::HashMap;

    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();

    let mut pos_santa = (0, 0);
    let mut pos_robot = (0, 0);

    houses.insert((0, 0), 2);

    let mut santa: bool = true;
    for c in file.chars() {
        let mut pos = if santa { &mut pos_santa } else { &mut pos_robot };

        step(&mut pos, c);
        *houses.entry(*pos).or_insert(0) += 1;

        santa = !santa;

    }
    // println!("{:?}", houses);
    houses.len()
}

fn print_count(filename: &str, count: usize, took: f64) {
    println!("file: {}    count: {:>4}    took: {:.2}s", filename, count, took);
}

fn main() {
    use std::time::Instant;

    println!("--- PART I -----------------------------------------------");
    let filename = "data1.txt";
    let file = read_file(filename).unwrap();
    let time = Instant::now();
    let count = calc_santa(&file);
    let took = time.elapsed().as_secs_f64();
    print_count(filename, count, took);

    let filename = "data2.txt";
    let file = read_file(filename).unwrap();
    let time = Instant::now();
    let count = calc_santa(&file);
    let took = time.elapsed().as_secs_f64();
    print_count(filename, count, took);

    println!("--- PART II ----------------------------------------------");
    let filename = "data1.txt";
    let file = read_file(filename).unwrap();
    let time = Instant::now();
    let count = calc_santa_robot(&file);
    let took = time.elapsed().as_secs_f64();
    print_count(filename, count, took);

    let filename = "data2.txt";
    let file = read_file(filename).unwrap();
    let time = Instant::now();
    let count = calc_santa_robot(&file);
    let took = time.elapsed().as_secs_f64();
    print_count(filename, count, took);

    println!("----------------------------------------------------------");
}

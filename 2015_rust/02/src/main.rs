fn get_calc(line: &str) -> (i64, i64, i64, i64) {
    let mut it = line.split('x');

    let l: i64 = it.next().expect("missing l").parse().expect("bad l");
    let w: i64 = it.next().expect("missing w").parse().expect("bad w");
    let h: i64 = it.next().expect("missing h").parse().expect("bad h");
    assert!(it.next().is_none(), "too many dimensions");
    
    // area
    let area = (2 * l * w) + (2 * l * h) + (2 * w * h);

    // extra_area
    let extra_area = (l * w).min(l * h).min(w * h);

    // volume
    let volume = l * w * h;

    // perimeter
    let perimeter = (2 * (l + w)).min(2 * (l + h)).min(2 * (w + h));

    (area, extra_area, volume, perimeter)
}

fn get_sum_area(file: &str) -> i64 {
    let mut sum = 0;
    for line in file.lines() {
        let (area, extra, _, _) = get_calc(line);
        sum += area + extra;
    }
    sum
}

fn get_sum_len(file: &str) -> i64 {
    let mut sum = 0;
    for line in file.lines() {
        let (_, _, volume, perimeter) = get_calc(line);
        sum += perimeter + volume;
    }
    sum
}

fn print_sum(filename: &str, sum: i64, took: f64) {
    println!("file: {}    sum: {:>7}    took: {:.2}s", filename, sum, took);
}


fn main() -> std::io::Result<()> {
    use std::time::Instant;

    println!("--- PART I -----------------------------------------------");
    let filename = "data1.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let sum = get_sum_area(file);
    let took = time.elapsed().as_secs_f64();
    print_sum(filename, sum, took);

    let filename = "data2.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let sum = get_sum_area(file);
    let took = time.elapsed().as_secs_f64();
    print_sum(filename, sum, took);

    println!("--- PART II ----------------------------------------------");
    let filename = "data1.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let sum = get_sum_len(file);
    let took = time.elapsed().as_secs_f64();
    print_sum(filename, sum, took);

    let filename = "data2.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let sum = get_sum_len(file);
    let took = time.elapsed().as_secs_f64();
    print_sum(filename, sum, took);

    println!("----------------------------------------------------------");

    Ok(())
}

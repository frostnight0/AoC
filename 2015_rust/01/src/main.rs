fn get_basement_pos(data: &str) -> usize {
    let mut floor: i64 = 0;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            other => panic!("unexpected char: {other:?}"),
        }
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("basement never reached");
}

fn get_floor(data: &str) -> i64 {
    let mut floor: i64 = 0;
    for c in data.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            other => panic!("unexpected char: {other:?}"),
        }
    }
    floor
}

fn print_floor(filename: &str, floor: i64, took: f64) {
    println!("file: {}    floor: {:>3}    took: {:.2}s", filename, floor, took);
}
fn print_pos(filename: &str, pos: usize, took: f64) {
    println!("file: {}    position: {:>4}    took: {:.2}s", filename, pos, took);
}

fn main() -> std::io::Result<()> {
    use std::time::Instant;

    println!("--- PART I -----------------------------------------------");
    let filename = "data1.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let floor = get_floor(&file);
    let took = time.elapsed().as_secs_f64();
    print_floor(filename, floor, took);

    let filename = "data2.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let floor = get_floor(&file);
    let took = time.elapsed().as_secs_f64();
    print_floor(filename, floor, took);

    println!("--- PART II ----------------------------------------------");
    let filename = "data1.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let pos = get_basement_pos(&file);
    let took = time.elapsed().as_secs_f64();
    print_pos(filename, pos, took);

    let filename = "data2.txt";
    let file = std::fs::read_to_string(filename)?;
    let file = file.trim();
    let time = Instant::now();
    let pos = get_basement_pos(&file);
    let took = time.elapsed().as_secs_f64();
    print_pos(filename, pos, took);

    println!("----------------------------------------------------------");

    Ok(())
}

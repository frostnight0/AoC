fn sum(file: &str, cells: usize) -> i64 {
    let mut sum: i64 = 0;
    for line in file.lines() {
        // println!("{line}");
        sum += largest_joltage(&line, cells);
    }
    // println!("{sum}");

    sum
}

fn largest_joltage(line: &str, cells: usize) -> i64 {
    // let cells: usize = 2;

    let mut vector = Vec::<i32>::new();
    let mut rez    = Vec::<i32>::new();

    for c in line.chars() {
        let i = c.to_string().parse::<i32>().unwrap();
        vector.push(i);
        // println!("{i}");
    }
    // println!("{:?}", vector);
    // println!("{:?}", vector.len());

    let window_size = vector.len() - cells + 1;
    // println!("ws: {:?}", window_size);

    for cell in 0..cells {
        // println!("{cell}");
        let slice_vector = vector[cell..(window_size+cell)].to_vec();
        // println!("{:?}", vector);
        // println!("{:?}", slice_vector);

        let mut lnum = 0;
        let mut lidx = 0;
        for idx in 0..(slice_vector.len()) {
            let val = slice_vector[idx];
            // println!("[{idx}] {val}");
            if lnum < val {
                lnum = val;
                lidx = idx;
                let summ = idx + cell;
                // println!("cell {cell} + idx {idx} = {summ}");
                for n in 0..(summ + 1) {
                    vector[n] = -1;
                }
            }
        }
        // println!("largest: [{lidx}]: {lnum}");
        rez.push(lnum);
    }
    // println!("rez: {:?}", rez);
    let mut s = "".to_string();
    for n in 0..rez.len() {
        s = format!("{}{}", s, rez[n].to_string());
    }
    let num = format!("{s}").parse::<i64>().unwrap();
    // println!("{num}");
    num
}
fn run(filename: &str, file: &str, cells: usize) {
    use std::time::Instant;

    let time = Instant::now();
    let sum = sum(&file, cells);
    println!("{}   sum:{:>17}   cells:{:>3}   took:{:>5.2}s", filename, sum, cells, time.elapsed().as_secs_f64());
}

fn main() -> std::io::Result<()> {
    let filename_1st = "1st.txt";
    let filename_2nd = "2nd.txt";

    let file_1st = std::fs::read_to_string(filename_1st)?;
    let file_2nd = std::fs::read_to_string(filename_2nd)?;

    println!("--- PART I -----------------------------------------------");
    run(&filename_1st, &file_1st, 2);
    run(&filename_2nd, &file_2nd, 2);
    println!("----PART II ----------------------------------------------");
    run(&filename_1st, &file_1st, 12);
    run(&filename_2nd, &file_2nd, 12);
    println!("----------------------------------------------------------");
    
    Ok(())
}


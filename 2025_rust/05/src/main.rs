fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn get_databoxes(file: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut id_ranges_str: Vec<String> = Vec::new();
    let mut ids_str: Vec<String> = Vec::new();

    let mut second_part = false;
    for line in file.lines() {
        if line == "" {
            second_part = true;
        }

        if second_part == false {
            id_ranges_str.push(line.to_string());
        } else {
            ids_str.push(line.to_string());
        }
    }
    ids_str.remove(0);

    let mut ids: Vec<i64> = Vec::new();

    for i in ids_str.iter() {
        let id: i64 = i.parse().unwrap();
        ids.push(id);
    }

    let mut id_ranges: Vec<(i64, i64)> = Vec::new();

    for range in id_ranges_str.iter() {
        let (start_str, end_str) = range.split_once("-").unwrap();

        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();

        id_ranges.push((start, end))
    }

    (id_ranges, ids)
}

fn is_fresh(id_range: (i64, i64), id: i64) -> bool {
    let (start, end) = id_range;
    let mut rez: bool = false;

    if id >= start && id <= end {
        rez = true;
    }

    rez
}

fn get_sum_of_fresh(id_ranges: Vec<(i64, i64)>, ids: Vec<i64>) -> i64 {
    let mut sum = 0;

    for id in ids.iter() {
        for id_range in id_ranges.iter() {
            if is_fresh(*id_range, *id) {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn get_sum_of_fresh_all(id_ranges: Vec<(i64, i64)>, ids: Vec<i64>) -> i64 {
    let mut ids_all: Vec<i64> = Vec::new();

    println!("{:?}", id_ranges);
    
    println!("len: {:?}", id_ranges.len());
    
    fn get_coliding(id_ranges: Vec<(i64, i64)>) -> usize {
        let mut coliding = 0;
        for r in 0..id_ranges.len() {
            // println!("-----------------------------------");
            // println!("r: {:?}", id_ranges[r]);
        
            let (r_start, r_end) = id_ranges[r];

            for s in r+1..id_ranges.len() {
                // println!("s: {:?}", id_ranges[s]);

                let (s_start, s_end) = id_ranges[s];
            
                if r_start <= s_end && r_end >= s_start {
                    // println!("coliding! {:?}", id_ranges[s]);
                    coliding += 1;
                }
            }
        }
        coliding
    }

    let coliding = get_coliding(id_ranges);

    println!("coliding: {}", coliding);

    for i in 0..10 {
        let mut good_ranges: Vec<(i64, i64)> = Vec::new();
        let mut bad_ranges: Vec<(i64, i64)> = Vec::new();



    }

    0
}

fn run_it(get_sum: fn(Vec<(i64, i64)>, Vec<i64>) -> i64, id_ranges: Vec<(i64, i64)>, ids: Vec<i64>) -> (i64, f64) {
    use std::time::Instant;

    let time = Instant::now();
    let sum  = get_sum(id_ranges, ids);
    let took = time.elapsed().as_secs_f64();
    (sum, took)
}

fn print_it(filename: &str, sum: i64, took: f64) {
    println!("{}   sum:{:>7}   took:{:>5.2}s", filename, sum, took);
}

fn main() -> std::io::Result<()> {
    let filename_1st = "1st.txt";
    let filename_2nd = "2nd.txt";

    let file_1st = std::fs::read_to_string(filename_1st)?;
    let file_2nd = std::fs::read_to_string(filename_2nd)?;

    println!("--- PART I -----------------------------------------------");
    let (id_ranges, ids) = get_databoxes(&file_1st);

    let (sum, took) = run_it(get_sum_of_fresh, id_ranges, ids);
    print_it(filename_1st, sum, took);

    let (id_ranges, ids) = get_databoxes(&file_2nd);

    let (sum, took) = run_it(get_sum_of_fresh, id_ranges, ids);
    print_it(filename_2nd, sum, took);

    println!("----PART II ----------------------------------------------");
    let (id_ranges, ids) = get_databoxes(&file_1st);

    let (sum, took) = run_it(get_sum_of_fresh_all, id_ranges, ids);
    print_it(filename_1st, sum, took);

    // let (id_ranges, ids) = get_databoxes(&file_2nd);

    // let (sum, took) = run_it(get_sum_of_fresh_all, id_ranges, ids);
    // print_it(filename_2nd, sum, took);

    println!("----------------------------------------------------------");

    Ok(())
}


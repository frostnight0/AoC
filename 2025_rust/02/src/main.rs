use std::time::Instant;

fn process_range(start: i64, end: i64, mode: u8) -> Vec<i64> {
    let mut vector = Vec::<i64>::new();

    let mut i = start;
    // println!("{i}");

    while i <= end {
        let s: String = i.to_string();
        // println!("{} [{}]", s, s.len());
        
        if mode == 1 {
            if comparator_1(&s) {
                vector.push(i);
            }
            
        } else if mode == 2 {
            if comparator_2(&s) {
                vector.push(i);
            }

        } else {
            panic!("Error: unknown mode!");
        }

        i += 1;
    }

    vector
}

fn id_sum(file: &str, mode: u8) -> i64 {
    let mut id_sum: i64 = 0;

    // println!("{file}");

    let ranges = file.split(",");

    for range in ranges {
        let range = range.trim();
        // println!("{range}");
    
        let (start, end) = range.split_once("-").unwrap();
        
        let start: i64 = start.parse().unwrap();
        let end: i64   = end.parse().unwrap();

        // println!("range ==> {start} : {end}");
        let id_list = process_range(start, end, mode);
        
        for id in id_list {
            // println!("{id}");
            id_sum += id;
        }
    }

    // println!("{id_sum}");
    id_sum
}

fn comparator_1(s: &str) -> bool {
    let split_point = s.len() / 2;
    let (part_1, part_2) = s.split_at(split_point);

    // println!("{} | {}", part_1, part_2);

    part_1 == part_2
}

fn comparator_2(s: &str) -> bool {
    let mut final_equel = false;
    // println!("s: {s}");

    let mut div = s.len() / 2;
    // println!("div: {div}");

    while div > 0 {
        if s.len() % div == 0 {
            // println!("you can divide by {}", div);

            let mut vector = Vec::<&str>::new();
            let mut part_1;
            let mut part_2 = s;
            for _ in 0..(part_2.len()/div) {
                (part_1, part_2) = part_2.split_at(div);
                // println!("{n}: {:?}", part_1);
                vector.push(part_1)
            }

            let equel = vector.iter().all(|element| element == &vector[0]);
            // println!("{:?} equel: {}", vector, equel);

            if equel {
                final_equel = true;
            }

        }
        div -= 1;
    }
    
    // println!("final_equel: {:?}", final_equel);
    final_equel
}

fn run(filename: &str, file: &str, mode: u8) {
    let time = Instant::now();
    let sum = id_sum(&file, mode);
    println!("{} {:>13}   took: {:>4.2}s", filename, sum, time.elapsed().as_secs_f64());
}

fn main() -> std::io::Result<()> {
    let filename_1st = "1st.txt";
    let filename_2nd = "2nd.txt";

    let file_1st = std::fs::read_to_string(filename_1st)?;
    let file_2nd = std::fs::read_to_string(filename_2nd)?;

    println!("--- PART I ----------------------------------------");
    run(&filename_1st, &file_1st, 1);
    run(&filename_2nd, &file_2nd, 1);
    println!("----PART II ---------------------------------------");
    run(&filename_1st, &file_1st, 2);
    run(&filename_2nd, &file_2nd, 2);
    println!("---------------------------------------------------");
    
    Ok(())
}

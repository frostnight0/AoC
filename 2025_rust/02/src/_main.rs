fn process_range(start: i32, end: i32) -> () {
    let mut i = start;
    // println!("{i}");

    while i <= end {
        let s: String = i.to_string();

        println!("{s}");
        
        let mut a = [0; 10];
        for c in s.chars() {
            // println!("{c}");

            if c == '0' {
                a[0] += 1;
            } else if c == '1' {
                a[1] += 1;
            } else if c == '2' {
                a[2] += 1;
            } else if c == '3' {
                a[3] += 1;
            } else if c == '4' {
                a[4] += 1;
            } else if c == '5' {
                a[5] += 1;
            } else if c == '6' {
                a[6] += 1;
            } else if c == '7' {
                a[7] += 1;
            } else if c == '8' {
                a[8] += 1;
            } else if c == '9' {
                a[9] += 1;
            }
        }

        // print if count is not zero
        let mut twin = true;
        for (index, value) in a.iter().enumerate() {
            if *value != 0 {
                if *value % 2 != 0 {
                    twin = false;
                    // println!("count {}: {}", index, *value);
                }
            }
        }
        if twin == true {
            println!("found: {}", s);
        }

        // println!("count 0: {}", a[0]);
        // println!("count 1: {}", a[1]);
        // println!("count 2: {}", a[2]);
        // println!("count 3: {}", a[3]);
        // println!("count 4: {}", a[4]);
        // println!("count 5: {}", a[5]);
        // println!("count 6: {}", a[6]);
        // println!("count 7: {}", a[7]);
        // println!("count 8: {}", a[8]);
        // println!("count 9: {}", a[9]);

        

        i += 1;
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("1st.txt")?;

    println!("{file}");

    let ranges = file.split(",");

    for range in ranges {
        let range = range.trim();
        // println!("{range}");
    
        let (start, end) = range.split_once("-").unwrap();
        
        let start: i32 = start.parse().unwrap();
        let end: i32   = end.parse().unwrap();

        println!("{start} : {end}");
        process_range(start, end);

    }

    Ok(())
}

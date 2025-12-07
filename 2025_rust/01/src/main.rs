fn zeros(filename: &str, mode: u8) -> std::io::Result<i32> {
    let file = std::fs::read_to_string(filename)?;

    let mut i = 50;
    let mut z = 0;   // zeros (landed)
    let mut t = 0;   // zeros (ticket)

    for line in file.lines() {
        let (direction, steps) = line.split_at(1);
        let mut steps: i32 = steps.parse().unwrap();  // shadowing

        while steps > 0 {

            if direction == "L" {
                i-=1;
            } else {
                i+=1;
            }

            if i < 0 {
                i += 100;
            }

            if i > 99 {
                i -= 100;
            }

            if i == 0 {
                t +=1;
            }
            
            steps -= 1
        }

        if i == 0 {
            z +=1;
        }
    }
    // println!("{t} {z}");
    if mode == 1 {
        return Ok(z);
    } else if mode == 2 {
        return Ok(t);
    } else {
        panic!("Error: unknown mode!");
    }
}

fn run(filename: &str, mode: u8) {
    use std::time::Instant;

    let time = Instant::now();
    let sum = zeros(&filename, mode).unwrap();
    println!("{} {:>6}   took: {:>4.2}s", filename, sum, time.elapsed().as_secs_f64());
}

fn main() -> std::io::Result<()> {
    let file_1st = "1st.txt";
    let file_2nd = "2nd.txt";

    println!("--- PART I ----------------------------------------");
    run(&file_1st, 1);
    run(&file_2nd, 1);
    println!("----PART II ---------------------------------------");
    run(&file_1st, 2);
    run(&file_2nd, 2);
    println!("---------------------------------------------------");

    Ok(())
}


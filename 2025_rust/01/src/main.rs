fn zeros(filename: &str) -> std::io::Result<(i32, i32)> {
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
    Ok((z, t))
}

fn main() -> std::io::Result<()> {
    let file_1st = "1st.txt";
    let file_2nd = "2nd.txt";

    let (z_1st, t_1st) = zeros(file_1st)?;
    let (z_2nd, t_2nd) = zeros(file_2nd)?;

    println!("--------------------------------------------");
    println!("{file_1st}: {z_1st} {t_1st}");
    println!("{file_2nd}: {z_2nd} {t_2nd}");
    println!("--------------------------------------------");

    Ok(())
}


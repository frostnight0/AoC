// fn type_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
//
// fn print_matrix(matrix: &Vec<Vec<char>>) {
//     for (idx_word, word) in matrix.iter().enumerate() {
//         for (idx_letter, letter) in word.iter().enumerate() {
//             print!("{}", letter);
//         }
//         println!();
//     }
// }

fn get_matrix(file: &str) -> Vec<Vec<char>> {
    // print!("{}", file);
    let word_size = file.lines().next().unwrap().len();
    let mut word_count = 0;
    for _ in file.lines() {
        word_count += 1;
    }

    // println!("word_count: {word_count} x word_size: {word_size}");

    let mut matrix = vec![vec![' '; word_size]; word_count];

    for (idx_word, word) in file.lines().enumerate() {
        // println!("word: {}", word);
        for (idx_letter, letter) in word.chars().enumerate() {
            // println!("letter: {}", letter);
            matrix[idx_word][idx_letter] = letter;
        }
    }

    // println!("----------------------------------------------------------");
    // print_matrix(&matrix);
    // println!("----------------------------------------------------------");

    matrix
}

fn check_char(matrix: &Vec<Vec<char>>, w: usize, l: usize) -> i32 {
    if matrix[w][l] == '@' {
        return 1;
    } else {
        return 0;
    }
}

fn get_neighbours_count(matrix: &Vec<Vec<char>>, w: usize, l: usize) -> i32 {
    // println!("w:[w], L:[{l}]");

    let w_limit = matrix.len() -1;
    let l_limit = matrix[0].len() -1;

    let mut count = 0;
    let mut w_adj: usize;
    let mut l_adj: usize;

    // 1
    if w > 0 && l > 0 {
        w_adj = w - 1;
        l_adj = l - 1;
        count += check_char(matrix, w_adj, l_adj);
    }

    // 2
    if w > 0 {
        w_adj = w - 1;
        count += check_char(matrix, w_adj, l);
    }

    // 3
    if w > 0 && l < l_limit {
        w_adj = w - 1;
        l_adj = l + 1;
        count += check_char(matrix, w_adj, l_adj);
    }

    // 4
    if l > 0 {
        l_adj = l - 1;
        count += check_char(matrix, w, l_adj);
    }

    // 6
    if l < l_limit {
        l_adj = l + 1;
        count += check_char(matrix, w, l_adj);
    }

    // 7
    if w < w_limit && l > 0 {
        w_adj = w + 1;
        l_adj = l - 1;
        count += check_char(matrix, w_adj, l_adj);
    }

    // 8
    if w < w_limit {
        w_adj = w + 1;
        count += check_char(matrix, w_adj, l);
    }

    // 9
    if w < w_limit && l < l_limit {
        w_adj = w + 1;
        l_adj = l + 1;
        count += check_char(matrix, w_adj, l_adj);
    }

    // print!("[{}]", count);

    count
}

fn get_sum(matrix: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    for (idx_word, _) in matrix.iter().enumerate() {
        for (idx_letter, letter) in matrix[idx_word].iter().enumerate() {
            let count = get_neighbours_count(matrix, idx_word, idx_letter);
            if count < 4 && *letter == '@'  {
                sum += 1;
            }
        }
    }

    // println!("{sum}");

    sum
}

fn delete_rolls(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut matrix_clone = matrix.clone();

    for (idx_word, _) in matrix.iter().enumerate() {
        for (idx_letter, letter) in matrix[idx_word].iter().enumerate() {
            let count = get_neighbours_count(matrix, idx_word, idx_letter);
            if count < 4 && *letter == '@'  {
                matrix_clone[idx_word][idx_letter] = 'x';
            }
        }
    }

    // print_matrix(&matrix_clone);

    matrix_clone
}

fn get_sum_of_rolls(matrix: &Vec<Vec<char>>) -> i32 {
    let mut matrix = matrix.clone();
    // print_matrix(&matrix);
    // println!("{}", type_of(matrix));

    let mut sum_of_rolls = 0;
    let mut sum = get_sum(&matrix);
    // println!("{sum}");
    sum_of_rolls += sum;

    while sum > 0 {
        matrix = delete_rolls(&matrix);
        // print_matrix(&matrix);
        sum = get_sum(&matrix);
        // println!("{sum}");
        sum_of_rolls += sum;
    }

    sum_of_rolls
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
    let matrix = get_matrix(&file_1st);

    let (sum, took) = run_it(get_sum, &matrix);
    print_it(filename_1st, sum, took);

    let matrix = get_matrix(&file_2nd);

    let (sum, took) = run_it(get_sum, &matrix);
    print_it(filename_2nd, sum, took);

    println!("----PART II ----------------------------------------------");
    let matrix = get_matrix(&file_1st);

    let (sum, took) = run_it(get_sum_of_rolls, &matrix);
    print_it(filename_1st, sum, took);

    let matrix = get_matrix(&file_2nd);

    let (sum, took) = run_it(get_sum_of_rolls, &matrix);
    print_it(filename_2nd, sum, took);
    println!("----------------------------------------------------------");

    Ok(())
}


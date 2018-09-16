pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();

    let max_values_row: Vec<u64> = input.iter()
                                    .map(|row| find_row_max(row))
                                    .collect();
    let min_values_column: Vec<u64> = (0..input[0].len()) //input[0].len() = amount of columns
                                    .map(|column| find_column_min(input, column))
                                    .collect();
    
    // use enumerate to make it easier to enter index if necessary
    for (i, row) in input.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {     
            // check if element is both the max value in the row and
            // the min value in the column
            // use i and j to check with respective values in precalculated vectors
            if element == &max_values_row[i] && element == &min_values_column[j] {
                output.push((i, j));
            }
        }
    }

    output
}

// find the max value in a given row
// returns zero if no value can be found
fn find_row_max(input: &Vec<u64>) -> u64 {   
    *input.iter().max().unwrap_or(&0)
}

// find the max value in a give column
// returns zero if no value can be found
fn find_column_min(input: &[Vec<u64>], column: usize) -> u64 {
    let mut buffer = vec![];

    for row in input {
        buffer.push(row[column])
    }

    *buffer.iter().min().unwrap_or(&0)
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();

    //check if matrix is empty, return empty vector if this is the case
    if input[0].len() == 0 {
        return output;
    }
    // use enumerate to make it easier to enter index if necessary
    for (i, row) in input.iter().enumerate() {
        let max = find_row_max(row);

        for (j, element) in row.iter().enumerate() {
            let min = find_column_min(input, j);

            // check if element is both the max value in the row and
            // the min value in the column
            if element == &max && element == &min {
                output.push((i, j));
            }
        }
    }
    
    output
}

// find the max value in a given row
fn find_row_max(input: &Vec<u64>) -> u64 {   
    *input.iter().max().unwrap()
}

// find the max value in a give column
fn find_column_min(input: &[Vec<u64>], column: usize) -> u64 {
    let mut buffer = vec![];

    for row in input {
        buffer.push(row[column])
    }

    *buffer.iter().min().unwrap()
}

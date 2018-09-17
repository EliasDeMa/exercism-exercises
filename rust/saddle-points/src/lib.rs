pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let max_values_row: Vec<u64> = input.iter()
                                    .map(|row| find_row_max(row))
                                    .collect();
    let min_values_column: Vec<u64> = (0..input[0].len()) //input[0].len() = amount of columns
                                    .map(|column| find_column_min(input, column))
                                    .collect();
    
    input.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, col)| ((i, j), col))
                .filter(|&((i, j), col)| *col == max_values_row[i] && *col == min_values_column[j])
                .map(|((i,j), _col)| (i,j))           
            })
        .collect()  
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

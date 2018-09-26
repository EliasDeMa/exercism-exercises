pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = Vec::new();

        for i in 0..self.row_count {
            let mut row: Vec<u32> = Vec::new();
            // adds right amount of variables to each row
            for j in 0..=i {
                // calculate right value for each position => C(i,j) == C(n,k)
                row.push(PascalsTriangle::combination(i, j))
            }
            result.push(row);
        }

        result
    }

    // factorial function used for the combination function
    fn factorial(number: u32 ) -> u32 {
        match number {
            0 | 1 => 1,
            _ => number * PascalsTriangle::factorial(number - 1)
        }
    }

    // calculate value at given position in row with combination
    // or binomial coefficient
    fn combination(n: u32, k: u32) -> u32 {
        PascalsTriangle::factorial(n) / (PascalsTriangle::factorial(k) * PascalsTriangle::factorial(n - k))
    }
}

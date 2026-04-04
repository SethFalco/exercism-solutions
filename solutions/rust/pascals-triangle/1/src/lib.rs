pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            0: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::new();

        for i in 0..self.0 {
            let mut curr_row = vec![1u32];

            if i == 0 {
                rows.push(curr_row);
                continue;
            }

            let prev_row = rows.last().unwrap();

            for ii in 1..i {
                let left = prev_row[(ii - 1) as usize];
                let right = prev_row[ii as usize];
                curr_row.push(left + right);
            }

            curr_row.push(1);
            rows.push(curr_row);
        }

        rows
    }
}

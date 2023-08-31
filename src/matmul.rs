use std::vec;

pub(crate) fn multiply(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let a_dims = [a.len(), a[0].len()];
    let b_dims = [b.len(), b[0].len()];

    if a_dims[1] != b_dims[0] {
        println!("Inner dimensions DO NOT match, we cannot multiply these matrices");
        return vec![]
    }

    let mut output: Vec<Vec<f64>> = vec![];

    for a_row in a {
        let mut this_row: Vec<f64> = vec![0.; a_row.len()];
        for a_i in 0..a_row.len() {
            for b_i in 0..b[a_i].len() {
                this_row[b_i] += &b[a_i][b_i] * a_row[a_i];
            }
        }
        output.push(this_row);
    }

    output
}
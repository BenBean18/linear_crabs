use std::vec;

pub(crate) fn matmul(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Find the dimensions of a and b (rows x columns)
    let a_dims = [a.len(), a[0].len()];
    let b_dims = [b.len(), b[0].len()];

    // If a's columns do not match b's rows, we cannot do this matrix multiplication
    if a_dims[1] != b_dims[0] {
        println!("Inner dimensions DO NOT match, we cannot multiply these matrices");
        return vec![]
    }

    // Create a variable for the output matrix
    let mut output: Vec<Vec<f64>> = vec![];


    // The "row operations" way of thinking of matrix multiplication:
    // - Each row in A represents what that row will be in the new matrix, in terms of the rows of B.
    // - e.g. A = 1 0 = I
    // -          0 1
    // -      B = 1 2
    // -          3 4
    // - The first row of A, [1 0], says that the first row of the new matrix is:
    // - 1 * first row of B + 0 * second row of B = 1 * [1 2] + 0 * [3 4]

    // For each row in A, e.g. [1 0]
    for a_row in a {
        // Create a vector to store the current row in the output matrix
        let mut this_row: Vec<f64> = vec![0.; b[0].len()];
        // For each index in the row (represents scalar to multiply by corresponding row of B), index 0 = 1 in this case
        for i in 0..a_row.len() {
            // For each element in B's (i)th row (which is B[0] = [1 2])
            for j in 0..b[i].len() {
                // Multiply it (in this example, the number 1) by the (i)th element in the current row of A
                // Add it to the current total in the (j)th spot of the row
                this_row[j] += &b[i][j] * a_row[i];
            }
        }
        // Add the finished row to the matrix
        output.push(this_row);
    }

    output // Return the matrix (isn't rust nice)
}

pub(crate) fn print_mat(mat: Vec<Vec<f64>>) {
    for row in mat {
        for e in row {
            print!("{e} ");
        }
        println!();
    }
}

// a transpose function would make this easier

pub(crate) fn find_elimination_matrix(mat: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // - Choose the first nonzero number in the first column as a pivot
    // - Use the pivot to zero out that column in the rows below it (these coefficients are what is stored in the elim. matrix)
    // - Continue column by column until you're upper triangular 

    let mut column: usize = 0; // buddy, if you've got more than 4294967296 columns, we have a problem
    let mut pivot: f64 = 0.;
    for row in mat {
        if row[column] != 0. {
            if pivot == 0. {
                pivot = row[column];
            } else {
                // -row[column]/pivot is what we need here
            }
        }
    }

    vec![]
}
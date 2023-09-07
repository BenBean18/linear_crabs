use rand::Rng;

pub(crate) fn matmul(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> { // O(n^3)
    // Find the dimensions of a and b (rows x columns)
    let a_dims = [a.len(), a[0].len()];
    let b_dims = [b.len(), b[0].len()];

    // If a's columns do not match b's rows, we cannot do this matrix multiplication
    if a_dims[1] != b_dims[0] {
        println!("Inner dimensions DO NOT match ({}x({} != {})x{}), we cannot multiply these matrices", a_dims[0], a_dims[1], b_dims[0], b_dims[1]);
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
    for a_row in a { // O(n)
        // Create a vector to store the current row in the output matrix
        let mut this_row: Vec<f64> = vec![0.; b[0].len()];
        // For each index in the row (represents scalar to multiply by corresponding row of B), index 0 = 1 in this case
        for i in 0..a_row.len() { // O(n)
            // For each element in B's (i)th row (which is B[0] = [1 2])
            for j in 0..b[i].len() { // O(n)
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


pub(crate) fn identity(n: usize) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = vec![vec![0.; n]; n];

    for row in 0..n {
        for column in 0..n {
            if column == row {
                output[row][column] = 1.;
            }
        }
    }
    output
}

pub(crate) fn empty(n: usize) -> Vec<Vec<f64>> {
    vec![vec![0.; n]; n]
}

// a transpose function would make this easier

// Gauss-Jordan elimination
// For each column (index i):
// - Swap rows until we have a valid pivot in the i'th column
// - Once there is a valid pivot, use it to eliminate all nonzero values in the i'th column
// - For each value, create an elimination matrix and multiply it by the initial elimination matrix

pub(crate) fn gauss_jordan(mut mat: Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, &'static str> {
    // I think the time complexity of this is O(n^5)
    // Which is BAD
    if mat.len() > mat[0].len() {
        return Err("# rows > # columns, this is nonsquare and nonaugmented")
    }

    let mut elim_mat: Vec<Vec<f64>> = identity(mat.len()); // O(n^2)

    let mut pivot = 0.;
    let mut j: i64 = 0;

    while j < mat.len() as i64 { // O(n)
        // for each column, j
        let mut new_j = j;
        let mut range: Vec<i64> = (0..mat.len() as i64 - j).collect::<Vec<_>>();
        range.extend((-j..0).collect::<Vec<_>>());

        let mut found_working_pivot = false;

        for current_row in range { // O(n)
            if mat[j as usize][j as usize] == 0. {
                // row exchange
                // Need to handle re-elimination when you have to go above to find a pivot
                let mut this_elim_mat = identity(mat.len()); // O(n^2)
                this_elim_mat[j as usize] = vec![0.; mat.len()];
                this_elim_mat[j as usize][(j+current_row) as usize] = 1.;
                this_elim_mat[(j+current_row) as usize] = vec![0.; mat.len()];
                this_elim_mat[(j+current_row) as usize][j as usize] = 1.;
                elim_mat = matmul(this_elim_mat.clone(), elim_mat); // O(n^3)
                mat = matmul(this_elim_mat.clone(), mat); // O(n^3)
                // println!("Row exchange, new matrix:");
                // print_mat(mat.clone());
                if current_row < 0 {
                    new_j = j + current_row;
                    // println!("Backward row exchange :(");
                }
            } else {
                found_working_pivot = true;
                break;
            }
        }
        if !found_working_pivot {
            println!("Matrix is singular");
            return Err("Matrix is singular")
        }
        pivot = mat[j as usize][j as usize];
        // println!("Picked pivot {j}, {j} = {pivot}");
        if j != new_j {
            j = new_j;
            continue;
        }
        for i in 0..mat.len() { // O(n)
            // for each row, i
            if i != j as usize {
                let mut this_elim_mat = identity(mat.len()); // O(n^2)
                this_elim_mat[i][j as usize] = -mat[i][j as usize]/pivot;
                this_elim_mat[i][i] = 1.;
                // println!("r{} = {}r{} + r{}", i+1, -mat[i][j as usize]/pivot, j+1, i+1);
                elim_mat = matmul(this_elim_mat.clone(), elim_mat); // O(n^3)
                mat = matmul(this_elim_mat.clone(), mat); // O(n^3)
            }
        }

        // Make identity matrix
        for i in 0..mat.len() { // O(n)
            // for each row, i
            if i == j as usize {
                let mut this_elim_mat = identity(mat.len()); // O(n^2)
                this_elim_mat[i][i] = 1./mat[i][i];
                elim_mat = matmul(this_elim_mat.clone(), elim_mat); // O(n^3)
                mat = matmul(this_elim_mat.clone(), mat); // O(n^3)
            }
        }

        // print_mat(mat.clone());
        j += 1;
    }

    return Ok(elim_mat)
}

pub(crate) fn random(n: usize) -> Vec<Vec<f64>> {
    let mut mat: Vec<Vec<f64>> = empty(n);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = (rng.gen::<i8>() as f64);
        }
    }
    mat
}
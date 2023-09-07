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
            print!("{:.2} ", e);
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
    // I think the time complexity of this is now O(n^3) which would be a LOT better
    if mat.len() > mat[0].len() {
        return Err("# rows > # columns, this is nonsquare and nonaugmented")
    }

    let mut elim_mat: Vec<Vec<f64>> = identity(mat.len()); // O(n^2)

    let mut pivot = 0.;
    let mut j: i64 = 0;

    while j < mat.len() as i64 { // O(n)
        // for each column, j

        // set the next column to be the current one (this might be changed later)
        let mut new_j = j;

        // 0...maximum down row (rows - j)...-1...maximum up row (-j)
        let mut range: Vec<i64> = (0..mat.len() as i64 - j).collect::<Vec<_>>();
        range.extend((-j..0).collect::<Vec<_>>());

        let mut found_working_pivot = false;

        // try to find a nonzero pivot, starting with the current row and going down.
        // we only want to go up if we REALLY need to (since that requires reelimination of the above row)
        for current_row in range { // O(n)
            if mat[j as usize][j as usize] == 0. {
                // If the element along the main diagonal is zero, do a row exchange with current_row

                // Need to handle re-elimination when you have to go above to find a pivot

                // This is O(n) because you are switching two rows
                let old_next_row = &mat.clone()[(j + current_row) as usize];
                mat[(j + current_row) as usize] = mat[j as usize].clone();
                mat[j as usize] = old_next_row.clone();

                // println!("Row exchange, new matrix:");
                // print_mat(mat.clone());

                // If we have to go backwards, subtract from the column variable (have to reeliminate above rows now that they are switched)
                if current_row < 0 {
                    new_j = j + current_row;
                    // println!("Backward row exchange :(");
                }
            } else {
                // If there is a nonzero element in our position of the main diagonal,
                // we have found a working pivot and should exit the loop.
                found_working_pivot = true;
                break;
            }
        }

        // If we haven't found a working pivot, the matrix must be singular (not exactly sure how to prove this at 9:45pm)
        if !found_working_pivot {
            println!("Matrix is singular");
            return Err("Matrix is singular")
        }

        // Set the pivot to the one that we found
        pivot = mat[j as usize][j as usize];
        // println!("Picked pivot {j}, {j} = {pivot}");

        // If we had to move our column back, go back to the start of the loop and reeliminate all necessary above rows
        if j != new_j {
            j = new_j;
            continue;
        }

        // For each row
        for i in 0..mat.len() { // O(n)
            // for each row, i
            if i != j as usize {

                // Store what we have to multiply the row containing the pivot by to zero out our row/column
                // We'll be zeroing our row/column so if we calculate this each column it will only work the first time,
                // then be zero.
                let multiple = -mat[i][j as usize]/pivot;

                // O(n) since you have to do something for each element
                for col in 0..mat[0].len() {

                    // For each column,

                    // println!("col {col} before = {} -> += {} * {}", mat[i][col], mat[j as usize][col], multiple);

                    // Multiply the corresponding column in the pivot row by the multiplier from before, and add it to the current element
                    mat[i][col] += mat[j as usize][col] * multiple;

                    // println!("col {col} after = {}", mat[i][col]);
                }

                // print_mat(mat.clone());
            }
        }

        // Make the eliminated matrix an identity matrix
        // If we have a matrix representing the system:
        // x + y + z = 6
        // 3x + 2y + z = 10
        // 2x + y + 2z = 10
        
        // aka
        // 1 1 1 6
        // 3 2 1 10
        // 2 1 2 10

        // then making the square part an identity matrix yields
        // 1 0 0 a
        // 0 1 0 b
        // 0 0 1 c

        // or:
        // x = a
        // y = b
        // z = c
        for i in 0..mat.len() { // O(n)
            // for each row, i
            if i == j as usize {
                let multiple = 1./mat[i][i]; // Store what we have to multiply by to make the row a row of the identity matrix

                // O(n) since you have to do something for each element
                for col in 0..mat[0].len() { // O(n)
                    // Multiply each element in the row by the above multiplier
                    mat[i][col] *= multiple;
                }
            }
        }

        // print_mat(mat.clone());

        // Increment the column
        j += 1;
    }

    // Return the eliminated matrix
    return Ok(mat)
}

pub(crate) fn random(n: usize) -> Vec<Vec<f64>> {
    let mut mat: Vec<Vec<f64>> = empty(n);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = (rng.gen::<i8>() as f64);
        }
        mat[i].push((rng.gen::<i8>() as f64));
    }
    mat
}
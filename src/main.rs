mod math;

fn main() {
    // let mat = math::matmul(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    // math::print_mat(mat);
    // println!();
    // let mat_to_elim = math::random(10);
    // math::print_mat(mat_to_elim.clone());
    // match math::gauss_jordan(mat_to_elim) {
    //     Ok(matrix) => math::print_mat(matrix),
    //     Err(s) => println!("{s}")
    // };

    // let augmented_mat = vec![vec![2., 1., 3.,   1.],
    //                                         vec![4., 2., 7.,   3.],
    //                                         vec![2., 2., 4.,   2.]];
    
    // let augmented_inverse_mat = vec![vec![2., 1., 3.,   1., 0., 0.],
    //                                                 vec![4., 2., 7.,   0., 1., 0.],
    //                                                 vec![2., 2., 4.,   0., 0., 1.]];
    // match math::gauss_jordan(augmented_mat) {
    //     Ok(matrix) => math::print_mat(matrix),
    //     Err(s) => println!("{s}")
    // };

    // math::print_mat(math::matmul(vec![vec![3., -1., -0.5], vec![1., -1., 1.], vec![-2., 1., 0.]], vec![vec![1.], vec![3.], vec![2.]]));

    math::print_mat(math::inverse(vec![vec![1., 2.], vec![3., 4.]]));

    // Analyze time complexity!
    // Results here: https://docs.google.com/spreadsheets/d/1CM8Jm6n9gNG3USm1nQ7uSp-EzdMskY_v8iUA_9z4BJI/edit?usp=sharing, see v2

    // for i in 400..700 {
    //     use std::time::Instant;
    //     let mat = math::random(i);
    //     let now = Instant::now();
    //     math::gauss_jordan(mat);
    //     let elapsed = now.elapsed();
    //     println!("{},{}", i, elapsed.as_nanos());
    // }

    // let elim_mat = math::elimination_matrix(mat_to_elim.clone());
    //let inv_elim_mat = math::elimination_matrix(mat_to_inv.clone());
    // math::print_mat(math::do_elimination(elim_mat.clone(), mat_to_elim)); // now it's upper triangular :)
    //math::print_mat(inv_elim_mat.clone());
    //math::print_mat(math::matmul(inv_elim_mat.clone(), mat_to_inv.clone())); // now it's upper triangular :)
}
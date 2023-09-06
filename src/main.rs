mod math;

fn main() {
    let mat = math::matmul(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    math::print_mat(mat);
    println!();
    let mat_to_elim = vec![vec![1., 2., 3., 10.], vec![4., 8., 6., 20.], vec![7., 8., 9., 30.]];
    match math::gauss_jordan(mat_to_elim) {
        Ok(matrix) => math::print_mat(matrix),
        Err(s) => println!("{s}")
    };

    // let elim_mat = math::elimination_matrix(mat_to_elim.clone());
    //let inv_elim_mat = math::elimination_matrix(mat_to_inv.clone());
    // math::print_mat(math::do_elimination(elim_mat.clone(), mat_to_elim)); // now it's upper triangular :)
    //math::print_mat(inv_elim_mat.clone());
    //math::print_mat(math::matmul(inv_elim_mat.clone(), mat_to_inv.clone())); // now it's upper triangular :)
}
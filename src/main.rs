mod math;

fn main() {
    let mat = math::matmul(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    math::print_mat(mat);
    println!();
    let mat_to_elim = vec![vec![2., 4., -2., 2.], vec![4., 9., -3., 8.], vec![-2., -3., 7., 10.]];
    let mat_to_inv = vec![vec![0., 5., 58.], vec![1., 2., 2.]];
    // let elim_mat = math::elimination_matrix(mat_to_elim.clone());
    let inv_elim_mat = math::elimination_matrix(mat_to_inv.clone());
    // math::print_mat(math::do_elimination(elim_mat.clone(), mat_to_elim)); // now it's upper triangular :)
    math::print_mat(math::do_elimination(inv_elim_mat.clone(), mat_to_inv.clone())); // now it's upper triangular :)
}
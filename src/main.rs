mod math;

fn main() {
    let mat = math::matmul(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    math::print_mat(mat);
    println!();
    math::print_mat(math::matmul(vec![vec![1., 2., 4.]], vec![vec![2.], vec![2.], vec![2.]]));
}
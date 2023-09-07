mod math;

fn main() {
    // let mat = math::matmul(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    // math::print_mat(mat);
    // println!();
    // let mat_to_elim = vec![vec![1., 1., 1., 10.], vec![1., 0., 1., 20.], vec![1., 1., 1., 30.]];
    // match math::gauss_jordan(mat_to_elim) {
    //     Ok(matrix) => math::print_mat(matrix),
    //     Err(s) => println!("{s}")
    // };

    // Analyze time complexity!
    // Results here: https://docs.google.com/spreadsheets/d/1CM8Jm6n9gNG3USm1nQ7uSp-EzdMskY_v8iUA_9z4BJI/edit?usp=sharing

    for i in 2..100 {
        use std::time::Instant;
        let mat = math::random(i);
        let now = Instant::now();
        math::gauss_jordan(mat);
        let elapsed = now.elapsed();
        println!("{},{}", i, elapsed.as_nanos());
    }

    // let elim_mat = math::elimination_matrix(mat_to_elim.clone());
    //let inv_elim_mat = math::elimination_matrix(mat_to_inv.clone());
    // math::print_mat(math::do_elimination(elim_mat.clone(), mat_to_elim)); // now it's upper triangular :)
    //math::print_mat(inv_elim_mat.clone());
    //math::print_mat(math::matmul(inv_elim_mat.clone(), mat_to_inv.clone())); // now it's upper triangular :)
}
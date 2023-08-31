mod matmul;

fn main() {
    println!("Hello, world!");
    let mat = matmul::multiply(vec![vec![1., 0.], vec![-2., 1.]], vec![vec![1., 3.], vec![2., 4.]]);
    for row in mat {
        for e in row {
            print!("{e} ");
        }
        println!();
    }
}
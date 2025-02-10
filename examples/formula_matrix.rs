use numberlab::formula::matrix::Matrix;

fn main() {
    let matrix1 = Matrix::from_array([[10, 2, 3], [3, 40, 1]]);
    let matrix2 = Matrix::from_array([[1, 2, 3], [3, 4, 5]]);
    let matrix3 = Matrix::from_array([[1, 2], [3, 4], [5, 6]]);

    println!("M1 : {}", matrix1);
    println!("M2 : {}", matrix2);
    println!("M3 : {}", matrix3);
    println!("M1 + M2 : {}", matrix1.clone() + matrix2.clone());
    println!("M1 - M2 : {}", matrix1.clone() - matrix2.clone());
    println!("M1 * M3 : {}", matrix1.clone() * matrix3.clone());
    println!("M1 / M3 : {}", matrix1.clone() / matrix2.clone());
}

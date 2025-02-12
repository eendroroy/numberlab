use numberlab::mat;
use numberlab::structure::matrix::Matrix;
use numberlab::structure::matrix::MatrixTrait;

fn main() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5]];
    let matrix3 = mat![[1, 2], [3, 4], [5, 6]];
    let matrix4 = mat![[1, 2, 3], [3, 4, 5], [5, 6, 7]];

    println!("M1 : {}", matrix1);
    println!("M2 : {}", matrix2);
    println!("M3 : {}", matrix3);
    println!("M1 + M2 : {}", matrix1.clone() + matrix2.clone());
    println!("M1 - M2 : {}", matrix1.clone() - matrix2.clone());
    println!("M1 * M3 : {}", matrix1.clone() * matrix3.clone());
    println!("M1 / M3 : {}", matrix1.clone() / matrix2.clone());

    println!();

    println!("Transpose of M1 : {}", matrix1.transpose());
    println!("Transpose of M2 : {}", matrix2.transpose());
    println!("Transpose of M3 : {}", matrix3.transpose());

    println!();


    println!("I1  : {}", Matrix::<i32, 1>::identity());
    println!("I2  : {}", Matrix::<i32, 2>::identity());
    println!("I10 : {}", Matrix::<i32, 10>::identity());

    println!();

    println!("Upper Triangular of {} : {}", matrix4, matrix4.upper_triangular());
    println!("Lower Triangular of {} : {}", matrix4, matrix4.lower_triangular());
}

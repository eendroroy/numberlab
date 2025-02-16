use numberlab::mat;
use numberlab::structure::matrix::{identity, Matrix};

fn main() {
    let matrix1 = mat![[10, 2, 3], [3, 40, 1]];
    let matrix2 = mat![[1, 2, 3], [3, 4, 5]];
    let matrix3 = mat![[1, 2], [3, 4], [5, 6]];
    let matrix4 = mat![[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]];

    println!("ROW 0 of M1 : {:?}", matrix1.row(0));
    println!("ROW 1 of M1 : {:?}", matrix1.row(1));
    println!("COL 0 of M1 : {:?}", matrix1.col(0));
    println!("COL 1 of M1 : {:?}", matrix1.col(1));
    println!("COL 2 of M1 : {:?}", matrix1.col(2));

    println!();

    println!("M1 : {}", matrix1);
    println!("M2 : {}", matrix2);
    println!("M3 : {}", matrix3);
    println!("M1 + M2 : {}", matrix1.clone() + matrix2.clone());
    println!("M1 - M2 : {}", matrix1.clone() - matrix2.clone());
    println!("M1 * M3 : {}", matrix1.clone() * matrix3.clone());

    println!();

    println!("M2 + 2 : {}", matrix2.clone() + 2);
    println!("M2 - 2 : {}", matrix2.clone() - 2);
    println!("M2 * 2 : {}", matrix2.clone() * 2);
    println!("M4  / 2 : {}", matrix4.clone() / 2.0);

    println!();

    println!("Transpose of M1 : {}", matrix1.transpose());
    println!("Transpose of M2 : {}", matrix2.transpose());
    println!("Transpose of M3 : {}", matrix3.transpose());

    println!();

    println!("I1  : {}", identity::<i8, 1>());
    println!("I2  : {}", identity::<i8, 2>());
    println!("I5  : {}", identity::<i8, 5>());
    println!("I10 : {}", identity::<i8, 10>());

    println!();

    let mut um4 = matrix4.clone();
    let mut lm4 = matrix4.clone();

    um4.upper_triangular();
    lm4.lower_triangular();

    println!("UM of {} is : {}", matrix4, um4);
    println!("LM of {} is : {}", matrix4, lm4);

    println!();

    println!("UM of {} is : {}", matrix4, matrix4.to_lower_triangular());
    println!("LM of {} is : {}", matrix4, matrix4.to_lower_triangular());

    println!();

    println!("M1 is_square : {}", matrix1.is_square());
    println!("UM is_square : {}", matrix2.is_square());

    println!();

    println!("M1 is_identity : {}", matrix1.is_identity());
    println!("I9 is_identity : {}", identity::<i8, 9>().is_identity());

    println!();

    println!("UM4 is_upper_triangular : {}", um4.is_upper_triangular());
    println!("UM4 is_lower_triangular : {}", um4.is_lower_triangular());
    println!("LM4 is_upper_triangular : {}", lm4.is_upper_triangular());
    println!("LM4 is_lower_triangular : {}", lm4.is_lower_triangular());
}

use numberlab::pattern::pascal::{pascal_triangle, pascal_triangle_binomial};

fn main() {
    let n = 5;

    pascal_triangle(n).iter().for_each(|row| {
        println!("{:?}", row);
    });

    println!("{:?}", pascal_triangle_binomial(10));
}

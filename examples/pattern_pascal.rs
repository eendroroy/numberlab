use numberlab::pattern::pascal::pascal_triangle;

fn main() {
    let pascal = pascal_triangle(20);

    pascal.iter().for_each(|row| {
        println!("{:?}", row);
    });
}

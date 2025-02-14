fn print_left_triangle(n: usize) {
    let repeated = "*".repeat(n);
    print!("{}", repeated);
}

fn main() {
    let n = 5;
    print_left_triangle(n);
}

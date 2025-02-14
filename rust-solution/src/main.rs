fn print_left_triangle(n: usize) {
    /*
     * Prints a left-aligned triangle with n rows.
     *
     * @param {number} n - The number of rows in the triangle
     * @returns {void}
     */

    for i in 1..=n {
        let stars = "*".repeat(i);
        println!("{}", stars);
    }
}

fn print_centered_triangle(n: usize) {
    /*
     * Prints a centered triangle with n rows.
     *
     * @param {number} n - The number of rows in the triangle
     * @returns {void}
     */

    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let stars = "*".repeat(2 * i - 1);
        println!("{}{}", spaces, stars);
    }
}

fn main() {
    let n = 5;
    println!("Left-aligned triangle");
    print_left_triangle(n);

    println!("\nCentered triangle");
    print_centered_triangle(n);
}

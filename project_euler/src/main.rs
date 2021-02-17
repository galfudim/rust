mod p_one;
mod p_two;
mod p_three;
mod p_six;

fn main() {
    let sum = p_six::sum_squares(100);
    let square = p_six::square_sums(100);
    println!("{} - {} = {}", square, sum, square-sum);
}
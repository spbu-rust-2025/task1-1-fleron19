use std::io;

fn main() {
    let mut input = String::new(); 
    let sum;
    let _ = io::stdin().read_line(&mut input);
    let mut parts = input.split_whitespace().map(|input| input.parse::<i32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            sum = a + b;
            println!("{}", sum);
        }
        _ => {} 
    }
}

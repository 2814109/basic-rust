fn main() {
    let v = &["apples", "cake", "coffee"];

    for text in v {
        println!("I like {}.", text);
    }

    let mut sum = 0;
    for n in 1..11 {
        sum += n;
    }

     println!("sum = {}", sum);
}

fn main() {
    let name: &str = "CommitCrazy";
    let mut count = 0;
    for i in 1..=5 {
        count += i;
        println!("Hello, {name}! (count: {count})");
    }
    let even_or_odd = match count % 2 {
        0 => { "even" }
        _ => { "odd" }
    };
    println!("Count {count} is {even_or_odd}")
}

fn main() {
    let mut res = 42;
    let option:Option<i64> = Some(12);
    
    if let Some(x) = option
    {
        res += x;
    }

    println!("{res}");
}

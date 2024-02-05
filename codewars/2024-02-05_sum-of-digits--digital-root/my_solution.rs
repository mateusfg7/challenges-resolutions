fn digital_root(n: i64) -> i64 {
    let n_as_string = n.to_string();
    let mut sum: u32 = 0;
    
    for c in n_as_string.chars() {
        sum += c.to_digit(10).unwrap()
    }
    
    if sum < 10 { i64::from(sum) }
    else { digital_root(i64::from(sum)) }    
}

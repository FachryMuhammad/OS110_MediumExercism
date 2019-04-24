pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64{
        panic!("Square must be between 1 and 64");
    }
    let mut grains;
    grains = 2_u64.pow(s-1);
    return grains;
}

pub fn total() -> u64 {
    let mut total = 0;
    for s in 1..65 {
        total += square(s);
    }
    return total;
}
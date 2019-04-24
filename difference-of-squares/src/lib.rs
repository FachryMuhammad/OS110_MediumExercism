pub fn square_of_sum(n: u32) -> u32 {
    let mut hasil: u32 = 0;
    for i in 0..n+1 {
        hasil += i 
    }
    return hasil.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut hasil: u32 = 0;
    for i in 0..n+1 {
        hasil += i.pow(2)
    }
    return hasil;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut f:Vec<u32> = Vec::new();

    for i in 1..limit {
        for j in 0..factors.len() {
            if factors[j] != 0 {
                if i % factors[j] == 0 {
                    f.push(i);
                }
            }
        } 
    }

    f.dedup();

    let mut b:u32 = 0;
    for i in f.iter() {
        b = b+i;
    }
    b
}
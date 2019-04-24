// Perfect Numbers link
// https://exercism.io/my/solutions/c67fad99f313414494c1b9ccdaf23e6f

// Nama     : Fachry Muhammad
// NRM      : 1313617019
// Prodi    : Ilmu Komputer

// source of learning:
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs
// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html?highlight=derive()#appendix-c-derivable-traits
// https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms


// Note:
// Aliquot = Hasil bagian
// Abudant = posisi ketika jumlah hasil bagian lebih dari angka-angka pemfaktoran dari sebuah angka utama
// Perfect = posisi ketika jumlah hasil bagian sama dengan angka-angka pemfaktoran dari sebuah angka utama 
// Deficient = posisi ketika jumlah hasil bagian kurang dari angka-angka pemfaktoran dari sebuah angka utama

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num==0 {return None}
    if num==1 {return Some(Classification::Deficient)}
    let aliquot = (1..num).filter(|x|num%x==0).fold(0,|acc,x|acc+x);
    match (aliquot>num,aliquot==num,aliquot<num) {
        (true,false,false) => return Some(Classification::Abundant),
        (false,true,false) => return Some(Classification::Perfect),
        (false,false,true) => return Some(Classification::Deficient),
        _ => return None,
    }
}
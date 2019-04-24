// Triangle Problem's link
// https://exercism.io/my/solutions/72f5e8ab512d45a0bdb9fb18a146dea8


// Nama     : Fachry Muhammad
// NRM      : 1313617019
// Prodi    : Ilmu Komputer

// source of learning:
// wikipedia triangle; https://en.wikipedia.org/wiki/Triangle_inequality
// and also https://blog.ruangguru.com/mengenal-berbagai-macam-jenis-segitiga
// and also library used https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs

pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {

        // jika salah satu sisi tidak ada, maka bukan segitiga 
        if sides[0] <= 0 || sides[1] <= 0 || sides[2] <= 0{
            return None;
        }
        // jika sisi_a + sisi_b <= sisi_c, maka bukan segitiga
        else if (sides[0]+sides[1]) <= sides[2] || (sides[1]+sides[2]) <= sides[0] || (sides[2]+sides[0]) <= sides[1] {
            return None;

        }
        else{
            Some(Triangle{a: sides[0] ,b: sides[1] ,c: sides[2]});
        }

        Some(Triangle {
                a: sides[0], 
                b: sides[1], 
                c: sides[2]})
    }

    ///Fungsi pengecek segitiga sama kaki
    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.b == self.c 
    }

    ///Fungsi pengecek segitiga sama sisi
    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    ///Fungsi pengecek segitiga sembarang
    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b) || (self.b == self.c) || (self.a == self.c)
    }
}
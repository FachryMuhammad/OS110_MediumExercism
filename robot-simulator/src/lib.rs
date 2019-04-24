// Robot Simulator
// https://exercism.io/my/solutions/7c8b0b363468407ebfe986a46b8eec32


// Nama     : Fachry Muhammad
// NRM      : 1313617019
// Prodi    : Ilmu Komputer


// this problem is Mentored by Rama Lesmana (Romeless)


// source of learning:
// Mentor by tier 1 student Rama Lesmana (Romeless)
// wikipedia triangle; https://en.wikipedia.org/wiki/Triangle_inequality
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html?highlight=mutable#mutable-references
// https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms



#[derive(PartialEq, Debug)]

// Define the enumeration element of direction (points of the compass) that we going to use
pub enum Direction {
    North,
    East,
    South,
    West,
}

// Define the basic element of struct that we going to use
pub struct Robot {
	point_x: i32,
	point_y: i32,
	dir: Direction,
}

// Define the main function
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
			point_x: x,
			point_y : y,
			dir : d,
		}
    }

    // Fungsi perintah untuk belok kanan
    pub fn turn_right(mut self) -> Self {
        // Ketika arah berubah, maka arah lama akan digantikan dengan arah yang baru
        self.dir = match self.dir {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		};
		
		self
    }

    /// Fungsi perintah untuk belok kiri
    pub fn turn_left(mut self) -> Self {
        // Ketika arah berubah, maka arah lama akan digantikan dengan arah yang baru
        self.dir = match self.dir {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
		};
		
		self
    }


    // Refrensi gambar arah mata angin Kompas dan Nilai Diagram Cartesius
    //
    //                      N                       +Y
    //                    W + E                  -X  0 +X
    //                      S                       -Y

    /// Fungsi perintah untuk maju ke depan sesuai dengan  arah dan nilai diagram Kartesius, (dengan nilai 1 point)
    pub fn advance(mut self) -> Self {
        // Ketika maju, maka nilai point_x akan bertambah/berkurang sesuai dengan arah dari 'Direction' 
        self.point_x+= match self.dir {
			Direction::East => 1,
			Direction::West => -1,
			_ => 0,
		};
		// Ketika maju, maka nilai point_y akan bertambah/berkurang sesuai dengan arah dari 'Direction' 
		self.point_y += match self.dir {
			Direction::North => 1,
			Direction::South => -1,
			_ => 0,
		};
		
		self
    }

    /// Fungsi untuk membaca perintah
    pub fn instructions(mut self, instructions: &str) -> Self {
        for chars in instructions.chars() {
			self = match chars {
				'L' => self.turn_left(),
				'R' => self.turn_right(),
				'A' => self.advance(),
				_ => self,
			}
		}
		
		self
    }

    /// Fungsi untuk menyimpan lokasi
    pub fn position(&self) -> (i32, i32) {
        (self.point_x, self.point_y)
    }

    /// Fungsi untuk menyimpan arah direction
    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
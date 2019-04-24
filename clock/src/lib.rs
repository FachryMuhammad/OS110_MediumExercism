// Clock Problem's link
// https://exercism.io/my/solutions/bf353ef066724d2f9ef1d764ac76bfcd


// Nama     : Fachry Muhammad
// NRM      : 1313617019
// Prodi    : Ilmu Komputer

// source of learning:
// https://doc.rust-lang.org/std/fmt/index.html
// and also; https://doc.rust-lang.org/std/fmt/trait.Display.html
// and also; https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs
// and also this; https://doc.rust-lang.org/book/appendix-03-derivable-traits.html?highlight=derive()#appendix-c-derivable-traits


use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:02}:{:02}", self.hour, self.minute)
		}
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        let mut c = Clock { hour: 0, minute: 0 };
        c.add_hours(hour);
        c.add_minutes(minute)
    }

    /// Fungsi penambahan `jam` ke dalam clock
    fn add_hours(&mut self, hours: i32) {
        self.hour += hours;

        // Penambahan Sisa Kelebihan Jam ke hari
        self.hour %= 24;

        // Penambahan Sisa kekurangan Jam dari hari
        if self.hour < 0 {
            self.hour = 24 + self.hour;
        }
    }

    /// Fungsi Penambahan `menit` ke dalam clock. 
    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.add_hours(minutes / 60);
        self.minute += minutes % 60;

        // Sisa Kelebihan menit ke jam
        if self.minute > 60 {
            self.minute %= 60;
            self.add_hours(1);
        }

        // Sisa Kekurangan menit dari jam
        if self.minute < 0 {
            self.minute = 60 + self.minute;
            self.add_hours(-1);
        }
        self
    }
}

// Penjelasan Fungsi add_minutes (dan add_hours)
// Input dapat berupa bilangan positif dan negatif,
// Sehingga dapat menyebabkan sisa kelebihan dam kekurangan menit/jam.
// Sisa kelebihan dan kekurangan akan merubah jumlah jam yang ada.
// Dengan menggunakan clock untuk mengecek dan meminjam/menambah menit ketika fungsi Clock::new().add_minutes() berjalan
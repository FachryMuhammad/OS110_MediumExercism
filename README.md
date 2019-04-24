# OS110_MediumExercism
This is my take on the second task of OS110 class


 # List of Medium Problem i have chosen :
 1. [Triangle](https://exercism.io/my/solutions/72f5e8ab512d45a0bdb9fb18a146dea8)
 2. [Clock](https://exercism.io/my/solutions/bf353ef066724d2f9ef1d764ac76bfcd)
 3. [Robot Simulator](https://exercism.io/my/solutions/7c8b0b363468407ebfe986a46b8eec32)
 4. [Perfect Numbers](https://exercism.io/my/solutions/c67fad99f313414494c1b9ccdaf23e6f)
 
 # List of Beginner Problem i have chosen :
 1. [Reverse String](https://exercism.io/my/solutions/8c652b9cf5f645e6b3d1ba05af96ec1d)
 2. [Raindrop](https://exercism.io/my/solutions/d9501c6489da4bb6baf5985ca2a6ef3e)
 4. [Nth Prime](https://exercism.io/my/solutions/4e90e0da3e794c92941e7d11e60fde90)
 5. [Beer Song](https://exercism.io/my/solutions/7ccf5e696dec4ffbb49d52c5050024db)
 6. [Proverb](https://exercism.io/my/solutions/1a454ddd0f7b4f1fb67e8ba459207472)
 7. [Difference of Squares](https://exercism.io/my/solutions/f63223908aff431e8575afd19a7ff9d8)
 8. [Sum Of Multiplies](https://exercism.io/my/solutions/aaf2c329f0e24e7b848ddc93431cbe43)
 9. [Grains](https://exercism.io/my/solutions/b732ef6c52d447878d6760f6bd40953f)
 10. [Leap](https://exercism.io/my/solutions/f69e4fe7b30b468d818fb6783208bce0)
 11. [Hello World!](https://exercism.io/my/solutions/da0f0fdcad1c4e71a92edea6efa1737d)
 
 
## Essay of "Robot Simulator" Task
 This essay's purpose is to study and research the 'Robot Simulator' Task from exercism Rust Track's.
 
 
### Introduction of the Task

Write a robot simulator.

A robot factory's test facility needs a program to verify robot movements.

The robots have three possible movements:

- turn right
- turn left
- advance

Robots are placed on a hypothetical infinite grid, facing a particular
direction (north, east, south, or west) at a set of {x,y} coordinates,
e.g., {3,8}, with coordinates increasing to the north and east.

The robot then receives a number of instructions, at which point the
testing facility verifies the robot's new position, and in which
direction it is pointing.

- The letter-string "RAALAL" means:
  - Turn right
  - Advance twice
  - Turn left
  - Advance once
  - Turn left yet again
- Say a robot starts at {7, 3} facing north. Then running this stream
  of instructions should leave it at {9, 4} facing west.
  
  
 ### Identification of Problem
  
 From the introduction above we know that:
  
 1. The robot has two sets of parameter(directional and positional)
 
    - Directional parameter is the current direction of the robot is heading
      ex: `{North, east, south, and west}`

    - Positional parameter is the current direction of the robot is at
      ex: `{point_x, point_y}`
     
 2. The robot has two sets of instruction(directional and positional) -> Has three instruction in total 
 
    - Directional instruction is to change the current direction of the robot is heading
      ex: `{turn_left, turn_right]` -> only turn left or right

    - Positional instuction is the change current direction of the robot is at
      ex: `{advance}` -> only move in one direction by one point

      so, the robot has three kinds of instructions in total.
     
 3. The program has a function to read string of character in order to control the robot
      ex: `{"RAALAL"}`-> only three sets of characters/commands

    Based on those set of data that we know, we can start to work on this problem. 
  
 ### Starting point to Approach to the problem
  
  To solve the problem we need to construct those sets of parameter and instruction.
  
  1. Starting with defining the direction with `enumeration`
  
      ```
       pub enum Direction {
         North,
         East,
         South,
         West,
       }
      ```
  
  2. And then we are to define the basic structure or `struct` 
  
      ```
       pub struct Robot {
         point_x: i32,
         point_y: i32,
         dir: Direction,
       }
      ```
  
  3. Define the mainbody of all the function
  
     ```
     impl Robot {
         pub fn new(x: i32, y: i32, d: Direction) -> Self {
             Robot {
        point_x: x,
        point_y : y,
        dir : d,
       }
     }
     ```
     
  4. Complete and arrange all the function required according to the robot main function
  
  
### Logical Approach to the problem
  
  
  There are basically six set of function required in order to complete this task:
  
  1. Turning Right Function or `turn_right`
      ```
      pub fn turn_right(mut self) -> Self {
        // This function will replace the old direction, with the new direction after the robot turns right
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            };
     	self
      }
      ```
      
      `turn_right` allow the program to change the old set of direction, with the new one relative by turning/going to the right 
      
  2.Turning left Function or `turn_left`
      ```
     pub fn turn_left(mut self) -> Self {
        // This function will replace the old direction, with the new direction after the robot turns left
        self.dir = match self.dir {
           Direction::North => Direction::West,
           Direction::East => Direction::North,
           Direction::South => Direction::East,
           Direction::West => Direction::South,
           };
      self
      }
      ```
      
      `turn_left` allow the program to change the old set of direction, with the new one relative by turning/going to the left
      
  3. Advance function of `advance`
      ```
      pub fn advance(mut self) -> Self {
        // when advancing, value of point_x will increase/decrease based on the 'Direction' relative to the cartesian diagram value
        self.point_x+= match self.dir {
               Direction::East => 1,
               Direction::West => -1,
               _ => 0,
              };
        // when advancing, value of point_y will increase/decrease based on the 'Direction' relative to the cartesian diagram value
        self.point_y += match self.dir {
               Direction::North => 1,
               Direction::South => -1,
               _ => 0,
              };
         self
      }
      ```
      `advance` allow the program to relocate the location of the robot by one point of unit of movement (to the front). this movement is relative to the direction of the robot currently heading. The value/result of the relocation is determent by the relative position to the cartesian diagram value or the directional compass heading.
      
      ```
      // Illustration / point of referrence of compass wind direction relative to the cartesian diagram value
      //
      //                         N                          +Y
      //                       W + E                     -X  0 +X
      //                         S                          -Y
      //  
      ```
     
      
      
  4. Reading Instruction function or `instruction`
      ```
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
      ```
      
      `instruction` will allow the program to read string of character in order to represent the robot's instruction.
      
   5. Positioning Function or `position`
      ```
       pub fn position(&self) -> (i32, i32) {
        (self.point_x, self.point_y)
      }
      ```
      
      `position` will allow the program to store the current position of the robot
   
   6. Directioning Function or `direction`
      ```
      pub fn direction(&self) -> &Direction {
        &self.dir
      }
      ```
      
      `direction` will allow the program to store the current direction of the robot heading

  ### The Final Result / Code

          ```
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
          ```


# Conclusion

Rust is intended to be a language for highly concurrent and highly safe systems, and "programming in the large", that is, creating and maintaining boundaries that preserve large-system integrity. It is important to master the syntax of Rust lang in order to fully understand the full capability and functionality of this programming language.

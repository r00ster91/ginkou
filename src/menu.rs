use std::io::{self, Read};

pub mod terminal;

pub type Option = &'static str;

pub struct Menu {
    pub options: Vec<Option>,
    index: usize,
    length: usize,
}

// Maybe these will be used in the future
// const ARROW_KEY_UP: &[u8; 3] = b"\x1B[A";
// const ARROW_KEY_DOWN: &[u8; 3] = b"\x1B[B";

impl Menu {
    pub fn new(options: Vec<Option>) -> Self {
        Menu {
            length: options.len() - 1,
            options,
            index: 0,
        }
    }

    pub fn read(&mut self) -> Option {
        terminal::echo(&io::stdin(), false);

        loop {
            for (index, option) in self.options.iter().enumerate() {
                if self.index == index {
                    // https://en.wikipedia.org/wiki/Geometric_Shapes
                    print!("▶ ");
                } else {
                    print!("  ");
                }
                println!("{}", option);
            }

            let mut buffer = [0];
            io::stdin().read_exact(&mut buffer).unwrap();

            let key = buffer[0].to_ascii_uppercase();

            if key == b'W' {
                self.index = if self.index == 0 {
                    self.length
                } else {
                    self.index - 1
                }
            } else if key == b'S' {
                self.index = if self.index == self.length {
                    0
                } else {
                    self.index + 1
                }
            } else if key == b'\n' {
                terminal::echo(&io::stdin(), true);
                return self.options[self.index];
            }

            // Move the cursor up
            println!("\x1B[{}A", self.length + 2);
        }
    }
}

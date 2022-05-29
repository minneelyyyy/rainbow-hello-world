
use mincolors::colors;
use std::io::Write;

fn main() {
    let text = "Hello, World!";
    let increment: u16 = 10;
    let mut hue: u16 = 0;
    let mut index: u16 = 0;

    loop {
        for character in text.chars() {
            let color = colors::hsv_to_rgb(colors::HSV { h: (hue + increment * index) % 360, s: 60, v: 80 });
            print!("{}", mincolors::rgb!(color.r, color.g, color.b, character));

            index += 1;
        }

        std::io::stdout().flush().expect("failed to flush stdout");
        std::thread::sleep(std::time::Duration::from_millis(100));

        hue = hue + increment;
        index = 0;

        print!("\r");
    }
}

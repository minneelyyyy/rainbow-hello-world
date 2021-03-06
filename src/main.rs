
use mincolors::colors;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let text = if args.is_empty() { "Hello, World!" } else { &args };
    let increment: u16 = 5;
    let mut index: u16 = 0;
    let mut hue: u16 = 0;
    
    loop {
        for character in text.chars() {
            let color = colors::hsv_to_rgb(
                colors::HSV {
                    h: (hue + increment * index) % 360,
                    s: 60,
                    v: 80
                }
            );

            print!("{}", mincolors::rgb!(color.r, color.g, color.b, character));

            index += 1;
        }

        std::io::stdout().flush().expect("failed to flush stdout");
        std::thread::sleep(std::time::Duration::from_millis(100));

        hue = (hue + increment) % 360;
        index = 0;

        print!("\r");
    }
}

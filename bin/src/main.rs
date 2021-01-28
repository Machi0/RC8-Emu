use lib::chip8::Chip8;
use lib::gui::Gui;
use std::time::SystemTime;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("Invalid Arguments \nEnter: [ROM path] Optional{[Resolution Scale] [Delay]}");
    }
    let rom_path = &args[1];
    let mut scale: u32 = 10;
    let mut delay: u128 = 4;

    if args.len() == 4 {
        scale = args[2].parse().unwrap();
        delay = args[3].parse().unwrap();
    }


    let mut gui = Gui::new(scale); 

    let mut c8 = Chip8::new();
    c8.load_fontset();
    c8.load_rom(rom_path);

    let mut quit: bool = false;

    let start = SystemTime::now();
    let mut last_time = start.elapsed().unwrap();

    while !quit {
        quit = gui.process_input(&mut c8.keypad);

        let current_time = start.elapsed().unwrap();
        let delay_time = current_time.as_millis() - last_time.as_millis();

        if delay_time > delay {
            last_time = current_time;
            c8.interpret();
            gui.render_frame(&c8.gfx);
        }
    }
}


use lib::chip8::Chip8;
use std::time::SystemTime;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut c8 = Chip8::new();
    c8.load_fontset();
    c8.load_rom("./ROMS/Stars [Sergey Naydenov, 2010].ch8");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("RC8-Emu", 64*6, 32*6) //Multiply by scale
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(64, 32).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut quit: bool = false;

    let start = SystemTime::now();
    let mut last_time = start.elapsed().unwrap();

    while !quit {
        quit = process_input(&mut event_pump, &mut c8.keypad);

        let current_time = start.elapsed().unwrap();
        let delay_time = current_time.as_millis() - last_time.as_millis();

        if delay_time > 2 {
            last_time = current_time;
            c8.interpret();
            render_frame(&mut canvas, &c8.gfx);
        }
    }
}

fn process_input(event_pump: &mut sdl2::EventPump, keypad: &mut [u8]) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit{..} => return true,
            Event::KeyDown{keycode: Some(Keycode::Escape), ..} => return true,

            Event::KeyDown{keycode: Some(Keycode::X), ..}    => {keypad[0] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Num1), ..} => {keypad[1] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Num2), ..} => {keypad[2] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Num3), ..} => {keypad[3] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Q), ..}    => {keypad[4] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::W), ..}    => {keypad[5] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::E), ..}    => {keypad[6] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::A), ..}    => {keypad[7] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::S), ..}    => {keypad[8] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::D), ..}    => {keypad[9] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Z), ..}    => {keypad[0xA] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::C), ..}    => {keypad[0xB] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::Num4), ..} => {keypad[0xC] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::R), ..}    => {keypad[0xD] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::F), ..}    => {keypad[0xE] = 1; break;},
            Event::KeyDown{keycode: Some(Keycode::V), ..}    => {keypad[0xF] = 1; break;},


            Event::KeyUp{keycode: Some(Keycode::X), ..}    => {keypad[0] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Num1), ..} => {keypad[1] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Num2), ..} => {keypad[2] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Num3), ..} => {keypad[3] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Q), ..}    => {keypad[4] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::W), ..}    => {keypad[5] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::E), ..}    => {keypad[6] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::A), ..}    => {keypad[7] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::S), ..}    => {keypad[8] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::D), ..}    => {keypad[9] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Z), ..}    => {keypad[0xA] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::C), ..}    => {keypad[0xB] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::Num4), ..} => {keypad[0xC] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::R), ..}    => {keypad[0xD] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::F), ..}    => {keypad[0xE] = 0; break;},
            Event::KeyUp{keycode: Some(Keycode::V), ..}    => {keypad[0xF] = 0; break;},

            _ => {}
        }
    }

    return false;
}

fn render_frame(canvas: &mut sdl2::render::Canvas<Window>, screen: &[u8]) -> () {
    canvas.set_draw_color(Color::RGB(156, 159, 76));
    canvas.clear();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    canvas.set_draw_color(Color::RGB(57, 74, 30));

    for i in screen.iter() {
        if *i != 0 {
            canvas.draw_point(Point::new(x, y)).unwrap();
        }

        x += 1;
        if x == 64 {
            x = 0;
            y += 1;
        }
    }

    canvas.present();
}
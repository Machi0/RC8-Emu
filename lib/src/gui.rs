use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Gui {
    canvas: sdl2::render::Canvas<Window>,
    event_pump: sdl2::EventPump,
}

impl Gui {
    pub fn new(scale: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("RC8-Emu", 64*scale, 32*scale) //Multiply by scale
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(64, 32).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        Gui {
            canvas: canvas,
            event_pump: event_pump,
        }
    }

    pub fn render_frame(&mut self, screen: &[u8]) -> () {
        self.canvas.set_draw_color(Color::RGB(156, 159, 76));
        self.canvas.clear();

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        self.canvas.set_draw_color(Color::RGB(57, 74, 30));

        for i in screen.iter() {
            if *i != 0 {
                self.canvas.draw_point(Point::new(x, y)).unwrap();
            }

            x += 1;
            if x == 64 {
                x = 0;
                y += 1;
            }
        }

        self.canvas.present();
    }

    pub fn process_input(&mut self, keypad: &mut [u8]) -> bool {
        for event in self.event_pump.poll_iter() {
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
}
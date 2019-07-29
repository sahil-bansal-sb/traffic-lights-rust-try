use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum LightColor {
    RED,
    YELLOW,
    GREEN,
}

#[derive(Debug)]
pub struct LightFace {
    active_color: LightColor,
    active_time: u32,
    time: u32,
}

impl LightFace {
    pub fn new(active_color: LightColor, active_time:u32, time: u32) -> LightFace {
        LightFace {
            active_color,
            active_time,
            time,
        }
    }

    pub fn get_active_color_as_str (&self) -> &str {
        match self.active_color {
            LightColor::RED => "🛑",
            LightColor::YELLOW => "🔔",
            LightColor::GREEN => "✔️",
        }
    }

    pub fn get_active_color (&self) -> LightColor {
        match self.active_color {
            LightColor::RED => LightColor::RED,
            LightColor::YELLOW => LightColor::YELLOW,
            LightColor::GREEN => LightColor::GREEN,
        }
    }

    pub fn get_active_time (&self) -> u32 {
        self.active_time
    }

    pub fn set_active_color (&mut self, new_state: LightColor) {
        self.active_color = new_state;
    }

    pub fn set_active_time (&mut self, new_time: u32) {
        self.active_time = new_time;
    }

    pub fn start(&mut self, no: u32) {
            // println!("printing the state of {}", no);
            // thread::sleep(Duration::from_secs(2));
            // println!("{} state is : {:?}", no, self);
        loop{
            thread::sleep(Duration::from_secs(1));
            self.active_time -= 1000;
            if self.active_time == 0 {
                println!("switching the light {}", no);
                println!("from {}", self.get_active_color_as_str());
                self.switch();
                println!("to {}", self.get_active_color_as_str());
            }
        }
    }

    pub fn clone(&self) -> LightFace{
        LightFace{
            active_time: self.active_time,
            active_color: self.get_active_color(),
            time: self.time,
        }
    }

    pub fn switch(&mut self) {
        match self.active_color {
            LightColor::RED => {
                // wait for the time and switch
                self.active_color = LightColor::GREEN;
                self.active_time = self.time - 2000;
            },
            LightColor::GREEN => {
                self.active_color = LightColor::YELLOW;
                self.active_time = 2000;
            },
            LightColor::YELLOW => {
                self.active_color = LightColor::RED;
                self.active_time = self.time*3;
            },
        };
    }
}
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
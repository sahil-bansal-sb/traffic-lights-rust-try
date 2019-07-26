mod trafic_light;

pub use trafic_light::LightFace;
pub use trafic_light::LightColor;

#[derive(Debug)]
pub struct TraficLight{
    time: u32,
    lights: [LightFace; 4],
}

// let's say, if time-period of a road junction is t secs
// then red will be up for (t*3-2) sec
// and  yellow will be up for (2) sec
// and green will be up for t sec

impl TraficLight {
    pub fn new (time: u32) -> TraficLight {
        let t1 = LightFace::new(LightColor::RED, time*3, time);
        let t2 = LightFace::new(LightColor::RED, time*2, time);
        let t3 = LightFace::new(LightColor::RED, time, time);
        let t4 = LightFace::new(LightColor::GREEN, time - 2000, time);

        let lights: [LightFace; 4] = [t1, t2, t3, t4];

        // create 4 threads, one for each Light Face

        TraficLight{
            time,
            lights,
        }
    }

    pub fn printStateOfAll(&self){
        println!("Light one: {:#?}", self.lights[0]);
        println!("Light two: {:#?}", self.lights[1]);
        println!("Light three: {:#?}", self.lights[2]);
        println!("Light four: {:#?}", self.lights[3]);
    }
}
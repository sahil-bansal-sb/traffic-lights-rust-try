mod trafic_light;

use std::thread;
use std::time::Duration;

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

    fn startClock(&self){
        let mut time :u64 = 0;
        loop{
            thread::sleep(Duration::from_secs(1));
            print!("{}", time);
            time += 1;
        }
    }

    pub fn start(&self) {
        println!("starting the lights");
        let mut handlers : Vec<thread::JoinHandle<_>> = Vec::with_capacity(4);
        for i in 0..4 {
            let mut tf = self.lights[i].clone();
            let handle = thread::spawn(move || {
                tf.start(i as u32);
            });
            handlers.push(handle);
        }
        let mut count = 0;
        let mut handleCount = thread::spawn(move || {
            loop{
                thread::sleep(Duration::from_secs(1));
                count += 1;
                println!("{}", count);
            }
        });
        println!("started the lights");
        println!("joining all handlers");
        for handle in handlers{
            handle.join().unwrap();
        }
        handleCount.join().unwrap();
        println!("joined");
    }

    // need to start different thread for each of the light face

    pub fn printStateOfAll(&self){
        println!("Light one: {:#?}", self.lights[0]);
        println!("Light two: {:#?}", self.lights[1]);
        println!("Light three: {:#?}", self.lights[2]);
        println!("Light four: {:#?}", self.lights[3]);
    }
}
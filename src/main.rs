// mod trafic_light;

// pub use trafic_light::LightFace;
// pub use trafic_light::LightColor;

mod trafic_lights;

pub use trafic_lights::TraficLight;

fn main() {
    let t1 = TraficLight::new(15000);
    t1.printStateOfAll();
    t1.start();
}

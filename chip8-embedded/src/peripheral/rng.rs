use microbit::hal::rng::Rng as HalRng;

pub struct Rng {
    rng: HalRng,
}

impl Rng {

    pub fn new(hardware_rng: microbit::pac::RNG) -> Self {
        let rng = HalRng::new(hardware_rng);
        Rng { rng }
    }

    pub fn gen_random_byte(&mut self) -> u8 {
        self.rng.random_u8()
    }
}

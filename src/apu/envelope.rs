const MAX_DECAY: u8 = 15;

struct Devider {
    reload_value: u8,
    counter: u8,
}

impl Devider {
    fn new() -> Devider {
        Devider {
            reload_value: 0,
            counter: 0,
        }
    }

    fn tick(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = self.reload_value;
            true
        } else {
            self.counter -= 1;
            false
        }
    }

    fn reset(&mut self) {
        self.counter = self.reload_value;
    }

    fn set_reload_value(&mut self, value: u8) {
        self.reload_value = value;
    }
}

struct DecayCounter {
    decay: u8,
    loop_flag: bool,
}

impl DecayCounter {
    fn new() -> DecayCounter {
        DecayCounter {
            decay: MAX_DECAY,
            loop_flag: false,
        }
    }

    fn reset_decay(&mut self) {
        self.decay = MAX_DECAY;
    }

    fn tick(&mut self) {
        if self.decay > 0 {
            self.decay -= 1;
        } else if self.loop_flag {
            self.decay = MAX_DECAY;
        }
    }

    fn decay(&self) -> u8 {
        self.decay
    }

    fn set_loop_flag(&mut self, flag: bool) {
        self.loop_flag = flag;
    }
}

pub struct Envelope {
    devider: Devider,
    decay: DecayCounter,
    start_flag: bool,
    constant_volume: u8,
    constant_flag: bool,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            devider: Devider::new(),
            decay: DecayCounter::new(),
            start_flag: true,
            constant_volume: 0,
            constant_flag: false,
        }
    }

    pub fn set_volume(&mut self, v: u8) {
        self.devider.set_reload_value(v);
        self.constant_volume = v;
    }

    pub fn set_constant_flag(&mut self, flag: bool) {
        self.constant_flag = flag;
    }

    pub fn set_loop_flag(&mut self, flag: bool) {
        self.decay.set_loop_flag(flag);
    }

    pub fn set_start_flag(&mut self) {
        self.start_flag = true;
    }

    pub fn tick(&mut self) {
        if self.start_flag {
            self.start_flag = false;
            self.decay.reset_decay();
            self.devider.reset();
            return;
        }

        if self.devider.tick() {
            self.decay.tick();
        }
    }

    pub fn volume(&self) -> f32 {
        let decay = if self.constant_flag {
            self.constant_volume
        } else {
            self.decay.decay()
        };
        f32::from(decay) / f32::from(MAX_DECAY)
    }
}

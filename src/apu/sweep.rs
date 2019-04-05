use super::divider::Divider;
use super::timer::Timer;

#[derive(Clone, Copy)]
pub enum NegateMode {
    OneComplement,
    TwoComplement,
}

pub struct Sweep {
    devider: Divider,
    negate_flag: bool,
    enable_flag: bool,
    shift_counter: u8,
    reload_flag: bool,
    negate_mode: NegateMode,
}

impl Sweep {
    pub fn new(negate_mode: NegateMode) -> Sweep {
        Sweep {
            devider: Divider::new(),
            negate_flag: false,
            enable_flag: false,
            shift_counter: 0,
            reload_flag: false,
            negate_mode,
        }
    }

    pub fn tick(&mut self, timer: &mut Timer) {
        if self.devider.tick() && self.enable_flag && !self.is_mute(&timer) {
            timer.set_period(self.target_period(&timer));
        }
        if self.reload_flag {
            self.reload_flag = false;
            self.devider.reset();
        }
    }

    pub fn is_mute(&self, timer: &Timer) -> bool {
        self.target_period(timer) > 0x7ff
    }

    pub fn target_period(&self, timer: &Timer) -> u16 {
        let amount = timer.period() >> self.shift_counter;
        match (self.negate_flag, self.negate_mode) {
            (false, _) => timer.period() + amount,
            (true, NegateMode::OneComplement) => timer.period() - amount + 1,
            (true, NegateMode::TwoComplement) => timer.period() - amount,
        }
    }

    pub fn store(&mut self, val: u8) {
        self.reload_flag = true;

        let enable = (val & 0x80) != 0;
        self.enable_flag = enable;
        let p = (val >> 4) & 0x7;
        self.devider.set_reload_value(p);
        let n = ((val >> 3) & 0x1) != 0;
        self.negate_flag = n;
        let s = val & 0x7;
        self.shift_counter = s;
    }
}

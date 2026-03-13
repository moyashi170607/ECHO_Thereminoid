#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use ch32_hal::time::Hertz;
use ch32_hal::timer::low_level::CountingMode;
use ch32_hal::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_time::Instant;
use hal::delay::Delay;
use hal::gpio::{Input, Level, Output, Pull};
use {ch32_hal as hal, panic_halt as _};

use echo_music::{
    DISTANCE_CM_MAX, DISTANCE_CM_MIN, TimerUs, distance_to_freq, measure_distance_cm,
};

struct InstantTimer {
    et: embassy_time::Instant,
}

impl TimerUs for InstantTimer {
    fn now_us(&mut self) -> u64 {
        self.et.elapsed().as_micros()
    }
}

#[qingke_rt::entry]
fn main() -> ! {
    //ピンの初期化
    let p = hal::init(hal::Config::default());

    let mut delay = Delay;

    // ブザー (PC3, TIM1 CH3)
    let buzzer_pin = PwmPin::new_ch3::<0>(p.PC3);
    let mut pwm = SimplePwm::new(
        p.TIM1,
        None,
        None,
        Some(buzzer_pin),
        None,
        Hertz::hz(440),
        CountingMode::default(),
    );
    let ch = hal::timer::Channel::Ch3;
    pwm.set_duty(ch, pwm.get_max_duty() / 2);
    pwm.enable(ch);

    // HC-SR04
    let mut trig = Output::new(p.PD0, Level::Low, Default::default());
    let mut echo = Input::new(p.PC6, Pull::Down);

    //起動からの時間
    let mut boot_time = InstantTimer { et: Instant::now() };

    loop {
        // 距離に応じてブザー周波数を変える
        if let Some(distance_cm) =
            measure_distance_cm(&mut trig, &mut echo, &mut delay, &mut boot_time)
        {
            if (distance_cm <= DISTANCE_CM_MAX) && (distance_cm >= DISTANCE_CM_MIN) {
                let freq_hz = distance_to_freq(distance_cm);
                pwm.set_frequency(Hertz::hz(freq_hz));
                pwm.set_duty(ch, pwm.get_max_duty() / 2);
                pwm.enable(ch);
            } else {
                pwm.disable(ch);
            }
        } else {
            pwm.disable(ch);
        }

        // 次の測定まで最低 60ms 待つ（HC-SR04 の推奨インターバル）
        delay.delay_ms(60);
    }
}

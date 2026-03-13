#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

//音がなる範囲(cm)
pub const DISTANCE_CM_MAX: u64 = 40;
pub const DISTANCE_CM_MIN: u64 = 0;

//鳴らす音の範囲(Hz)
pub const FREQ_HZ_MAX: u64 = 2000;
pub const FREQ_HZ_MIN: u64 = 100;

pub trait TimerUs {
    //起動から経過した時間をマイクロ秒単位で返す
    fn now_us(&mut self) -> u64;
}

pub fn distance_to_freq(distance_cm: u64) -> u32 {
    return (FREQ_HZ_MAX - distance_cm * (FREQ_HZ_MAX - FREQ_HZ_MIN) / DISTANCE_CM_MAX) as u32;
}

// HC-SR04 で距離を測定
pub fn measure_distance_cm<O: OutputPin, I: InputPin, D: DelayNs, T: TimerUs>(
    trig: &mut O,
    echo: &mut I,
    delay: &mut D,
    boot_time: &mut T,
) -> Option<u64> {
    //ECHO が HIGH になるまで待つときのタイムアウト
    const WAIT_TIMEOUT: u64 = 10_000;
    //ECHO が HIGH のままのときのタイムアウト
    const ECHO_TIMEOUT: u64 = 25_000;
    //音の速度(cm/s)
    const CM_PER_SECOND: u64 = 34_300;
    //1cm あたりのマイクロ秒数
    const US_PER_CM: u64 = 1_000_000 / CM_PER_SECOND;

    // Trig: 10µs パルスを送信
    let _ = trig.set_high();
    delay.delay_us(10);
    let _ = trig.set_low();

    // ECHO が HIGH になるまで待つ
    let wait_start_us = boot_time.now_us();
    while echo.is_low().unwrap_or(false) {
        if (boot_time.now_us() - wait_start_us) > WAIT_TIMEOUT {
            return None;
        }
    }

    // ECHO が HIGH の間を計測
    let echo_start_us = boot_time.now_us();
    while echo.is_high().unwrap_or(false) {
        if (boot_time.now_us() - echo_start_us) > ECHO_TIMEOUT {
            return None;
        }
    }

    let duration_us = boot_time.now_us() - echo_start_us;
    Some(duration_us / US_PER_CM / 2)
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate;

    if speed <= 4 {
        success_rate = 1.0;
    } else if speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    221.0 * success_rate * speed as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

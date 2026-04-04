const YEAR_TO_SECONDS: f64 = 31557600.0;

macro_rules! impl_planet {
     ( $( $t:ident, $x:expr ),* ) => {
        $(
            pub struct $t;

            impl Planet for $t {

                fn years_during(d: &Duration) -> f64 {
                    (d.seconds as f64 / YEAR_TO_SECONDS) / $x as f64
                }
            }
        )*
    }
}

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {

    fn from(seconds: u64) -> Self {
        Self {
            seconds
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

impl_planet!(
    Mercury, 0.2408467,
    Venus, 0.61519726,
    Earth, 1,
    Mars, 1.8808158,
    Jupiter, 11.862615,
    Saturn, 29.447498,
    Uranus, 84.016846,
    Neptune, 164.79132
);

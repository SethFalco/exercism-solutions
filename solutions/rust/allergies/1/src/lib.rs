/// Creates an enum paired with an iterable that contains all elements in the enum.
macro_rules! iterable_enum {
    ( $enum_name:ident, $iter_name:ident, $num:expr, $( $t:ident, $x:expr ),* ) => {
        static $iter_name: [$enum_name; $num] = [
            $(
                $enum_name::$t,
            )*
        ];

        #[repr(u32)]
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $enum_name {
            $(
                $t = $x,
            )*
        }
    };
}

iterable_enum!(Allergen, ALLERGENS, 8,
    Eggs, 1,
    Peanuts, 2,
    Shellfish, 4,
    Strawberries, 8,
    Tomatoes, 16,
    Chocolate, 32,
    Pollen, 64,
    Cats, 128
);

pub struct Allergies {
    score: u32
}

impl Allergies {

    pub fn new(score: u32) -> Self {
        Self {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score | *allergen as u32 == self.score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter().filter(|x| self.is_allergic_to(x)).cloned().collect()
    }
}

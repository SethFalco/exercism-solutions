pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {

    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let mana = match self.mana {
            Some(_) => Some(100 as u32),
            None => None
        };

        let player = Player {
            health: 100,
            mana,
            level: self.level
        };

        Some(player)
    }

    /// Returns damage dealt by the player.
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            if self.health < mana_cost {
                self.health = 0
            } else {
                self.health = self.health - mana_cost
            };

            return 0;
        }

        let mana = self.mana.unwrap();

        if mana < mana_cost {
            return 0;
        }

        self.mana = Some(mana - mana_cost);

        mana_cost * 2
    }
}

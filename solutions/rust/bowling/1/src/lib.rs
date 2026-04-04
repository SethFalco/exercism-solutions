use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    /// All throws that occured this game.
    throw_history: Vec<u16>,

    /// Indexes of throws that are valued at double points.
    bonuses: Vec<usize>,

    /// Current frame of the game, 1..=10.
    frame: u16,

    /// Current throw of the frame, between 1..=4.
    throws: u16,

    /// Number of pins remaining in the frame.
    pins: u16,

    /// Extra throws earned during the game.
    extra: u16,

    /// Game is finished.
    done: bool
}

impl BowlingGame {

    pub fn new() -> Self {
        Self {
            throw_history: Vec::new(),
            bonuses: Vec::new(),
            frame: 1,
            throws: 0,
            pins: 10,
            extra: 0,
            done: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.done {
            return Err(Error::GameComplete);
        }

        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.throw_history.push(pins);
        self.throws += 1;
        self.pins -= pins;

        if self.extra != 0 {
            self.extra -= 1;

            if self.pins == 0 {
                self.pins = 10;
            }

            if self.extra == 0 {
                self.done = true;
            }

            return Ok(());
        }

        if self.throws == 1 && self.pins == 0 {
            if self.frame != 10 {
                self.bonuses.push(self.throw_history.len());
                self.bonuses.push(self.throw_history.len() + 1);
                self.finish_frame(0);
            } else {
                self.finish_frame(2);
            }
        } else if self.throws == 2 {
            if self.pins == 0 {
                if self.frame != 10 {
                    self.bonuses.push(self.throw_history.len());
                    self.finish_frame(0);
                } else {
                    self.finish_frame(1);
                }
            } else {
                self.finish_frame(0);
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.done {
            return None;
        }

        let mut score = self.throw_history.iter().fold(0, u16::add);

        score += self.bonuses.iter().fold(0, |acc, &i| {
            if self.throw_history.len() > i {
                return acc + self.throw_history[i]
            }

            acc
        });

        Some(score)
    }

    fn finish_frame(&mut self, extra: u16) {
        self.pins = 10;

        if extra == 0 {
            self.throws = 0;

            if self.frame == 10 {
                self.done = true;
            }
        } else {
            self.extra = extra;
        }

        if self.frame != 10 {
            self.frame += 1;
        }
    }
}

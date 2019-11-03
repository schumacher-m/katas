pub struct Game {
    frame: u16,
    frames: [Frame; 10],
    roll: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    score: [u16; 2],
    bonus_score: [u16; 2],
    spare: bool,
    strike: bool
}

impl Frame {
    pub fn score(&self) -> u16 {
        let score:u16 = self.score.iter().sum();
        let bonus_score:u16 = self.bonus_score.iter().sum();
        score + bonus_score
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            frame: 0,
            frames: [Frame { score: [0, 0], bonus_score: [0, 0], spare: false, strike: false }; 10],
            roll: 0,
        }
    }

    pub fn roll(&mut self, score: u16) {
        if self.frame >= 10 {
            self.frames[9].bonus_score[self.roll as usize] = score;
            self.roll += 1;
        } else {
            self.frames[self.frame as usize].score[self.roll as usize] = score;
            if self.roll == 1 {
                let current_score = self.frames[self.frame as usize].score[0];
                if (current_score + score) == 10 {
                    self.frames[self.frame as usize].spare = true;
                }
                self.frame += 1;
                self.roll = 0;
            } else if self.roll == 0 {
                self.roll = 1;
                if score == 10 {
                    self.frames[self.frame as usize].strike = true;
                    self.frame += 1;
                    self.roll = 0;
                }
            }
        }
    }

    pub fn score(&self) -> u16 {
        println!("{:?}", self.frames);
        let mut score = 0;
        for (i, frame) in self.frames.iter().enumerate() {
            score += frame.score();
            println!("INdex: {:?}", i);
            if frame.spare {
                if i < 8 {
                    score += self.frames[i+1].score[0];
                }
            } else if frame.strike {
                if i < 7 {
                    score += self.frames[i+1].score[0];
                    score += self.frames[i+2].score[0];
                } else if i < 8 {
                    score += self.frames[i+1].score[0];
                    score += self.frames[9].bonus_score[0];
                } else if i == 9 {
                    score += self.frames[9].bonus_score[0];
                    score += self.frames[9].bonus_score[1];
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod bowling {
    use super::*;

    #[test]
    fn return_frame_count() {
        let mut game = Game::new();
        game.roll(0);
        game.roll(0);
        game.roll(0);
        game.roll(0);
        assert_eq!(game.frame, 2);
        assert_eq!(game.roll, 0);
        game.roll(0);
        game.roll(0);
        game.roll(0);
        game.roll(0);
        assert_eq!(game.frame, 4);
    }

    #[test]
    fn set_spare_flag() {
        let mut game = Game::new();
        game.roll(5);
        game.roll(5);
        assert_eq!(game.frames[0].spare, true);
    }

    #[test]
    fn set_strike_flag() {
        let mut game = Game::new();
        game.roll(10);
        game.roll(10);
        assert_eq!(game.frames[0].strike, true);
        assert_eq!(game.frames[1].strike, true);
    }

    #[test]
    fn zero_score_on_all_empty_throws() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn set_frame_score() {
        let mut game = Game::new();
        game.roll(1);
        game.roll(2);
        game.roll(3);
        assert_eq!(game.frames[0].score[0], 1);
        assert_eq!(game.frames[0].score[1], 2);
        assert_eq!(game.frames[1].score[0], 3);
    }

    #[test]
    fn all_single_score_throws() {
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(1);
        }
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn single_spare_score() {
        let mut game = Game::new();

        game.roll(5);
        game.roll(5);
        game.roll(6);
        game.roll(1);

        for _ in 0..20 - 4 {
            game.roll(0);
        }
        assert_eq!(game.score(), 16 + 7);
    }

    #[test]
    fn all_spare_game () {
        let mut game = Game::new();
        for _ in 0..22 {
            game.roll(5);
        }
        assert_eq!(game.score(), 150);
    }

    #[test]
    fn perfect_game_score() {
        let mut game = Game::new();
        for _ in 0..12 {
            game.roll(10);
        }
        assert_eq!(game.score(), 300);
    }
}

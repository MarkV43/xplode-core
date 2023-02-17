use rand::{rngs::StdRng, thread_rng, Rng, RngCore, SeedableRng};

use crate::board::Board;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileValue {
    Bomb,
    Safe(u8),
}

impl Default for TileValue {
    fn default() -> Self {
        Self::Safe(0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TileState {
    Flag,
    #[default]
    Hidden,
    Open,
}

#[derive(Clone, Default, Debug)]
pub struct Tile {
    pub value: TileValue,
    pub state: TileState,
}

impl Tile {
    pub fn new(value: TileValue, state: TileState) -> Self {
        Self { value, state }
    }
}

pub struct Game {
    board: Board<Tile>,
}

impl Game {
    pub fn new(width: usize, height: usize, bombs: usize) -> Self {
        Self::new_func(width, height, bombs, thread_rng(), |_, _| true)
    }

    pub fn new_safe(width: usize, height: usize, bombs: usize, cx: usize, cy: usize) -> Self {
        Self::new_func(width, height, bombs, thread_rng(), |x, y| {
            (x, y) != (cx, cy)
        })
    }

    pub fn new_safe_zero(width: usize, height: usize, bombs: usize, cx: usize, cy: usize) -> Self {
        Self::new_func(width, height, bombs, thread_rng(), |x, y| {
            x.abs_diff(cx) > 1 || y.abs_diff(cy) > 1
        })
    }

    pub fn new_safe_zero_seeded(
        width: usize,
        height: usize,
        bombs: usize,
        cx: usize,
        cy: usize,
        seed: u64,
    ) -> Self {
        Self::new_func(width, height, bombs, StdRng::seed_from_u64(seed), |x, y| {
            x.abs_diff(cx) > 1 || y.abs_diff(cy) > 1
        })
    }

    fn new_func(
        width: usize,
        height: usize,
        bombs: usize,
        mut rng: impl RngCore,
        check: impl Fn(usize, usize) -> bool,
    ) -> Self {
        let mut board = Board::<Tile>::new(width, height);

        for _ in 0..bombs {
            let mut x;
            let mut y;

            loop {
                x = rng.gen_range(0..width);
                y = rng.gen_range(0..height);

                if board.get(x, y).value != TileValue::Bomb && check(x, y) {
                    break;
                }
            }

            board.set(x, y, Tile::new(TileValue::Bomb, TileState::Hidden));
        }

        Self::calculate_neighbors(&mut board);

        Self { board }
    }

    fn calculate_neighbors(board: &mut Board<Tile>) {
        for x in 0..board.get_width() {
            for y in 0..board.get_height() {
                let mut count = 0;
                if board.get(x, y).value == TileValue::Bomb {
                    continue;
                }

                for ix in x.saturating_sub(1)..board.get_width().min(x + 2) {
                    for iy in y.saturating_sub(1)..board.get_height().min(y + 2) {
                        if let TileValue::Bomb = board.get(ix, iy).value {
                            count += 1;
                        }
                    }
                }

                let tile = board.get_mut(x, y);
                tile.value = TileValue::Safe(count);
            }
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> Option<TileValue> {
        let tile = self.board.get_mut(x, y);

        match tile.state {
            TileState::Hidden => tile.state = TileState::Open,
            _ => return None,
        }

        Some(tile.value)
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        let tile = self.board.get_mut(x, y);

        match tile.state {
            TileState::Flag => tile.state = TileState::Hidden,
            TileState::Hidden => tile.state = TileState::Flag,
            TileState::Open => {}
        }
    }

    pub fn set_flag(&mut self, x: usize, y: usize, flag: bool) {
        let tile = self.board.get_mut(x, y);

        if tile.state == TileState::Open {
            return;
        }

        tile.state = if flag {
            TileState::Flag
        } else {
            TileState::Hidden
        };
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> (&TileState, Option<&TileValue>) {
        let tile = self.board.get(x, y);

        let value = if tile.state == TileState::Open {
            Some(&tile.value)
        } else {
            None
        };

        (&tile.state, value)
    }

    #[must_use]
    pub fn get_width(&self) -> usize {
        self.board.get_width()
    }

    #[must_use]
    pub fn get_height(&self) -> usize {
        self.board.get_height()
    }
}

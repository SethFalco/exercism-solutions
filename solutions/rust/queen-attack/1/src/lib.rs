#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {
            return None;
        }

        Some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.file == other.position.file {
            return true;
        }

        if self.position.rank == other.position.rank {
            return true;
        }

        let rank_diff = self.position.rank.abs_diff(other.position.rank);
        let file_diff = self.position.file.abs_diff(other.position.file);

        if rank_diff == file_diff {
            return true;
        }

        return false;
    }
}

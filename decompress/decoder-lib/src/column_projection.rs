#[derive(Debug, Clone, Copy)]
pub struct ColumnProjection {
    mask_1: u32,
    mask_2: u32,
}

impl ColumnProjection {
    pub const MAX_COL_IDX: u32 = 63;

    pub fn new(mask_1: u32, mask_2: u32) -> Self {
        Self { mask_1, mask_2 }
    }

    pub fn mask_1(&self) -> u32 {
        self.mask_1
    }

    pub fn mask_2(&self) -> u32 {
        self.mask_2
    }

    pub fn contains(&self, col_idx: u32) -> bool {
        if col_idx > Self::MAX_COL_IDX {
            false
        } else if col_idx < 32 {
            (self.mask_1 & (1 << col_idx)) != 0
        } else {
            (self.mask_2 & (1 << (col_idx - 32))) != 0
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.mask_1 as u64 | ((self.mask_2 as u64) << 32)
    }
}

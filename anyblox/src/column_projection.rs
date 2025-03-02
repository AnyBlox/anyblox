use smallvec::{smallvec, SmallVec};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct ColumnProjection {
    bitmask: u64,
}

impl ColumnProjection {
    pub const MAX_COL_IDX: usize = 63;

    pub fn all(count: usize) -> Result<Self, ColumnIndexError> {
        if count > Self::MAX_COL_IDX {
            return Err(ColumnIndexError::from(ColumnIndexErrorReason::Overflow(
                Self::MAX_COL_IDX + 1,
            )));
        }
        if count == Self::MAX_COL_IDX {
            let bitmask = u64::MAX;
            Ok(Self { bitmask })
        } else {
            let bitmask = (1_u64 << count) - 1;
            Ok(Self { bitmask })
        }
    }

    pub fn from_indices<I>(iter: I) -> Result<Self, ColumnIndexError>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut bitmask = 0_u64;
        for col in iter {
            if col > Self::MAX_COL_IDX {
                return Err(ColumnIndexError::from(ColumnIndexErrorReason::Overflow(col)));
            }
            bitmask |= 1 << col;
        }
        if bitmask == 0 {
            return Err(ColumnIndexError::from(ColumnIndexErrorReason::Empty));
        }
        Ok(Self { bitmask })
    }

    pub fn get_indices(&self) -> SmallVec<[usize; 64]> {
        let mut vec = smallvec![];
        let mut mask = self.bitmask;
        while mask != 0 {
            let next = mask.trailing_zeros() as usize;
            vec.push(next);
            mask ^= 1 << next;
        }

        vec
    }

    pub fn project_schema(
        &self,
        schema: &arrow::datatypes::Schema,
    ) -> Result<arrow::datatypes::Schema, arrow::error::ArrowError> {
        schema.project(&self.get_indices())
    }

    pub fn contains(&self, col_idx: usize) -> bool {
        if col_idx > Self::MAX_COL_IDX {
            false
        } else {
            (self.bitmask & (1 << col_idx)) != 0
        }
    }

    pub fn raw_mask(&self) -> u64 {
        self.bitmask
    }

    pub fn from_raw_mask(mask: u64) -> Self {
        Self { bitmask: mask }
    }
}

impl From<ColumnProjection> for decoder_lib::column_projection::ColumnProjection {
    fn from(value: ColumnProjection) -> Self {
        let mask_1 = value.bitmask as u32;
        let mask_2 = (value.bitmask >> 32) as u32;
        Self::new(mask_1, mask_2)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ColumnIndexError {
    #[from]
    reason: ColumnIndexErrorReason,
}

#[derive(Debug, Error)]
enum ColumnIndexErrorReason {
    #[error(
        "column index {0} is larger than the maximum supported of {}",
        ColumnProjection::MAX_COL_IDX
    )]
    Overflow(usize),
    #[error("empty column projection does not make sense")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::ColumnProjection;
    use decoder_lib::column_projection::ColumnProjection as DecoderColumnProjection;
    use smallvec::{smallvec, SmallVec};

    #[test]
    fn empty_projection_invalid() {
        let result = ColumnProjection::from_indices([]);
        assert!(result.is_err());
    }

    #[test]
    fn all_columns() {
        let projection = ColumnProjection::all(17).expect("ColumnProjection::all");
        let decoder_projection = DecoderColumnProjection::from(projection);

        for idx in 0..17 {
            assert!(decoder_projection.contains(idx));
        }

        let expected_indices = (0_usize..17).collect::<SmallVec<[_; 64]>>();
        assert_eq!(expected_indices, projection.get_indices());
    }

    #[test]
    fn selected_columns() {
        let projection = ColumnProjection::from_indices([1, 3, 5]).expect("ColumnProjection::from_indices");
        let decoder_projection = DecoderColumnProjection::from(projection);

        for idx in [1, 3, 5] {
            assert!(decoder_projection.contains(idx));
        }
        for idx in [0, 2, 4] {
            assert!(!decoder_projection.contains(idx));
        }
        for idx in 6..64 {
            assert!(!decoder_projection.contains(idx));
        }

        let expected: SmallVec<[usize; 64]> = smallvec![1, 3, 5];
        assert_eq!(expected, projection.get_indices());
    }
}

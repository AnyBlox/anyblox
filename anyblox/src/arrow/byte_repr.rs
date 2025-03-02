/// Utility trait to allow representing types returned from Arrow arrays as consistent
/// byte representations.
///
/// Used for sinks.
pub trait ByteRepr {
    fn pass_as_bytes<F, R>(&self, f: F) -> R
    where
        F: FnMut(&[u8]) -> R;
}

impl ByteRepr for &[u8] {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        f(&(self.len() as u32).to_le_bytes());
        f(self)
    }
}

impl ByteRepr for &str {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        f(&(self.len() as u32).to_le_bytes());
        f(self.as_bytes())
    }
}

impl ByteRepr for bool {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        if *self {
            f(&[1])
        } else {
            f(&[0])
        }
    }
}

macro_rules! byte_repr_for_num {
    ($num:ty) => {
        impl ByteRepr for $num {
            fn pass_as_bytes<F, R>(&self, mut f: F) -> R
            where
                F: FnMut(&[u8]) -> R,
            {
                f(&self.to_le_bytes())
            }
        }
    };
}

byte_repr_for_num!(i8);
byte_repr_for_num!(u8);
byte_repr_for_num!(i16);
byte_repr_for_num!(u16);
byte_repr_for_num!(i32);
byte_repr_for_num!(u32);
byte_repr_for_num!(i64);
byte_repr_for_num!(u64);
byte_repr_for_num!(i128);
byte_repr_for_num!(u128);
byte_repr_for_num!(f32);
byte_repr_for_num!(f64);

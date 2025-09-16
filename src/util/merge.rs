// https://github.com/Noah2610/deathframe/blob/develop/deathframe_core/src/components/component_helpers/merge/mod.rs

/// Merge types together.
pub trait Merge: Sized {
    /// Consumes both values, merges them together,
    /// and returns a new instance of `Self`.
    fn merge(self, other: Self) -> Self;
}

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge(self, other: Self) -> Self {
        match self {
            Some(s) => match other {
                Some(o) => Some(s.merge(o)),
                None => Some(s),
            },
            None => match other {
                Some(o) => Some(o),
                None => other,
            },
        }
    }
}

impl Merge for bool {
    fn merge(self, other: Self) -> Self {
        other || self
    }
}

impl Merge for u8 {
    fn merge(self, other: Self) -> Self {
        if other != 0 {
            other
        } else {
            self
        }
    }
}

// Impl Merge for all primitive types with macro (Copilot):
macro_rules! impl_merge_primitive {
    ($($t:ty),*) => {
        $(
            impl Merge for $t {
                fn merge(self, other: Self) -> Self {
                    other
                }
            }
        )*
    };
}

impl_merge_primitive!(
    // bool,
    // u8,
    // u16,
    // u32,
    // u64,
    // usize,
    // i8,
    // i16,
    // i32,
    // i64,
    // isize,
    // f32,
    // f64,
    // char,
    String,
    std::path::PathBuf
);

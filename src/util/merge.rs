// https://github.com/Noah2610/deathframe/blob/develop/deathframe_core/src/components/component_helpers/merge/mod.rs

/// Merge types together.
pub trait Merge: Sized {
    /// Merge other value into self.
    fn merge(&mut self, other: Self);

    /// Consumes both values, merges them together,
    /// and returns a new instance of `Self`.
    fn merged(mut self, other: Self) -> Self {
        self.merge(other);
        self
    }
}

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge(&mut self, other: Self) {
        match self.as_mut() {
            Some(s) => match other {
                Some(o) => s.merge(o),
                None => (),
            },
            None => match other {
                Some(o) => *self = Some(o),
                None => (),
            },
        }
    }
}

impl Merge for bool {
    fn merge(&mut self, other: Self) {
        *self = other || *self;
    }
}

impl Merge for u8 {
    fn merge(&mut self, other: Self) {
        if other != 0 {
            *self = other;
        }
    }
}

// Impl Merge for all primitive types with macro (Copilot):
macro_rules! impl_merge_primitive {
    ($($t:ty),*) => {
        $(
            impl Merge for $t {
                fn merge(&mut self, other: Self) {
                    *self = other;
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

/// Tabled a trait responsible for providing a header fields and a row fields.
///
/// It's urgent that `header` len is equal to `fields` len.
///
/// ```text
/// Self::headers().len() == self.fields().len()
/// ```
pub trait Tabled {
    /// A length of fields and headers,
    /// which must be the same.
    const LENGTH: usize;

    /// Fields method must return a list of cells.
    ///
    /// The cells will be placed in the same row, preserving the order.
    fn fields(&self) -> Vec<String>;
    /// Headers must return a list of column names.
    fn headers() -> Vec<String>;
}

impl<T> Tabled for &T
where
    T: Tabled,
{
    const LENGTH: usize = T::LENGTH;

    fn fields(&self) -> Vec<String> {
        T::fields(self)
    }
    fn headers() -> Vec<String> {
        T::headers()
    }
}

macro_rules! tuple_table {
    ( $($name:ident)+ ) => {
        impl<$($name: Tabled),+> Tabled for ($($name,)+){
            const LENGTH: usize = $($name::LENGTH+)+ 0;

            fn fields(&self) -> Vec<String> {
                #![allow(non_snake_case)]
                let ($($name,)+) = self;
                let mut fields = Vec::with_capacity(Self::LENGTH);
                $(fields.append(&mut $name.fields());)+
                fields
            }

            fn headers() -> Vec<String> {
                let mut fields = Vec::with_capacity(Self::LENGTH);
                $(fields.append(&mut $name::headers());)+
                fields
            }
        }
    };
}

tuple_table! { A }
tuple_table! { A B }
tuple_table! { A B C }
tuple_table! { A B C D }
tuple_table! { A B C D E }
tuple_table! { A B C D E F }

macro_rules! default_table {
    ( $t:ty ) => {
        impl Tabled for $t {
            const LENGTH: usize = 1;

            fn fields(&self) -> Vec<String> {
                vec![format!("{}", self)]
            }
            fn headers() -> Vec<String> {
                vec![stringify!($t).to_string()]
            }
        }
    };
}

default_table!(&str);
default_table!(String);

default_table!(char);

default_table!(bool);

default_table!(isize);
default_table!(usize);

default_table!(u8);
default_table!(u16);
default_table!(u32);
default_table!(u64);
default_table!(u128);

default_table!(i8);
default_table!(i16);
default_table!(i32);
default_table!(i64);
default_table!(i128);

default_table!(f32);
default_table!(f64);

impl<T: std::fmt::Display, const N: usize> Tabled for [T; N] {
    const LENGTH: usize = N;

    fn fields(&self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }

    fn headers() -> Vec<String> {
        (0..N).map(|i| format!("{}", i)).collect()
    }
}

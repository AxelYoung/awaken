pub type Lst<T> = Vec<T>;

#[macro_export]
macro_rules! lst{
  ($($element:expr),*) => {
    vec![$($element), *]
  };
}
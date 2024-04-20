#[macro_export]
macro_rules! source {
  () => { format!("{:30}", format!("{}:{}", file!(), line!())) };
}

#[macro_export]
macro_rules! fformat {
  ($($arg: tt)*) => { || { crate::source!() + "Failed to " + &format!($($arg)*) } };
}

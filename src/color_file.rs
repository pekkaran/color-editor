use crate::all::*;

pub trait ColorFile {
  fn load_file(path: &Path) -> bool;
}

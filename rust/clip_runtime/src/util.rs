use ndarray::Array2;

pub fn box_u32_i64(li: Box<[u32]>) -> Vec<i64> {
  li.into_vec().into_iter().map(|x| x as i64).collect()
}

pub fn box_iter_ndarray<T: std::clone::Clone>(
  boxli: impl Iterator<Item = Vec<T>> + ExactSizeIterator,
) -> anyhow::Result<Array2<T>> {
  let mut vec = Vec::new();
  let len = boxli.len();
  for b in boxli {
    vec.extend_from_slice(&b);
  }
  Ok(Array2::from_shape_vec((len, vec.len() / len), vec)?)
}

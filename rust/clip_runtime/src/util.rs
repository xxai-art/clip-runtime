use ndarray::{Array2, ArrayBase, Dim, IxDynImpl, OwnedRepr};
use ort::tensor::{DynOrtTensor, FromArray, InputTensor};

pub fn box_u32_i64(li: Box<[u32]>) -> Vec<i64> {
  li.into_vec().into_iter().map(|x| x as i64).collect()
}

pub fn box_iter_ndarray<T: std::clone::Clone>(
  boxli: impl ExactSizeIterator + Iterator<Item = Vec<T>>,
) -> anyhow::Result<InputTensor>
where
  InputTensor: FromArray<T>,
{
  let mut vec = Vec::new();
  let len = boxli.len();
  for b in boxli {
    vec.extend_from_slice(&b);
  }
  Ok(InputTensor::from_array(
    Array2::from_shape_vec((len, vec.len() / len), vec)?.into_dyn(),
  ))
}

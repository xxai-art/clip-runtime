use ndarray::{ArrayBase, Dim, IxDynImpl, OwnedRepr};

pub type Arr = ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>;

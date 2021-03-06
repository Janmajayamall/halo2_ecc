use halo2_proofs::{arithmetic::FieldExt, circuit::AssignedCell, plonk::Assigned};

mod add;
mod add_incomplete;
mod mul_fixed;

pub struct ECCPoint<F: FieldExt> {
    x: AssignedCell<Assigned<F>, F>,
    y: AssignedCell<Assigned<F>, F>,
}

use halo2_proofs::{
    circuit::Region,
    poly::{Rotation }, arithmetic::{FieldExt, Field},
    plonk::{Column, Advice, Selector, ConstraintSystem}
        
};

pub struct Config {
    q_add: Selector,
    lambda: Column<Advice>,
    x_p: Column<Advice>,
    y_p: Column<Advice>,
    x_qr: Column<Advice>,
    y_qr: Column<Advice>,
    alpha: Column<Advice>,
    beta: Column<Advice>,
    gamma: Column<Advice>,
    delta: Column<Advice>,
}

impl Config {
    fn configure(
        meta: &mut ConstraintSystem<impl Field>,
        lambda: Column<Advice>,
        x_p: Column<Advice>,
        y_p: Column<Advice>,
        x_qr: Column<Advice>,
        y_qr: Column<Advice>,
        alpha: Column<Advice>,
        beta: Column<Advice>,
        gamma: Column<Advice>,
        delta: Column<Advice>
    ) {
        let q_add = meta.selector();

        // meta.create_gate("add", ||)
    }
}

fn main() {

}
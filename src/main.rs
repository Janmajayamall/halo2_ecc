use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Region, AssignedCell, layouter, Layouter},
    poly::{Rotation }, arithmetic::{FieldExt, Field},
    plonk::{Column, Advice, Selector, ConstraintSystem, Constraints}
        
};

pub struct ECCPoint<F: FieldExt> {
    x: AssignedCell<F, F>,
    y: AssignedCell<F, F>
}

pub struct Config<F> {
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
    _market: PhantomData<F>
}

impl<F: FieldExt> Config<F> {
    fn configure(
        meta: &mut ConstraintSystem<F>,
        lambda: Column<Advice>,
        x_p: Column<Advice>,
        y_p: Column<Advice>,
        x_qr: Column<Advice>,
        y_qr: Column<Advice>,
        alpha: Column<Advice>,
        beta: Column<Advice>,
        gamma: Column<Advice>,
        delta: Column<Advice>
    ) -> Self {
        let q_add = meta.selector();

        meta.create_gate("add", |meta|{
            let q_add = meta.query_selector(q_add);
            let x_p = meta.query_advice(x_p, Rotation::cur());
            let y_p = meta.query_advice(y_p, Rotation::cur());

            /// TODO: finish the gate
            [q_add * (x_p * y_p)]
        });

        Config {
            q_add,
            lambda,
            x_p,
            y_p,
            x_qr,
            y_qr,
            alpha,
            beta,
            gamma,
            delta,
            _market: PhantomData,
        }
    }

    fn assign(
        layouter: &mut impl Layouter<F>,
        p: ECCPoint<F>,
        q: ECCPoint<F>
    ) {
        // assignt the region
        // layouter.assign_region(name, assignment)
    }
}

fn main() {

}
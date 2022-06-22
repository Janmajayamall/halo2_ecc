use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Region, AssignedCell, layouter, Layouter},
    poly::{Rotation }, arithmetic::{FieldExt, Field},
    plonk::{Column, Advice, Selector, ConstraintSystem, Constraints, Expression}
        
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

            // given points
            let x_p = meta.query_advice(x_p, Rotation::cur());
            let y_p = meta.query_advice(y_p, Rotation::cur());
            let x_q = meta.query_advice(x_qr, Rotation::cur());
            let y_q = meta.query_advice(y_qr, Rotation::cur());

            // resulting point
            let x_r = meta.query_advice(x_qr, Rotation::next());
            let y_r = meta.query_advice(y_qr, Rotation::next());

            // derived values
            let lambda = meta.query_advice(lambda, Rotation::cur());
            let alpha = meta.query_advice(alpha, Rotation::cur());
            let beta = meta.query_advice(beta, Rotation::cur());
            let gamma = meta.query_advice(gamma, Rotation::cur());
            let delta = meta.query_advice(delta, Rotation::cur());

            // some expressions
            let x_q_minus_x_p = x_q.clone() - x_p.clone();
            let y_q_plus_y_p = y_q.clone() + y_p.clone();
            let if_alpha = x_q_minus_x_p.clone() * alpha.clone();
            let if_beta = x_p.clone() * beta.clone();
            let if_gamma = x_q.clone() * gamma.clone();
            let if_delta = y_q_plus_y_p.clone() * delta.clone();

            // constants
            let one = Expression::Constant(F::one());
            
            // polynomial constraints
            let p1a = {
                let tmp = (x_q_minus_x_p.clone() * lambda.clone()) - (y_q.clone() - y_p.clone());
                x_q_minus_x_p.clone() * tmp
            };
            let p2a = {
                let two = Expression::Constant(F::from_u128(2));
                let three = Expression::Constant(F::from_u128(3));
                let tmp = two.clone() * y_p.clone() * lambda.clone();
                let tmp = tmp.clone() - (three.clone() * x_p.clone().square());
                (one.clone() - if_alpha.clone()) * tmp
            };
            let p3a = x_p.clone() * x_q.clone() * x_q_minus_x_p.clone() * (lambda.clone().square() - x_p.clone() - x_q.clone() - x_r.clone());
            let p3b = x_p.clone() * x_q.clone() * x_q_minus_x_p.clone() * ((lambda.clone() * (x_p.clone() - x_r.clone())) - x_p.clone() - y_p.clone() - y_r.clone());
            let p3c = x_p.clone() * x_q.clone() * y_q_plus_y_p.clone() * (lambda.clone().square() - x_p.clone() - x_q.clone() - x_r.clone());
            let p3d = x_p.clone() * x_q.clone() * y_q_plus_y_p.clone() * ((lambda.clone() * (x_p.clone() - x_r.clone())) - x_p.clone() - y_p.clone() - y_r.clone());
            let p4a = (one.clone() - if_beta.clone()) * (x_r.clone() - x_q.clone());
            let p4b = (one.clone() - if_beta.clone()) * (y_r.clone() - y_q.clone());
            let p5a = (one.clone() - if_gamma.clone()) * (x_r.clone() - x_p.clone());
            let p5b= (one.clone() - if_gamma.clone()) * (y_r.clone() - y_p.clone());
            let p6a = (one.clone() - if_alpha.clone() - if_delta.clone()) * x_r.clone();
            let p6b = (one - if_alpha - if_delta) * y_r;


            Constraints::with_selector(q_add, [
                ("1a", p1a),
                ("2a", p2a),
                ("3a", p3a),
                ("3b", p3b),
                ("3c", p3c),
                ("3d", p3d),
                ("4a", p4a),
                ("4b", p4b),
                ("5a", p5a),
                ("5b", p5b),
                ("6a", p6a),
                ("6b", p6b),
            ])
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
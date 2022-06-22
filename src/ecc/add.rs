use super::ECCPoint;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Region},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Constraints, Error, Expression, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

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
    _market: PhantomData<F>,
}

impl<F: FieldExt> Config<F> {
    #[allow(clippy::too_many_arguments)]
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        lambda: Column<Advice>,
        x_p: Column<Advice>,
        y_p: Column<Advice>,
        x_qr: Column<Advice>,
        y_qr: Column<Advice>,
        alpha: Column<Advice>,
        beta: Column<Advice>,
        gamma: Column<Advice>,
        delta: Column<Advice>,
    ) -> Self {
        let q_add = meta.selector();

        meta.enable_equality(x_p);
        meta.enable_equality(y_p);
        meta.enable_equality(x_qr);
        meta.enable_equality(y_qr);

        meta.create_gate("add", |meta| {
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
            let if_alpha = x_q_minus_x_p.clone() * alpha;
            let if_beta = x_p.clone() * beta;
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
            let p3a = x_p.clone()
                * x_q.clone()
                * x_q_minus_x_p.clone()
                * (lambda.clone().square() - x_p.clone() - x_q.clone() - x_r.clone());
            let p3b = x_p.clone()
                * x_q.clone()
                * x_q_minus_x_p.clone()
                * ((lambda.clone() * (x_p.clone() - x_r.clone()))
                    - x_p.clone()
                    - y_p.clone()
                    - y_r.clone());
            let p3c = x_p.clone()
                * x_q.clone()
                * y_q_plus_y_p.clone()
                * (lambda.clone().square() - x_p.clone() - x_q.clone() - x_r.clone());
            let p3d = x_p.clone()
                * x_q.clone()
                * y_q_plus_y_p.clone()
                * ((lambda.clone() * (x_p.clone() - x_r.clone()))
                    - x_p.clone()
                    - y_p.clone()
                    - y_r.clone());
            let p4a = (one.clone() - if_beta.clone()) * (x_r.clone() - x_q.clone());
            let p4b = (one.clone() - if_beta.clone()) * (y_r.clone() - y_q.clone());
            let p5a = (one.clone() - if_gamma.clone()) * (x_r.clone() - x_p.clone());
            let p5b = (one.clone() - if_gamma.clone()) * (y_r.clone() - y_p.clone());
            let p6a = (one.clone() - if_alpha.clone() - if_delta.clone()) * x_r.clone();
            let p6b = (one - if_alpha - if_delta) * y_r;

            Constraints::with_selector(
                q_add,
                [
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
                ],
            )
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

    pub fn assign(
        self,
        region: &mut Region<'_, F>,
        offset: usize,
        p: ECCPoint<F>,
        q: ECCPoint<F>,
    ) -> Result<ECCPoint<F>, Error> {
        self.q_add.enable(region, offset)?;

        p.x.copy_advice(|| "x_p", region, self.x_p, offset)?;
        p.y.copy_advice(|| "y_p", region, self.y_p, offset)?;
        q.x.copy_advice(|| "x_q", region, self.x_qr, offset)?;
        q.y.copy_advice(|| "y_q", region, self.y_qr, offset)?;

        let x_p = p.x.value();
        let y_p = p.y.value();
        let x_q = q.x.value();
        let y_q = q.y.value();

        let alpha = (x_q - x_p).invert();
        let beta = x_p.invert();
        let gamma = x_q.invert();
        region.assign_advice(|| "α", self.alpha, offset, || alpha)?;
        region.assign_advice(|| "β", self.beta, offset, || beta)?;
        region.assign_advice(|| "γ", self.beta, offset, || gamma)?;

        let delta = x_p
            .zip(x_q)
            .zip(y_p)
            .zip(y_q)
            .map(|((((x_p, x_q), y_p), y_q))| {
                if x_q == x_p {
                    (y_q + y_p).invert()
                } else {
                    Assigned::Zero
                }
            });
        region.assign_advice(|| "δ", self.delta, offset, || delta)?;

        let lambda = x_p
            .zip(x_q)
            .zip(y_p)
            .zip(y_q)
            .map(|(((x_p, x_q), y_p), y_q)| {
                if x_q != x_p {
                    (y_q - y_p) * (x_q - x_p).invert()
                } else if y_p.is_zero_vartime() {
                    (x_p.square() * F::from_u128(3)) * (y_p * F::from_u128(2)).invert()
                } else {
                    Assigned::Zero
                }
            });
        region.assign_advice(|| "λ", self.lambda, offset, || lambda)?;

        // Calculate and assing x_r, y_r
        let x_r = lambda
            .zip(x_p)
            .zip(x_q)
            .map(|((lambda, x_p), x_q)| lambda.square() - x_p - x_q);
        let x_r_cell = region.assign_advice(|| "x_r", self.x_qr, offset + 1, || x_r)?;
        let y_r = lambda
            .zip(x_p)
            .zip(x_r)
            .zip(y_p)
            .map(|(((lambda, x_p), x_r), y_p)| ((x_p - x_r) * lambda) - y_p);
        let y_r_cell = region.assign_advice(|| "y_r", self.x_qr, offset + 1, || y_r)?;

        let result = ECCPoint {
            x: x_r_cell,
            y: y_r_cell,
        };

        Ok(result)
    }
}

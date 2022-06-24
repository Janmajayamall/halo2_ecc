use super::ECCPoint;
use halo2_proofs::{
    arithmetic::{CurveAffine, FieldExt},
    circuit::Region,
    plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Expression, Fixed, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

pub struct Config<C: CurveAffine, F: FieldExt, const WINDOW_SIZE: usize> {
    q_running_sum: Selector,
    z_fixed: Column<Fixed>,
    u: Column<Advice>,
    z: Column<Advice>,
    x_p: Column<Advice>,
    y_p: Column<Advice>,
    lagrange_coeff: [Column<Advice>; WINDOW_SIZE],
    _marker: PhantomData<F>,
    _marker1: PhantomData<C>,
}

impl<C: CurveAffine, F: FieldExt, const WINDOW_SIZE: usize> Config<C, F, WINDOW_SIZE> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        lagrange_coeff: [Column<Advice>; WINDOW_SIZE],
        z_fixed: Column<Fixed>,
        u: Column<Advice>,
        x_p: Column<Advice>,
        y_p: Column<Advice>,
        z: Column<Advice>,
    ) -> Self {
        let q_running_sum = meta.selector();

        let config = Config {
            q_running_sum,
            lagrange_coeff,
            z_fixed,
            u,
            z,
            x_p,
            y_p,
            _marker: PhantomData,
            _marker1: PhantomData,
        };

        // create gates
        // 1. Check running sum + for every word check range + for every word check x &
        // y values

        meta.create_gate("Running sum range and cords check", |meta| {
            let q_running_sum = meta.query_selector(config.q_running_sum);

            let z_curr = meta.query_advice(config.z, Rotation::cur());
            let z_next = meta.query_advice(config.z, Rotation::next());

            let word = z_curr - z_next * F::from(1 << WINDOW_SIZE);

            // running sum range check
            let range = (1 << WINDOW_SIZE) as usize;
            let sum_check = (1..range).fold(word.clone(), |acc, i| {
                acc * (Expression::Constant(F::from(i as u64)) - word.clone())
            });

            // coords check
            let x_p = meta.query_advice(config.x_p, Rotation::cur());
            let y_p = meta.query_advice(config.y_p, Rotation::cur());
            let u = meta.query_advice(config.u, Rotation::cur());
            let z = meta.query_fixed(config.z_fixed, Rotation::cur());

            let word_powers: Vec<Expression<F>> = (0..range)
                .map(|exp| {
                    (0..exp).fold(Expression::Constant(F::one()), |acc, i| acc * word.clone())
                })
                .collect();

            let interpolated_x = word_powers.iter().zip(config.lagrange_coeff).fold(
                Expression::Constant(F::one()),
                |acc, (word_power, coeff)| {
                    acc + (word_power.clone() * meta.query_advice(coeff, Rotation::cur()))
                },
            );

            let x_check = interpolated_x - x_p;
            let y_check = (y_p + z) - u.square();
            let on_curve = y_p.square()
                - (x_p.clone().square() * x_p)
                - Expression::Constant(CurveAffine::b());

            Constraints::with_selector(
                q_running_sum,
                [
                    ("sum_check", sum_check),
                    ("x_check", x_check),
                    ("y_check", y_check),
                    ("on_curve", on_curve),
                ],
            )
        });

        config
    }
}

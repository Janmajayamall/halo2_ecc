use super::ECCPoint;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Region,
    plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Expression, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

pub struct Config<F: FieldExt> {
    q_add_incomplete: Selector,
    x_p: Column<Advice>,
    y_p: Column<Advice>,
    x_qr: Column<Advice>,
    y_qr: Column<Advice>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Config<F> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        x_p: Column<Advice>,
        y_p: Column<Advice>,
        x_qr: Column<Advice>,
        y_qr: Column<Advice>,
    ) -> Self {
        let q_add_incomplete = meta.selector();

        meta.enable_equality(x_p);
        meta.enable_equality(y_p);
        meta.enable_equality(x_qr);
        meta.enable_equality(y_qr);

        meta.create_gate("incomplete addition", |meta| {
            let q_add_incomplete = meta.query_selector(q_add_incomplete);

            // given points
            let x_p = meta.query_advice(x_p, Rotation::cur());
            let y_p = meta.query_advice(y_p, Rotation::cur());
            let x_q = meta.query_advice(x_qr, Rotation::cur());
            let y_q = meta.query_advice(y_qr, Rotation::cur());

            // resulting point
            let x_r = meta.query_advice(x_qr, Rotation::next());
            let y_r = meta.query_advice(y_qr, Rotation::next());

            let p1 = ((x_r.clone() + x_q.clone() + x_p.clone())
                * (x_p.clone() - x_q.clone()).square())
                - (y_p.clone() - y_q.clone()).square();
            let p2 = ((y_r + y_q.clone()) * (x_p - x_q.clone())) - ((y_p - y_q) * (x_q - x_r));

            Constraints::with_selector(q_add_incomplete, [("p1", p1), ("p2", p2)])
        });

        Config {
            q_add_incomplete,
            x_p,
            y_p,
            x_qr,
            y_qr,
            _marker: PhantomData,
        }
    }

    pub fn assign(
        self,
        region: &mut Region<'_, F>,
        offset: usize,
        p: ECCPoint<F>,
        q: ECCPoint<F>,
    ) -> Result<ECCPoint<F>, Error> {
        self.q_add_incomplete.enable(region, offset)?;

        p.x.copy_advice(|| "x_p", region, self.x_p, offset)?;
        p.y.copy_advice(|| "y_p", region, self.y_p, offset)?;
        q.x.copy_advice(|| "x_q", region, self.x_qr, offset)?;
        q.y.copy_advice(|| "y_q", region, self.y_qr, offset)?;

        // calculate `R`
        let x_p = p.x.value();
        let y_p = p.y.value();
        let x_q = q.x.value();
        let y_q = q.y.value();
        let tangent = (y_q - y_p) * (x_q - x_p).invert();
        let x_r = tangent.clone().square() - x_q - x_p;
        let y_r = (tangent * (x_q - x_r)) - y_q;

        let x_r_cell = region.assign_advice(|| "x_r", self.x_qr, offset + 1, || x_r)?;
        let y_r_cell = region.assign_advice(|| "y_r", self.y_qr, offset + 1, || y_r)?;

        Ok(ECCPoint {
            x: x_r_cell,
            y: y_r_cell,
        })
    }
}

//! **A curved body's measured volume, area and pads do not read its
//! storage** — `topo`'s planar storage-invariance rows on bodies with
//! cones, spheres, tori and spline walls: each body measured, then
//! relabeled every way the data model declares free (each cycle loop's
//! anchor moved along its cycle, each face's ring list reversed, the
//! faces walked in another order), and the bits compared.

#![allow(clippy::panic)]

use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Tol, Vec3};
use sweep::test_support::{ball_poled_z_at, domed_cavity, lantern, swept_elbow, waisted_at};
use topo::test_support::{mass_properties_in_face_order, relabeled};
use topo::{Body, FaceKey, LoopBoundary, MassProperties};

/// A scalar whose measurement this suite compares by its bits.
trait Bits: Decide + CertifiedBounds + topo::AtRestPolicy {
    fn bits(x: Self) -> [u64; 2];
}

impl Bits for f64 {
    fn bits(x: Self) -> [u64; 2] {
        [x.to_bits(), x.to_bits()]
    }
}

impl Bits for Interval {
    fn bits(x: Self) -> [u64; 2] {
        [x.lo().to_bits(), x.hi().to_bits()]
    }
}

fn bits<T: Bits>(m: &MassProperties<T>) -> [u64; 6] {
    let [v0, v1] = T::bits(m.volume);
    let [a0, a1] = T::bits(m.surface_area);
    [v0, v1, a0, a1, m.volume_pad.to_bits(), m.area_pad.to_bits()]
}

fn assert_storage_invariant<T: Bits>(name: &str, body: &Body<T>, tol: Tol) {
    let arena: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let base = mass_properties_in_face_order(body, &arena, tol)
        .unwrap_or_else(|e| panic!("{name} at the {} scalar: {e}", T::NAME));
    let want = bits(&base);
    let longest = body
        .loops()
        .filter_map(|(_, lp)| match lp.boundary {
            LoopBoundary::Cycle { first } => body.loop_cycle(first).map(|c| c.len()),
            LoopBoundary::Empty { .. } => None,
        })
        .max()
        .unwrap_or(1);
    let mut rotated = arena.clone();
    rotated.rotate_left(1);
    let reversed: Vec<FaceKey> = arena.iter().rev().copied().collect();
    for steps in 0..longest {
        for reverse_rings in [false, true] {
            let moved = relabeled(body, steps, reverse_rings);
            for (order, faces) in [
                ("arena", &arena),
                ("reversed", &reversed),
                ("rotated", &rotated),
            ] {
                let got = mass_properties_in_face_order(&moved, faces, tol)
                    .unwrap_or_else(|e| panic!("{name} relabeled at the {} scalar: {e}", T::NAME));
                assert_eq!(
                    bits(&got),
                    want,
                    "{name} at the {} scalar, ε = {:e}: anchors moved {steps}, rings reversed \
                     {reverse_rings}, faces in {order} order measure {got:?} against {base:?}",
                    T::NAME,
                    tol.eps(),
                );
            }
        }
    }
}

fn ball<T: Bits>(tol: Tol) -> Body<T> {
    let c = |x: f64| T::from_f64(x);
    ball_poled_z_at(c(0.75), Vec3::new(c(0.25), c(-0.5), c(1.5)), tol)
}

#[test]
fn a_curved_bodys_measurement_does_not_read_its_storage_at_f64() {
    let tol = Tol::witness();
    assert_storage_invariant::<f64>("the waisted revolution", &waisted_at(tol), tol);
    assert_storage_invariant::<f64>("a ball", &ball(tol), tol);
    assert_storage_invariant::<f64>("the lantern", &lantern(tol), tol);
    assert_storage_invariant::<f64>("the domed cavity", &domed_cavity(tol), tol);
}

#[test]
fn a_curved_bodys_measurement_does_not_read_its_storage_at_the_interval_scalar() {
    let tol = Tol::witness();
    assert_storage_invariant::<Interval>("the waisted revolution", &waisted_at(tol), tol);
    assert_storage_invariant::<Interval>("a ball", &ball(tol), tol);
}

/// The swept elbow's walls are spline faces, so its measurement is the
/// quadrature's and its pads are not zero.
#[test]
fn a_spline_walled_bodys_measurement_and_pads_do_not_read_its_storage() {
    let tol = Tol::witness();
    let body = swept_elbow(tol);
    assert_storage_invariant::<f64>("the swept elbow", &body, tol);
}

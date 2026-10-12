//! **A body's measured volume, area and pads do not read its storage.**
//!
//! Each row measures a body, then the same body relabeled in every way
//! the data model declares free — each cycle loop's anchor moved along
//! its cycle, each face's ring list reversed, the faces walked in
//! another order — and asserts the measurement's bits unchanged, at
//! `f64` and at the interval scalar. The suite runs at whatever ε the
//! run commits, so the CI rows take it at each ε they run.

use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point3, Tol};

use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
use crate::props::{AtRestPolicy, MassProperties, mass_properties_in_face_order, relabeled};
use crate::test_support::{holed_block, plant_disc_face, prism_z};

/// A scalar whose measurement this suite compares by its bits.
trait Bits: Decide + CertifiedBounds + AtRestPolicy {
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

/// The measurement's bits: volume, area, and the two pads.
fn bits<T: Bits>(m: &MassProperties<T>) -> [u64; 6] {
    let [v0, v1] = T::bits(m.volume);
    let [a0, a1] = T::bits(m.surface_area);
    [v0, v1, a0, a1, m.volume_pad.to_bits(), m.area_pad.to_bits()]
}

/// Every relabeling this suite applies: anchor steps × ring order ×
/// face order (arena, reversed, rotated by one, by two).
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
    let orders: Vec<(&str, Vec<FaceKey>)> = vec![
        ("arena", arena.clone()),
        ("reversed", arena.iter().rev().copied().collect()),
        ("rotated by one", {
            let mut v = arena.clone();
            v.rotate_left(1);
            v
        }),
        ("rotated by two", {
            let mut v = arena.clone();
            v.rotate_left(2);
            v
        }),
    ];
    let mut compared = 0;
    for steps in 0..longest {
        for reverse_rings in [false, true] {
            let moved = relabeled(body, steps, reverse_rings);
            for (order, faces) in &orders {
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
                compared += 1;
            }
        }
    }
    assert!(
        compared >= 8,
        "{name}: every relabeling was compared ({compared})"
    );
}

/// A hexagonal prism, its profile centred at `(c, c)` and its height
/// from `c` up, so `c` sets how far from the origin it sits.
fn hex_prism<T: Bits>(c: f64, tol: Tol) -> Body<T> {
    let hex: Vec<(f64, f64)> = (0..6)
        .map(|k| {
            let a = core::f64::consts::FRAC_PI_3 * f64::from(k) + 0.1;
            (c + 1.3 * a.cos(), c + 0.7 * a.sin())
        })
        .collect();
    prism_z::<T>(&hex, c, c + 0.9, tol).body
}

/// The notch prism with a disc planted in its top: planes bounded by
/// lines and by a circle, the circle a ring of the top.
fn disc_in_notch<T: Bits>(tol: Tol) -> Body<T> {
    let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let p = prism_z::<T>(&notch, 0.0, 1.0, tol);
    let mut body = p.body;
    let outer = body.faces[p.top_face].outer;
    let LoopBoundary::Cycle { first } = body.loops[outer].boundary else {
        panic!("the top's outer loop is a cycle");
    };
    let pt = |x: f64| T::from_f64(x);
    plant_disc_face(
        &mut body,
        first,
        Point3::new(pt(1.0), pt(0.6), pt(1.0)),
        pt(0.4),
        tol,
    );
    body
}

/// `far` is how far from the origin the far prism sits: as far as its
/// planes still certify at ε = 1e-12 at the scalar.
fn every_fixture<T: Bits>(far: f64) {
    let tol = Tol::witness();
    assert_storage_invariant::<T>("a hexagonal prism", &hex_prism(0.0, tol), tol);
    assert_storage_invariant::<T>(
        "a hexagonal prism far from the origin",
        &hex_prism(far, tol),
        tol,
    );
    assert_storage_invariant::<T>(
        "a block with two holes",
        &holed_block(4.0, &[1.0, 3.0], tol),
        tol,
    );
    assert_storage_invariant::<T>("a disc in a notch prism's top", &disc_in_notch(tol), tol);
}

#[test]
fn a_planar_bodys_measurement_does_not_read_its_storage_at_f64() {
    every_fixture::<f64>(1.0e3);
}

#[test]
fn a_planar_bodys_measurement_does_not_read_its_storage_at_the_interval_scalar() {
    every_fixture::<Interval>(1.0e2);
}

/// The symbolic lane's rounded-once sum re-values to the plain lane's
/// sum, at the parameter it was built at and at another: its DAG is
/// the computation, so the re-valuation instrument reads it as such.
#[test]
fn a_symbolic_rounded_once_sum_revalues_to_the_plain_lanes_bits() {
    use geom_core::sym::revalue::revalue;
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Real, Sym, SymBudget};
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let symbol = ParamSymbol::new(7);
    let coefficients = [1.0e8, 0.1, -1.0e8, 3.0e-9, 0.7];
    let plain = |p: f64| f64::sum_rounded_once(&coefficients.map(|c| c * p));
    let (revalued, _) = with_session(budget, || {
        let x = Sym::<f64>::param(symbol, 0.3);
        let s = Sym::sum_rounded_once(&coefficients.map(|c| Sym::from_f64(c) * x));
        [0.3, 1.7]
            .map(|p| revalue(s.node(), &move |q| (q == symbol).then_some(p)).map(f64::to_bits))
    });
    assert_eq!(
        revalued,
        [0.3, 1.7].map(|p| Some(plain(p).to_bits())),
        "re-valued at the build's parameter and at another"
    );
    assert_ne!(
        plain(1.7).to_bits(),
        coefficients.iter().fold(0.0, |a, c| a + c * 1.7).to_bits(),
        "the premise: the sequential sum rounds elsewhere, so a chain of plain additions \
         would not re-value to it"
    );
}

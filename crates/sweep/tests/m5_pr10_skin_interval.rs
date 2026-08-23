//! M5 PR 10 §2, the Interval lane (feature `interval`).
//!
//! Q8's claim is that the produced NURBS **is** the definition — its
//! control bits are DATA, not a measurement. The consequence this row
//! checks is exact: lifting the `f64`-chosen structure to the
//! certified scalar gives the SAME surface, and every interval
//! evaluation ENCLOSES the `f64` one. Nothing here re-selects
//! structure at `Interval`; that would be a different surface, and the
//! definitional posture forbids it.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_brep::SketchSegment;
use geom_core::Tol;
use geom_core::{Affine3, Band, Bounds, Interval, Point2, Real, Vec3};
use sweep::skin::{lift_surface, make_compatible, segment_curve, skin, skin_parameters};

fn ring() -> f64 {
    Band::linear(Tol::witness())
        .expect("the linear band resolves")
        .zero()
}

/// The arc strip of the acceptance loft (three sections, middle
/// scaled), made compatible.
fn strip() -> Vec<NurbsCurve3<f64>> {
    let at = |z: f64, s: f64| {
        segment_curve(
            0,
            SketchSegment::Arc {
                a: Point2::new(2.0 * s, 0.0),
                b: Point2::new(2.0 * s, 1.0 * s),
                bulge: 0.25,
            },
            Affine3::translation(Vec3::new(0.0, 0.0, z)),
        )
        .expect("converts")
    };
    make_compatible(&[at(0.0, 1.0), at(1.0, 1.6), at(2.0, 1.0)]).expect("compatible")
}

/// The lifted structure is bit-identical, and every interval sample
/// encloses the `f64` sample — the same surface, evaluated with
/// enclosures.
#[test]
fn the_interval_lane_encloses_the_same_definitional_surface() {
    let compat = strip();
    let params = skin_parameters(&compat).expect("parameters");
    let exact = skin(&compat, 2).expect("skins");
    let lifted = lift_surface::<Interval>(&exact).expect("lifts");

    assert_eq!(lifted.knots_u().knots(), exact.knots_u().knots());
    assert_eq!(lifted.knots_v().knots(), exact.knots_v().knots());
    assert_eq!(lifted.weights(), exact.weights());

    // Sample counts DERIVE from the structure, not from a literal.
    let (nu, nv) = exact.control_counts();
    let (su, sv) = (4 * nu, 4 * nv);
    for i in 0..=su {
        #[allow(clippy::cast_precision_loss)]
        let u = i as f64 / su as f64;
        for j in 0..=sv {
            #[allow(clippy::cast_precision_loss)]
            let v = j as f64 / sv as f64;
            let a = exact.eval(u, v);
            let b = lifted.eval(Interval::from_f64(u), Interval::from_f64(v));
            for (exact_c, enclosure) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
                assert!(
                    enclosure.lo() <= exact_c && exact_c <= enclosure.hi(),
                    "({u}, {v}): {exact_c} escapes [{}, {}]",
                    enclosure.lo(),
                    enclosure.hi()
                );
            }
        }
    }

    // The interpolation claim holds at the certified scalar too: each
    // section's enclosure contains the section curve's own point.
    for (k, v) in params.iter().enumerate() {
        for i in 0..=su {
            #[allow(clippy::cast_precision_loss)]
            let u = i as f64 / su as f64;
            let on_section = compat[k].eval(u);
            let enclosed = lifted.eval(Interval::from_f64(u), Interval::from_f64(*v));
            for (want, got) in [
                (on_section.x, enclosed.x),
                (on_section.y, enclosed.y),
                (on_section.z, enclosed.z),
            ] {
                let pad = ring();
                assert!(
                    got.lo() - pad <= want && want <= got.hi() + pad,
                    "section {k} at u = {u}: {want} outside [{}, {}] (+/- {pad})",
                    got.lo(),
                    got.hi()
                );
            }
        }
    }
}

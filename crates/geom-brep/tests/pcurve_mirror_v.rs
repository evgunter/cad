//! **`Pcurve::mirror_v` and `Pcurve::shift_branch` at the door, over
//! every image kind the enum has.** The five variants are each linear
//! in their chart-space coefficients, so an affine map of the chart is
//! a coefficient-wise map of the image (`Pcurve::map_affine`); these
//! rows measure that claim variant by variant rather than on the one
//! kind a body fixture happens to mint — a plane face carries harmonic
//! images and nothing else, and a mutant answering wrongly on every
//! other arm leaves every body-level suite green.
//!
//! Authored across the unit's review lane (the kinds table, the
//! `map_points` equivalence, the `EdgeCurve`-door re-certification) and
//! its fix pass (the shift row). A reviewer's rows are ordinary rows.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_brep::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, SurfaceKey,
};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
use slotmap::SlotMap;

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).unwrap()
}

fn nurbs(weights: Vec<f64>) -> Arc<NurbsCurve2<f64>> {
    Arc::new(
        NurbsCurve2::new(
            kv(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
            vec![
                Point2::new(0.10, 0.20),
                Point2::new(0.40, -0.70),
                Point2::new(0.70, 1.00),
            ],
            weights,
        )
        .unwrap(),
    )
}

/// One image of every kind, with a non-zero `v` channel in each and
/// the interval its rows sample: `(name, image, t0, t1)`. The
/// `General` net is genuinely rational, so the `(−a)/b = −(a/b)` limb
/// of the NURBS arm is exercised and not only the polynomial one.
fn kinds() -> Vec<(&'static str, Pcurve<f64>, f64, f64)> {
    vec![
        (
            "Harmonic",
            Pcurve::Harmonic {
                p0: Point2::new(0.3, 0.4),
                pa: Vec2::new(1.0, 0.25),
                pb: Vec2::new(-0.5, 1.0),
                pl: Vec2::new(0.125, -0.375),
            },
            0.2,
            1.4,
        ),
        (
            "IsoLine",
            Pcurve::IsoLine {
                p0: Point2::new(0.1, 0.2),
                pl: Vec2::new(0.6, 0.8),
            },
            0.0,
            1.0,
        ),
        (
            "IsoArc",
            Pcurve::IsoArc {
                p0: Point2::new(0.2, -0.3),
                pd: Vec2::new(0.9, 1.7),
                t0: 0.25,
                angle: 1.5,
                breaks: kv(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1),
            },
            0.25,
            1.75,
        ),
        (
            "Fitted",
            Pcurve::Fitted(nurbs(vec![1.0, 1.0, 1.0])),
            0.0,
            1.0,
        ),
        (
            "General",
            Pcurve::General(nurbs(vec![1.0, 0.5, 2.0])),
            0.0,
            1.0,
        ),
    ]
}

fn schedule(t0: f64, t1: f64) -> impl Iterator<Item = f64> {
    (0..9u32).map(move |i| t0 + (t1 - t0) * f64::from(i) / 8.0)
}

/// The mirrored image is the mirrored locus at the nine schedule
/// samples, bit for bit, in every kind — `u` untouched, `v` the exact
/// negation — the variant is kept, and `mirror_v ∘ mirror_v` is the
/// identity on the image's `Debug`.
#[test]
fn mirror_v_is_the_exact_reflection_and_a_bitwise_involution_in_every_kind() {
    for (name, p, t0, t1) in kinds() {
        let m = p.mirror_v();
        assert_eq!(
            core::mem::discriminant(&m),
            core::mem::discriminant(&p),
            "{name}: the variant moved"
        );
        for (i, t) in schedule(t0, t1).enumerate() {
            let (a, b) = (p.eval(t), m.eval(t));
            assert_eq!(
                a.x.to_bits(),
                b.x.to_bits(),
                "{name}: u moved at sample {i}"
            );
            assert_eq!(
                (-a.y).to_bits(),
                b.y.to_bits(),
                "{name}: v is not the exact negation at sample {i} ({} vs {})",
                a.y,
                b.y
            );
        }
        assert_eq!(
            format!("{:?}", m.mirror_v()),
            format!("{p:?}"),
            "{name}: involution"
        );
    }
}

/// The NURBS arms go through `NurbsCurve2::map_points`, and that door
/// produces the bits a re-validated rebuild through `NurbsCurve2::new`
/// with the original knots and weights produces: the two constructors
/// agree on a pointwise map of a validated net, which is what lets the
/// mirror be infallible.
#[test]
fn the_nurbs_arms_are_map_points_and_agree_with_a_re_validated_rebuild() {
    let mut seen = 0;
    for (name, p, _, _) in kinds() {
        let (Pcurve::Fitted(img) | Pcurve::General(img)) = &p else {
            continue;
        };
        seen += 1;
        let via_new = NurbsCurve2::new(
            img.knots().clone(),
            img.control()
                .iter()
                .map(|q| Point2::new(q.x, -q.y))
                .collect(),
            img.weights().to_vec(),
        )
        .unwrap_or_else(|e| panic!("{name}: the rebuild refused {e:?}"));
        let via_door = img.map_points(|q| Point2::new(q.x, -q.y));
        assert_eq!(
            format!("{via_new:?}"),
            format!("{via_door:?}"),
            "{name}: map_points differs from the re-validated rebuild"
        );
        let (Pcurve::Fitted(m) | Pcurve::General(m)) = p.mirror_v() else {
            panic!("{name}: the variant moved")
        };
        assert_eq!(format!("{:?}", *m), format!("{via_door:?}"), "{name}");
    }
    assert_eq!(seen, 2, "both NURBS kinds were exercised");
}

/// `shift_branch` through the same door, in every kind: the first
/// channel of the locus moves by `k·period` at every schedule sample
/// (to rounding — a translated sum is not the sum translated bit for
/// bit), the second channel is untouched bit for bit, and the variant
/// is kept. The harmonic arm's rows in `pcurve_cache` and `sweep`'s
/// `m5_pr6_pcurves` pin its coefficients; this row is the other four.
#[test]
fn shift_branch_translates_u_and_leaves_v_bit_identical_in_every_kind() {
    let (k, period) = (2.0, core::f64::consts::TAU);
    for (name, p, t0, t1) in kinds() {
        let s = p.shift_branch(k, period);
        assert_eq!(
            core::mem::discriminant(&s),
            core::mem::discriminant(&p),
            "{name}: the variant moved"
        );
        for (i, t) in schedule(t0, t1).enumerate() {
            let (a, b) = (p.eval(t), s.eval(t));
            assert!(
                (b.x - a.x - k * period).abs() <= 1e-12,
                "{name}: u moved by {} at sample {i}, not {}",
                b.x - a.x,
                k * period
            );
            assert_eq!(
                a.y.to_bits(),
                b.y.to_bits(),
                "{name}: v moved at sample {i}"
            );
        }
    }
}

/// A plane-chart case for the `EdgeCurve`-door row: `(name, image,
/// carrier, t0, t1)`.
type PlaneCase = (&'static str, Pcurve<f64>, Curve3<f64>, f64, f64);

fn plane(normal: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal,
        u_ref: Vec3::unit_x(),
    }
}

/// A chart image on the plane `z = 0`, certified as an `EdgeCurve`
/// against `normal`'s plane through one keyed arena.
fn chart_edge(
    key: SurfaceKey,
    image: Pcurve<f64>,
    carrier: Curve3<f64>,
    t0: f64,
    t1: f64,
    n: Vec3<f64>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let (start, end) = (carrier.eval(t0), carrier.eval(t1));
    EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(key, image),
            carrier,
            param_start: t0,
            param_end: t1,
        },
        start,
        end,
        |_| Some(plane(n)),
        band,
    )
}

/// At the `EdgeCurve` door: a chart image on a plane whose normal is
/// then negated (what `Body::revert` does). The image as stored
/// refuses `ChartResidual` on the reverted plane at sample 0 (each
/// image here starts off the `u_ref` axis, so its first sample already
/// has a `v` channel to get wrong); the
/// mirrored image certifies there with a certificate `Debug`-identical
/// to the one it carried across; the door is an involution. Three of
/// the kinds a plane can carry — a harmonic circle, an iso line, a
/// general NURBS line — each with a non-zero `v` channel.
#[test]
fn mirrored_chart_images_recertify_on_the_reverted_plane_with_the_same_certificate() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let key = arena.insert(plane(Vec3::unit_z()));
    let cases: Vec<PlaneCase> = vec![
        (
            "Harmonic",
            Pcurve::Harmonic {
                p0: Point2::new(0.3, 0.4),
                pa: Vec2::new(1.0, 0.0),
                pb: Vec2::new(0.0, 1.0),
                pl: Vec2::new(0.0, 0.0),
            },
            Curve3::Circle {
                center: Point3::new(0.3, 0.4, 0.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            0.2,
            1.4,
        ),
        (
            "IsoLine",
            Pcurve::IsoLine {
                p0: Point2::new(0.1, 0.2),
                pl: Vec2::new(0.6, 0.8),
            },
            Curve3::Line {
                origin: Point3::new(0.1, 0.2, 0.0),
                dir: Vec3::new(0.6, 0.8, 0.0),
            },
            0.0,
            1.0,
        ),
        (
            "General",
            Pcurve::General(Arc::new(
                NurbsCurve2::new(
                    kv(vec![0.0, 0.0, 1.0, 1.0], 1),
                    vec![Point2::new(0.1, 0.2), Point2::new(0.7, 1.0)],
                    vec![1.0, 1.0],
                )
                .unwrap(),
            )),
            Curve3::Line {
                origin: Point3::new(0.1, 0.2, 0.0),
                dir: Vec3::new(0.6, 0.8, 0.0),
            },
            0.0,
            1.0,
        ),
    ];
    for (name, image, carrier, t0, t1) in cases {
        let fwd = chart_edge(key, image, carrier.clone(), t0, t1, Vec3::unit_z())
            .unwrap_or_else(|e| panic!("{name}: forward certification refused {e:?}"));
        let (start, end) = (carrier.eval(t0), carrier.eval(t1));
        let rev = plane(-Vec3::unit_z());
        let stale = fwd.recertify(start, end, |_| Some(rev.clone()), band);
        assert!(
            matches!(
                stale,
                Err(CertifyError::ResidualExceeded {
                    check: CertCheck::ChartResidual,
                    sample: 0
                })
            ),
            "{name}: the stored image on the reverted plane: {stale:?}"
        );
        let mirrored = fwd.with_chart_v_mirrored();
        let fresh = mirrored
            .recertify(start, end, |_| Some(rev.clone()), band)
            .unwrap_or_else(|e| panic!("{name}: mirrored refused on the reverted plane {e:?}"));
        assert_eq!(
            format!("{fresh:?}"),
            format!("{:?}", mirrored.certificate()),
            "{name}: the travelling certificate is not the fresh run's"
        );
        assert_eq!(
            format!("{:?}", mirrored.with_chart_v_mirrored()),
            format!("{fwd:?}"),
            "{name}: door involution"
        );
    }
}

/// A description with no chart image is invariant under the door: the
/// intrinsic arms state nothing in chart coordinates, so the curve
/// comes back `Debug`-identical.
#[test]
fn a_description_without_a_chart_image_is_invariant_under_the_door() {
    let band = Band::linear(Tol::witness()).unwrap();
    let carrier = Curve3::Line {
        origin: Point3::new(0.1, 0.2, 0.0),
        dir: Vec3::new(0.6, 0.8, 0.0),
    };
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let a = arena.insert(plane(Vec3::unit_z()));
    let b = arena.insert(Surface::Plane {
        origin: Point3::new(0.1, 0.2, 0.0),
        normal: Vec3::new(0.8, -0.6, 0.0),
        u_ref: Vec3::unit_z(),
    });
    let (start, end) = (carrier.eval(0.0), carrier.eval(1.0));
    let edge = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: a,
                s2: b,
                witness: carrier.eval(0.5),
            },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
        start,
        end,
        |k| arena.get(k).cloned(),
        band,
    )
    .expect("the line is the two planes' intersection");
    assert_eq!(
        format!("{:?}", edge.with_chart_v_mirrored()),
        format!("{edge:?}")
    );
}

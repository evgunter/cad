//! **Tangent walls whose rims cross on the tangent ruling**: an inner
//! rod (radius 1 about `(1, 0)`) and an outer one (radius 2 about the
//! origin), tangent inside one another along `x = 2, y = 0`, each cut
//! by a plane through `(2, 0, 0)`. Where the cut rims cross there, the
//! two walls leave the point to either side of the rims and part
//! (`topo::boolean::reduce` `sheets_part`), so both rims split there.
//! A cut plane slanted about the x axis makes that rim an ellipse; two
//! slanted the same way about it, two ellipses in two planes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::k_stats::Bracket;
use geom_core::{Affine3, Arc2, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;
use topo::boolean::{BooleanError, BooleanOp, BooleanResult, SweepStrategy, boolean_op_with};

fn tol() -> Tol {
    Tol::witness()
}

/// The rods' extent below and above the cuts.
const H: f64 = 3.0;

/// A rod of radius `r` about `(cx, 0)`, `z ∈ [−H, H]`, its two vertices
/// at `±y`, off the tangent ruling.
fn rod(r: f64, cx: f64) -> Body<f64> {
    let rim = RawLoop::new((0..2).map(|k| {
        let at = FRAC_PI_2 + PI * f64::from(k);
        (
            Point2::new(cx + r * at.cos(), r * at.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, 0.0),
                radius: r,
                sweep: PI,
            }),
        )
    }));
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![rim])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 2.0 * H,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, -H));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&lift, tol()), tol()).unwrap()
}

/// `body` cut below (`below`) or above the plane `z = y·tan(tilt)`.
fn cut(body: &Body<f64>, tilt: f64, below: bool) -> Body<f64> {
    let z = if below { (-10.0, 0.0) } else { (0.0, 10.0) };
    let brick = sweep::test_support::brick((-10.0, 10.0), (-10.0, 10.0), z, tol());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), tilt);
    let half = topo::transform_rigid(&brick, &geom_core::test_support::rigid(&turn, tol()), tol())
        .unwrap();
    match op(BooleanOp::Intersect, body, &half) {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("cutting a rod at tilt {tilt}: {other:?}"),
    }
}

fn op(o: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    let a = topo::test_support::finished("a", a.clone(), tol());
    let b = topo::test_support::finished("b", b.clone(), tol());
    boolean_op_with(
        o,
        &a,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
}

/// The inner rod below `z = y·tan(t_a)` and the outer above
/// `z = y·tan(t_b)`, their rims crossing at `(2, 0, 0)`: a circle where
/// a tilt is zero, an ellipse elsewhere, in two planes where the tilts
/// differ.
fn rims(t_a: f64, t_b: f64) -> (Body<f64>, Body<f64>) {
    (
        cut(&rod(1.0, 1.0), t_a, true),
        cut(&rod(2.0, 0.0), t_b, false),
    )
}

/// `o` on `x` and `y`, with the signs `bool_touch_sheets_part` decided
/// on the way, in decision order.
fn op_reading_the_parting(
    o: BooleanOp,
    x: &Body<f64>,
    y: &Body<f64>,
) -> (Result<BooleanResult<f64>, BooleanError>, Vec<Sign>) {
    let bracket = Bracket::open();
    let got = op(o, x, y);
    let parted = bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate == "bool_touch_sheets_part")
        .map(|v| v.sign)
        .collect();
    (got, parted)
}

/// The ops on [`rims`] at `(t_a, t_b)`, each held to its closed form,
/// tiers 1–3, and tier 3′ where `census` says it passes; and each read
/// as the walls parting at the crossing, once.
///
/// `A` is `3π`, `B` is `12π`: each plane's `y`-moment vanishes on its
/// disc. They share the part of `A`'s disc between the two planes, on
/// the side where `B`'s dips below `A`'s: `∫|y|·|tan t_a − tan t_b|`
/// over a half disc, `(2/3)|tan t_a − tan t_b|`. The walls part there:
/// each leaves the crossing away from the other, about the ruling, so
/// `bool_touch_sheets_part` reads Negative. With `A` first the sweep
/// reads `A`'s rim against `B`'s wall, bounded there by `B`'s rim; with
/// `B` first, the other way about.
fn builds_at_its_closed_form((t_a, t_b): (f64, f64), ops: &[(BooleanOp, &str, bool)]) {
    let (a, b) = rims(t_a, t_b);
    let shared = 2.0 / 3.0 * (t_a.tan() - t_b.tan()).abs();
    let (va, vb) = (3.0 * PI, 12.0 * PI);
    for &(o, name, census) in ops {
        let (closed, x, y) = match name {
            "A−B" => (va - shared, &a, &b),
            "A∩B" => (shared, &a, &b),
            "A∪B" => (va + vb - shared, &a, &b),
            "B−A" => (vb - shared, &b, &a),
            _ => panic!("no row {name}"),
        };
        let what = format!("tilts ({t_a}, {t_b}) {name}");
        let (got, parted) = op_reading_the_parting(o, x, y);
        assert_eq!(parted, [Sign::Negative], "{what}: the walls' parting");
        let res = match got {
            Ok(BooleanResult::Body(res)) => res,
            other => panic!("{what}: {other:?}"),
        };
        let body = &res.body;
        topo::validate(body).unwrap_or_else(|e| panic!("{what}: tier 1: {e:?}"));
        topo::validate_closed(body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
        topo::validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
        let m = topo::test_support::pinned_volume(&what, body, tol());
        assert!(
            (m.volume - closed).abs() <= m.volume_pad + 1e-9 * closed,
            "{what}: volume {} ± {} against the closed form {closed}",
            m.volume,
            m.volume_pad
        );
        let three_prime = topo::validate_pseudomanifold(body, &res.contacts, tol());
        if census {
            three_prime.unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
        } else {
            let e = format!("{:?}", three_prime.expect_err(&what));
            assert!(
                e.contains("CensusEscalated") && e.contains("curve_pair_coplanar_sub_side"),
                "{what}: tier 3′ moved: {e}"
            );
        }
    }
}

/// **The walls part where an ellipse rim crosses a wall's ellipse rim**
/// in another plane (tilts `(0.2, 0.5)`), read with the crossing rim's
/// operand first; the body builds at its closed form, tiers 1–3′.
#[test]
fn an_ellipse_rim_crossing_a_walls_ellipse_rim_reads_the_walls_parting() {
    builds_at_its_closed_form((0.2, 0.5), &[(BooleanOp::Subtract, "A−B", true)]);
}

/// **The other handedness builds too**: tilts `(0.5, 0.2)`, `tan t_a >
/// tan t_b`, so the two rims share the half of `A`'s disc at `y > 0`
/// and `B`'s section arc leaves the pinch along the tangent of the rim
/// its corner arrives on, A∖B at its closed form, tiers 1–3′.
#[test]
fn a_rim_crossing_with_the_inner_plane_steeper_builds() {
    builds_at_its_closed_form((0.5, 0.2), &[(BooleanOp::Subtract, "A−B", true)]);
}

/// **Every op, both handednesses**: a flat rim against a slanted one
/// and two rims in two planes, each with `tan t_a` below `tan t_b` and
/// above it, up to their near-equality at `(0.45, 0.5)` and
/// `(0.5, 0.45)`. Every body passes tiers 1–3 at its closed form. Tier
/// 3′ passes on `A ∖ B` and `A ∩ B`; on `A ∪ B` and `B ∖ A` it
/// escalates `curve_pair_coplanar_sub_side` on the coplanar touch of
/// `B`'s rim and the section arc, a touch the curve-pair reader cannot
/// place (`work/tangrim/two-coplanar-ellipses-have-no-meeting-method.md`).
#[test]
fn rims_crossing_on_tangent_walls_build_every_op() {
    for tilts in [
        (0.0, 0.3),
        (0.0, 1.0),
        (0.2, 0.5),
        (-0.3, 0.6),
        (0.45, 0.5),
        (0.0, -0.5),
        (0.3, -0.4),
        (0.5, 0.2),
        (0.5, 0.45),
    ] {
        builds_at_its_closed_form(
            tilts,
            &[
                (BooleanOp::Subtract, "A−B", true),
                (BooleanOp::Intersect, "A∩B", true),
                (BooleanOp::Union, "A∪B", false),
                (BooleanOp::Subtract, "B−A", false),
            ],
        );
    }
}

/// **Two slanted rims tangent on the ruling refuse at their touch**
/// before the walls' parting is asked. The inner rod below
/// `z = y·tan t`, the outer above it: both rims are ellipses in one
/// plane, tangent at `(2, 0, 0)`. The crossing layer reads their touch
/// in band, the edge is deferred, and every op refuses
/// `CurvedPierceUnsupported` (the coplanar ellipse touch,
/// `work/tangrim/two-coplanar-ellipses-have-no-meeting-method.md`). Cut
/// flat, the same rims are circles, read and built.
#[test]
fn two_slanted_rims_tangent_on_the_ruling_refuse_at_their_touch() {
    for tilt in [0.0, 0.3] {
        let a = cut(&rod(1.0, 1.0), tilt, true);
        let b = cut(&rod(2.0, 0.0), tilt, false);
        let (va, vb) = (3.0 * PI, 12.0 * PI);
        for (o, name, closed) in [
            (BooleanOp::Union, "A∪B", va + vb),
            (BooleanOp::Subtract, "A−B", va),
        ] {
            let what = format!("tilt {tilt} {name}");
            let got = op(o, &a, &b);
            if tilt != 0.0 {
                assert!(
                    matches!(got, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{what}: {got:?}"
                );
                continue;
            }
            let res = match got {
                Ok(BooleanResult::Body(res)) => res,
                other => panic!("{what}: {other:?}"),
            };
            let body = &res.body;
            topo::validate_geometric(body, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            topo::validate_pseudomanifold(body, &res.contacts, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
            let m = topo::test_support::pinned_volume(&what, body, tol());
            assert!(
                (m.volume - closed).abs() <= m.volume_pad + 1e-9 * closed,
                "{what}: volume {} ± {} against {closed}",
                m.volume,
                m.volume_pad
            );
        }
    }
}

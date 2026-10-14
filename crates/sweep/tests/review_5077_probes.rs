//! Review probes for PR 5077 (a blend's result is Boolean semantics;
//! the crossing recourses name the reorder).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom_core::{Point2, Point3};
use sweep::blend::{BlendError, FILLET3_CLEARANCE_RECOURSE, FILLET3_RING_RECOURSE};
use topo::{Body, EdgeKey, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, edge, the_box, tol, volume_enclosure};
use crate::common::cavity::{brick, cut, rod};

fn cube() -> Body<f64> {
    brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0))
}

/// The unit cube's edge at `(1, 1)`, along `z`.
fn crease(body: &Body<f64>) -> EdgeKey {
    edge(body, [1.0, 1.0, 0.0], [1.0, 1.0, 1.0])
}

/// The box's two top edges turning at `(0, 0, 1)`.
fn turn(body: &Body<f64>) -> [EdgeKey; 2] {
    [
        edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]),
    ]
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    let at = |x: &Body<f64>| sweep::test_support::at_rest(x, tol());
    topo::subtract(&at(a), &at(b), tol())
        .map_err(|e| format!("{e:?}"))
        .map(|o| o.body().expect("material").body.clone().into_body())
}

/// **The ring recourse's reorder leg, walked to a build.** A bore of
/// radius `a = 0.02` along the unit cube's `(1, 1)` edge pokes `0.01`
/// across the trimline into the kept material: it crosses the band and
/// refuses `RingClearance` at the cap carrying `FILLET3_RING_RECOURSE`
/// under both verbs. Followed — the edge blended on the cube, the bore
/// subtracted after — the reorder builds tier-3 under both verbs (the
/// fillet's band and the bore are parallel cylinders), at the rule's
/// `S − R − H`: the chamfer less the disc's segment beyond the chord,
/// the fillet less the lens the disc shares with the arc's circle.
#[test]
fn the_ring_recourses_reorder_leg_builds_on_a_parallel_bore() {
    let (a, poke) = (0.02, 0.01);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let (centre, kept) = match verb {
            // From the chord's midpoint `a − poke` towards the old vertex;
            // the segment of the disc beyond `poke` from its centre.
            Verb::Chamfer => (
                1.0 - D / 2.0 + (a - poke) * FRAC_1_SQRT_2,
                a * a * (poke / a).acos() - poke * (a * a - poke * poke).sqrt(),
            ),
            // From the arc's centre `1 − D`, `D + a − poke` along the ray;
            // the lens the disc shares with the arc's circle.
            Verb::Fillet => {
                let (d, r, big) = (D + a - poke, a, D);
                let lens = r * r * ((d * d + r * r - big * big) / (2.0 * d * r)).acos()
                    + big * big * ((d * d + big * big - r * r) / (2.0 * d * big)).acos()
                    - 0.5 * ((-d + r + big) * (d + r - big) * (d - r + big) * (d + r + big)).sqrt();
                (1.0 - D + d * FRAC_1_SQRT_2, lens)
            }
        };
        let bore = rod(Point2::new(centre, centre), a, -0.5, 1.5);
        let bored = cut("bore", &cube(), &bore);
        let err = verb
            .run(&bored, &[crease(&bored)])
            .err()
            .unwrap_or_else(|| panic!("{verb:?}: a bore crossing the trimline refuses"));
        assert!(
            matches!(err, BlendError::RingClearance { bounded: false, .. }),
            "{verb:?}: the bore's ring crosses the cap's sliver, got {err:?}"
        );
        assert!(
            err.to_string().contains(FILLET3_RING_RECOURSE),
            "{verb:?}: the refusal carries the ring recourse: {err}"
        );
        let plain = cube();
        let blended = verb
            .run(&plain, &[crease(&plain)])
            .expect("the cube's edge blends")
            .body;
        let out = subtract(&blended, &bore)
            .unwrap_or_else(|e| panic!("{verb:?}: the reordered request builds, got {e}"));
        validate_geometric(&out, tol())
            .unwrap_or_else(|e| panic!("{verb:?}: the reorder is tier-3 valid, got {e:?}"));
        let (v, pad) = volume_enclosure(&out);
        let closed = 1.0 - verb.section() - kept;
        assert!(
            (v - closed).abs() <= pad + 1e-12,
            "{verb:?}: the reorder is `S − R − H`: {v} ± {pad} vs {closed}"
        );
    }
}

/// **A box void straddling the mitre plane, inside both bands' prisms**:
/// it refuses `WhollyInside`, as C8 says a void with no cap cycle does.
#[test]
fn a_void_straddling_the_mitre_inside_both_prisms_refuses_wholly_inside() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let body = cut(
            "void",
            &the_box(),
            &brick(
                Point3::new(0.005, 0.005, 0.99),
                Point3::new(0.015, 0.015, 0.995),
            ),
        );
        let got = verb.run(&body, &turn(&body));
        assert!(
            matches!(got, Err(BlendError::WhollyInside { .. })),
            "{verb:?}: a void inside both prisms refuses WhollyInside, got {:?}",
            got.err()
        );
    }
}

/// **An L-shaped void inside the union of a mitre's two bands but inside
/// neither band's prism alone** — the case `crates/sweep/README.md`'s new
/// sentence names ("one crossing a band's region refuses as a
/// crossing"). It does refuse as a crossing, `FaceClearance` unbounded,
/// with the clearance recourse. The row that sentence cites
/// (`band_planar_mitre::a_mitre_over_a_void_refuses_where_its_bands_reach_it`)
/// is not this case: its void runs down to `z = 0.5`, outside the union.
#[test]
fn an_l_void_inside_only_a_mitres_union_refuses_as_a_crossing() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let leg_x = cut(
            "void leg x",
            &the_box(),
            &brick(
                Point3::new(0.005, 0.005, 0.99),
                Point3::new(0.5, 0.015, 0.995),
            ),
        );
        let body = cut(
            "void leg y",
            &leg_x,
            &brick(
                Point3::new(0.005, 0.005, 0.99),
                Point3::new(0.015, 0.5, 0.995),
            ),
        );
        assert_eq!(body.faces().count(), 14, "the box and one L void");
        let got = verb.run(&body, &turn(&body)).err();
        assert!(
            matches!(got, Some(BlendError::FaceClearance { bounded: false, .. })),
            "{verb:?}: the L void refuses as a crossing, got {got:?}"
        );
        let shown = got.unwrap().to_string();
        assert!(shown.contains(FILLET3_CLEARANCE_RECOURSE), "{shown}");
    }
}

/// **A blind hole entering from a support face, wholly inside the
/// band's removed material.** C8 says "a blind feature … refuses as a
/// bound where its wall meets the end face's plane"; this one's wall
/// never meets an end face's plane, and it refuses at the clearance
/// screen (`FaceClearanceUncertified`) — the arm C8 files under the
/// crossings — whose sentence reads the hole's ring and the edge as
/// "two boundary features … nearer than their blends set back".
#[test]
fn a_blind_hole_from_the_support_face_inside_the_region_refuses_at_the_screen() {
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let body = cut(
            "blind hole",
            &the_box(),
            &rod(Point2::new(1.0, 0.015), 0.005, 0.97, 1.5),
        );
        let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let got = verb.run(&body, &[e]).err();
        assert!(
            matches!(got, Some(BlendError::FaceClearanceUncertified { .. })),
            "{verb:?}: the blind hole refuses at the screen, got {got:?}"
        );
    }
}

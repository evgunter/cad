//! **The turning charts whose orientation nothing pinned**: a lofted
//! chart carrying an AUTHORED ROLL, a swept path that REVERSES its
//! curvature, and a swept path with nonzero TORSION.
//!
//! The corpus already pins two turning families, and only two: the
//! quarter-turn arc elbow (integral in the concave-sense suite,
//! rational in the skin-integrality suite) and the constant-pitch
//! helix at ½, 1 and 2 turns (the long-turn sweep suite). Every other
//! chart the tree turns — including the one whose roll the tree
//! AUTHORS rather than picking up from a path — reached the material
//! side of its walls untested. These rows are that complement.
//!
//! The claim is the corpus's, unchanged: a wall's
//! `sense_sign · (S_u × S_v)` has material against it and void along
//! it, at every sample of a chart that turns, decided by an oracle
//! that reads POSITIONS off the shipped charts and never a `sense`, a
//! winding or a normal ([`common::orient`]).
//!
//! # What each fixture is for, and what it would take back
//!
//! - **The authored roll.** `common::approx::twisted_lofted` is the
//!   tree's only lofted pair related by a ROTATION rather than by
//!   their placements: the roll lives in the section, and no path
//!   carries it. Its walls are bilinear saddles. The offset consumer
//!   suite builds it and asks whether its walls are curved; nothing
//!   asked which way they FACE.
//! - **The reversal.** Two opposed quarter arcs glued at an
//!   inflection — the tour's S duct. Neither half is new, and that is
//!   the point: the chart's roll changes HAND halfway along, which no
//!   quarter-turn arc and no helix does, and a frame that mints its
//!   wall senses off a traversal argument is entitled to be asked
//!   about the sample where the hand changes.
//! - **The torsion.** The twisted cubic `(At, Bt², Ct³)` — the tour's
//!   twisted duct, and the canonical nowhere-planar spine. A helix
//!   has torsion too but holds it CONSTANT with the curvature; here
//!   both vary continuously and no arc is anywhere in the path.
//!
//! Each row carries an anti-vacuity condition on the shape (a chart
//! that does not turn makes the claim above free) and, where the
//! shape has one, a HANDEDNESS pin: the roll's sign, the reversal's
//! two signs, the frame's signed non-planarity. Those read positions
//! only, so a mirrored fixture reddens them while a flipped `sense`
//! does not — and the walls-face-out rows are the other way round.
//! Neither half covers for the other.
//!
//! # Which index answers, and one of them is measured refusing
//!
//! `common::orient` carries two. The authored roll and the torsion
//! duct are answered by the FIXED-CHORD index: their level planes stay
//! orientable against the stacking chord and their level height falls
//! monotonically, both asserted by the index on every query.
//!
//! The INFLECTING duct is not, and the way it fails is worth the row
//! on its own: its planes stay comfortably orientable — the stacking
//! chord bisects the S — while the monotone height, the index's other
//! and stronger condition, breaks in the second arc. The index refuses
//! there rather than answering from whichever root the bisection lands
//! on, so that row runs on the continuity index instead. A shape that
//! separates the two conditions in the direction the index's docs
//! predict, on a body the tour ships.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6};

use geom::NurbsCurve3;
use geom_core::{Point3, Tol, Vec3};
use sweep::{Lofted, sweep_body};

use crate::common;
use common::orient::{
    LevelIndex, along_v, assert_caps_face_out, assert_walls_face_out, loft_contains, ring_centroid,
    wall_outward_at, wall_point_at,
};
use common::{normal_start_place, quad};

/// Half-width of the swept square section, and the step off a wall the
/// material-side probe takes: a fifth of the half-width, so an inward
/// step from a column at `u = 0.25` or `0.75` stays clear of the
/// neighbouring walls, and an order of magnitude above the level
/// polyline's chord bound (the shared `assert_probe_step` coupling
/// enforces that half).
const H: f64 = 0.25;
const PROBE_DELTA: f64 = 0.05;

/// The probe step for the authored-roll loft, whose section is the
/// 2 x 2 square the offset suite builds it with.
const LOFT_PROBE_DELTA: f64 = 0.05;

// ---------------------------------------------------------------------
// The authored roll
// ---------------------------------------------------------------------

/// The signed angle, about `axis`, from `a` to `b` — both measured
/// from the same centre. Positive is counterclockwise seen from the
/// tip of `axis`.
fn signed_angle_about(axis: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    let n = axis / axis.norm();
    let a = a - n * a.dot(n);
    let b = b - n * b.dot(n);
    a.cross(b).dot(n).atan2(a.dot(b))
}

/// **Every wall of an AUTHORED-ROLL loft faces out of the material.**
///
/// Run at three angles, of which the first is the one the offset
/// consumer suite ships (`0.05`): a row that only ever saw a large
/// roll would not be a row about the fixture the tree actually builds,
/// and a row that only ever saw the small one would clear its own
/// anti-vacuity bar by a hair.
///
/// ANTI-VACUITY, in two parts, because the fixture's own parameter is
/// the thing that could quietly go to zero:
///
/// - the chart TURNS: one wall's outward normal at the two ends of `v`
///   is `theta` apart, asserted against nine tenths of `theta` so the
///   bar tracks the parameter instead of a transcribed number;
/// - the roll has the authored HAND: the level ring at the top is
///   rotated by `+theta` about the stacking direction relative to the
///   bottom one, measured from positions off the shipped charts. A
///   fixture rolled the other way passes every containment probe in
///   this file — the point set is a mirror image and the walls still
///   face out of it — so nothing else here can catch it.
#[test]
fn an_authored_roll_loft_faces_out_at_every_level() {
    for theta in [0.05, FRAC_PI_6, FRAC_PI_4] {
        let lofted = common::approx::twisted_lofted(theta);
        assert_eq!(topo::validate(&lofted.body), Ok(()), "roll {theta}: tier 1");
        assert_eq!(
            topo::validate_closed(&lofted.body),
            Ok(()),
            "roll {theta}: tier 2"
        );

        let wall = lofted.side_faces[0][0];
        let roll = theta - FRAC_PI_2;
        let n0 = wall_outward_at(&lofted.body, wall, 0.5, 0.0).1;
        let n1 = wall_outward_at(&lofted.body, wall, 0.5, 1.0).1;
        assert!(
            n0.dot(n1) <= (0.9 * roll.abs()).cos(),
            "roll {roll}: the chart must turn with the roll, not by cos = {} — a \
             loft between two copies of one section would read 1 and the rows \
             below would be the prism again",
            n0.dot(n1)
        );

        let (c0, c1) = (ring_centroid(&lofted, 0.0), ring_centroid(&lofted, 1.0));
        let hand = signed_angle_about(
            c1 - c0,
            wall_point_at(&lofted.body, wall, 0.5, 0.0) - c0,
            wall_point_at(&lofted.body, wall, 0.5, 1.0) - c1,
        );
        assert!(
            (hand - roll).abs() < 1e-9,
            "authored {theta}: the top ring must be rotated by {roll} about the \
             stacking direction, not {hand} — the fixture's hand is authored and \
             a mirrored one is a different body"
        );

        let oracle = |q| loft_contains(&lofted, q);
        assert_walls_face_out(&lofted, &oracle, &along_v(), LOFT_PROBE_DELTA, 4);
        assert_caps_face_out(&lofted, &oracle, LOFT_PROBE_DELTA);
    }
}

// ---------------------------------------------------------------------
// The reversal, and the torsion
// ---------------------------------------------------------------------

/// Radius of the two opposed arcs of the inflecting path.
const S_RADIUS: f64 = 2.0;

/// The inflecting path: a quarter arc of radius [`S_RADIUS`] turning
/// one way, then a quarter arc of the same radius turning the other,
/// in the world `x = 0` plane, interpolated at degree 3 through 17
/// exact points. The tangent runs `+z -> +y -> +z` and never reverses,
/// so the path-following frame is total.
fn inflecting_path() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_RADIUS * (1.0 - th.cos()), S_RADIUS * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(
                0.0,
                S_RADIUS + S_RADIUS * ph.sin(),
                2.0 * S_RADIUS - S_RADIUS * ph.cos(),
            )
        }))
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).expect("the inflecting path interpolates")
}

/// The twisted cubic `(At, Bt², Ct³)` on `t` in `[-1, 1]`, degree 3
/// through 33 exact points. Its torsion `12ABC/|r' x r''|²` has a
/// constant numerator, so no point of it has an osculating plane the
/// curve stays in: the spine is nowhere planar and carries no arc.
fn torsion_path() -> NurbsCurve3<f64> {
    let (a, b, c) = (2.2, 1.3, 1.5);
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0_f64.mul_add(f64::from(k) / 32.0, -1.0);
            Point3::new(a * t, b * t * t, c * t * t * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).expect("the twisted cubic interpolates")
}

/// A square section swept along `path` at `stations`, tier-1 and
/// tier-2 checked.
fn swept_square(path: &NurbsCurve3<f64>, stations: usize, what: &str) -> Lofted<f64> {
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    let swept = sweep_body::<f64>(
        &profile,
        normal_start_place(path),
        path,
        stations,
        3,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the {what} sweeps: {e:?}"));
    assert_eq!(topo::validate(&swept.body), Ok(()), "{what}: tier 1");
    assert_eq!(topo::validate_closed(&swept.body), Ok(()), "{what}: tier 2");
    swept
}

/// Steps the spine is read at when a row measures how the body turns.
const SPINE_STEPS: usize = 64;

/// The chords of the body's own spine — the level rings' centroids
/// along `v`, differenced. POSITIONS only: how a body turns is a fact
/// about where its material is, and a row that read a chart normal to
/// say it would be reading the datum the containment rows are testing.
fn spine_chords(lofted: &Lofted<f64>, steps: usize) -> Vec<Vec3<f64>> {
    #[allow(clippy::cast_precision_loss)]
    let centres: Vec<Point3<f64>> = (0..=steps)
        .map(|i| ring_centroid(lofted, i as f64 / steps as f64))
        .collect();
    centres.windows(2).map(|w| w[1] - w[0]).collect()
}

/// **Every wall of a sweep whose path REVERSES its turn faces out of
/// the material — on both sides of the inflection.**
///
/// ANTI-VACUITY, and it is the whole reason this shape is not the
/// elbow twice: the chart's roll must change HAND. The path is planar
/// (the world `x = 0` plane), so each half's turn is signed about `x`,
/// and the two halves must carry opposite signs, each reaching nine
/// tenths of the quarter turn its arc subtends. An end-to-end reading
/// cannot see this at all — the tangent starts and finishes at `+z`,
/// exactly as on a straight path.
///
/// The signs are read off wall NORMALS, which a flipped `sense` would
/// negate uniformly — so this condition survives such a flip and says
/// nothing about it. The containment rows below are what a flip
/// reddens.
#[test]
fn an_inflecting_path_sweep_faces_out_through_the_reversal() {
    let path = inflecting_path();
    let swept = swept_square(&path, 13, "inflecting duct");

    let plane_normal = Vec3::new(1.0, 0.0, 0.0);
    let spine = spine_chords(&swept, SPINE_STEPS);
    let signed_turn = |half: &[Vec3<f64>]| -> f64 {
        half.windows(2)
            .map(|w| signed_angle_about(plane_normal, w[0], w[1]))
            .sum()
    };
    let mid = spine.len() / 2;
    let (first, second) = (signed_turn(&spine[..=mid]), signed_turn(&spine[mid..]));
    let bar = 0.9 * FRAC_PI_2;
    assert!(
        first <= -bar && second >= bar,
        "the two halves of the path must turn the section plane OPPOSITE ways, \
         by at least {bar} rad each: measured {first} then {second} — a path \
         that turned one way throughout is the elbow row retyped, and an \
         end-to-end reading of this one cannot tell it from a straight tube"
    );

    // The fixed-chord index cannot answer here and says so: its level
    // planes stay well inside the orientability guard (the chord bisects
    // the S at 45 degrees to both ends) while the level HEIGHT stops
    // falling near v = 0.86, where the second arc's planes fan back over
    // a point the first arc's have already passed. That is the gap the
    // index's own docs name between its two conditions, measured on a
    // shipped shape rather than argued.
    let index = LevelIndex::build(&swept);
    let oracle = |q| index.contains(q);
    assert_walls_face_out(&swept, &oracle, &along_v(), PROBE_DELTA, 4);
    assert_caps_face_out(&swept, &oracle, PROBE_DELTA);
}

/// **Every wall of a sweep along a nowhere-planar path faces out of
/// the material.**
///
/// ANTI-VACUITY: the frame must genuinely leave a plane, which is what
/// separates this fixture from every arc and from the S above. Three
/// wall normals along `v` are read and their SIGNED triple product
/// asserted — zero for any planar path, whatever it turns through, and
/// its sign is the frame's hand. A mirrored spine negates it while
/// leaving every containment probe in this row green.
#[test]
fn a_torsion_bearing_path_sweep_faces_out_along_the_twist() {
    let path = torsion_path();
    let swept = swept_square(&path, 17, "twisted duct");

    let spine = spine_chords(&swept, SPINE_STEPS);
    let (a, b, c) = (spine[0], spine[spine.len() / 2], spine[spine.len() - 1]);
    let chirality = a.cross(b).dot(c) / (a.norm() * b.norm() * c.norm());
    assert!(
        chirality > 0.1,
        "the spine must leave the plane, with the hand the twisted cubic has: \
         three of its chords have signed triple product {chirality} once \
         normalized — a planar path reads zero however far it turns, and a \
         mirrored spine reads the negation"
    );

    let oracle = |q| loft_contains(&swept, q);
    assert_walls_face_out(&swept, &oracle, &along_v(), PROBE_DELTA, 4);
    assert_caps_face_out(&swept, &oracle, PROBE_DELTA);
}

//! **`arc_onto(target)` and the arrival token on `Via`/`Center`**
//! (PATHS-DESIGN §2a item 6; §2 Closure; §6 PQ4).
//!
//! Every cell of item 6's two tables, the ruling's `B` and `D`, the
//! round trip against `tangent_arc_to`, and the four token cells with
//! their refusal twins. Each program is replayed at `f64` and at
//! `Interval` through the one driver, which calls the typed binders, so
//! a row is a claim about both surfaces at both scalars.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{tol, try_replay_at};
use geom_core::{Interval, Point2, Real};
use profile::{
    ArcData, ArcSweep, ConstructedLoop, ConstructedProfile, Open, PathErrorKind, ReplayError,
    ReplayErrorKind, Segment, SketchPlane, Start, Step, Target, TipState, Verb,
};

fn pt(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn xy(p: Point2<f64>) -> (f64, f64) {
    (p.x, p.y)
}

fn to(x: f64, y: f64) -> Step<f64> {
    Step::LineTo(Target::Point(pt(x, y)))
}

fn toward(dx: f64, dy: f64) -> Step<f64> {
    Step::Toward { dx, dy }
}

fn onto(x: f64, y: f64) -> Step<f64> {
    Step::ArcOnto(Target::Point(pt(x, y)))
}

/// A leg end at `(2, 0)` heading `+x`.
fn leg_end() -> Vec<Step<f64>> {
    vec![Step::At(pt(0.0, 0.0)), to(2.0, 0.0)]
}

fn with(mut lead: Vec<Step<f64>>, tail: &[Step<f64>]) -> Vec<Step<f64>> {
    lead.extend_from_slice(tail);
    lead
}

fn validates<T: profile::ArcCarrierScalar>(lp: &ConstructedLoop<T>, what: &str) {
    if let Err(e) = ConstructedProfile::new(SketchPlane::xy(), vec![lp.clone()]).validate(tol()) {
        panic!("{what}: the closed loop does not validate: {e}");
    }
}

/// The program closes at both scalars, the two agree on the
/// constructed joints, and both validate. Returns the `f64` loop.
fn closes(steps: &[Step<f64>], what: &str) -> ConstructedLoop<f64> {
    let base = try_replay_at::<f64>(steps).unwrap_or_else(|e| panic!("{what}, f64: {e}"));
    let iv = try_replay_at::<Interval>(steps).unwrap_or_else(|e| panic!("{what}, Interval: {e}"));
    assert_eq!(
        base.constructed_joints(),
        iv.constructed_joints(),
        "{what}: the constructed joints differ between the lanes"
    );
    validates(&base, what);
    validates(&iv, what);
    base
}

/// The program refuses at step `at` with `kind`, at both scalars.
/// Returns the `f64` refusal's message.
fn refuses(steps: &[Step<f64>], at: usize, kind: PathErrorKind, what: &str) -> String {
    fn read<T: Real>(
        r: Result<ConstructedLoop<T>, ReplayError<T>>,
    ) -> (usize, PathErrorKind, String) {
        match r {
            Err(ReplayError {
                step,
                kind: ReplayErrorKind::Path(e),
            }) => (step, e.kind(), e.to_string()),
            Err(e) => panic!("not a geometry refusal: {e}"),
            Ok(_) => panic!("closed; a refusal was expected"),
        }
    }
    let (s64, k64, msg) = read(try_replay_at::<f64>(steps));
    let (siv, kiv, _) = read(try_replay_at::<Interval>(steps));
    assert_eq!((s64, k64), (at, kind), "{what}, f64: {msg}");
    assert_eq!((siv, kiv), (at, kind), "{what}, Interval");
    msg
}

/// The program's last step is a lattice violation at `state`.
fn ill_typed(steps: &[Step<f64>], state: TipState, what: &str) {
    let verb = steps.last().expect("a step").verb();
    for r in [
        try_replay_at::<f64>(steps)
            .err()
            .map(|e| (e.step, format!("{:?}", e.kind))),
        try_replay_at::<Interval>(steps)
            .err()
            .map(|e| (e.step, format!("{:?}", e.kind))),
    ] {
        assert_eq!(
            r,
            Some((
                steps.len() - 1,
                format!(
                    "{:?}",
                    ReplayErrorKind::<f64>::Transition {
                        state,
                        verb: Some(verb)
                    }
                )
            )),
            "{what}"
        );
    }
}

fn arc_of(lp: &ConstructedLoop<f64>, k: usize) -> geom_core::Arc2<f64> {
    match lp.segments()[k] {
        Segment::Arc(arc) => arc,
        Segment::Line => panic!("segment {k} is a line"),
    }
}

/// The travel tangent of `arc` at `w` (a point on it).
fn travel(arc: geom_core::Arc2<f64>, w: Point2<f64>) -> (f64, f64) {
    let r = w - arc.centre;
    let s = arc.sweep.signum() / r.norm();
    (-r.y * s, r.x * s)
}

// ------------------------------------------------------------------
// Where it starts (§2a item 6, the first table)
// ------------------------------------------------------------------

/// From the entry's plain point there is no junction, and the derived
/// departure becomes `Start`'s direction: an arc from the origin to
/// `(2, 0)` arriving south leaves north, so a closer arriving north is
/// an undeclared tangent seam, and the same closer with the token
/// closes with joint 0 constructed.
#[test]
fn from_the_entry_the_derived_departure_is_starts_direction() {
    let lead = vec![
        Step::At(pt(0.0, 0.0)),
        onto(2.0, 0.0),
        toward(0.0, -1.0),
        to(3.0, -2.0),
        to(0.0, -2.0),
    ];
    let msg = refuses(
        &with(lead.clone(), &[Step::LineTo(Target::Start)]),
        5,
        PathErrorKind::SeamTangent,
        "an undeclared north arrival at an entry that leaves north",
    );
    assert!(
        msg.contains("Start.arrives_tangent()"),
        "the recourse names the token: {msg}"
    );
    let lp = closes(
        &with(lead, &[Step::LineTo(Target::StartArriving)]),
        "the declared north arrival",
    );
    assert_eq!(
        lp.constructed_joints(),
        &[0],
        "joint 0 is the declared seam"
    );
    let arc = arc_of(&lp, 0);
    assert_eq!(
        (arc.centre.x, arc.centre.y, arc.radius, arc.sweep),
        (1.0, 0.0, 1.0, -core::f64::consts::PI),
        "the half circle over the chord, clockwise"
    );
}

/// From a leg end the derived departure is junction-checked: sharp
/// proceeds, tangent refuses `JunctionTangent` naming
/// `.tangent().tangent_arc_to(p)`, reversed refuses `JunctionCusp`.
#[test]
fn from_a_leg_end_the_derived_departure_is_junction_checked() {
    // (1, -1)/√2 out of a +x leg end: a 45° corner.
    closes(
        &with(
            leg_end(),
            &[
                onto(3.0, 1.0),
                toward(-1.0, 1.0),
                to(0.0, 1.0),
                Step::LineTo(Target::Start),
            ],
        ),
        "a sharp derived departure",
    );
    let msg = refuses(
        &with(leg_end(), &[onto(3.0, 1.0), toward(0.0, 1.0)]),
        3,
        PathErrorKind::JunctionTangent,
        "a derived departure along the incoming leg",
    );
    assert!(
        msg.contains(".tangent().tangent_arc_to(p)"),
        "the recourse names the tangent-departing spelling: {msg}"
    );
    // The recourse builds that very arc: it arrives along (0, 1).
    let recourse = Open
        .at(pt(0.0, 0.0))
        .line_to(pt(2.0, 0.0), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(pt(3.0, 1.0), tol())
        .unwrap()
        .tangent()
        .line(1.0, tol())
        .unwrap()
        .line_to(pt(0.0, 2.0), tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap();
    assert_eq!(
        xy(recourse.loop_.vertices()[3]),
        (3.0, 2.0),
        "the recourse arrives north"
    );
    refuses(
        &with(leg_end(), &[onto(3.0, 1.0), toward(0.0, -1.0)]),
        3,
        PathErrorKind::JunctionCusp,
        "a derived departure reversing the incoming leg",
    );
}

/// A fillet arrival's bare anchor refuses every target, at the verb.
#[test]
fn a_fillet_arrivals_bare_anchor_refuses_arc_leg_on_open_fillet() {
    let lead = vec![
        Step::At(pt(0.0, 0.0)),
        toward(1.0, 0.0),
        Step::Fillet { radius: 0.5 },
        Step::At(pt(3.0, 1.0)),
    ];
    for target in [
        Target::Point(pt(4.0, 2.0)),
        Target::Start,
        Target::StartArriving,
    ] {
        refuses(
            &with(lead.clone(), &[Step::ArcOnto(target)]),
            4,
            PathErrorKind::ArcLegOnOpenFillet,
            &format!("arc_onto({target:?}) on a bare fillet anchor"),
        );
    }
}

/// Directed, `Angle`, `Open` and the entry have no row; the verb's rows
/// are the two point states, and its pending states take exactly the
/// two directors.
#[test]
fn arc_onto_is_ill_typed_off_a_point_and_its_pending_states_take_one_director() {
    assert_eq!(
        Verb::ArcOnto.states(),
        &[TipState::PlainPoint, TipState::DirectedPoint],
        "arc_onto leaves a point tip only"
    );
    for state in [TipState::ArcOnto, TipState::ArcOntoStart] {
        let takers: Vec<Verb> = Verb::ALL
            .iter()
            .copied()
            .filter(|v| v.states().contains(&state))
            .collect();
        assert_eq!(
            takers,
            vec![Verb::Angle, Verb::Toward],
            "{state:?} awaits one director"
        );
    }
    let o = onto(3.0, 1.0);
    ill_typed(&[o], TipState::Entry, "at the entry");
    ill_typed(
        &[Step::Angle(0.0), o],
        TipState::Angle,
        "on an angle-first entry",
    );
    ill_typed(
        &[Step::At(pt(0.0, 0.0)), Step::Angle(0.0), o],
        TipState::DirectedPlain,
        "on a directed entry",
    );
    ill_typed(
        &with(leg_end(), &[Step::Angle(1.0), o]),
        TipState::DirectedIncoming,
        "on a directed leg end",
    );
    ill_typed(
        &[
            Step::At(pt(0.0, 0.0)),
            Step::Angle(0.0),
            Step::Fillet { radius: 0.5 },
            o,
        ],
        TipState::Open,
        "on an open fillet arrival",
    );
    for follower in [
        Step::At(pt(4.0, 4.0)),
        Step::FarEndTo(pt(4.0, 4.0)),
        Step::Tangent,
        Step::Turn(0.5),
        Step::Line(1.0),
        to(4.0, 4.0),
    ] {
        ill_typed(
            &with(leg_end(), &[o, follower]),
            TipState::ArcOnto,
            &format!("{follower:?} after arc_onto(p)"),
        );
        ill_typed(
            &with(leg_end(), &[Step::ArcOnto(Target::Start), follower]),
            TipState::ArcOntoStart,
            &format!("{follower:?} after arc_onto(Start)"),
        );
    }
}

// ------------------------------------------------------------------
// What the target says (§2a item 6, the second table)
// ------------------------------------------------------------------

/// An interior target lands a directed point incoming the director, its
/// departure free: `.tangent()` runs on along the director exactly, and
/// a director turns a corner there.
#[test]
fn an_interior_target_lands_a_directed_point_incoming_the_director() {
    let tail = [onto(3.0, 2.0), toward(0.0, 1.0)];
    let lp = closes(
        &with(
            with(leg_end(), &tail),
            &[
                Step::Tangent,
                Step::Line(1.0),
                to(0.0, 3.0),
                Step::LineTo(Target::Start),
            ],
        ),
        "the tangent continuation off the landed point",
    );
    assert_eq!(
        xy(lp.vertices()[3]),
        (3.0, 3.0),
        "the run continues along (0, 1), bit for bit"
    );
    let lp = closes(
        &with(
            with(leg_end(), &tail),
            &[
                toward(-1.0, 0.0),
                Step::Line(3.0),
                Step::LineTo(Target::Start),
            ],
        ),
        "a corner at the landed point",
    );
    assert_eq!(xy(lp.vertices()[3]), (0.0, 2.0), "the corner turns west");
    // The arc arrives along (0, 1): its travel tangent at (3, 2).
    let (tx, ty) = travel(arc_of(&lp, 1), pt(3.0, 2.0));
    assert!(
        tx.abs() < 1e-15 && (ty - 1.0).abs() < 1e-15,
        "arrival ({tx}, {ty})"
    );
}

/// `Start` with a director is the sharp seam, classified as every seam
/// is: a zero turn refuses `SeamTangent`, whose recourse is the token.
#[test]
fn start_with_a_director_is_the_classified_sharp_seam() {
    let d = [
        Step::At(pt(0.0, 0.0)),
        toward(0.0, 1.0),
        Step::Line(2.0),
        to(1.0, 2.0),
        Step::ArcOnto(Target::Start),
    ];
    let msg = refuses(
        &with(d.to_vec(), &[toward(0.0, 1.0)]),
        5,
        PathErrorKind::SeamTangent,
        "a director along Start's own direction",
    );
    assert!(
        msg.contains("Start.arrives_tangent()"),
        "the recourse is the token: {msg}"
    );
    let lp = closes(&with(d.to_vec(), &[toward(-1.0, 1.0)]), "a sharp seam");
    assert!(
        lp.constructed_joints().is_empty(),
        "nothing is constructed at a sharp seam"
    );
}

/// **The ruling's `B`**: entered at the spine's foot, the lower bump
/// arrives west and closes the sharp seam; the waist turns about 124°
/// and the loop validates (neither bump touches the spine).
#[test]
fn the_b_closes_at_the_sharp_seam_and_validates() {
    let b = Open
        .at(pt(0.0, 0.0))
        .toward(0.0, 1.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .toward(1.0, 0.0, tol())
        .unwrap()
        .tangent_arc_to(pt(0.25, 1.0), tol())
        .unwrap()
        .arc_onto(Start, tol())
        .unwrap()
        .toward(-1.0, 0.0, tol())
        .unwrap();
    let lp = closes(&b.program, "the B");
    let (upper, lower) = (arc_of(&lp, 1), arc_of(&lp, 2));
    assert_eq!(
        (lower.centre.x, lower.centre.y, lower.radius),
        (0.0, 0.53125, 0.53125),
        "the lower bump is the circle tangent to west at the foot, through the waist"
    );
    assert!(lower.sweep < 0.0, "it runs clockwise into the foot");
    let waist = pt(0.25, 1.0);
    let (ux, uy) = travel(upper, waist);
    let (lx, ly) = travel(lower, waist);
    assert!(
        (ux + 15.0 / 17.0).abs() < 1e-15 && (uy + 8.0 / 17.0).abs() < 1e-15,
        "the upper bump ends heading (-15, -8)/17: ({ux}, {uy})"
    );
    assert!(
        (lx - 15.0 / 17.0).abs() < 1e-15 && (ly + 8.0 / 17.0).abs() < 1e-15,
        "the lower bump leaves heading (15, -8)/17: ({lx}, {ly})"
    );
    let turn = (ux * lx + uy * ly).acos().to_degrees();
    let about = (-161.0f64 / 289.0).acos().to_degrees();
    assert!(
        (turn - about).abs() < 1e-9 && turn > 123.0,
        "the waist turns {turn}°, not {about}°"
    );
}

/// **The ruling's `D`**: `arc_onto(Start.arrives_tangent())` completes
/// at the verb, the token supplying `Start`'s direction: the cap arrives
/// north at the entry, joint 0 is constructed, and the departure at the
/// tip is a checked corner.
#[test]
fn the_d_closes_with_the_token_supplied_and_validates() {
    let d = Open
        .at(pt(0.0, 0.0))
        .toward(0.0, 1.0, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .line_to(pt(1.0, 2.0), tol())
        .unwrap()
        .arc_onto(Start.arrives_tangent(), tol())
        .unwrap();
    let lp = closes(&d.program, "the D");
    assert_eq!(
        lp.constructed_joints(),
        &[0],
        "the seam's joint is constructed"
    );
    let cap = arc_of(&lp, 2);
    assert_eq!(
        (cap.centre.x, cap.centre.y, cap.radius),
        (2.5, 0.0, 2.5),
        "the cap is the circle tangent to north at the entry, through the tip"
    );
    let (tx, ty) = travel(cap, pt(0.0, 0.0));
    assert!(
        tx.abs() < 1e-15 && (ty - 1.0).abs() < 1e-15,
        "arrival ({tx}, {ty})"
    );
}

/// Both ends tangent refuses at the DEPARTURE: on the stadium the cap
/// the token supplies would leave tangent to the incoming straight, so
/// it refuses `JunctionTangent` and the stadium stays
/// `.tangent().tangent_arc_to(Start.arrives_tangent())`.
#[test]
fn the_token_on_the_stadium_refuses_at_the_departure() {
    let stadium = vec![
        Step::At(pt(0.0, 0.0)),
        toward(1.0, 0.0),
        Step::Line(2.0),
        Step::Tangent,
        Step::TangentArcTo(Target::Point(pt(2.0, 2.0))),
        Step::Tangent,
        Step::Line(2.0),
    ];
    let msg = refuses(
        &with(stadium.clone(), &[Step::ArcOnto(Target::StartArriving)]),
        7,
        PathErrorKind::JunctionTangent,
        "arc_onto's tangent closer on the stadium",
    );
    assert!(msg.contains(".tangent().tangent_arc_to(p)"), "{msg}");
    closes(
        &with(
            stadium,
            &[Step::Tangent, Step::TangentArcTo(Target::StartArriving)],
        ),
        "the stadium's own closer",
    );
}

/// The degeneracies are `tangent_arc_to`'s: a target at the tip and an
/// arrival along the chord BEHIND are degenerate chords; along the
/// chord AHEAD is a straight segment.
#[test]
fn the_degeneracies_mirror_tangent_arc_to() {
    refuses(
        &with(leg_end(), &[onto(2.0, 0.0), toward(0.0, 1.0)]),
        3,
        PathErrorKind::DegenerateArcChord,
        "a target at the tip",
    );
    refuses(
        &[
            Step::At(pt(0.0, 0.0)),
            Step::ArcOnto(Target::Start),
            toward(1.0, 0.0),
        ],
        2,
        PathErrorKind::DegenerateArcChord,
        "Start targeted from the entry itself",
    );
    refuses(
        &[Step::At(pt(0.0, 0.0)), Step::ArcOnto(Target::StartArriving)],
        1,
        PathErrorKind::DegenerateArcChord,
        "the token targeted from the entry itself",
    );
    refuses(
        &with(leg_end(), &[onto(3.0, 1.0), toward(-1.0, -1.0)]),
        3,
        PathErrorKind::DegenerateArcChord,
        "an arrival along the chord, behind",
    );
    // An oblique director: its unit ray is rounded, so the target sits
    // a sub-band `across` off it, and the line is the DECISION's, not
    // an exact zero's.
    on_the_ray_is_a_line(
        &with(
            leg_end(),
            &[
                onto(3.0, 1.0),
                toward(1.0, 1.0),
                to(0.0, 1.0),
                Step::LineTo(Target::Start),
            ],
        ),
        1,
        "an arrival along the chord, ahead",
    );
}

/// A target decided on the departure line is the straight segment at
/// both scalars: at f64 the leg at `k` closes stored as a line, and at
/// Interval it closes as a line too or escalates — never as an arc read
/// off a sub-band `across` (a ~1e16 carrier at f64, an unbounded
/// enclosure at Interval). Returns the f64 loop.
fn on_the_ray_is_a_line(steps: &[Step<f64>], k: usize, what: &str) -> ConstructedLoop<f64> {
    let base = try_replay_at::<f64>(steps).unwrap_or_else(|e| panic!("{what}, f64: {e}"));
    validates(&base, what);
    assert!(
        matches!(base.segments()[k], Segment::Line),
        "{what}, f64: segment {k} is {:?}, not the straight segment",
        base.segments()[k]
    );
    match try_replay_at::<Interval>(steps) {
        Ok(iv) => {
            validates(&iv, what);
            assert!(
                matches!(iv.segments()[k], Segment::Line),
                "{what}, Interval: segment {k} is {:?}, not the straight segment",
                iv.segments()[k]
            );
            assert_eq!(
                base.constructed_joints(),
                iv.constructed_joints(),
                "{what}: the lanes' constructed joints"
            );
        }
        Err(ReplayError {
            kind: ReplayErrorKind::Path(e),
            ..
        }) if e.kind() == PathErrorKind::Escalated => {}
        Err(e) => panic!("{what}, Interval: neither a line nor an escalation: {e}"),
    }
    base
}

/// The silent case: a director decided along the chord to the token's
/// `Start`. At f64 the closer is the straight segment, its seam joint
/// constructed; at Interval it was an arc of unbounded radius that no
/// later junction read, so the loop validated.
#[test]
fn the_token_closer_along_its_own_chord_is_a_line_at_both_scalars() {
    for (dx, dy) in [(1.0, 1.0), (3.0, 1.0)] {
        let start = pt(1.0, 1.0);
        let back = pt(start.x - dx, start.y - dy);
        let lp = on_the_ray_is_a_line(
            &[
                Step::At(start),
                toward(dx, dy),
                Step::Line(2.0),
                to(-1.0, 3.0),
                to(back.x, back.y),
                Step::ArcOnto(Target::StartArriving),
            ],
            3,
            &format!("arc_onto(Start.arrives_tangent()) along ({dx}, {dy})"),
        );
        assert_eq!(
            lp.constructed_joints(),
            &[0],
            "the seam's joint is constructed"
        );
    }
}

/// The same reading through a `.tangent()` follower, which reads no
/// junction at the landed point: through `tangent_arc_to` and through
/// `arc_onto`.
#[test]
fn a_tangent_follower_after_a_leg_along_its_chord_reads_a_line() {
    let ahead = [
        Step::Tangent,
        Step::Line(1.0),
        to(0.0, 2.0),
        Step::LineTo(Target::Start),
    ];
    on_the_ray_is_a_line(
        &with(
            with(
                leg_end(),
                &[
                    toward(1.0, 1.0),
                    Step::TangentArcTo(Target::Point(pt(3.0, 1.0))),
                ],
            ),
            &ahead,
        ),
        1,
        "tangent_arc_to along the departure, then .tangent()",
    );
    on_the_ray_is_a_line(
        &with(with(leg_end(), &[onto(3.0, 1.0), toward(1.0, 1.0)]), &ahead),
        1,
        "arc_onto along the chord, then .tangent()",
    );
}

// ------------------------------------------------------------------
// The round trip against tangent_arc_to
// ------------------------------------------------------------------

/// `arc_onto` is `tangent_arc_to` run backwards, through its kernel, so
/// the arc from `A` to `P` arriving along `t₁` is, bit for bit, the arc
/// `tangent_arc_to` builds from `P` leaving along `−t₁` to `A`,
/// reversed: the same carrier and the sweep negated. That holds wherever
/// the two directors are exact negations — `.toward` components, as
/// here; an `.angle(θ)` against `.angle(θ + π)` agrees only to rounding,
/// since `θ + π` rounds before `sin_cos` sees it.
#[test]
fn arc_onto_is_tangent_arc_to_reversed_bit_for_bit() {
    let (a, p) = (pt(0.0, 0.0), pt(3.0, 1.0));
    let forward = vec![
        Step::At(p),
        toward(-3.0, -4.0),
        Step::TangentArcTo(Target::Point(a)),
        Step::LineTo(Target::Start),
    ];
    let mirrored = vec![
        Step::At(a),
        Step::ArcOnto(Target::Point(p)),
        toward(3.0, 4.0),
        Step::LineTo(Target::Start),
    ];
    let (f, m) = (closes(&forward, "forward"), closes(&mirrored, "mirrored"));
    assert_eq!(
        format!("{:?}", m.segments()[0]),
        format!("{:?}", f.reversed().segments()[1]),
        "f64: arc_onto's arc is the tangent arc retraced"
    );
    let fi = try_replay_at::<Interval>(&forward).unwrap();
    let mi = try_replay_at::<Interval>(&mirrored).unwrap();
    assert_eq!(
        format!("{:?}", mi.segments()[0]),
        format!("{:?}", fi.reversed().segments()[1]),
        "Interval: arc_onto's arc is the tangent arc retraced"
    );
    assert!(
        matches!(m.segments()[0], Segment::Arc(arc) if arc.sweep != 0.0),
        "the compared segment is a real arc"
    );
}

// ------------------------------------------------------------------
// The token on Via and Center (§2 Closure: check or supply)
// ------------------------------------------------------------------

/// A `+x` side, then up to `(2, 1)`: the closing arc about `(0, 2.5)`
/// arrives along `+x` at the entry.
fn up_the_side() -> Vec<Step<f64>> {
    vec![
        Step::At(pt(0.0, 0.0)),
        toward(1.0, 0.0),
        Step::Line(2.0),
        to(2.0, 1.0),
    ]
}

/// The `Via` and `Center` LEGS fix their end tangent, so the token
/// CHECKS it: a tangent arrival closes with joint 0 constructed, an
/// off-tangent one refuses `SeamArrivalOffDirection`, and the tangent
/// one undeclared still refuses `SeamTangent`.
#[test]
fn the_via_and_center_legs_check_the_token() {
    let via = |q: Point2<f64>, target| Step::ArcTo(ArcData::Via { q, target });
    let center = |c: Point2<f64>, target| {
        Step::ArcTo(ArcData::Center {
            c,
            winding: ArcSweep::Ccw,
            target,
        })
    };
    for (what, tangent, off) in [
        (
            "Via",
            via(pt(-2.5, 2.5), Target::StartArriving),
            via(pt(-2.5, 3.0), Target::StartArriving),
        ),
        (
            "Center",
            center(pt(0.0, 2.5), Target::StartArriving),
            center(pt(-0.2, 2.9), Target::StartArriving),
        ),
    ] {
        let lp = closes(&with(up_the_side(), &[tangent]), what);
        assert_eq!(lp.constructed_joints(), &[0], "{what}: the declared seam");
        refuses(
            &with(up_the_side(), &[off]),
            4,
            PathErrorKind::SeamArrivalOffDirection,
            &format!("{what}: an off-tangent arrival with the token"),
        );
    }
    refuses(
        &with(up_the_side(), &[via(pt(-2.5, 2.5), Target::Start)]),
        4,
        PathErrorKind::SeamTangent,
        "Via: the tangent arrival undeclared",
    );
    refuses(
        &with(up_the_side(), &[center(pt(0.0, 2.5), Target::Start)]),
        4,
        PathErrorKind::SeamTangent,
        "Center: the tangent arrival undeclared",
    );
}

/// Up-left off a `+x` side into a fillet whose arc arrival is the
/// circle about `(0, 2)` tangent to `+x` at the entry.
fn into_the_arrival(spec: ArcData<f64>) -> Vec<Step<f64>> {
    vec![
        Step::At(pt(0.0, 0.0)),
        toward(1.0, 0.0),
        Step::Line(2.0),
        toward(-1.0, 1.0),
        Step::FilletArc { radius: 0.2, spec },
    ]
}

/// The `Center` ARRIVAL fixes its end tangent too: the token checks.
#[test]
fn the_center_arrival_checks_the_token() {
    let center = |c: Point2<f64>, target| ArcData::Center {
        c,
        winding: ArcSweep::Ccw,
        target,
    };
    let lp = closes(
        &into_the_arrival(center(pt(0.0, 2.0), Target::StartArriving)),
        "the declared Center arrival",
    );
    assert_eq!(
        lp.constructed_joints(),
        &[0, 2, 3],
        "the declared seam, and the fillet arc's two ends"
    );
    refuses(
        &into_the_arrival(center(pt(0.3, 2.0), Target::StartArriving)),
        4,
        PathErrorKind::SeamArrivalOffDirection,
        "an off-tangent Center arrival with the token",
    );
    refuses(
        &into_the_arrival(center(pt(0.0, 2.0), Target::Start)),
        4,
        PathErrorKind::SeamTangent,
        "the tangent Center arrival undeclared",
    );
}

/// The `Via` ARRIVAL leaves its end tangent to the director, so the
/// token SUPPLIES it: the carrier is the circle through `q` tangent to
/// `Start`'s direction, and the close completes at the verb. Its twins:
/// the same tangent supplied by a director with no token is an
/// undeclared tangent seam, and a `q` on `Start`'s tangent line names
/// no circle.
#[test]
fn the_via_arrival_takes_its_direction_from_the_token() {
    let via = |q: Point2<f64>, target| ArcData::Via { q, target };
    let lp = closes(
        &into_the_arrival(via(pt(0.0, 4.0), Target::StartArriving)),
        "the token-supplied Via arrival",
    );
    assert_eq!(
        lp.constructed_joints(),
        &[0, 2, 3],
        "the seam's joint is constructed, beside the fillet arc's two ends"
    );
    let last = arc_of(&lp, lp.segments().len() - 1);
    assert!(
        (last.centre - pt(0.0, 2.0)).norm() < 1e-12 && (last.radius - 2.0).abs() < 1e-12,
        "the closing run rides the circle through q tangent to +x at the entry: {last:?}"
    );
    refuses(
        &with(
            into_the_arrival(via(pt(0.0, 4.0), Target::Start)),
            &[toward(1.0, 0.0)],
        ),
        5,
        PathErrorKind::SeamTangent,
        "the same tangent from a director, undeclared",
    );
    refuses(
        &into_the_arrival(via(pt(3.0, 0.0), Target::StartArriving)),
        4,
        PathErrorKind::ArcViaCollinear,
        "a through-point on Start's tangent line",
    );
}

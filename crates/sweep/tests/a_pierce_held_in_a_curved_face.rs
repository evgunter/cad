//! **A pierce point held inside a curved face of the piercing operand,
//! or a plane face holding one over a curved pierced face.**
//!
//! An operand whose solids touch along a line, one's edge lying in the
//! other's face, pierces a face of the other operand where that face
//! crosses its own. The section of the two passes the point, and the
//! face holding it takes a ring vertex there
//! (`topo::boolean::vtxfac::faces_holding_pierce`). The rows:
//!
//! - **A cylinder wall holds the point**: an upright rod with a square
//!   prism whose edge lies along one of the rod's rulings, joined, and a
//!   plate whose top that edge pierces inside the wall. Three poses of
//!   the ruling, both join orders, every op with the plate either way
//!   round, and every member order of the fold, carried.
//! - **A plane face holds it over a curved pierced face**: two prisms,
//!   one's edge lying in the other's side face, joined, and a lying
//!   cylinder whose top that edge pierces.
//! - **A ringed wall holds it**: the joined rod and square pocketed
//!   beside the point, so the wall carries the pocket's ring, which the
//!   wall's region reads with its outer loop
//!   (`topo::boolean::ruled_region`). The plate against it builds,
//!   either operand first, and so does pocketing after the join.
//! - **A wall the ruling pierces twice**: the joined rod and square
//!   against a slab whose two faces the ruling pierces inside the wall.
//!   Both points are read before either is minted, so every op and
//!   every member order builds.
//!
//! A curved face holding the point over a curved pierced face refuses
//! `HeldPierceUnbuilt` typed: no row reaches it, since the rod alone
//! already refuses against the lying cylinder (`GermFrameUnsupported`)
//! and against a coaxial ball (`CurvedBooleanUnsupported`) or cone
//! (`GermFrameUnsupported`).
//!
//! Each body that builds is sound: tier 3, tier 3′ on the carried fold,
//! its material at probes about the point against the members' closed
//! forms, and its volume against theirs (the lying cylinder's and the
//! pocket's parts by quadrature, far below the tolerance). Each op also
//! passes tier 3′ on its operands' carried records, but for what the
//! census says, typed, it cannot yet decide of two lumps' curved faces,
//! and the line contact an intersect or subtract of the lying-cylinder
//! row leaves unrecorded (`a-carried-line-contact-an-op-cuts-is-dropped`);
//! and its tessellation passes `check_mesh` where one is made, but for
//! the slab less the twice-pierced pair, which meets a known panic
//! (`a-rod-less-an-inscribed-square-prism-builds-and-the-tessellator-panics`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol};
use sweep::test_support::{brick, corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult};

/// The rod's radius and axis.
const R: f64 = 0.4;
const C: (f64, f64) = (1.5, 1.0);
/// The rod's height range.
const ROD_Z: (f64, f64) = (0.5, 2.0);
/// The square's height range, staggered off the rod's.
const SQ_Z: (f64, f64) = (0.44, 1.81);
/// The square's area: its diagonals are both `0.6`.
const SQ_AREA: f64 = 0.18;
/// The lying cylinder's radius; its axis runs along `x` at `y = 1`,
/// `z = 1 − RC`, so its top passes `(·, 1, 1)`.
const RC: f64 = 2.0;
/// The lying cylinder's spans along `x`. Over `LYING_CAPPED` its end cap
/// crosses both prisms, so neither leaves a ring on its wall, and the
/// in-solid walk reads every probe of every body. Over `LYING_WHOLE` it
/// runs past both either way, and each leaves a ring: the walk leaves
/// the bodies' outside unread there (`VolumeUncertified`), and a fold
/// that rings the wall before the second prism comes reads that prism's
/// edge against the ringed wall's region.
const LYING_CAPPED: (f64, f64) = (1.47, 3.0);
const LYING_WHOLE: (f64, f64) = (0.5, 3.0);
/// The plate.
const PLATE: [(f64, f64); 3] = [(0.0, 3.0), (0.0, 2.0), (0.0, 1.0)];
/// The ruling's bearings off the rod's seams (at `0` and `π`).
const TURNS: [f64; 3] = [0.5, 2.0, -1.0];

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(
    plane: profile::SketchPlane<f64>,
    lp: profile::ProfileLoop<f64>,
    depth: f64,
) -> AtRestBody<f64> {
    let p = profile::Profile::new(plane, vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the cylinder", body, tol())
}

/// The upright rod, through the extrude door: two wall faces, their
/// seam rulings at `±x` of its axis.
fn rod() -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(C.0, C.1), R, tol()).unwrap();
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, ROD_Z.0));
    extruded(plane, lp.into(), ROD_Z.1 - ROD_Z.0)
}

/// The cylinder lying along `x` over `span`.
fn lying(span: (f64, f64)) -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(1.0, 1.0 - RC), RC, tol()).unwrap();
    let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_yz(Point3::new(
        span.0, 0.0, 0.0,
    )));
    extruded(plane, lp.into(), span.1 - span.0)
}

/// The square's corners, unturned and counterclockwise: its corner at
/// `(R, 0)` from the rod's axis, its other corners outside the rod.
const SQ: [(f64, f64); 4] = [(R, 0.0), (R + 0.3, -0.3), (R + 0.6, 0.0), (R + 0.3, 0.3)];

fn turned(turn: f64, (x, y): (f64, f64)) -> (f64, f64) {
    let (s, c) = turn.sin_cos();
    (C.0 + c * x - s * y, C.1 + s * x + c * y)
}

/// The square prism outside the rod whose edge is the rod's ruling at
/// bearing `turn`.
fn square_foot(turn: f64) -> Vec<(f64, f64)> {
    SQ.iter().map(|&p| turned(turn, p)).collect()
}

fn prism(foot: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "a prism",
        prism_at(corners(foot), z.0, z.1 - z.0, tol()),
        tol(),
    )
}

/// The point at bearing `a` (degrees) `r` from the rod's axis.
fn bearing(a: f64, r: f64) -> (f64, f64) {
    let (s, c) = a.to_radians().sin_cos();
    (C.0 + r * c, C.1 + r * s)
}

/// An upright prism: its footprint (counterclockwise) and its height
/// range.
type Upright = (Vec<(f64, f64)>, (f64, f64));

/// Two prisms touching along the rod's axis line: one over the sector
/// 120°–170°, the other over a triangle whose side holds the line.
fn line_prisms() -> [Upright; 2] {
    [
        (
            vec![C, bearing(120.0, 0.4), bearing(170.0, 0.4)],
            (0.47, 1.7),
        ),
        (
            vec![bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
            (0.44, 1.81),
        ),
    ]
}

// The closed forms.

/// Inside the convex polygon `foot` (counterclockwise), strictly.
fn in_polygon(foot: &[(f64, f64)], (x, y): (f64, f64)) -> bool {
    (0..foot.len()).all(|i| {
        let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
        (b.0 - a.0) * (y - a.1) - (b.1 - a.1) * (x - a.0) > 0.0
    })
}

fn area(foot: &[(f64, f64)]) -> f64 {
    (0..foot.len())
        .map(|i| {
            let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0
}

/// The convex polygon's width along `x` at height `y`.
fn width(foot: &[(f64, f64)], y: f64) -> f64 {
    let xs: Vec<f64> = (0..foot.len())
        .filter_map(|i| {
            let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
            let (lo, hi) = (a.1.min(b.1), a.1.max(b.1));
            (lo <= y && y <= hi && a.1 != b.1).then(|| a.0 + (b.0 - a.0) * (y - a.1) / (b.1 - a.1))
        })
        .collect();
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if xs.is_empty() { 0.0 } else { hi - lo }
}

/// Composite Simpson's rule over `[a, b]`.
fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let n = 4000;
    let h = (b - a) / f64::from(n);
    let sum: f64 = (0..=n)
        .map(|i| {
            let w = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            w * f(a + h * f64::from(i))
        })
        .sum();
    sum * h / 3.0
}

/// The lying cylinder's top over `y`.
fn roof(y: f64) -> f64 {
    1.0 - RC + (RC * RC - (y - 1.0).powi(2)).sqrt()
}

/// The part of the convex polygon `foot` (counterclockwise) at `x ≥ x0`.
fn clip_at(foot: &[(f64, f64)], x0: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..foot.len() {
        let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
        if a.0 >= x0 {
            out.push(a);
        }
        if (a.0 < x0) != (b.0 < x0) {
            let t = (x0 - a.0) / (b.0 - a.0);
            out.push((x0, a.1 + t * (b.1 - a.1)));
        }
    }
    out
}

/// The volume of the upright prism over `foot` from `z0` that lies in
/// the lying cylinder from `x0`, whose top stays between `z0` and the
/// prism's top over `foot`: the integral of the width of `foot` past
/// `x0` times the roof's height over `z0`, piecewise between the clipped
/// corners' heights.
fn under_roof(foot: &[(f64, f64)], z0: f64, x0: f64) -> f64 {
    let foot = clip_at(foot, x0);
    let mut ys: Vec<f64> = foot.iter().map(|p| p.1).collect();
    ys.sort_by(f64::total_cmp);
    ys.windows(2)
        .map(|w| simpson(&|y| width(&foot, y) * (roof(y) - z0), w[0], w[1]))
        .sum()
}

/// The area of the box `x × y` within the rod's disc, which reaches
/// neither of the disc's `x` extremes inside it: the integral over `x`
/// of the box's span of the disc's chord, piecewise between the `x`
/// where the circle crosses the box's sides.
fn box_in_disc(x: (f64, f64), y: (f64, f64)) -> f64 {
    let half = |u: f64| (R * R - (u - C.0).powi(2)).max(0.0).sqrt();
    let mut xs = vec![x.0, x.1];
    for edge in [y.0, y.1] {
        let d = (R * R - (edge - C.1).powi(2)).max(0.0).sqrt();
        xs.extend(
            [C.0 - d, C.0 + d]
                .into_iter()
                .filter(|&u| x.0 < u && u < x.1),
        );
    }
    xs.sort_by(f64::total_cmp);
    let chord = |u: f64| {
        let (lo, hi) = (C.1 - half(u), C.1 + half(u));
        (hi.min(y.1) - lo.max(y.0)).max(0.0)
    };
    xs.windows(2).map(|w| simpson(&chord, w[0], w[1])).sum()
}

fn in_rod(q: [f64; 3]) -> bool {
    (q[0] - C.0).powi(2) + (q[1] - C.1).powi(2) < R * R && ROD_Z.0 < q[2] && q[2] < ROD_Z.1
}

fn in_lying(span: (f64, f64), q: [f64; 3]) -> bool {
    span.0 < q[0] && q[0] < span.1 && (q[1] - 1.0).powi(2) + (q[2] - 1.0 + RC).powi(2) < RC * RC
}

fn in_plate(q: [f64; 3]) -> bool {
    (0..3).all(|i| PLATE[i].0 < q[i] && q[i] < PLATE[i].1)
}

fn in_prism(foot: &[(f64, f64)], z: (f64, f64), q: [f64; 3]) -> bool {
    z.0 < q[2] && q[2] < z.1 && in_polygon(foot, (q[0], q[1]))
}

/// Probes about the rod: a 9 × 9 × 14 grid within 0.95 of its axis
/// across, from `z = 0.06` to `1.98`, offset off the fixtures' planes
/// and rulings.
fn probes() -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..14 {
                let across = |n: i32, d: f64| 0.235f64.mul_add(f64::from(n) - 4.0, d);
                out.push([
                    C.0 + across(i, 0.0137),
                    C.1 + across(j, 0.0071),
                    0.148f64.mul_add(f64::from(k), 0.0593),
                ]);
            }
        }
    }
    out
}

fn inside_of(body: &AtRestBody<f64>, q: [f64; 3]) -> Option<bool> {
    let band = geom_core::Band::linear(tol()).unwrap();
    match topo::point_in_solid(body, Point3::new(q[0], q[1], q[2]), band, tol()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        Ok(topo::SolidContainment::OnBoundary)
        | Err(
            topo::PointInSolidError::Escalated { .. }
            | topo::PointInSolidError::VolumeUncertified
            | topo::PointInSolidError::Loop(topo::PointInLoopError::Escalated { .. }),
        ) => None,
        Err(e) => panic!("point in solid at {q:?}: {e:?}"),
    }
}

/// **A body sound against a closed form**: tier 3, its volume within
/// its certified bracket, and its material at the probes against
/// `holds`, at least `read_pct`% of them read.
fn sound(
    what: &str,
    body: &AtRestBody<f64>,
    holds: &dyn Fn([f64; 3]) -> bool,
    volume: f64,
    read_pct: usize,
) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    // Within the certified bracket, which a face read by quadrature
    // widens, and the closed form's own quadrature.
    let m =
        topo::mass_properties(body, tol(), topo::test_support::pinned_resolution(tol())).unwrap();
    assert!(
        (m.enclosure.volume - volume).abs() < m.enclosure.volume_pad + 1e-9,
        "{what}: volume {} ± {}, closed form {volume}",
        m.enclosure.volume,
        m.enclosure.volume_pad
    );
    let probes = probes();
    let mut read = 0;
    for &q in &probes {
        if let Some(got) = inside_of(body, q) {
            assert_eq!(got, holds(q), "{what}: material at {q:?}");
            read += 1;
        }
    }
    assert!(
        read * 100 >= read_pct * probes.len(),
        "{what}: {read} of {} probes read",
        probes.len()
    );
}

fn joined(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    joined_carried(what, a, b).0
}

/// [`joined`], with the contact records the union carries for a later
/// op to read its touches by.
fn joined_carried(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> (AtRestBody<f64>, topo::OperandRecords) {
    match topo::union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => (r.body.clone(), r.contacts.carried()),
        other => panic!("{what}: {other:?}"),
    }
}

/// The material an op makes of its operands' (A's, then B's).
type Material = fn(bool, bool) -> bool;

const OPS: [(&str, topo::BooleanOp, Material); 3] = [
    ("union", topo::BooleanOp::Union, |a, b| a || b),
    ("intersection", topo::BooleanOp::Intersect, |a, b| a && b),
    ("subtraction", topo::BooleanOp::Subtract, |a, b| a && !b),
];

/// An operand in closed form: its body, the contact records it carries,
/// its material and its volume.
struct Closed<'a> {
    body: &'a AtRestBody<f64>,
    records: topo::OperandRecords,
    holds: &'a dyn Fn([f64; 3]) -> bool,
    volume: f64,
}

/// **Every op of `held` and `other`, either operand first**, each
/// operand carrying its records: each builds a body [`sound`] against
/// the two closed forms, `common` their common part's volume,
/// `read_pct`% of the probes read. Each passes tier 3′ on the op's
/// records, but for what the census cannot yet decide and the line
/// contact an op `drops_line` names leaves unrecorded. Each that
/// `meshes` names for its op and order tessellates to a mesh
/// `check_mesh` passes, or refuses typed; the others meet a known
/// tessellator panic.
#[allow(clippy::too_many_arguments)] // the row's closed forms and its two pinned run sets
fn every_op(
    what: &str,
    held: &Closed,
    other: &Closed,
    common: f64,
    read_pct: usize,
    meshes: &dyn Fn(&str, bool) -> bool,
    drops_line: &dyn Fn(&str, bool) -> bool,
) {
    for (name, op, material) in OPS {
        for held_first in [true, false] {
            let what = format!("{name} of {what}, the held operand first: {held_first}");
            let (x, y) = if held_first {
                (held, other)
            } else {
                (other, held)
            };
            let inputs = topo::BooleanInputs {
                records_a: x.records.clone(),
                records_b: y.records.clone(),
            };
            let r = match topo::boolean_op_with(
                op,
                x.body,
                y.body,
                &inputs,
                topo::SweepStrategy::Realized,
                tol(),
            ) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what}: {other:?}"),
            };
            let body = &r.body;
            // Tier 3′ reads every touch the op made, and the census may
            // say, typed, that it cannot yet decide whether two lumps'
            // curved faces touch. An op `drops_line` names leaves the
            // line contact unrecorded
            // (`a-carried-line-contact-an-op-cuts-is-dropped`), pinned here
            // so its fix shows.
            let findings = topo::validate_pseudomanifold(body, &r.contacts, tol())
                .err()
                .unwrap_or_default();
            let unrecorded = findings
                .iter()
                .filter(|e| {
                    matches!(
                        e,
                        topo::ValidationError::UnrecordedContact {
                            contact: topo::CensusContact::EdgeFaceOverlap { .. },
                            ..
                        }
                    )
                })
                .count();
            assert!(
                findings.iter().all(|e| {
                    matches!(e, topo::ValidationError::CensusUndecidable { .. })
                        || matches!(e, topo::ValidationError::UnrecordedContact { .. })
                }) && unrecorded == usize::from(drops_line(name, held_first)),
                "{what}: tier 3′: {findings:?}"
            );
            if meshes(name, held_first)
                && let Ok(mesh) = mesh::tessellate(body, 1e-4, tol())
            {
                assert_eq!(
                    mesh::validate::check_mesh(&mesh),
                    Ok(()),
                    "{what}: the mesh"
                );
            }
            let volume = match name {
                "union" => x.volume + y.volume - common,
                "intersection" => common,
                _ => x.volume - common,
            };
            sound(
                &what,
                body,
                &|q| material((x.holds)(q), (y.holds)(q)),
                volume,
                read_pct,
            );
        }
    }
}

/// **The fold of `members` by union in every member order, carried**:
/// each step's records serve the next, so tier 3′ reads every contact
/// the fold made. Each order builds a body [`sound`] against `holds` and
/// `volume`, `read_pct`% of the probes read, that passes tier 3′.
fn every_order(
    what: &str,
    members: &[AtRestBody<f64>],
    holds: &dyn Fn([f64; 3]) -> bool,
    volume: f64,
    read_pct: usize,
) {
    for order in topo::test_support::meeting::orders(members.len()) {
        let what = format!("{what}, member order {order:?}");
        let mut body = members[order[0]].clone();
        let mut carried = topo::OperandRecords::default();
        let mut last = None;
        for (k, &i) in order.iter().enumerate().skip(1) {
            let inputs = topo::BooleanInputs {
                records_a: carried,
                ..topo::BooleanInputs::default()
            };
            let r = match topo::boolean_op_with(
                topo::BooleanOp::Union,
                &body,
                &members[i],
                &inputs,
                topo::SweepStrategy::Realized,
                tol(),
            ) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what}: step {k}: {other:?}"),
            };
            carried = r.contacts.carried();
            body = r.body.clone();
            last = Some(r);
        }
        let r = last.expect("at least two members");
        sound(&what, &r.body, holds, volume, read_pct);
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
            Ok(()),
            "{what}: tier 3′"
        );
    }
}

/// **The cylinder wall holds the point**: the square's edge on the
/// rod's ruling pierces the plate's top inside the rod's wall. Every
/// op of the joined pair and the plate, the pair joined either way
/// round, either operand first; and every member order of the fold.
#[test]
fn a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order() {
    let plate = finished(
        "the plate",
        brick(PLATE[0], PLATE[1], PLATE[2], tol()),
        tol(),
    );
    let rod_volume = PI * R * R * (ROD_Z.1 - ROD_Z.0);
    let sq_volume = SQ_AREA * (SQ_Z.1 - SQ_Z.0);
    let common = PI * R * R * (PLATE[2].1 - ROD_Z.0) + SQ_AREA * (PLATE[2].1 - SQ_Z.0);
    for turn in TURNS {
        let foot = square_foot(turn);
        let (rod, sq) = (rod(), prism(&foot, SQ_Z));
        let in_pair = |q: [f64; 3]| in_rod(q) || in_prism(&foot, SQ_Z, q);
        for (order, a, b) in [("rod ∪ square", &rod, &sq), ("square ∪ rod", &sq, &rod)] {
            let (pair, records) = joined_carried(&format!("turn {turn}: {order}"), a, b);
            every_op(
                &format!("turn {turn}: ({order}) and the plate"),
                &Closed {
                    body: &pair,
                    records,
                    holds: &in_pair,
                    volume: rod_volume + sq_volume,
                },
                &Closed {
                    body: &plate,
                    records: topo::OperandRecords::default(),
                    holds: &in_plate,
                    volume: 6.0,
                },
                common,
                99,
                &|_, _| true,
                &|_, _| false,
            );
        }
        every_order(
            &format!("turn {turn}: the rod, the square and the plate"),
            &[rod, sq, plate.clone()],
            &|q| in_pair(q) || in_plate(q),
            6.0 + rod_volume + sq_volume - common,
            99,
        );
    }
}

/// **A plane face holds the point over a curved pierced face**: two
/// prisms touching along the rod's axis line, one's side face holding
/// the other's edge, joined, and the lying cylinder, whose top that
/// edge pierces inside the side face. Over [`LYING_CAPPED`], every op and
/// every member order builds, every probe read. Over [`LYING_WHOLE`],
/// every op builds, its probes read where the walk reads them (the
/// union reads 1116 of 1134, the cylinder less the pair its inside
/// alone, 495); and every member order of the fold builds, the orders
/// that ring the wall before the second prism comes included.
#[test]
fn a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order() {
    let prisms = line_prisms();
    let bodies: Vec<_> = prisms.iter().map(|(foot, z)| prism(foot, *z)).collect();
    let in_prisms = |q: [f64; 3]| prisms.iter().any(|(foot, z)| in_prism(foot, *z, q));
    let prisms_volume: f64 = prisms
        .iter()
        .map(|(foot, z)| area(foot) * (z.1 - z.0))
        .sum();
    for (span, read_pct) in [(LYING_CAPPED, 99), (LYING_WHOLE, 43)] {
        let lying = lying(span);
        let in_cylinder = |q| in_lying(span, q);
        let common: f64 = prisms
            .iter()
            .map(|(foot, z)| under_roof(foot, z.0, span.0))
            .sum();
        let lying_volume = PI * RC * RC * (span.1 - span.0);
        for (order, a, b) in [
            ("sector ∪ triangle", &bodies[0], &bodies[1]),
            ("triangle ∪ sector", &bodies[1], &bodies[0]),
        ] {
            let (pair, records) = joined_carried(order, a, b);
            every_op(
                &format!("({order}) and the lying cylinder over {span:?}"),
                &Closed {
                    body: &pair,
                    records,
                    holds: &in_prisms,
                    volume: prisms_volume,
                },
                &Closed {
                    body: &lying,
                    records: topo::OperandRecords::default(),
                    holds: &in_cylinder,
                    volume: lying_volume,
                },
                common,
                read_pct,
                &|_, _| true,
                &|name, held_first| name == "intersection" || (name == "subtraction" && held_first),
            );
        }
        every_order(
            &format!("the two prisms and the lying cylinder over {span:?}"),
            &[bodies[0].clone(), bodies[1].clone(), lying],
            &|q| in_prisms(q) || in_cylinder(q),
            lying_volume + prisms_volume - common,
            read_pct,
        );
    }
}

/// **A ringed wall holding the point builds**: the joined rod and
/// square pocketed beside the ruling, at bearing 2 rad on the wall face
/// the ruling lies in, so that face carries the pocket's ring. The
/// wall's region reads the ring with the outer loop, so the point is
/// placed: the plate against the pocketed pair builds, either operand
/// first, and so does pocketing last, each sound against the closed
/// forms less the pocket's part of the rod.
#[test]
fn a_ringed_wall_holding_the_pierce_builds_either_operand_first_and_pocketed_last() {
    let plate = finished(
        "the plate",
        brick(PLATE[0], PLATE[1], PLATE[2], tol()),
        tol(),
    );
    let (px, py) = bearing(2.0f64.to_degrees(), R);
    let (pocket_x, pocket_y, pocket_z) =
        ((px - 0.06, px + 0.06), (py - 0.06, py + 0.06), (1.3, 1.6));
    let pocket = finished(
        "the pocket",
        brick(pocket_x, pocket_y, pocket_z, tol()),
        tol(),
    );
    let in_pocket = |q: [f64; 3]| {
        [pocket_x, pocket_y, pocket_z]
            .iter()
            .zip(q)
            .all(|(&(lo, hi), u)| lo < u && u < hi)
    };
    let cut = |body: &AtRestBody<f64>, what: &str| match topo::subtract(body, &pocket, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what} ∖ the pocket: {other:?}"),
    };
    let rod_volume = PI * R * R * (ROD_Z.1 - ROD_Z.0);
    let sq_volume = SQ_AREA * (SQ_Z.1 - SQ_Z.0);
    let common = PI * R * R * (PLATE[2].1 - ROD_Z.0) + SQ_AREA * (PLATE[2].1 - SQ_Z.0);
    // The pocket stands clear of the plate and the square: it takes the
    // rod alone.
    let pocketed_volume = box_in_disc(pocket_x, pocket_y) * (pocket_z.1 - pocket_z.0);
    for turn in [0.5, 1.0] {
        let foot = square_foot(turn);
        let sq = prism(&foot, SQ_Z);
        let pair = joined(&format!("turn {turn}: rod ∪ square"), &rod(), &sq);
        let pocketed = cut(&pair, &format!("turn {turn}: the pair"));
        let holds = |q| (in_rod(q) || in_prism(&foot, SQ_Z, q) || in_plate(q)) && !in_pocket(q);
        let volume = 6.0 + rod_volume + sq_volume - common - pocketed_volume;
        for (what, a, b) in [
            ("the pocketed pair ∪ the plate", &pocketed, &plate),
            ("the plate ∪ the pocketed pair", &plate, &pocketed),
        ] {
            let what = format!("turn {turn}: {what}");
            sound(&what, &joined(&what, a, b), &holds, volume, 99);
        }
        let whole = joined(&format!("turn {turn}: the pair ∪ the plate"), &pair, &plate);
        sound(
            &format!("turn {turn}: pocketed last"),
            &cut(&whole, &format!("turn {turn}: the pair and the plate")),
            &holds,
            volume,
            99,
        );
    }
}

/// **A wall the ruling pierces twice builds**: the joined rod and square
/// against a slab whose top and bottom the ruling pierces inside the
/// wall. Both points are read before either ring vertex is minted, so
/// the second does not read the first's strut as a ring. Every op of the
/// pair and the slab, the pair joined either way round, either operand
/// first, and every member order of the fold, at the three turns.
#[test]
fn a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order() {
    const SLAB_Z: (f64, f64) = (0.8, 1.2);
    let slab = finished("the slab", brick(PLATE[0], PLATE[1], SLAB_Z, tol()), tol());
    let in_slab = |q: [f64; 3]| {
        PLATE[0].0 < q[0]
            && q[0] < PLATE[0].1
            && PLATE[1].0 < q[1]
            && q[1] < PLATE[1].1
            && SLAB_Z.0 < q[2]
            && q[2] < SLAB_Z.1
    };
    let slab_volume = 6.0 * (SLAB_Z.1 - SLAB_Z.0);
    let rod_volume = PI * R * R * (ROD_Z.1 - ROD_Z.0);
    let sq_volume = SQ_AREA * (SQ_Z.1 - SQ_Z.0);
    // The slab's height lies within both the rod's and the square's.
    let common = (PI * R * R + SQ_AREA) * (SLAB_Z.1 - SLAB_Z.0);
    for turn in TURNS {
        let foot = square_foot(turn);
        let (rod, sq) = (rod(), prism(&foot, SQ_Z));
        let in_pair = |q: [f64; 3]| in_rod(q) || in_prism(&foot, SQ_Z, q);
        for (order, a, b) in [("rod ∪ square", &rod, &sq), ("square ∪ rod", &sq, &rod)] {
            let (pair, records) = joined_carried(&format!("turn {turn}: {order}"), a, b);
            every_op(
                &format!("turn {turn}: ({order}) and the slab"),
                &Closed {
                    body: &pair,
                    records,
                    holds: &in_pair,
                    volume: rod_volume + sq_volume,
                },
                &Closed {
                    body: &slab,
                    records: topo::OperandRecords::default(),
                    holds: &in_slab,
                    volume: slab_volume,
                },
                common,
                99,
                // The slab less the pair meets the tessellator panic
                // `a-rod-less-an-inscribed-square-prism-builds-and-the-tessellator-panics`
                // holds: a ruling segment the lumps share, both its ends
                // held pierces.
                &|name, held_first| !(name == "subtraction" && !held_first),
                &|_, _| false,
            );
        }
        every_order(
            &format!("turn {turn}: the rod, the square and the slab"),
            &[rod, sq, slab.clone()],
            &|q| in_pair(q) || in_slab(q),
            slab_volume + rod_volume + sq_volume - common,
            99,
        );
    }
}

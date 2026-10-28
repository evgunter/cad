//! Review probes (atrest-flush, KNuPYm), helpers copied from
//! `curved_copies_filling_one_volume.rs`.
//!
//! **Copies whose coincident faces are curved, or planar with an arc
//! rim, interfere** (`topo::census`'s aligned face-pair arm on curved
//! carriers, and its same-side reading of an arc-bounded disc): two
//! revolved cylinders or balls of one volume are one
//! [`ValidationError::SameSideInterference`], however every boundary
//! cell of the two coincides, and copies that only abut or stand off in
//! the band are none.
//!
//! Every body is turned off the world axes and moved off the origin
//! ([`tilted`]), so no reading rests on an axis-aligned carrier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ContactRecords, EntityId, ValidationError};

/// The rectangle `[r0, r1] × [y0, y1]` of the sketch, revolved about `y`
/// by `turn`: a cylinder (`r0 = 0`) or a tube, or a wedge of either.
fn rect(r0: f64, r1: f64, y0: f64, y1: f64, turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(r0, y0), 0.0),
            (Point2::new(r1, y0), 0.0),
            (Point2::new(r1, y1), 0.0),
            (Point2::new(r0, y1), 0.0),
        ],
        turn,
        Tol::witness(),
    )
}

/// The unit square over `[0, h]`, revolved about `y` by `turn`: a
/// cylinder of radius 1 and height `h`, its walls two half-bands on one
/// key and its caps two discs, or a wedge of it.
fn cylinder(h: f64, turn: Revolution<f64>) -> Body<f64> {
    rect(0.0, 1.0, 0.0, h, turn)
}

/// The unit ball, its poles on `y`.
fn ball() -> Body<f64> {
    lune(Revolution::Full)
}

/// The unit ball revolved by `turn` only: a lune, or the ball.
fn lune(turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        turn,
        Tol::witness(),
    )
}

/// The unit ball's half below `y = 0` (`upper` false) or above it.
fn hemisphere(upper: bool) -> Body<f64> {
    let bulge = (PI / 8.0).tan();
    let profile = if upper {
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), bulge),
            (Point2::new(0.0, 1.0), 0.0),
        ]
    } else {
        vec![
            (Point2::new(0.0, -1.0), bulge),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ]
    };
    revolved_about_y(profile, Revolution::Full, Tol::witness())
}

/// The unit ball's cap above `y = 1/2`: its sphere face is the
/// complement on the sphere of [`cut_ball`]'s, meeting it along the rim
/// circle only.
fn upper_cap() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.5), 0.0),
            (Point2::new(0.75_f64.sqrt(), 0.5), (PI / 12.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The ball of radius 2 with the unit ball hollowed out of it.
fn shell() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, -2.0), 1.0),
            (Point2::new(0.0, 2.0), 0.0),
            (Point2::new(0.0, 1.0), -1.0),
            (Point2::new(0.0, -1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// `body` reflected across the plane through the origin normal to
/// `normal`.
fn mirrored(body: &Body<f64>, normal: Vec3<f64>) -> Body<f64> {
    let r = topo::Reflection::across(Point3::new(0.0, 0.0, 0.0), normal, Tol::witness()).unwrap();
    topo::mirror_body(body, &r, Tol::witness()).unwrap()
}

/// A move by `v`.
fn moved(v: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::translation(v).unwrap()
}

/// The unit ball with its cap above `y = 1/2` cut off: a sphere face
/// that is a proper subset of the ball's, and a disc on the cut.
fn cut_ball() -> Body<f64> {
    let rim = Point2::new(0.75_f64.sqrt(), 0.5);
    revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), (core::f64::consts::PI / 6.0).tan()),
            (rim, 0.0),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, "a row's axis", Band::linear(Tol::witness()).unwrap()).unwrap()
}

/// The pose every row reads its bodies in: a turn about an axis off
/// every world axis, then a move off the origin.
fn tilted() -> RigidMotion<f64> {
    RigidMotion::rotation_then_translation(
        unit(Vec3::new(0.3, 0.2, 1.0)),
        0.7,
        Vec3::new(0.31, -0.17, 0.42),
    )
    .unwrap()
}

/// A turn by `angle` about the bodies' own axis, `y`.
fn spun(angle: f64) -> RigidMotion<f64> {
    RigidMotion::rotation(UnitVec3::exact_y(), angle).unwrap()
}

/// `body` moved by `inner`, then by [`tilted`].
fn placed(body: &Body<f64>, inner: RigidMotion<f64>) -> Body<f64> {
    let motion = tilted().compose(inner).unwrap();
    topo::transform_rigid(body, &motion, Tol::witness()).unwrap()
}

/// `a` and `b` as two solids of one body.
fn two(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut body = a.clone();
    topo::graft_disjoint_all_keyed(&mut body, b).unwrap();
    body
}

/// What the census finds over `body` at rest, each solid its own group,
/// and the flush pairs between them.
fn at_rest(body: &Body<f64>) -> (topo::RecordedAtRest, topo::FlushAtRest) {
    let groups: BTreeMap<_, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (recorded, flush)
}

/// The findings the census reads as interference between two solids.
fn interference<'a>(body: &Body<f64>, findings: &'a [ValidationError]) -> Vec<&'a ValidationError> {
    findings
        .iter()
        .filter(|f| topo::census_interference_solids(body, f).is_some())
        .collect()
}

/// The surface kind of face `f`'s carrier.
fn kind(body: &Body<f64>, f: EntityId) -> Option<geom::SurfaceKind> {
    let EntityId::Face(f) = f else { return None };
    Some(body.get_surface(body.get_face(f)?.surface)?.kind())
}

/// The undecidable face pairs on two carriers of `kind` each.
fn undecided_pairs(
    body: &Body<f64>,
    findings: &[ValidationError],
    of: geom::SurfaceKind,
) -> Vec<String> {
    findings
        .iter()
        .filter(|f| {
            matches!(f, ValidationError::CensusUndecidable { a, b, .. }
                if kind(body, *a) == Some(of) && kind(body, *b) == Some(of))
        })
        .map(|f| format!("{f:?}"))
        .collect()
}

/// The faces of each flush row.
fn flush_faces(flush: &topo::FlushAtRest) -> Vec<[topo::FaceKey; 2]> {
    flush
        .rows
        .iter()
        .map(|row| {
            row.cells.map(|cell| match cell {
                topo::RowCell::Result {
                    cell: topo::Cell::Face(f),
                } => f,
                other => panic!("a flush row names two faces: {other:?}"),
            })
        })
        .collect()
}

/// That `body`'s two solids read as one `SameSideInterference` and no
/// other interference, with no unrecorded touch left beside it (a
/// coincident vertex, edge or rim is a touch in the closure of the
/// pair's shared area, which the pair answers for), and that `flush`
/// face pairs lie flush, each sharing only boundary, none of them the
/// interfering pair.
fn assert_one_same_side(label: &str, body: &Body<f64>, flush_rows: usize) -> topo::RecordedAtRest {
    let (recorded, flush) = at_rest(body);
    let [ValidationError::SameSideInterference { a, b }] =
        interference(body, &recorded.findings)[..]
    else {
        panic!(
            "{label}: one same-side interference: {:?}",
            recorded.findings
        );
    };
    let flushed = flush_faces(&flush);
    assert_eq!(
        flushed.len(),
        flush_rows,
        "{label}: the flush pairs: {flushed:?}"
    );
    assert!(
        !flushed.iter().any(|p| p.contains(a) && p.contains(b)),
        "{label}: the overlapping pair is not flush: {flushed:?}"
    );
    assert!(
        !recorded
            .findings
            .iter()
            .any(|f| matches!(f, ValidationError::UnrecordedContact { .. })),
        "{label}: every touch is in the closure of the shared area: {:?}",
        recorded.findings
    );
    assert!(
        recorded.rows.is_empty() && flush.findings.is_empty(),
        "{label}: an overlapping pair records nothing: {:?} {flush:?}",
        recorded.rows
    );
    recorded
}

fn report(label: &str, body: &Body<f64>) -> (usize, Vec<[topo::FaceKey; 2]>, Vec<String>) {
    let (recorded, flush) = at_rest(body);
    let inter = interference(body, &recorded.findings).len();
    let loud: Vec<String> = recorded
        .findings
        .iter()
        .chain(&flush.findings)
        .map(|f| format!("{f:?}"))
        .collect();
    let rows = flush_faces(&flush);
    for [a, b] in &rows {
        let d = |f: topo::FaceKey| {
            let face = body.get_face(f).unwrap();
            let back = tilted().inverse().unwrap();
            let pts: Vec<String> = body
                .vertex_points()
                .filter(|(v, _)| body.faces_of_vertex(*v).is_some_and(|fs| fs.contains(&f)))
                .map(|(_, p)| {
                    let p = back.transform_point(p);
                    format!("({:.4},{:.6},{:.4})", p.x, p.y, p.z)
                })
                .collect();
            format!(
                "{f:?} solid {:?} {:?} sense {:?} verts {}",
                body.solid_of_face(f),
                body.get_surface(face.surface).map(|s| s.kind()),
                face.sense,
                pts.join(" ")
            )
        };
        eprintln!("    row: {} | {}", d(*a), d(*b));
    }
    eprintln!(
        "{label}: interference {inter}, flush rows {rows:?}, findings {}",
        loud.len()
    );
    (inter, rows, loud)
}

/// Overlapping copies: never zero interference with nothing loud, and no
/// flush row naming two faces that share interior (checked by hand from
/// the printout where flush rows appear).
#[test]
fn probe_overlaps_are_never_silent() {
    let half = Revolution::Partial(PI);
    let reflex = Revolution::Partial(1.5 * PI);
    let rest = |b: &Body<f64>, at| placed(b, spun(at));
    let wedge = cylinder(1.0, half);
    let flipped = moved(Vec3::new(0.0, 0.0, 0.0))
        .compose(RigidMotion::rotation(UnitVec3::exact_x(), PI).unwrap())
        .unwrap();
    let mut silent = Vec::new();
    for (label, body) in [
        (
            "half cylinder and its self-mirror",
            two(
                &rest(&wedge, 0.0),
                &rest(&mirrored(&wedge, Vec3::new(1.0, 0.0, 0.0)), 0.0),
            ),
        ),
        (
            "half lune and its self-mirror",
            two(
                &rest(&lune(half), 0.0),
                &rest(&mirrored(&lune(half), Vec3::new(1.0, 0.0, 0.0)), 0.0),
            ),
        ),
        (
            "ball and its mirror",
            two(
                &rest(&ball(), 0.0),
                &rest(&mirrored(&ball(), Vec3::new(1.0, 0.0, 0.0)), 0.0),
            ),
        ),
        (
            "hemisphere and its mirror",
            two(
                &rest(&hemisphere(true), 0.0),
                &rest(&mirrored(&hemisphere(true), Vec3::new(1.0, 0.0, 0.0)), 0.0),
            ),
        ),
        (
            "cut ball and its mirror",
            two(
                &rest(&cut_ball(), 0.0),
                &rest(&mirrored(&cut_ball(), Vec3::new(0.0, 0.0, 1.0)), 0.0),
            ),
        ),
        (
            "reflex wedges a half turn apart",
            two(
                &rest(&cylinder(1.0, reflex), 0.0),
                &rest(&cylinder(1.0, reflex), PI),
            ),
        ),
        (
            "reflex lunes a quarter turn apart",
            two(&rest(&lune(reflex), 0.0), &rest(&lune(reflex), FRAC_PI_2)),
        ),
        (
            "half-height half cylinder in a half",
            two(&rest(&wedge, 0.0), &rest(&cylinder(0.5, half), 0.0)),
        ),
        (
            "thin wedge inside a half",
            two(
                &rest(&wedge, 0.0),
                &rest(&cylinder(1.0, Revolution::Partial(0.01)), 1.0),
            ),
        ),
        (
            "cut balls flipped",
            two(&rest(&cut_ball(), 0.0), &placed(&cut_ball(), flipped)),
        ),
        (
            "upper caps spun",
            two(&rest(&upper_cap(), 0.0), &rest(&upper_cap(), 1.3)),
        ),
        (
            "upper cap in cut ball's mirror",
            two(
                &rest(&upper_cap(), 0.0),
                &rest(&mirrored(&cut_ball(), Vec3::new(0.0, 1.0, 0.0)), 0.0),
            ),
        ),
        (
            "tube and cylinder",
            two(
                &rest(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), 0.0),
                &rest(&cylinder(1.0, Revolution::Full), 0.4),
            ),
        ),
    ] {
        let (inter, _, loud) = report(label, &body);
        if inter == 0 && loud.is_empty() {
            silent.push(label);
        }
        if inter == 0 {
            eprintln!("  {label} findings: {loud:?}");
        }
    }
    assert!(silent.is_empty(), "overlaps read silent: {silent:?}");
}

/// Pins stacked along the axis, overlapping by `k·ε` (negative: a gap):
/// the axial shortcut and the faces' reading across the band, for whole
/// and half pins.
#[test]
fn probe_pins_overlapping_axially_across_the_band() {
    let eps = Tol::witness().eps();
    let mut bad = Vec::new();
    for (name, turn) in [
        ("full", Revolution::Full),
        ("half", Revolution::Partial(PI)),
    ] {
        let pin = cylinder(1.0, turn);
        for k in [-3.0, 0.0, 0.5, 0.9, 3.0, 9.0, 11.0, 20.0, 100.0, 1e4] {
            let up = topo::transform_rigid(
                &pin,
                &moved(Vec3::new(0.0, 1.0 - k * eps, 0.0)),
                Tol::witness(),
            )
            .unwrap();
            let body = two(&placed(&pin, spun(0.0)), &placed(&up, spun(0.0)));
            let label = format!("{name} pins overlapping {k} eps");
            let (inter, rows, loud) = report(&label, &body);
            if k > 10.5 && inter == 0 {
                bad.push(label.clone());
            }
            // Flush rows past the band are legitimate here (read by hand
            // from the printout): the cross-half walls share a seam
            // ruling run as long as the overlap, and the half pins'
            // start and end rectangles abut along the axis over it.
            let _ = rows;
            if k > 1.0 && k < 10.5 && inter == 0 && loud.is_empty() {
                bad.push(format!("{label}: silent in the gap"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

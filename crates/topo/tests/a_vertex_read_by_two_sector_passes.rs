//! **A vertex read by two sector passes builds where its pierce only
//! touches, and refuses typed where its pierce crosses or it pierces
//! twice, in every op and both operand orders.**
//!
//! A vertex-on-face pass hangs struts at its piercing vertex where the
//! vertex crosses the face, and only there. Where the other solid holds
//! its own contact at the piercing point, the vertex is read again: by
//! a pair, where a vertex of the other solid rests on the face, or by a
//! second pierce, where two of its faces meet there. A touching vertex
//! read again by a pair builds, its edges classed against the face and
//! the pair together; a crossing one, and a second pierce, refuse
//! `VertexReadTwice` before any pass writes at the vertex.
//!
//! The scenes, each at every pose, pyramids standing on their apexes
//! at `MEET` or hanging from it inside the plate:
//! - **the arches**: the plate united with three standing pyramids,
//!   which keeps their apexes on its top with no vertex of the top
//!   there, against a fourth; **one standing pyramid**, the same with
//!   one arch; the fourth **over** the arch, the two crossing;
//! - a pyramid **hanging** inside the plate below the arch;
//! - **the cavity**: the plate less a hanging pyramid, against a
//!   standing one, a hanging one crossing it, and one hanging inside
//!   its void through its floor;
//! - **the strut hangers**: a prism through the top (`meeting::wedge`,
//!   and `meeting::leaned`) beside one standing pyramid, whose minted
//!   vertex at `MEET` crosses the top before the pair reads it;
//! - **two blocks in face contact**, one body built through the Euler
//!   doors, against a standing pyramid and against the prism, whose
//!   vertex at `MEET` pierces both blocks' faces;
//! - **several partners**: two pyramids united at their apexes, against
//!   the arch, the arch above a void, and the arch alone; a pyramid
//!   inside an island in the void, and one inside a void in the arch;
//!   a pyramid over a void in a quadrilateral arch, with the plate and
//!   without;
//! - **two arches sharing a ray**, united with the plate, against a
//!   pyramid with an edge along that ray, which lies on both arches'
//!   boundaries (`vtxfac::along_partners`);
//! - **two blocks glued** at their corners' face, against a pyramid
//!   whose edge runs through the contact face, on both blocks' flush
//!   faces (`vtxfac::along_partners`);
//! - a pyramid with an edge lying **along** the arch's face;
//! - a pyramid **lying** on the plate, an edge on its top, which a
//!   touching vertex pairs with (`vtxfac::partner_side`), pyramids along
//!   that edge, and the lying pyramid's corner tipped off the top; and
//!   pyramids with a face on the top or on the lying pyramid's, which
//!   refuse `PartnerAlongFace`;
//! - a **dart** on the plate, whose apex is a reflex edge, and the plate
//!   less a dart or a near-flat quadrilateral, a void below the top
//!   whose apex is one too, each read as a polygon cone
//!   (`sectors::cone_read`);
//! - the plate alone, and the arches without it, read once.
//!
//! A kernel op called without `BooleanInputs` records carries none of
//! its operands' records, so the operands' own contacts do not reach a
//! result here, and tier 3′ is asserted where the result holds no such contact and
//! otherwise refuses only those, at `MEET`, or a touch there the
//! census misreads (`three_prime`). Each result's material near `MEET`
//! is the op's over the operands' (`material_holds`), and each class
//! the naming reads there is the other operand's containment
//! (`classes_hold`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, PLATE, Pose, along_lying_ray, apex_pyramid, at, bearing, corners, leaned, lying, mix,
    near_flat, nest, nest_polygon, on_the_lying_faces, orders, posed_box, posed_boxes, posed_prism,
    poses, wedge,
};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::VolumeOracle;
use topo::{
    AlongFace, AtRestBody, BooleanError, BooleanErrorKind, BooleanOp, BooleanResult, CensusContact,
    EntityId, Operand, SectorRead, SideCode, SolidContainment, ValidationError, intersect,
    mass_properties, point_in_solid, point_in_solid_of, readback, subtract, union,
    validate_geometric, validate_pseudomanifold,
};

const THIRD: f64 = 1.0 / 3.0;

/// The ops in which the flush pyramid's pierce crosses the top, which
/// refuses before its pair is read.
const FLUSH_CROSSES: [&str; 2] = ["x ∪ y", "y ∪ x"];

fn t() -> Tol {
    Tol::witness()
}

/// The pyramid with its apex at [`MEET`] over `base`, relative to it.
fn tet(base: [[f64; 3]; 3], pose: &Pose) -> AtRestBody<f64> {
    apex_pyramid(&base, pose, t())
}

fn standing(bearing: f64, rise: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    tet(corners(bearing, rise, r), pose)
}

fn hanging(bearing: f64, drop: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    tet(corners(bearing, -drop, r), pose)
}

/// The cavity's void: a pyramid hanging from [`MEET`] inside the plate.
fn void() -> [[f64; 3]; 3] {
    corners(120.0, -0.5, 0.4)
}

/// A dart's base: a quadrilateral above [`MEET`] with a dent.
fn dart() -> [[f64; 3]; 4] {
    let at = |deg: f64, r: f64| {
        let (s, k) = deg.to_radians().sin_cos();
        [r * k, r * s, 0.5]
    };
    [at(30.0, 0.45), at(60.0, 0.6), at(90.0, 0.45), at(60.0, 0.5)]
}

/// A dart's base below [`MEET`], its dent towards the top's bearing 120°.
fn dart_below() -> [[f64; 3]; 4] {
    [
        bearing(100.0, 0.45, -0.5),
        bearing(120.0, 0.6, -0.5),
        bearing(140.0, 0.45, -0.5),
        bearing(120.0, 0.5, -0.5),
    ]
}

/// The far end of the lying pyramid's ray along the top.
fn lying_ray() -> Option<[f64; 3]> {
    Some(lying(0.0)[2])
}

/// `base` mirrored through the top.
fn below_the_top(base: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    base.map(|[x, y, z]| [x, y, -z])
}

/// The arch: a pyramid standing on [`MEET`].
fn arch() -> [[f64; 3]; 3] {
    corners(60.0, 0.5, 0.4)
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    mass_properties(b, t(), topo::test_support::pinned_resolution(t()))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
}

/// The census's `TouchUnanalysed` class: two solids of the result touch
/// along a line where one holds a slit or zero-area ring (the plate less
/// a void lying under the top, or a groove the pyramid under the ray cuts
/// below it) and the other an edge, which the touch analysis does not
/// read (`work/cleave/a-boolean-places-two-germs-along-a-slit-line.md`).
const SLIT_TOUCH: &str = "they touch where a face of one meets it along an edge and again";

/// Every refusal of tier 3′ is at [`MEET`]: a contact there no record
/// backs, an operand's own, which a kernel op called without
/// `BooleanInputs` records does not carry, or a touch there between two
/// solids where one passes into the other (a pyramid notched by
/// another, `TouchInterference`), or that the touch analysis does not
/// read along a slit's line ([`SLIT_TOUCH`]). [`material_holds`] reads
/// those results right. Where `along`, a pyramid's edge lies in a face
/// of the other's, and that edge's own contact, which no record
/// carries, may refuse too
/// (`work/flush/a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-unrecorded-contacts.md`).
/// Where `ray` names the far end of the lying pyramid's ray along the
/// top (relative to [`MEET`]), that ray's contacts may refuse too: its
/// corner on the top and the ray on it, the operand's own, likewise not
/// carried. The result's record carries an edge of the other operand's
/// along the ray. Where `glued` names the corners, in order round it,
/// of a rectangle in which one operand's own pieces touch, glued face
/// to face, their own contacts there may refuse too: coincident
/// corners, and edges overlapping along a side. Those are the operand's
/// records, which an op called without them does not carry
/// (`work/contacthold/a-booleans-records-are-complete-only-relative-to-the-records-passed-in.md`);
/// an edge of the other operand through the rectangle's interior is not
/// admitted. Returns whether 3′ held.
fn three_prime(
    what: &str,
    r: &topo::BooleanBody<f64>,
    pose: &Pose,
    along: bool,
    ray: Option<[f64; 3]>,
    glued: Option<[[f64; 3]; 4]>,
) -> bool {
    let Err(errors) = validate_pseudomanifold(&r.body, &r.contacts, t()) else {
        return true;
    };
    let meet = at(pose.at(MEET));
    let at_meet = |v| at(readback::vertex_point(&r.body, v).unwrap()) == meet;
    let far = ray.map(|c| pose.at([0, 1, 2].map(|k| MEET[k] + c[k])));
    let at_far = |v| Some(at(readback::vertex_point(&r.body, v).unwrap())) == far.map(at);
    // On the segment from `MEET` to the ray's far end, to a micron.
    let on_ray = |e| {
        let Some(c) = far else { return false };
        let o = pose.at(MEET);
        let edge = r.body.get_edge(e).unwrap();
        let plus = r.body.get_half_edge(edge.he_plus).unwrap();
        let next = r.body.get_half_edge(plus.next).unwrap();
        [plus.start, next.start].iter().all(|&v| {
            let q = readback::vertex_point(&r.body, v).unwrap();
            let (d, l) = (c - o, q - o);
            let s = l.dot(d) / d.dot(d);
            (-1e-9..=1.0 + 1e-9).contains(&s) && (l - d * s).norm() < 1e-6
        })
    };
    let band = Band::linear(t()).unwrap();
    let touches_meet = |e: &EntityId| match *e {
        EntityId::Solid(s) => matches!(
            point_in_solid_of(&r.body, s, pose.at(MEET), band, t()),
            Ok(SolidContainment::OnBoundary)
        ),
        _ => false,
    };
    // The glued rectangle's sides, posed: segment `k` runs from corner
    // `k` to the next.
    let sides: Vec<[Point3<f64>; 2]> = glued.map_or_else(Vec::new, |g| {
        (0..4)
            .map(|k| [g[k], g[(k + 1) % 4]].map(|c| pose.at(c)))
            .collect()
    });
    let point = |v| readback::vertex_point(&r.body, v).unwrap();
    // On side `k`, between its corners, to a micron.
    let on_side = |q: Point3<f64>, k: usize| {
        let [o, c] = sides[k];
        let (d, l) = (c - o, q - o);
        let s = l.dot(d) / d.dot(d);
        (-1e-9..=1.0 + 1e-9).contains(&s) && (l - d * s).norm() < 1e-6
    };
    let corner = |v| sides.iter().any(|&[o, _]| at(point(v)) == at(o));
    let along_a_side = |e| {
        let edge = r.body.get_edge(e).unwrap();
        let plus = r.body.get_half_edge(edge.he_plus).unwrap();
        let next = r.body.get_half_edge(plus.next).unwrap();
        let ends = [plus.start, next.start].map(point);
        (0..sides.len()).any(|k| ends.iter().all(|&q| on_side(q, k)))
    };
    // Whether `f`'s carrier is the glued rectangle's plane.
    let in_glued_plane = |f: topo::FaceKey| {
        let Some(&[o, c]) = sides.first() else {
            return false;
        };
        let Some(&[_, d]) = sides.get(1) else {
            return false;
        };
        let n = (c - o).cross(d - c);
        let n = n / n.norm();
        let surface = r
            .body
            .get_face(f)
            .and_then(|f| r.body.get_surface(f.surface));
        matches!(surface, Some(geom::Surface::Plane { origin, normal, .. })
            if (*origin - o).dot(n).abs() < 1e-6 && normal.cross(n).norm() < 1e-9)
    };
    for e in &errors {
        let ok = match e {
            ValidationError::UnrecordedContact { contact, .. } => match *contact {
                CensusContact::VertexVertex { a, b } => {
                    (at_meet(a) && at_meet(b)) || (corner(a) && at(point(a)) == at(point(b)))
                }
                CensusContact::EdgeEdgeOverlap { a, b } if along_a_side(a) && along_a_side(b) => {
                    true
                }
                CensusContact::VertexOnFace { vertex, .. } => at_meet(vertex) || at_far(vertex),
                // The glued pieces' own area contact, one face pair.
                CensusContact::ConformalPatch { ref finding } => {
                    in_glued_plane(finding.pair.a) && in_glued_plane(finding.pair.b)
                }
                CensusContact::EdgeFaceOverlap { edge, .. } => along || on_ray(edge),
                _ => false,
            },
            ValidationError::CensusUndecidable {
                a, b, what: class, ..
            } => class.starts_with(SLIT_TOUCH) && touches_meet(a) && touches_meet(b),
            ValidationError::TouchInterference { a, b, .. } => [a, b].iter().all(|&&cell| {
                topo::census_cell_solid(&r.body, cell)
                    .is_some_and(|s| touches_meet(&EntityId::Solid(s)))
            }),
            _ => false,
        };
        assert!(
            ok,
            "{what}: tier 3′ refuses only what is filed, at MEET, got {e:?}"
        );
    }
    false
}

/// Every class the result's naming read for an edge at [`MEET`] is the
/// other operand's containment of a point along the edge.
fn classes_hold(
    what: &str,
    r: &topo::BooleanBody<f64>,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    pose: &Pose,
) {
    let meet = at(pose.at(MEET));
    let band = Band::linear(t()).unwrap();
    for row in &r.naming.edge_classes {
        let (own, other) = match row.operand {
            Operand::A => (a, b),
            Operand::B => (b, a),
        };
        // A vertex the sweep minted is no key of the operand.
        let Ok(p) = readback::vertex_point(own, row.vertex) else {
            continue;
        };
        if at(p) != meet {
            continue;
        }
        let edge = own.get_edge(row.edge).unwrap();
        let plus = own.get_half_edge(edge.he_plus).unwrap();
        let far = if row.starts {
            own.get_half_edge(plus.next).unwrap().start
        } else {
            plus.start
        };
        let d = readback::vertex_point(own, far).unwrap() - p;
        let q = p + d * (0.02 / d.norm());
        let want = match point_in_solid(other, q, band, t()).unwrap() {
            SolidContainment::In => SideCode::In,
            SolidContainment::Out => SideCode::Out,
            SolidContainment::OnBoundary => SideCode::On,
        };
        assert_eq!(row.class, want, "{what}: {row:?} against the other operand");
    }
}

/// Points a tenth from [`MEET`]: towards 32 directions spread over the
/// sphere, and towards the mean of each vertex near it of `x` or `y`
/// and that vertex's two nearest, which lands inside each pyramid
/// (its base corners lie closer to each other than to another's).
fn samples(x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> Vec<Point3<f64>> {
    let meet = pose.at(MEET);
    let mut dirs: Vec<Vec3<f64>> = (0..32)
        .map(|k| {
            let z = 1.0 - (2.0 * f64::from(k) + 1.0) / 32.0;
            let (s, c) = (geom_core::GOLDEN_ANGLE * f64::from(k)).sin_cos();
            let r = z.mul_add(-z, 1.0).sqrt();
            Vec3::new(r * c, r * s, z)
        })
        .collect();
    for body in [x, y] {
        let near: Vec<Vec3<f64>> = body
            .vertices()
            .map(|(k, _)| readback::vertex_point(body, k).unwrap() - meet)
            .filter(|d| d.norm() > 1e-6 && d.norm() < 1.0)
            .collect();
        for (i, &d) in near.iter().enumerate() {
            let mut by: Vec<Vec3<f64>> = (0..near.len())
                .filter(|&j| j != i)
                .map(|j| near[j])
                .collect();
            by.sort_by(|e, f| (*e - d).norm().total_cmp(&(*f - d).norm()));
            if let [e, f, ..] = by[..] {
                dirs.push(d + e + f);
            }
        }
    }
    dirs.into_iter()
        .map(|d| meet + d * (0.1 / d.norm()))
        .collect()
}

fn inside(body: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    let band = Band::linear(t()).unwrap();
    match point_in_solid(body, q, band, t()).unwrap() {
        SolidContainment::In => Some(true),
        SolidContainment::Out => Some(false),
        SolidContainment::OnBoundary => None,
    }
}

/// The result's material at each probe `(q, in x, in y)` is `keep`'s
/// over the operands', wherever the result does not read `q` on its
/// boundary; an empty result holds none.
fn material_holds(
    what: &str,
    keep: impl Fn(bool, bool) -> bool,
    r: Option<&AtRestBody<f64>>,
    probes: &[(Point3<f64>, bool, bool)],
) {
    for &(q, x, y) in probes {
        let got = r.map_or(Some(false), |r| inside(r, q));
        if let Some(got) = got {
            assert_eq!(got, keep(x, y), "{what}: material at {q:?}");
        }
    }
}

/// Every op on `(x, y)`, in both orders, builds at tier 3, with tier 3′
/// as [`three_prime`] says, its naming's classes holding
/// ([`classes_hold`]), and volumes that add up: `|x ∪ y| + |x ∩ y| =
/// |x| + |y|` and `|x − y| + |x ∩ y| = |x|`, each alike in both orders.
/// Returns how many results held 3′.
fn builds(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> usize {
    builds_along(label, x, y, pose, false, None)
}

/// [`builds`], where `along` says an edge of one lies in a face of the
/// other, and `ray` names a ray an edge of each lies along
/// ([`three_prime`]).
fn builds_along(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    along: bool,
    ray: Option<[f64; 3]>,
) -> usize {
    builds_where(label, x, y, pose, along, ray, None)
}

/// The cells that refuse, by scene label and op, at every pose, and the
/// kind each refuses; every other cell builds.
///
/// - `LineWedges`: the result gate refuses coincident edges whose
///   wedges overlap, which every tier passed. They return with the
///   re-pairing
///   (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
/// - `EdgeInFace`: an edge lies in a face whose material parts it, and
///   the parting is not built. The pyramid out the bottom of the glued
///   blocks: `y − x` leaves an edge in the glued face, which its
///   material divides, and an end of it touches another edge
///   (`DividedEndTouched`).
const REFUSING: &[(&str, BooleanErrorKind)] = &[
    (
        "around the lying pyramid, x − y",
        BooleanErrorKind::LineWedges,
    ),
    (
        "inside the lying pyramid, y − x",
        BooleanErrorKind::LineWedges,
    ),
    (
        "on the lying pyramid's ray, beside it, mirrored into the void, y − x",
        BooleanErrorKind::LineWedges,
    ),
    (
        "a pyramid through the glued faces, out the bottom, y − x",
        BooleanErrorKind::EdgeInFace,
    ),
];

/// Whether `e` is the refusal [`REFUSING`] names for the cell.
fn pinned(label: &str, op: &str, e: &BooleanError) -> bool {
    let key = format!("{label}, {op}");
    REFUSING
        .iter()
        .any(|&(cell, kind)| cell == key && e.kind() == kind)
}

/// A cell [`REFUSING`] names that builds is a pin gone stale.
fn not_pinned(label: &str, op: &str) {
    let key = format!("{label}, {op}");
    assert!(
        !REFUSING.iter().any(|&(cell, _)| cell == key),
        "{key}: builds, where it is pinned to refuse"
    );
}

/// [`builds_along`], where `glued` names the faces two blocks of `y`
/// share ([`three_prime`]).
fn builds_where(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    along: bool,
    ray: Option<[f64; 3]>,
    glued: Option<[[f64; 3]; 4]>,
) -> usize {
    let mut held = 0;
    let mut v = [Some(0.0); 6];
    // Each op's keep over (in x, in y).
    let probes: Vec<_> = samples(x, y, pose)
        .into_iter()
        .filter_map(|q| Some((q, inside(x, q)?, inside(y, q)?)))
        .collect();
    let x_less: fn(bool, bool) -> bool = |x, y| x && !y;
    let y_less: fn(bool, bool) -> bool = |x, y| y && !x;
    let or: fn(bool, bool) -> bool = |x, y| x || y;
    let and: fn(bool, bool) -> bool = |x, y| x && y;
    let oracle = VolumeOracle::new(x, y, t());
    for (k, (what, keep, a, b, (kind, r))) in [
        (
            "x − y",
            x_less,
            x,
            y,
            (BooleanOp::Subtract, subtract(x, y, t())),
        ),
        (
            "y − x",
            y_less,
            y,
            x,
            (BooleanOp::Subtract, subtract(y, x, t())),
        ),
        ("x ∪ y", or, x, y, (BooleanOp::Union, union(x, y, t()))),
        ("y ∪ x", or, y, x, (BooleanOp::Union, union(y, x, t()))),
        (
            "x ∩ y",
            and,
            x,
            y,
            (BooleanOp::Intersect, intersect(x, y, t())),
        ),
        (
            "y ∩ x",
            and,
            y,
            x,
            (BooleanOp::Intersect, intersect(y, x, t())),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let op = what;
        let what = format!("{label}, {}, {what}", pose.label);
        v[k] = match r {
            Ok(BooleanResult::Body(r)) => {
                not_pinned(label, op);
                assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
                material_holds(&what, keep, Some(&r.body), &probes);
                classes_hold(&what, &r, a, b, pose);
                held += usize::from(three_prime(&what, &r, pose, along, ray, glued));
                oracle.assert(&what, kind, a, b, Some(&r.body));
                Some(volume(&r.body))
            }
            Ok(BooleanResult::Empty) => {
                not_pinned(label, op);
                material_holds(&what, keep, None, &probes);
                oracle.assert(&what, kind, a, b, None);
                Some(0.0)
            }
            Err(e) if pinned(label, op, &e) => None,
            Err(e) => panic!("{what}: builds, got {e:?}"),
        };
    }
    let [xy, yx, u, u2, i, i2] = v;
    let (vx, vy) = (volume(x), volume(y));
    let what = format!("{label}, {}", pose.label);
    let add = |a: Option<f64>, b: Option<f64>| Some(a? + b?);
    // Each sum, which every result enters, over the ops that built: a
    // cell that refuses ([`REFUSING`]) leaves out the sums
    // it enters, and the rest still hold.
    for (sum, want, which) in [
        (add(u, i), Some(vx + vy), "|x ∪ y| + |x ∩ y|"),
        (add(xy, i), Some(vx), "|x − y| + |x ∩ y|"),
        (add(yx, i), Some(vy), "|y − x| + |x ∩ y|"),
        (u2, u, "|y ∪ x|"),
        (i2, i, "|y ∩ x|"),
    ] {
        let (Some(sum), Some(want)) = (sum, want) else {
            continue;
        };
        assert!(
            (sum - want).abs() < 1e-9,
            "{what}: {which} {sum}, want {want}"
        );
    }
    held
}

/// Every op on `(x, y)`, in both orders, refuses on a vertex of `x`'s
/// operand that touches the top and pairs with a partner along it, its
/// apex at [`MEET`] or one the sweep mints where `x`'s edge crosses the
/// lying pyramid's ray, whichever pair is read first:
/// `PartnerAlongFace` as `how`, or, in the `crossing` ops, where that
/// vertex's pierce crosses the top, `VertexReadTwice` before the pair is
/// read.
fn refuses_beside_the_ray(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    how: AlongFace,
    crossing: &[&str],
) {
    let meet = at(pose.at(MEET));
    let at_meet_or_minted =
        |body: &AtRestBody<f64>, v| readback::vertex_point(body, v).map_or(true, |p| at(p) == meet);
    for (what, x_is, r) in [
        ("x − y", Operand::A, subtract(x, y, t())),
        ("y − x", Operand::B, subtract(y, x, t())),
        ("x ∪ y", Operand::A, union(x, y, t())),
        ("y ∪ x", Operand::B, union(y, x, t())),
        ("x ∩ y", Operand::A, intersect(x, y, t())),
        ("y ∩ x", Operand::B, intersect(y, x, t())),
    ] {
        let crosses = crossing.contains(&what);
        let what = format!("{label}, {}, {what}", pose.label);
        match r {
            Err(BooleanError::PartnerAlongFace {
                operand,
                vertex,
                partner,
                along,
                ..
            }) => {
                assert!(
                    !crosses,
                    "{what}: refuses at the crossing pierce, got {along:?}"
                );
                assert_eq!(operand, x_is, "{what}: the touching operand");
                assert!(
                    at_meet_or_minted(x, vertex) && at_meet_or_minted(y, partner),
                    "{what}: the pair is at MEET or minted"
                );
                assert_eq!(along, how, "{what}: how the partner meets the face");
            }
            Err(BooleanError::VertexReadTwice {
                operand,
                vertex,
                reads: [SectorRead::Pierce(_), SectorRead::Pair(partner)],
            }) => {
                assert!(crosses, "{what}: refuses {how:?}, got the crossing pierce");
                assert_eq!(operand, x_is, "{what}: the touching operand");
                assert!(
                    at_meet_or_minted(x, vertex) && at_meet_or_minted(y, partner),
                    "{what}: the pair is at MEET or minted"
                );
            }
            other => panic!(
                "{what}: refuses PartnerAlongFace, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}

/// Every op on `(x, y)`, in both orders, refuses `VertexReadTwice` on
/// `x`'s vertex at [`MEET`], with a pierce for its first read and
/// `second` for its next. Where `minted`, the sweep mints that vertex,
/// splitting an edge of `x` there, so `x` does not hold it.
fn refuses(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    second: &str,
    minted: bool,
) {
    let meet = at(pose.at(MEET));
    let at_meet = |body: &AtRestBody<f64>, v| at(readback::vertex_point(body, v).unwrap()) == meet;
    for (what, x_is, r) in [
        ("x − y", Operand::A, subtract(x, y, t())),
        ("y − x", Operand::B, subtract(y, x, t())),
        ("x ∪ y", Operand::A, union(x, y, t())),
        ("y ∪ x", Operand::B, union(y, x, t())),
        ("x ∩ y", Operand::A, intersect(x, y, t())),
        ("y ∩ x", Operand::B, intersect(y, x, t())),
    ] {
        let what = format!("{label}, {}, {what}", pose.label);
        match r {
            Err(BooleanError::VertexReadTwice {
                operand,
                vertex,
                reads: [SectorRead::Pierce(first), next],
            }) => {
                assert_eq!(operand, x_is, "{what}: the operand read twice");
                if minted {
                    assert!(
                        x.vertices().all(|(k, _)| k != vertex),
                        "{what}: the vertex read twice is minted"
                    );
                } else {
                    assert!(
                        at_meet(x, vertex),
                        "{what}: the vertex read twice is at MEET"
                    );
                }
                match (second, next) {
                    ("pair", SectorRead::Pair(partner)) => {
                        assert!(at_meet(y, partner), "{what}: its partner is at MEET");
                    }
                    ("pierce", SectorRead::Pierce(face)) => {
                        assert_ne!(first, face, "{what}: it pierces two faces");
                    }
                    _ => panic!("{what}: its second read is a {second}, got {next:?}"),
                }
            }
            other => panic!(
                "{what}: refuses VertexReadTwice with a pierce first, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}

/// The scenes at one pose.
struct Scene {
    plate: AtRestBody<f64>,
    cone: AtRestBody<f64>,
    over: AtRestBody<f64>,
    hang: AtRestBody<f64>,
    hang_over: AtRestBody<f64>,
    in_void: AtRestBody<f64>,
    bare: AtRestBody<f64>,
    arch: AtRestBody<f64>,
    arches: AtRestBody<f64>,
    one: AtRestBody<f64>,
    cavity: AtRestBody<f64>,
    both: AtRestBody<f64>,
    two_up: AtRestBody<f64>,
    two_down: AtRestBody<f64>,
    island: AtRestBody<f64>,
    in_island: AtRestBody<f64>,
    hollow: AtRestBody<f64>,
    in_hollow: AtRestBody<f64>,
    bare_hollow: AtRestBody<f64>,
    deep: AtRestBody<f64>,
    cross3: AtRestBody<f64>,
    on2: AtRestBody<f64>,
    cross_in: AtRestBody<f64>,
    along: AtRestBody<f64>,
    lying: AtRestBody<f64>,
    along_ray: Vec<(&'static str, AtRestBody<f64>)>,
    void_lying: AtRestBody<f64>,
    along_void_ray: Vec<(String, AtRestBody<f64>)>,
    flush: AtRestBody<f64>,
    on_lying: AtRestBody<f64>,
    dart: AtRestBody<f64>,
    dart_void: AtRestBody<f64>,
    flat_voids: [AtRestBody<f64>; 2],
    quad_hollow: AtRestBody<f64>,
    bare_quad_hollow: AtRestBody<f64>,
    blocks: AtRestBody<f64>,
    prism: AtRestBody<f64>,
    leaned: AtRestBody<f64>,
}

impl Scene {
    fn at(pose: &Pose) -> Self {
        let plate = posed_box("the plate", PLATE, pose, t());
        let [first, rest @ ..] = [60.0, 180.0, 300.0].map(|b| standing(b, 0.5, 0.4, pose));
        let bare = rest
            .iter()
            .fold(first, |u, a| built("the arches", union(&u, a, t())));
        let arch_body = tet(arch(), pose);
        let one = built("the plate and one arch", union(&plate, &arch_body, t()));
        let cavity = built(
            "the plate less a hanging pyramid",
            subtract(&plate, &tet(void(), pose), t()),
        );
        let hollow = built(
            "a void in the arch",
            subtract(&one, &tet(nest(arch(), 0.7), pose), t()),
        );
        // A quadrilateral arch and the void in it: the void's apex
        // reads through two hollow quadrilateral corners.
        let quad = [
            bearing(40.0, 0.45, 0.5),
            bearing(80.0, 0.45, 0.5),
            bearing(80.0, 0.25, 0.5),
            bearing(40.0, 0.25, 0.5),
        ];
        let quad_arch = apex_pyramid(&quad, pose, t());
        let quad_void = apex_pyramid(&nest_polygon(&quad, 0.7), pose, t());
        let pair = |what, [x, y]: [[[f64; 3]; 3]; 2]| {
            built(what, union(&tet(x, pose), &tet(y, pose), t()))
        };
        // A pyramid along the arch's outer face: one edge lies in it,
        // the other two outside the arch.
        let [p, q, _] = arch();
        let out = |c: [f64; 3]| {
            let (s, k) = 60f64.to_radians().sin_cos();
            [1.2 * c[0] + 0.2 * k, 1.2 * c[1] + 0.2 * s, 1.2 * c[2]]
        };
        let mid = [0, 1, 2].map(|k| 0.6 * (p[k] + q[k]));
        Self {
            cone: standing(240.0, 0.7, 0.5, pose),
            over: standing(50.0, 0.7, 0.5, pose),
            hang: hanging(240.0, 0.6, 0.5, pose),
            hang_over: hanging(130.0, 0.6, 0.5, pose),
            in_void: tet(nest(void(), 1.4), pose),
            arches: built("the plate and the arches", union(&plate, &bare, t())),
            both: built(
                "the arch less a hanging pyramid",
                subtract(&one, &tet(void(), pose), t()),
            ),
            two_up: pair(
                "two standing pyramids",
                [corners(40.0, 0.6, 0.5), corners(280.0, 0.6, 0.5)],
            ),
            two_down: pair(
                "two hanging pyramids",
                [corners(200.0, -0.6, 0.5), corners(110.0, -0.6, 0.5)],
            ),
            island: built(
                "an island in the void",
                union(&cavity, &tet(nest(void(), 0.7), pose), t()),
            ),
            in_island: tet(nest(nest(void(), 0.7), 0.7), pose),
            hollow: hollow.clone(),
            in_hollow: tet(nest(nest(arch(), 0.7), 0.7), pose),
            bare_hollow: built(
                "a void in the bare arch",
                subtract(&arch_body, &tet(nest(arch(), 0.7), pose), t()),
            ),
            deep: built(
                "an island in the void in the arch",
                union(&hollow, &tet(nest(nest(arch(), 0.7), 0.7), pose), t()),
            ),
            // Corners in the island, in the void only, and in the arch
            // only: its edges lie at three depths.
            cross3: tet(
                mix(
                    arch(),
                    [[THIRD, THIRD, THIRD], [0.75, 0.15, 0.1], [0.45, 0.22, 0.33]],
                    0.6,
                ),
                pose,
            ),
            // An edge on the void's face, inside the arch.
            on2: tet(
                mix(
                    arch(),
                    [[0.4, 0.4, 0.2], [0.45, 0.45, 0.1], [0.7, 0.25, 0.05]],
                    0.6,
                ),
                pose,
            ),
            // Corners in the island, in the void only, and in the plate
            // only, below the top.
            cross_in: tet(
                mix(
                    void(),
                    [[THIRD, THIRD, THIRD], [0.7, 0.2, 0.1], [1.2, -0.3, 0.1]],
                    0.6,
                ),
                pose,
            ),
            along: tet([mid, out(p), out(q)], pose),
            lying: built(
                "the plate and a lying pyramid",
                union(&plate, &tet(lying(0.0), pose), t()),
            ),
            along_ray: along_lying_ray()
                .into_iter()
                .map(|(label, base)| (label, tet(base, pose)))
                .collect(),
            // The lying scene mirrored through the top: a void below it
            // whose ray lies on it, and the same partners mirrored.
            void_lying: built(
                "the plate less a void lying under the top",
                subtract(&plate, &tet(below_the_top(lying(0.0)), pose), t()),
            ),
            along_void_ray: along_lying_ray()
                .into_iter()
                .map(|(label, base)| {
                    (
                        format!("{label}, mirrored into the void"),
                        tet(below_the_top(base), pose),
                    )
                })
                .collect(),
            flush: tet(on_the_lying_faces()[0], pose),
            on_lying: tet(on_the_lying_faces()[1], pose),
            // A pyramid over a quadrilateral with a dent: its apex is a
            // reflex edge, read as a polygon cone.
            dart: built(
                "the plate and a dart",
                union(&plate, &apex_pyramid(&dart(), pose, t()), t()),
            ),
            // A void below the top whose apex is a reflex edge, and two
            // near-flat quadrilateral voids, a corner a hair inside the
            // line of its neighbours.
            dart_void: built(
                "the plate less a dart",
                subtract(&plate, &apex_pyramid(&dart_below(), pose, t()), t()),
            ),
            // The second dent is ten zero bands, -1e-8 at the default
            // tolerance: as near flat as the run can tell from flat.
            flat_voids: [-1e-3, -10.0 * Band::linear(t()).unwrap().zero()].map(|dent| {
                built(
                    "the plate less a near-flat quadrilateral",
                    subtract(&plate, &apex_pyramid(&near_flat(dent), pose, t()), t()),
                )
            }),
            quad_hollow: built(
                "a void in a quadrilateral arch",
                subtract(
                    &built(
                        "the plate and a quadrilateral arch",
                        union(&plate, &quad_arch, t()),
                    ),
                    &quad_void,
                    t(),
                ),
            ),
            bare_quad_hollow: built(
                "a void in a bare quadrilateral arch",
                subtract(&quad_arch, &quad_void, t()),
            ),
            blocks: posed_boxes(
                "two blocks in face contact",
                &[PLATE, [(0.5, 2.5), (0.5, 1.5), (1.0, 1.5)]],
                pose,
                t(),
            ),
            prism: posed_prism(&wedge(200.0, 260.0, 0), pose, t()),
            leaned: posed_prism(&leaned(200.0, 260.0, 0, 300.0), pose, t()),
            arch: arch_body,
            bare,
            one,
            cavity,
            plate,
        }
    }
}

/// **A vertex that touches a face and pairs with a vertex resting on it
/// builds sound in every op**: a standing pyramid beside the arch, one
/// crossing it, one hanging inside the plate below it, and a standing
/// and a crossing hanging pyramid against the cavity, and one running
/// into its void; beside partners read as polygon cones: a dart on
/// the top, and a dart's or a near-flat quadrilateral's void below it;
/// and beside a pyramid lying on the top, standing, over it and hanging
/// below it. At rest; every pose is the slow matrix's.
#[test]
fn a_touching_vertex_paired_on_the_face_builds_sound_in_every_op() {
    let pose = &Pose::rest();
    let s = Scene::at(pose);
    builds("one standing pyramid", &s.cone, &s.one, pose);
    builds("a standing pyramid over the arch", &s.over, &s.one, pose);
    builds("a pyramid hanging below the arch", &s.hang, &s.one, pose);
    builds(
        "a standing pyramid over the cavity",
        &s.cone,
        &s.cavity,
        pose,
    );
    builds(
        "a hanging pyramid across the cavity",
        &s.hang_over,
        &s.cavity,
        pose,
    );
    builds(
        "a hanging pyramid into the void",
        &s.in_void,
        &s.cavity,
        pose,
    );
    // A void below the top is no partner of an edge above it: the edges
    // above class against the arch side alone.
    builds(
        "a standing pyramid over a dart void",
        &s.cone,
        &s.dart_void,
        pose,
    );
    builds("a pyramid over a dart void", &s.over, &s.dart_void, pose);
    // A dart's apex is a reflex edge, read as a polygon cone
    // (`sectors::cone_read`): beside it on the top, and below the top
    // beside a void whose apex is one, a touching vertex layers it.
    builds("a standing pyramid beside a dart", &s.cone, &s.dart, pose);
    builds("a pyramid over a dart", &s.over, &s.dart, pose);
    builds(
        "a pyramid hanging into a dart void",
        &s.hang,
        &s.dart_void,
        pose,
    );
    builds(
        "a pyramid hanging across a dart void",
        &s.hang_over,
        &s.dart_void,
        pose,
    );
    for (flat, dent) in s.flat_voids.iter().zip(["1e-3", "ten zero bands"]) {
        let what = format!("a standing pyramid over a quadrilateral void {dent} from flat");
        builds(&what, &s.cone, flat, pose);
        let what = format!("a pyramid hanging across a quadrilateral void {dent} from flat");
        builds(&what, &s.hang_over, flat, pose);
    }
    // A partner whose link runs along the top, a ray on it
    // (`vtxfac::partner_side`).
    for (label, x) in [
        ("standing beside a lying pyramid", &s.cone),
        ("over a lying pyramid", &s.over),
        ("hanging below a lying pyramid", &s.hang),
    ] {
        builds_along(label, x, &s.lying, pose, false, lying_ray());
    }
    // The crossed arch is one of three partners, each in turn, whichever
    // the pairs' order reads first.
    for b in [50.0, 170.0, 290.0] {
        let over = standing(b, 0.7, 0.5, pose);
        let what = format!("a standing pyramid over the arch at {b}°");
        builds(&what, &over, &s.arches, pose);
    }
}

/// **A vertex touching a face beside a partner whose link runs along
/// the face builds sound in every op** (`vtxfac::partner_side`): the
/// lying pyramid's ray along the top is an edge of its own on the top,
/// and pyramids under it, on it beside the lying one, around it, inside
/// it and continuing its face across it each pair with its apex; and
/// the scene mirrored through the top, a void below it whose ray lies
/// on it, which the edge-edge event reads under the top and in the
/// void's complement at once (`recl::HeldFace`): the plate less that
/// void holds the ray as a zero-area ring of the top, whose tip each
/// partner's apex meets ([`REFUSING`] names the cells that refuse). A
/// pyramid with a face flush on the top across the ray, and one with a
/// face on the lying pyramid's, refuse `PartnerAlongFace`: the layering
/// is not read beside a coincidence of the touch's or the pair's
/// (`vtxfac::partner_side`). At rest; every pose is the slow matrix's.
#[test]
fn a_touching_vertex_beside_a_partner_along_the_face_builds_sound_in_every_op() {
    let pose = &Pose::rest();
    let s = Scene::at(pose);
    for (label, x) in &s.along_ray {
        builds_along(label, x, &s.lying, pose, false, lying_ray());
    }
    for (label, x) in &s.along_void_ray {
        builds_along(label, x, &s.void_lying, pose, false, lying_ray());
    }
    refuses_beside_the_ray(
        "flush on the top",
        &s.flush,
        &s.lying,
        pose,
        AlongFace::Flush,
        &FLUSH_CROSSES,
    );
    refuses_beside_the_ray(
        "on the lying pyramid's face",
        &s.on_lying,
        &s.lying,
        pose,
        AlongFace::FacesInOnePlane,
        &[],
    );
}

/// **A vertex in several pairs, or touching a face beside nested
/// partners, reads each edge once**: two pyramids united at their
/// apexes, against the arch, the arch above a void, and the arch
/// alone; a pyramid inside an island in the void, and one inside a void
/// in the arch; and a pyramid with an edge lying along the arch's face.
/// At rest; every pose is the slow matrix's.
#[test]
fn a_vertex_in_several_pairs_or_beside_nested_partners_reads_each_edge_once() {
    let pose = &Pose::rest();
    let s = Scene::at(pose);
    builds(
        "two standing pyramids, one over the arch",
        &s.two_up,
        &s.one,
        pose,
    );
    builds(
        "two standing pyramids over the arch and void",
        &s.two_up,
        &s.both,
        pose,
    );
    builds(
        "two hanging pyramids beside the void",
        &s.two_down,
        &s.both,
        pose,
    );
    builds(
        "two standing pyramids over the bare arch",
        &s.two_up,
        &s.arch,
        pose,
    );
    builds(
        "a pyramid in the island in the void",
        &s.in_island,
        &s.island,
        pose,
    );
    builds(
        "a pyramid in the void in the arch",
        &s.in_hollow,
        &s.hollow,
        pose,
    );
    builds_along(
        "a pyramid along the arch",
        &s.along,
        &s.one,
        pose,
        true,
        None,
    );
    // A void in the arch reads through its complement: an edge on its
    // face is on the solid's, two boundaries deep, and a pyramid over a
    // void in the arch alone names as it did before any of this.
    builds(
        "a pyramid on the void's face in the arch",
        &s.on2,
        &s.hollow,
        pose,
    );
    builds(
        "a pyramid over a void in the bare arch",
        &s.over,
        &s.bare_hollow,
        pose,
    );
}

/// **A touching vertex with an edge along two arches' shared ray builds
/// in every op**: the plate united with two pyramids standing on
/// [`MEET`] that share an edge ray, against a pyramid standing there
/// with an edge along that ray. Its apex touches the top and pairs with
/// both arches' apexes, and the edge lies on both arches' boundaries;
/// the arches' faces part around the ray, so the edge is on the solid's
/// (`vtxfac::along_partners`). The pyramid lies outside both arches,
/// inside the wedge between their far faces, or with a face in each,
/// at every pose. Tier 3′ holds in one result of the first pyramid and
/// three of each other, at every pose; elsewhere it refuses only as
/// [`three_prime`] allows.
/// Slow set; the row below is its per-PR pose.
#[test]
fn a_touching_vertex_along_two_arches_shared_ray_builds_in_every_op() {
    for pose in &poses() {
        for (shape, held) in ARCH_PYRAMIDS.iter().zip([1, 3, 3]) {
            assert_eq!(
                along_two_arches_shared_ray(pose, shape),
                held,
                "{}",
                shape.0
            );
        }
    }
}

/// The per-PR pose of the row above: the pyramid outside both arches,
/// at rest.
#[test]
fn a_touching_vertex_along_two_arches_shared_ray_builds_at_rest() {
    assert_eq!(
        along_two_arches_shared_ray(&Pose::rest(), &ARCH_PYRAMIDS[0]),
        1
    );
}

/// A pyramid along the arches' shared ray: a label, and its base's two
/// corners off the ray as bearing, radius and height.
type ArchPyramid = (&'static str, [(f64, f64, f64); 2]);

/// The pyramids along the arches' shared ray.
const ARCH_PYRAMIDS: [ArchPyramid; 3] = [
    (
        "a pyramid along the arches' ray, outside both",
        [(160.0, 0.4, 0.5), (220.0, 0.4, 0.5)],
    ),
    (
        "a pyramid along the arches' ray, in the wedge between them",
        [(20.0, 0.2, 0.3), (60.0, 0.2, 0.3)],
    ),
    (
        "a pyramid along the arches' ray, a face in each",
        [(10.0, 0.3, 0.45), (70.0, 0.3, 0.45)],
    ),
];

/// The arches' scene ([`a_touching_vertex_along_two_arches_shared_ray_builds_in_every_op`])
/// at `pose`, against one of [`ARCH_PYRAMIDS`]; how many results held
/// tier 3′.
fn along_two_arches_shared_ray(pose: &Pose, &(label, corners): &ArchPyramid) -> usize {
    let plate = posed_box("the plate", PLATE, pose, t());
    let shared = bearing(40.0, 0.4, 0.5);
    let arches = [
        [bearing(0.0, 0.4, 0.5), shared, bearing(20.0, 0.25, 0.4)],
        [shared, bearing(80.0, 0.4, 0.5), bearing(60.0, 0.25, 0.4)],
    ]
    .map(|base| tet(base, pose));
    let line = arches.iter().fold(plate, |body, arch| {
        built("the plate and arches", union(&body, arch, t()))
    });
    let [(b, r, z), (c, s, w)] = corners;
    let base = [bearing(40.0, 0.6, 0.75), bearing(b, r, z), bearing(c, s, w)];
    builds_where(label, &tet(base, pose), &line, pose, false, None, None)
}

/// **A vertex whose edge runs through two blocks' glued faces builds in
/// every op**: two blocks in face contact, their corners at [`MEET`],
/// against a pyramid standing there whose edge runs through the contact
/// face's interior and leaves the blocks, out the side, the bottom, or
/// a far corner. The edge lies on both blocks' boundaries, their
/// contact faces flush along it: plane faces bounding material on
/// opposite sides, coincident only to within the band, so the edge is
/// `On` (`vtxfac::along_partners`), its class as the naming records it
/// and as the blocks read the point beside it ([`classes_hold`]). Tier 3′
/// is as [`three_prime`] allows. Out the bottom, `y − x` refuses
/// `EdgeInFace` ([`REFUSING`]). Slow set, at every pose; the row below
/// is its per-PR pose.
#[test]
fn an_edge_through_two_blocks_glued_faces_builds_in_every_op() {
    for pose in &poses() {
        for pyramid in &GLUED_PYRAMIDS {
            through_glued_faces(pose, pyramid);
        }
    }
}

/// The per-PR pose of the row above: the pyramid out the side, at rest.
#[test]
fn an_edge_through_two_blocks_glued_faces_builds_at_rest() {
    through_glued_faces(&Pose::rest(), &GLUED_PYRAMIDS[0]);
}

/// A pyramid through the blocks' glued faces: a label, the far end of
/// its edge along the contact face, its base's two corners off it
/// (bearing, radius and height), relative to [`MEET`], and whether the
/// edge leaves the blocks through a corner of their glued rectangle.
type GluedPyramid = (&'static str, [f64; 3], [(f64, f64, f64); 2], bool);

/// The pyramids through the blocks' glued faces.
const GLUED_PYRAMIDS: [GluedPyramid; 3] = [
    (
        "a pyramid through the glued faces, out the side",
        [0.0, 0.8, -0.8],
        [(200.0, 0.4, 0.2), (240.0, 0.4, 0.2)],
        true,
    ),
    (
        "a pyramid through the glued faces, out the bottom",
        [0.0, 0.3, -0.8],
        [(200.0, 0.4, 0.2), (240.0, 0.4, 0.2)],
        false,
    ),
    (
        "a pyramid through the glued faces, out a far corner",
        [0.0, 0.7, -0.7],
        [(170.0, 0.4, 0.3), (260.0, 0.4, 0.3)],
        true,
    ),
];

/// The glued blocks' scene ([`an_edge_through_two_blocks_glued_faces_builds_in_every_op`])
/// at `pose`, against one of [`GLUED_PYRAMIDS`]: every op builds, the
/// union names the edge through the glued faces `On`, and where the edge
/// leaves through a corner of the glued rectangle, the union's decision
/// there names both blocks' vertices at that corner.
fn through_glued_faces(pose: &Pose, &(label, along, corners, by_corner): &GluedPyramid) {
    let blocks = posed_boxes(
        "two blocks glued at their corners' face",
        &[
            [(1.0, 1.5), (1.0, 1.5), (0.5, 1.0)],
            [(1.5, 2.0), (1.0, 1.5), (0.5, 1.0)],
        ],
        pose,
        t(),
    );
    let [(b, r, z), (c, s, w)] = corners;
    let x = tet([along, bearing(b, r, z), bearing(c, s, w)], pose);
    // The blocks' own contacts on their glued rectangle, x = 1.5, which
    // no record carries here, may refuse 3′ too.
    let glued = [
        [1.5, 1.0, 0.5],
        [1.5, 1.5, 0.5],
        [1.5, 1.5, 1.0],
        [1.5, 1.0, 1.0],
    ];
    builds_where(label, &x, &blocks, pose, false, None, Some(glued));
    let meet = at(pose.at(MEET));
    let far = at(pose.at([0, 1, 2].map(|k| MEET[k] + along[k])));
    let r = match union(&x, &blocks, t()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{label}: x ∪ y: {:?}", other.map(|_| ())),
    };
    let through: Vec<_> = r
        .naming
        .edge_classes
        .iter()
        .filter(|row| row.operand == Operand::A)
        .filter(|row| {
            let Ok(p) = readback::vertex_point(&x, row.vertex) else {
                return false;
            };
            let edge = x.get_edge(row.edge).unwrap();
            let plus = x.get_half_edge(edge.he_plus).unwrap();
            let ends = [plus.start, x.get_half_edge(plus.next).unwrap().start]
                .map(|v| at(readback::vertex_point(&x, v).unwrap()));
            at(p) == meet && ends.contains(&far)
        })
        .collect();
    assert!(!through.is_empty(), "{label}: the union names the edge");
    for row in through {
        assert_eq!(row.class, SideCode::On, "{label}: {row:?}");
    }
    if by_corner {
        // The pyramid's edge leaves the blocks through a corner they share,
        // one vertex of each: it is split at one, and the other is decided
        // one with that split. The row naming that decision names both
        // corners, the edge's minting cell beside the corner decided on it.
        let corners = |row: &topo::Coincidence| {
            let mut corners: Vec<_> = row
                .every_cell()
                .filter_map(|c| match c {
                    topo::RowCell::Input {
                        input: Operand::B,
                        cell: topo::Cell::Vertex(v),
                    } => Some(v),
                    _ => None,
                })
                .collect();
            corners.sort();
            corners.dedup();
            corners.len()
        };
        assert!(
            r.coincidences.iter().any(|row| corners(row) == 2),
            "{label}: a row names both blocks' corners on the edge: {:?}",
            r.coincidences
        );
    }
}

/// **A vertex that crosses a face it is paired on, or pierces two
/// faces, refuses typed in every op**: the prisms beside one standing
/// pyramid, and a pyramid and the prism against two blocks in face
/// contact.
#[test]
fn a_vertex_crossing_a_face_it_pairs_on_or_piercing_two_refuses_typed_in_every_op() {
    for pose in poses() {
        let s = Scene::at(&pose);
        refuses(
            "a prism through the top",
            &s.prism,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses(
            "a leaned prism through the top",
            &s.leaned,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses("two blocks", &s.cone, &s.blocks, &pose, "pierce", false);
        // The prism's edge crosses the contact at `MEET`, so its first
        // pierce would hang struts there: only a refusal before that
        // pass writes keeps the second pierce off the written orbit.
        refuses(
            "a prism through two blocks",
            &s.prism,
            &s.blocks,
            &pose,
            "pierce",
            true,
        );
    }
}

/// **Every scene, at every pose, in every op and both orders, builds
/// sound or refuses typed**: 66 scenes, 1980 op cells.
#[test]
fn every_scene_builds_sound_or_refuses_typed_at_every_pose() {
    let mut held = 0;
    for pose in poses() {
        let s = Scene::at(&pose);
        for (label, x, y) in [
            ("the arches", &s.cone, &s.arches),
            ("one standing pyramid", &s.cone, &s.one),
            ("over the arch", &s.over, &s.one),
            ("over the arches", &s.over, &s.arches),
            ("hanging below the arch", &s.hang, &s.one),
            ("hanging across below the arch", &s.hang_over, &s.one),
            ("hanging below the arches", &s.hang, &s.arches),
            ("standing over the cavity", &s.cone, &s.cavity),
            ("hanging below the cavity", &s.hang, &s.cavity),
            ("hanging across the cavity", &s.hang_over, &s.cavity),
            ("hanging into the void", &s.in_void, &s.cavity),
            ("standing on the plate", &s.cone, &s.plate),
            ("hanging in the plate", &s.hang, &s.plate),
            ("the bare arches", &s.cone, &s.bare),
            ("hanging below the bare arches", &s.hang, &s.bare),
            ("a prism through the plate", &s.prism, &s.plate),
            ("a leaned prism through the plate", &s.leaned, &s.plate),
            ("two up, one over the arch", &s.two_up, &s.one),
            ("two up over the arch and void", &s.two_up, &s.both),
            ("two down beside the void", &s.two_down, &s.both),
            ("two up over the bare arch", &s.two_up, &s.arch),
            ("over the arch and void", &s.over, &s.both),
            ("hanging across the arch and void", &s.hang_over, &s.both),
            ("in the island in the void", &s.in_island, &s.island),
            ("in the void in the arch", &s.in_hollow, &s.hollow),
            ("over the void in the arch", &s.over, &s.hollow),
            ("beside the void in the arch", &s.cone, &s.hollow),
            ("crossing the void in the arch", &s.cross3, &s.hollow),
            ("on the void's face in the arch", &s.on2, &s.hollow),
            ("crossing the island in the void", &s.cross_in, &s.island),
            ("beside the island in the void", &s.cone, &s.island),
            ("over a void in the bare arch", &s.over, &s.bare_hollow),
            (
                "two up over a void in the bare arch",
                &s.two_up,
                &s.bare_hollow,
            ),
            ("crossing three levels in the arch", &s.cross3, &s.deep),
            ("on the void's face, the island in it", &s.on2, &s.deep),
            ("over the deep arch", &s.over, &s.deep),
            ("standing over a dart void", &s.cone, &s.dart_void),
            ("over a dart void", &s.over, &s.dart_void),
            ("beside a dart", &s.cone, &s.dart),
            ("over a dart", &s.over, &s.dart),
            ("hanging into a dart void", &s.hang, &s.dart_void),
            ("hanging across a dart void", &s.hang_over, &s.dart_void),
            ("hanging below a near-flat void", &s.hang, &s.flat_voids[0]),
            (
                "hanging across a near-flat void",
                &s.hang_over,
                &s.flat_voids[0],
            ),
            (
                "hanging across a nearer-flat void",
                &s.hang_over,
                &s.flat_voids[1],
            ),
            ("standing over a near-flat void", &s.cone, &s.flat_voids[0]),
            (
                "standing over a nearer-flat void",
                &s.cone,
                &s.flat_voids[1],
            ),
            ("over a quad void in a quad arch", &s.over, &s.quad_hollow),
            (
                "over a quad void in a bare quad arch",
                &s.over,
                &s.bare_quad_hollow,
            ),
        ] {
            held += builds(label, x, y, &pose);
        }
        held += builds_along("along the arch", &s.along, &s.one, &pose, true, None);
        held += builds_along("along the bare arch", &s.along, &s.arch, &pose, true, None);
        for (label, x) in [
            ("standing beside a lying pyramid", &s.cone),
            ("over a lying pyramid", &s.over),
            ("hanging below a lying pyramid", &s.hang),
        ] {
            held += builds_along(label, x, &s.lying, &pose, false, lying_ray());
        }
        for (label, x) in &s.along_ray {
            held += builds_along(label, x, &s.lying, &pose, false, lying_ray());
        }
        for (label, x) in &s.along_void_ray {
            held += builds_along(label, x, &s.void_lying, &pose, false, lying_ray());
        }
        refuses_beside_the_ray(
            "flush on the top",
            &s.flush,
            &s.lying,
            &pose,
            AlongFace::Flush,
            &FLUSH_CROSSES,
        );
        refuses_beside_the_ray(
            "on the lying pyramid's face",
            &s.on_lying,
            &s.lying,
            &pose,
            AlongFace::FacesInOnePlane,
            &[],
        );
        refuses(
            "a prism through the top",
            &s.prism,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses(
            "a leaned prism through the top",
            &s.leaned,
            &s.one,
            &pose,
            "pair",
            true,
        );
        refuses("two blocks", &s.cone, &s.blocks, &pose, "pierce", false);
        refuses(
            "a prism through two blocks",
            &s.prism,
            &s.blocks,
            &pose,
            "pierce",
            true,
        );
    }
    assert!(held > 0, "some result holds tier 3′");
}

/// **A pyramid resting where a solid touches itself holds tier 3′.**
/// The cavity, the plate less a pyramid hanging from [`MEET`], touches
/// itself there: the void's apex on the top's interior. A pyramid
/// standing on its apex at `MEET` rests on the top, and the plate and
/// one arch less a pyramid hanging below `MEET` leaves the arch resting
/// on the hollow. Each result's solids are read with their whole links
/// at `MEET` — the top's disc less the void's cone, not the void's apex
/// alone — and tier 3′ refuses no solid pair, at every pose: what it
/// refuses is the operand's own contact at `MEET`, which the op called
/// without records does not carry ([`three_prime`]).
#[test]
fn a_pyramid_resting_where_a_solid_touches_itself_holds_tier_three_prime() {
    for pose in poses() {
        let s = Scene::at(&pose);
        for (what, r) in [
            ("cavity ∪ cone", union(&s.cavity, &s.cone, t())),
            ("cone ∪ cavity", union(&s.cone, &s.cavity, t())),
            ("cavity − cone", subtract(&s.cavity, &s.cone, t())),
            ("one − hang", subtract(&s.one, &s.hang, t())),
        ] {
            let what = format!("{}, {what}", pose.label);
            let Ok(BooleanResult::Body(r)) = r else {
                panic!("{what}: builds, got {:?}", r.map(|_| ()));
            };
            let errors = validate_pseudomanifold(&r.body, &r.contacts, t()).err();
            let pairs: Vec<_> = errors
                .iter()
                .flatten()
                .filter(|e| {
                    matches!(e, ValidationError::CensusUndecidable { .. })
                        || common::is_decided_overlap(&r.body, e)
                })
                .collect();
            assert!(pairs.is_empty(), "{what}: {errors:?}");
            three_prime(&what, &r, &pose, false, None, None);
        }
    }
}

/// **Standing pyramids folded onto the plate one at a time build in
/// every member order**, at tier 3 and the closed-form volume; tier 3′
/// refuses only the earlier steps' contacts at `MEET`, which the last
/// step's record does not carry (`three_prime`).
#[test]
fn standing_pyramids_folded_onto_the_plate_build_in_every_member_order() {
    for pose in poses() {
        let mut members = vec![posed_box("the plate", PLATE, &pose, t())];
        members.extend([60.0, 240.0].map(|b| standing(b, 0.5, 0.4, &pose)));
        let want: f64 = members.iter().map(volume).sum();
        for order in orders(members.len()) {
            let what = format!("{}, member order {order:?} (0 = plate)", pose.label);
            let mut last = None;
            let mut body = members[order[0]].clone();
            for (k, &i) in order.iter().enumerate().skip(1) {
                match union(&body, &members[i], t()) {
                    Ok(BooleanResult::Body(r)) => {
                        body = r.body.clone();
                        last = Some(r);
                    }
                    other => panic!("{what}: step {k}, {:?}", other.map(|_| ())),
                }
            }
            let last = last.unwrap();
            assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
            three_prime(&what, &last, &pose, false, None, None);
            let v = volume(&body);
            assert!((v - want).abs() < 1e-9, "{what}: volume {v}, want {want}");
        }
    }
}

/// **The plate, the three arches and a fourth standing pyramid fold in
/// a sample of their 120 member orders**: every seventh order, at three
/// poses, at tier 3 and the closed-form volume, tier 3′ as
/// `three_prime` allows. Every order at every pose was run once and
/// built so (the PR that added this row).
#[test]
fn five_members_fold_onto_the_plate_in_sampled_member_orders() {
    let poses = poses();
    for pose in [&poses[0], &poses[2], &poses[4]] {
        let mut members = vec![posed_box("the plate", PLATE, pose, t())];
        members.extend([60.0, 180.0, 300.0].map(|b| standing(b, 0.5, 0.4, pose)));
        members.push(standing(240.0, 0.7, 0.5, pose));
        let want: f64 = members.iter().map(volume).sum();
        for order in orders(members.len()).into_iter().step_by(7) {
            let what = format!("{}, member order {order:?} (0 = plate)", pose.label);
            let mut last = None;
            let mut body = members[order[0]].clone();
            for (k, &i) in order.iter().enumerate().skip(1) {
                match union(&body, &members[i], t()) {
                    Ok(BooleanResult::Body(r)) => {
                        body = r.body.clone();
                        last = Some(r);
                    }
                    other => panic!("{what}: step {k}, {:?}", other.map(|_| ())),
                }
            }
            assert_eq!(validate_geometric(&body, t()), Ok(()), "{what}: tier 3");
            three_prime(&what, &last.unwrap(), pose, false, None, None);
            let v = volume(&body);
            assert!((v - want).abs() < 1e-9, "{what}: volume {v}, want {want}");
        }
    }
}

/// Whether `body` holds `q`: `None` on its boundary or where the read
/// escalates, so a probe within the band of a face reads nothing.
fn inside_or_unread(body: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    let band = Band::linear(t()).unwrap();
    match point_in_solid(body, q, band, t()) {
        Ok(SolidContainment::In) => Some(true),
        Ok(SolidContainment::Out) => Some(false),
        Ok(SolidContainment::OnBoundary) | Err(_) => None,
    }
}

/// Every op on `(x, y)`, in both orders, builds sound or refuses as
/// `typed` allows: a built result is tier 3, its material at the probes
/// about [`MEET`] the op's over the operands', and each class its naming
/// reads at `MEET` the other operand's containment a little along the
/// edge, wherever those read; where all six build, their volumes add up
/// as in [`builds_along`]. Returns how many built.
fn builds_or_refuses(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    typed: &dyn Fn(&BooleanError) -> bool,
) -> usize {
    let probes: Vec<_> = samples(x, y, pose)
        .into_iter()
        .filter_map(|q| Some((q, inside_or_unread(x, q)?, inside_or_unread(y, q)?)))
        .collect();
    let meet = at(pose.at(MEET));
    let x_less: fn(bool, bool) -> bool = |x, y| x && !y;
    let y_less: fn(bool, bool) -> bool = |x, y| y && !x;
    let or: fn(bool, bool) -> bool = |x, y| x || y;
    let and: fn(bool, bool) -> bool = |x, y| x && y;
    let mut v = [None; 6];
    let oracle = VolumeOracle::new(x, y, t());
    for (k, (what, keep, a, b, (kind, r))) in [
        (
            "x − y",
            x_less,
            x,
            y,
            (BooleanOp::Subtract, subtract(x, y, t())),
        ),
        (
            "y − x",
            y_less,
            y,
            x,
            (BooleanOp::Subtract, subtract(y, x, t())),
        ),
        ("x ∪ y", or, x, y, (BooleanOp::Union, union(x, y, t()))),
        ("y ∪ x", or, y, x, (BooleanOp::Union, union(y, x, t()))),
        (
            "x ∩ y",
            and,
            x,
            y,
            (BooleanOp::Intersect, intersect(x, y, t())),
        ),
        (
            "y ∩ x",
            and,
            y,
            x,
            (BooleanOp::Intersect, intersect(y, x, t())),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let op = what;
        let what = format!("{label}, {}, {what}", pose.label);
        if r.is_ok() {
            not_pinned(label, op);
        }
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) => {
                for &(q, inx, iny) in &probes {
                    assert!(!keep(inx, iny), "{what}: empty, but holds {q:?}");
                }
                oracle.assert(&what, kind, a, b, None);
                v[k] = Some(0.0);
                continue;
            }
            Err(e) => {
                assert!(
                    typed(&e) || pinned(label, op, &e),
                    "{what}: builds or refuses typed, got {e:?}"
                );
                continue;
            }
        };
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        for &(q, inx, iny) in &probes {
            if let Some(got) = inside_or_unread(&r.body, q) {
                assert_eq!(got, keep(inx, iny), "{what}: material at {q:?}");
            }
        }
        for row in &r.naming.edge_classes {
            let own = if row.operand == Operand::A { a } else { b };
            let other = if row.operand == Operand::A { b } else { a };
            let Ok(p) = readback::vertex_point(own, row.vertex) else {
                continue;
            };
            if at(p) != meet {
                continue;
            }
            let edge = own.get_edge(row.edge).unwrap();
            let plus = own.get_half_edge(edge.he_plus).unwrap();
            let far = if row.starts {
                own.get_half_edge(plus.next).unwrap().start
            } else {
                plus.start
            };
            let d = readback::vertex_point(own, far).unwrap() - p;
            let band = Band::linear(t()).unwrap();
            let want = match point_in_solid(other, p + d * (0.02 / d.norm()), band, t()) {
                Ok(SolidContainment::In) => SideCode::In,
                Ok(SolidContainment::Out) => SideCode::Out,
                Ok(SolidContainment::OnBoundary) => SideCode::On,
                Err(_) => continue,
            };
            assert_eq!(row.class, want, "{what}: {row:?} against the other operand");
        }
        oracle.assert(&what, kind, a, b, Some(&r.body));
        v[k] = Some(volume(&r.body));
    }
    if let [Some(xy), Some(yx), Some(u), Some(u2), Some(i), Some(i2)] = v {
        let (vx, vy) = (volume(x), volume(y));
        for (sum, want, which) in [
            (u + i, vx + vy, "|x ∪ y| + |x ∩ y|"),
            (xy + i, vx, "|x − y| + |x ∩ y|"),
            (yx + i, vy, "|y − x| + |x ∩ y|"),
            (u2, u, "|y ∪ x|"),
            (i2, i, "|y ∩ x|"),
        ] {
            assert!(
                (sum - want).abs() < 1e-9,
                "{label}, {}: {which} {sum}, want {want}",
                pose.label
            );
        }
    }
    v.iter().filter(|v| v.is_some()).count()
}

/// **A partner tipped off the face, just in and just out of the band on
/// either side, builds sound or refuses typed in every op**: the lying
/// pyramid's corner on the top lifted or sunk by half a zero band (it
/// reads on the top), three and five (in band) and a hundred (off it),
/// each a multiple of ε, so every tolerance row reads the same margins.
/// In band the plate and the lying pyramid do not unite: their own
/// touch escalates, so no partner in band reaches a pair. Within the
/// zero every scene builds as on the top or escalates where a sector
/// pair reads in band; off it the partner is strictly one side, or
/// crosses the top. A refusal is an escalation, the
/// coincidence a face of the pyramid on the top or on the lying one's
/// refuses (`PartnerAlongFace`, or `VertexReadTwice` where its pierce
/// crosses the top), or, a pyramid inside the lifted one, the
/// pierce germ that no sector holds alone
/// (`work/cleave/near-tangent-pierce-poses-reach-three-classification-invariants.md`).
#[test]
fn a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose() {
    let eps = t().eps();
    let mut built = 0;
    for pose in poses() {
        let s = Scene::at(&pose);
        for k in [0.5, -0.5, 3.0, -3.0, 5.0, -5.0, 100.0, -100.0] {
            let y = match union(&s.plate, &tet(lying(k * eps), &pose), t()) {
                Ok(BooleanResult::Body(r)) => r.body,
                other => {
                    assert!(
                        (2.0..=5.0).contains(&f64::abs(k))
                            && matches!(other, Err(BooleanError::Escalated { .. })),
                        "the plate and a pyramid lying {k}ε off the top, {}: builds out of \
                         band, escalates in it, got {:?}",
                        pose.label,
                        other.map(|_| ())
                    );
                    continue;
                }
            };
            assert!(
                f64::abs(k) < 1.0 || f64::abs(k) > 10.0,
                "{}: the plate and a pyramid lying {k}ε off the top build in band",
                pose.label
            );
            let germ = |e: &BooleanError| {
                k > 10.0
                    && matches!(e, BooleanError::ClassificationInvariant { what }
                        if *what == "pierce germ direction not uniquely within its sector")
            };
            for (label, x, faced) in [
                ("standing beside", &s.cone, false),
                ("over", &s.over, false),
                ("hanging below", &s.hang, false),
                ("flush on the top", &s.flush, true),
                ("on its face", &s.on_lying, true),
            ]
            .into_iter()
            .chain(s.along_ray.iter().map(|(label, x)| (*label, x, false)))
            {
                let inside = label.starts_with("inside");
                let label = format!("{label}, the pyramid lying {k}ε off the top");
                built += builds_or_refuses(&label, x, &y, &pose, &|e| {
                    matches!(e, BooleanError::Escalated { .. })
                        || (faced
                            && matches!(
                                e,
                                BooleanError::PartnerAlongFace { .. }
                                    | BooleanError::VertexReadTwice { .. }
                            ))
                        || (inside && germ(e))
                });
            }
        }
    }
    assert!(built > 0, "cells built");
}

/// **A partner tipped off the face still ends the cut**: with the lying
/// pyramid's corner a hundred ε off the top, its apex rests on the top
/// unimprinted, so the flush pyramid's edges end on the top where it
/// holds no vertex. `flush − y` keeps the flush pyramid's face over the
/// top and cuts the top where that face ends (`boolean::fate`, a lone
/// vertex minted at the apex): the row names a fragment of the top, not
/// the top whole, and the body holds tiers 3 and 3′ (`three_prime`).
#[test]
fn a_partner_tipped_off_the_face_still_ends_the_cut() {
    let eps = t().eps();
    for pose in poses() {
        let s = Scene::at(&pose);
        let y = built(
            "the plate and the tipped pyramid",
            union(&s.plate, &tet(lying(100.0 * eps), &pose), t()),
        );
        let BooleanResult::Body(out) = subtract(&s.flush, &y, t()).expect("flush − y builds")
        else {
            panic!("{}: flush − y is not empty", pose.label);
        };
        let rows: Vec<_> = out
            .naming
            .substituted
            .iter()
            .map(|c| (c.operand, c.face, out.naming.face_fragments_b.root(c.face)))
            .collect();
        assert!(
            matches!(rows.as_slice(), [(Operand::B, face, root)] if face != root),
            "{}: one row, a fragment cut from y's top, got {rows:?}",
            pose.label
        );
        let what = format!("{}: flush − y", pose.label);
        assert_eq!(validate_geometric(&out.body, t()), Ok(()), "{what}: tier 3");
        three_prime(&what, &out, &pose, false, None, None);
    }
}

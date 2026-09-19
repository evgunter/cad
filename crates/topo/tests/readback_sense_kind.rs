//! **The sense beside the pose, and the carrier-kind read** — the two
//! read-back doors `crates/editor-core/REFERENCES.md` DM1a and DM2
//! add.
//!
//! What is pinned here is the kernel half of the contract: the pose's
//! `sense` is the face record's stored flag and nothing else (both
//! senses, against the stored bit, so a door that silently kept the
//! chart normal would fail one row); `axis` is still the chart's
//! direction on either sense; and the carrier kind is the stored tag
//! copied out — on a FACE, refusing only the dangling arms, and on an
//! EDGE, refusing those and the null-edge scaffold that has no
//! certified carrier to read. Every carrier that exists has a kind, so
//! a tag read never has a "no answer" lane, and the NURBS rows are
//! where a kind read answers and a frame read cannot.
//!
//! The edge rows also pin the two seats against each other: the
//! predicate seat's `query::edge_carrier_kind` is this door flattened,
//! and the pair of refusals it flattens to one `None` are two
//! different facts about the body. That the seat is the flattening and
//! not a second copy of the walk is not a behaviour any row can see —
//! the two bodies are extensionally equal — so it is pinned as a fact
//! about the source text, in the last row here.
//!
//! The walk's third refusal, a curve key a live edge names and the
//! arena does not hold, is rowed in `readback.rs`'s own `mod tests`:
//! planting it needs a crate-private arena writer.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom::Curve3;
use geom::NurbsCurve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_brep::{OutwardNormal, SurfaceKind};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Tol, Vec3};
use topo::readback::{
    DanglingRef, ReadbackError, edge_carrier_kind, edge_pose, face_carrier_kind, face_pose,
};
use topo::{
    Body, CurveKind, EdgeKey, EntityId, FaceKey, FaceSurface, MevSite, NewVertexSide, Surface,
    query,
};

/// A seed face carrying `surface`.
fn seed_face(surface: Surface<f64>) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let seed = body
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    body.set_face_surface(seed.face, FaceSurface::New(surface))
        .expect("a live face takes a surface");
    (body, seed.face)
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **The sense is the stored flag, on both senses, and the axis stays
/// the chart's on both.** The flipped body is the discriminating
/// input `flipped_face_sense_for_tests` exists for: a door that folded
/// the sense into `axis` would move the axis here, and a door that
/// ignored the sense would report `true` on the flipped row.
#[test]
fn face_pose_reports_the_stored_sense_beside_an_uncorrected_axis() {
    let (body, face) = seed_face(plane());
    let flipped = body
        .flipped_face_sense_for_tests(face)
        .expect("the seed face is live");
    for (label, b, stored) in [("minted", &body, true), ("flipped", &flipped, false)] {
        let stored_flag = b.get_face(face).expect("live").sense;
        assert_eq!(stored_flag, stored, "{label}: the fixture's own sense");
        let pose = face_pose(b, face).expect("a planar carrier");
        assert_eq!(
            pose.sense, stored,
            "{label}: the pose copies the stored flag out"
        );
        assert_eq!(
            pose.axis.z, 1.0,
            "{label}: the axis is the chart's, uncorrected"
        );
        // The outward normal is the reader's to form, through the
        // type's one constructor.
        assert_eq!(
            OutwardNormal::from_chart(pose.axis, pose.sense).vec().z,
            if stored { 1.0 } else { -1.0 },
            "{label}"
        );
    }
}

/// **The carrier kind is the stored tag, for every analytic kind and
/// for the two that have no canonical frame.** `face_pose` refuses a
/// NURBS carrier (rule 3); the kind read does not, because the kind
/// is exactly what IS stored about such a face.
#[test]
fn face_carrier_kind_copies_the_tag_out_for_every_kind() {
    let rows: [(Surface<f64>, SurfaceKind); 3] = [
        (plane(), SurfaceKind::Plane),
        (
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            SurfaceKind::Cylinder,
        ),
        (
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            SurfaceKind::Sphere,
        ),
    ];
    for (surface, kind) in rows {
        let (body, face) = seed_face(surface);
        assert_eq!(face_carrier_kind(&body, face), Ok(kind));
        // The sense flip changes nothing about the kind: two facts.
        let flipped = body.flipped_face_sense_for_tests(face).expect("live");
        assert_eq!(face_carrier_kind(&flipped, face), Ok(kind));
    }
}

/// **The only refusal is a dangling key**, in the same vocabulary
/// `face_pose` uses, so a caller maps the two doors' refusals alike.
#[test]
fn face_carrier_kind_refuses_dangling_and_nothing_else() {
    let (body, face) = seed_face(plane());
    let empty = Body::<f64>::new();
    assert_eq!(
        face_carrier_kind(&empty, face),
        Err(ReadbackError::Dangling {
            what: DanglingRef::Entity(EntityId::Face(face)),
        })
    );
    assert_eq!(face_carrier_kind(&body, face), Ok(SurfaceKind::Plane));
    // The placeholder a seed face is minted with is a kind too (rule
    // 1: the tag is stored data), where `face_pose` has no frame to
    // report.
    let mut bare = Body::<f64>::new();
    let seed = bare
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    assert!(matches!(
        face_pose(&bare, seed.face),
        Err(ReadbackError::NoCanonicalFrame { .. })
    ));
    assert!(face_carrier_kind(&bare, seed.face).is_ok());
}

// ---------------------------------------------------------------------
// The edge side: the certified carrier's kind tag, and the walk it
// shares with `edge_pose`.
// ---------------------------------------------------------------------

/// A one-edge body whose edge carries a LINE, through `mev_line`.
fn line_edge() -> (Body<f64>, EdgeKey) {
    let mut body = Body::<f64>::new();
    let seed = body
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .expect("a straight strut off the seed vertex");
    (body, seg.edge)
}

/// The unit circle in the `z = 0` plane.
fn circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A `2 × 1` ellipse in the same plane.
fn ellipse() -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major: 2.0,
        minor: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A one-edge body whose edge carries the half of `carrier` over
/// `[0, π]`, described in the `z = 0` plane it lies in.
fn conic_edge(carrier: Curve3<f64>) -> (Body<f64>, EdgeKey) {
    let half = core::f64::consts::PI;
    let mut body = Body::<f64>::new();
    let seed = body
        .mvfs(carrier.eval(0.0))
        .expect("mvfs has no preconditions");
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .expect("a live face takes a surface");
    let made = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            carrier.eval(half),
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart(plane),
                carrier,
                param_start: 0.0,
                param_end: half,
            },
            Tol::witness(),
        )
        .expect("a conic half-arc at rest in its own plane");
    (body, made.edge)
}

/// A cube body with one wall edge re-stated on a NURBS carrier: the
/// degree-1 curve between its own endpoints, described as the
/// intersection of the two planes that already meet there. The locus
/// does not move; what changes is the carrier's KIND, which is the
/// only thing these rows read.
fn nurbs_edge() -> (Body<f64>, EdgeKey) {
    let cube = common::geometric_cube::<f64>(Tol::witness());
    let mut body = cube.body;
    let (edge_key, edge) = body
        .edges()
        .map(|(k, e)| (k, e.clone()))
        .next()
        .expect("a cube edge");
    let surface_of = |b: &Body<f64>, he| {
        let l = b.get_half_edge(he).expect("a live half-edge").parent_loop;
        b.get_face(b.get_loop(l).expect("a live loop").face)
            .expect("a live face")
            .surface
    };
    let (s1, s2) = (
        surface_of(&body, edge.he_plus),
        surface_of(&body, edge.he_minus),
    );
    let start = body
        .get_half_edge(edge.he_plus)
        .expect("a live half-edge")
        .start;
    let end = body.half_edge_end(edge.he_plus).expect("a live half-edge");
    let p0 = *body
        .get_point(body.get_vertex(start).expect("live").point)
        .expect("live");
    let p1 = *body
        .get_point(body.get_vertex(end).expect("live").point)
        .expect("live");
    let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("a degree-1 knot vector");
    let carrier = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(knots, vec![p0, p1], vec![1.0, 1.0]).expect("a two-point NURBS curve"),
    ));
    body.set_edge_curve(
        edge_key,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: carrier.eval(0.5),
            },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
        Tol::witness(),
    )
    .expect("a NURBS carrier on the intersection of the two walls");
    (body, edge_key)
}

/// **The carrier kind is the stored tag, for every curve kind — and
/// the query seat is exactly this door flattened.** The NURBS row is
/// the one that would be unreachable through a frame door: `edge_pose`
/// refuses it for want of a canonical frame (rule 3) while its kind is
/// precisely what the model stores about it.
#[test]
fn edge_carrier_kind_copies_the_tag_out_for_every_kind() {
    let rows = [
        ("line", line_edge(), CurveKind::Line),
        ("circle", conic_edge(circle()), CurveKind::Circle),
        ("ellipse", conic_edge(ellipse()), CurveKind::Ellipse),
        ("nurbs", nurbs_edge(), CurveKind::Nurbs),
    ];
    for (label, (body, edge), kind) in rows {
        assert_eq!(edge_carrier_kind(&body, edge), Ok(kind), "{label}");
        assert_eq!(
            query::edge_carrier_kind(&body, edge),
            Some(kind),
            "{label}: the seat is the door flattened"
        );
        // The frame door walks to the same carrier: it answers for
        // the three analytic kinds and refuses the NURBS one, while
        // the tag read answers for all four.
        match kind {
            CurveKind::Nurbs => assert!(matches!(
                edge_pose(&body, edge),
                Err(ReadbackError::NoCanonicalFrame { .. })
            )),
            _ => assert!(edge_pose(&body, edge).is_ok(), "{label}"),
        }
    }
}

/// **The two refusals, told apart.** A stale edge key and a null-edge
/// scaffold are one `None` at the predicate seat; the typed door says
/// which is which, in the vocabulary `edge_pose` refuses in — the
/// shared walk, verbatim on both.
#[test]
fn edge_carrier_kind_refuses_dangling_and_no_carrier_and_nothing_else() {
    let (body, edge) = line_edge();
    let empty = Body::<f64>::new();
    let stale = ReadbackError::Dangling {
        what: DanglingRef::Entity(EntityId::Edge(edge)),
    };
    assert_eq!(edge_carrier_kind(&empty, edge), Err(stale));
    assert_eq!(
        edge_pose(&empty, edge).err(),
        Some(stale),
        "one walk, one refusal"
    );

    let mut scaffold = Body::<f64>::new();
    let seed = scaffold
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs has no preconditions");
    let null = scaffold
        .mev_null(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            NewVertexSide::Above,
        )
        .expect("a null edge off the seed vertex");
    assert_eq!(
        edge_carrier_kind(&scaffold, null.edge),
        Err(ReadbackError::NoCarrier)
    );
    assert_eq!(
        edge_pose(&scaffold, null.edge).err(),
        Some(ReadbackError::NoCarrier),
        "one walk, one refusal"
    );

    // The delta: the seat cannot tell the two apart, and says so the
    // same way for both.
    assert_eq!(query::edge_carrier_kind(&empty, edge), None);
    assert_eq!(query::edge_carrier_kind(&scaffold, null.edge), None);
    // …while a live edge is neither.
    assert_eq!(edge_carrier_kind(&body, edge), Ok(CurveKind::Line));
}

/// The query seat's body, whitespace-normalised: the door called, and
/// its refusal flattened. Nothing else — no `get_edge`, no
/// `get_curve_geom`, no `certified`.
const SEAT_BODY: &str = "crate::readback::edge_carrier_kind(body, e).ok()";

/// **The predicate seat is the door flattened, structurally** — it
/// holds no walk of its own.
///
/// The rows above cannot see this and say so: the seat and a
/// hand-written arena walk are extensionally EQUAL, so restoring the
/// walk leaves every row in the workspace green. What is being pinned
/// is therefore a fact about the source text, in the shape
/// `editor-core`'s `wire_operand_door.rs` pins "built in one place":
/// the seat's body is one call, and a second reading of an edge's
/// carrier tag re-entering `query.rs` reds here.
///
/// It is a SOURCE-TEXT pin, so it says nothing about what the door
/// itself does — that is what the rows above are for — and nothing
/// about a walk written in a third file.
///
/// Ungated: it reads one file and runs no geometry.
#[test]
fn the_query_seat_holds_no_walk_of_its_own() {
    let query = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("query.rs");
    let text = std::fs::read_to_string(&query).expect("topo's query.rs is readable");
    // Comments and literals blanked: a doc comment quoting the call
    // must not answer for the body, and neither must a string.
    let code = test_utils::source::code_only(&text);

    let at = code
        .find("pub fn edge_carrier_kind")
        .expect("the predicate seat is declared in query.rs");
    let arg_open = at
        + code[at..]
            .find('(')
            .expect("a signature has an argument list");
    let arg_end =
        test_utils::source::balanced_end(&code, arg_open).expect("the argument list closes");
    let body_open = arg_end + code[arg_end..].find('{').expect("the seat has a body");
    let body_end = test_utils::source::balanced_end(&code, body_open).expect("the body closes");

    let body = code[body_open + 1..body_end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(
        body, SEAT_BODY,
        "query::edge_carrier_kind is the typed door flattened and nothing else; a walk here          would be a second reading of one edge's carrier tag"
    );
}

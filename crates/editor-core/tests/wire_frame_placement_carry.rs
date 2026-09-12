//! **An authored frame's `f64` placement is carried on the frame's own
//! value, not re-derived by each profile drawn on it** — the rows for
//! `NodeValue::placement_f64` and the READ that `wire::profile_plane_f64`
//! now is.
//!
//! The oracle is `fixture::plane_of`, which builds the plane from the
//! DOCUMENT's authored literals through `SketchPlane::from_frame` and
//! never touches the evaluator's `frame_from_slots` — so the equality
//! is a claim about the carried value, not a restatement of how it was
//! computed. Every number here is an exactly representable
//! axis-aligned literal, so the rows separate at every eps the gate
//! runs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, EvalOptions, Expr, Node, ParamName,
    ProfileDoc, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::Tol;

fn eval(
    doc: &ProfileDoc,
    prior: Option<&editor_core::Evaluation<f64>>,
) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The carried placement of a node's value, or `None` where the value
/// carries none.
fn carried(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> Option<profile::SketchPlane<f64>> {
    ev.value(node).expect("the node evaluated").placement_f64
}

/// Every component of a placement, as raw bits — the comparison an
/// approximate one would let through.
fn bits(p: &profile::SketchPlane<f64>) -> Vec<u64> {
    let a = &p.placement;
    [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
        .iter()
        .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
        .collect()
}

fn assert_same_plane(
    got: &profile::SketchPlane<f64>,
    want: &profile::SketchPlane<f64>,
    what: &str,
) {
    assert_eq!(bits(got), bits(want), "{what}: placement differs by bits");
}

/// The world points of a node's body, sorted by bits.
fn point_bits(ev: &editor_core::Evaluation<f64>, node: RecipeNodeId) -> Vec<(u64, u64, u64)> {
    let Some(ValuePayload::Body(b)) = ev.value(node).map(|v| &v.payload) else {
        panic!("node {} has no body", node.0)
    };
    let mut out: Vec<(u64, u64, u64)> = b
        .vertices()
        .filter_map(|(_, v)| b.get_point(v.point))
        .map(|p| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
        .collect();
    out.sort_unstable();
    out
}

fn p() -> ParamName {
    ParamName::new("lift")
}

/// A frame whose origin's z is the parameter `lift`, two square
/// profiles drawn on that one frame, and an extrude of the first.
/// Returns the document and (frame, first profile, extrude).
fn shared_frame_doc(lift: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("wire_frame_placement_carry", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::SetDocParam {
                name: p(),
                value: DocParam::continuous(Dimension::Length, lift),
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
    // Sketch +x along world +y and sketch +y along world +z: a frame
    // no reader can confuse with the identity, still exactly unit and
    // exactly perpendicular.
    let (doc, frame) = fixture::insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [
                fixture::len(2.0),
                fixture::len(-3.0),
                Expr::param(p(), Dimension::Length),
            ],
            u: [0.0, 1.0, 0.0].map(fixture::scl),
            v: [0.0, 0.0, 1.0].map(fixture::scl),
        }),
    );
    let (doc, first) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    let (doc, _second) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(4.0, 0.0, 0.5)])),
    );
    let (doc, extrude) = fixture::insert(
        doc,
        Node::Extrude {
            profile: first,
            distance: fixture::len(1.0),
        },
    );
    (doc, frame, first, extrude)
}

/// Row 1 — an authored frame's value carries the placement the
/// document's own literals denote, and only a frame's value carries
/// one.
#[test]
fn an_authored_frames_value_carries_its_f64_placement() {
    let doc = ProfileDoc::empty_derived("wire_frame_placement_carry_r1", Tol::witness());
    let (doc, frame) = fixture::insert(
        doc,
        fixture::frame([2.0, -3.0, 7.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    let (doc, plane_datum) = fixture::insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [0.0; 3].map(fixture::len),
            normal: [0.0, 0.0, 1.0].map(fixture::scl),
        }),
    );
    let ev = eval(&doc, None);
    let got = carried(&ev, frame).expect("an authored frame carries a placement");
    assert_same_plane(&got, &fixture::plane_of(&doc, frame), "the authored frame");
    assert!(
        carried(&ev, profile).is_none(),
        "a profile node carries no placement of its own"
    );
    assert!(
        carried(&ev, plane_datum).is_none(),
        "a datum PLANE is not a frame and carries no placement"
    );
}

/// The placement `shared_frame_doc(lift)`'s frame denotes, written out
/// by hand: the oracle for a frame whose origin is a PARAMETER, which
/// `fixture::plane_of` (literals only) cannot read.
fn shared_frame_plane(lift: f64) -> profile::SketchPlane<f64> {
    profile::SketchPlane::from_frame(
        geom_core::Point3::new(2.0, -3.0, lift),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
    )
}

/// Row 2 — the carry follows the parameter that drives the frame, and
/// a memo that served the frame's prior value cannot hand a reader the
/// old placement: the profiles drawn on the frame move with it, by
/// exactly the parameter's delta.
#[test]
fn the_carry_moves_with_the_parameter_and_the_memo_cannot_stale_it() {
    let (doc0, frame0, _first0, extrude0) = shared_frame_doc(0.0);
    let ev0 = eval(&doc0, None);
    assert_same_plane(
        &carried(&ev0, frame0).expect("a placement at lift = 0"),
        &shared_frame_plane(0.0),
        "lift = 0",
    );

    // The same document at lift = 7, evaluated WITH the lift = 0
    // evaluation as the memo's prior: the frame's key moves with its
    // nominal slots, so the carried placement must move too.
    let (doc7, frame7, _first7, extrude7) = shared_frame_doc(7.0);
    assert_eq!(
        (frame0, extrude0),
        (frame7, extrude7),
        "the two documents are built the same way"
    );
    let ev7 = eval(&doc7, Some(&ev0));
    assert_same_plane(
        &carried(&ev7, frame7).expect("a placement at lift = 7"),
        &shared_frame_plane(7.0),
        "lift = 7 over a lift = 0 prior",
    );

    // And the bodies moved with it: every world point rises by exactly
    // seven, which is exact in binary and so is a bit equality.
    let mut moved: Vec<(u64, u64, u64)> = point_bits(&ev0, extrude0)
        .into_iter()
        .map(|(x, y, z)| (x, y, (f64::from_bits(z) + 7.0).to_bits()))
        .collect();
    moved.sort_unstable();
    assert_eq!(
        point_bits(&ev7, extrude7),
        moved,
        "the profile on the frame placed from the moved carry"
    );
}

/// Row 3 — a DERIVED frame carries no placement, and that is the same
/// `None` `profile_plane_f64` answers for it: the profile drawn on it
/// still evaluates, placed at the lane.
#[test]
fn a_derived_frame_carries_no_placement_and_its_profile_still_builds() {
    let doc = ProfileDoc::empty_derived("wire_frame_placement_carry_r3", Tol::witness());
    let (doc, base) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile: base,
            distance: fixture::len(1.0),
        },
    );
    let (doc, derived) = fixture::insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: cube,
            face: fixture::fname(cube, editor_core::RoleSeg::Cap(editor_core::CapEnd::End)),
            spin: fixture::ang(0.0),
        }),
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            derived,
            vec![fixture::square(0.5, 0.5, 0.25)],
        )),
    );
    let (doc, up) = fixture::insert(
        doc,
        Node::Extrude {
            profile: boss,
            distance: fixture::len(0.5),
        },
    );
    let ev = eval(&doc, None);
    assert!(
        ev.value(derived).is_some(),
        "the derived frame evaluated: {:?}",
        ev.node_error(derived)
    );
    assert!(
        carried(&ev, derived).is_none(),
        "a derived frame has no document elaboration and so no carried placement"
    );
    assert!(
        ev.value(up).is_some(),
        "the profile on the derived frame still builds: {:?}",
        ev.node_error(up)
    );
    // It sits on the cube's top cap, which is where the lane read put
    // it — the carry's absence is not a fallback to the world xy.
    let zs: Vec<f64> = point_bits(&ev, up)
        .into_iter()
        .map(|(_, _, z)| f64::from_bits(z))
        .collect();
    assert!(
        zs.iter().all(|z| *z >= 1.0 - 1e-12),
        "the boss stands on the top cap, got z values {zs:?}"
    );
}

/// Row 4 — **the once-ness itself.** A frame's nine slots resolve into
/// a placement once per frame per evaluation, not once per profile
/// drawn on it, and the verdict log says so: the `datum_unit_norm`
/// decisions the placement makes are the FRAME's — two per scalar it
/// is read at, and it is read at two — they do not multiply with the
/// profiles, and no profile's log holds one.
///
/// The oracle is the log, not the carry — a reader that went back to
/// re-deriving the plane would put those decisions back on every
/// profile and move both counts, whatever `placement_f64` held.
#[test]
fn the_frames_axes_are_decided_once_per_frame_not_once_per_profile() {
    let axis_decisions = |ev: &editor_core::Evaluation<f64>, node: RecipeNodeId| {
        ev.value(node)
            .expect("the node evaluated")
            .verdicts
            .iter()
            .filter(|v| v.predicate == "datum_unit_norm")
            .count()
    };
    let (one, frame, first, _extrude) = shared_frame_doc(0.0);
    let ev_one = eval(&one, None);

    // A third and a fourth profile on the SAME frame.
    let (three, third) = fixture::insert(
        one.clone(),
        Node::Profile(fixture::desc(frame, vec![fixture::square(-4.0, 0.0, 0.5)])),
    );
    let (four, fourth) = fixture::insert(
        three,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.0, 4.0, 0.5)])),
    );
    let ev_four = eval(&four, None);

    assert_eq!(
        axis_decisions(&ev_one, frame),
        4,
        "the frame decides its two axes twice: once at the lane scalar for          the value it lands, once at the nominal for the placement it carries"
    );
    assert_eq!(
        axis_decisions(&ev_four, frame),
        axis_decisions(&ev_one, frame),
        "two more profiles on one frame decide its axes no further times"
    );
    for profile in [first, third, fourth] {
        assert_eq!(
            axis_decisions(&ev_four, profile),
            0,
            "profile {} reads the frame's placement and decides no axis",
            profile.0
        );
    }
}

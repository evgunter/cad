//! **Review probes, lane `placing-dual-6394-r1`, PR 6394 at `a18ea48`**
//! (D10 `[placement.values]`): a read slide beside a plane mate, the
//! same read once its plane is edited out from under it, the mated
//! slide beside a plane mate, and the read-back of a chain that turns
//! about a read axis off the origin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, EvalOptions, Frame, MateFault, Node, Operand, Placed, ProfileDoc, RecipeNodeId,
    RoleSeg, TargetAxis,
};
use fixture::{insert, len, on_frame, solve};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Vec3};

/// The wedge of `placement_values_direction`: a prism on the right
/// triangle `(0,0), (1,0), (0,1)`, a unit tall.
fn wedge(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The name of the wedge's face whose normal runs along `normal`.
fn face_along(doc: &ProfileDoc, wedge: RecipeNodeId, normal: [f64; 3]) -> editor_core::StableName {
    let ev = fixture::run(doc, &EvalOptions::default());
    let n = normal.iter().map(|c| c * c).sum::<f64>().sqrt();
    editor_core::all_faces(&ev, wedge)
        .into_iter()
        .find(|name| {
            editor_core::face_frame(&ev, wedge, name).is_ok_and(|pose| {
                let a = [pose.axis.x, pose.axis.y, pose.axis.z];
                let along: f64 = a.iter().zip(normal).map(|(c, u)| c * u / n).sum();
                (along.abs() - 1.0).abs() < 1e-12
            })
        })
        .unwrap_or_else(|| panic!("the wedge has a face across {normal:?}"))
}

/// The world, a floor tilted `tilt` about `x`, a copy of the wedge
/// standing in the world (whose slanted wall's normal, `(1, 1, 0)/√2`,
/// a value reads), and a second copy rested on the floor by its bottom,
/// slid 0.3 along that read normal and 0.2 along `x`. Answers the
/// document, the floor's placement and the rested copy's.
fn rested(
    tilt: f64,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    editor_core::Mate<editor_core::Formula>,
) {
    let doc = ProfileDoc::empty(
        editor_core::DocumentId::derive("review-placing-dual-6394-r1"),
        Tol::witness(),
    );
    let (doc, w) = wedge(doc);
    let slant = face_along(&doc, w, [1.0, 1.0, 0.0]);
    let (doc, flat, floor) = fixture::floor(
        doc,
        vec![(Coord::Spin(Some(TargetAxis::X)), fixture::ang(tilt))],
    );
    let (doc, standing) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::Node(w))),
    );
    let read = RoleRead::through(
        Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(Operand::output(standing, 0), vec![slant]),
        })),
        PoseRole::Normal,
    );
    let (doc, copy) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::Node(w))),
    );
    let bottom = Operand::select(
        Operand::Node(w),
        vec![fixture::fname(w, RoleSeg::Cap(CapEnd::Start))],
    );
    let rest = fixture::rest(bottom, flat_face(&flat));
    let (doc, _) = fixture::mated(
        doc,
        copy,
        rest.clone(),
        &[
            (Coord::SlideAlong(read), len(0.3)),
            (Coord::slide(TargetAxis::X), len(0.2)),
            (Coord::SPIN, fixture::ang(0.0)),
        ],
    );
    (doc, floor, copy, rest)
}

/// The floor's top face, unwrapped from the plane the fixture reads
/// it as (`fixture::rest` reads a face as its plane itself).
fn flat_face(plane: &Operand) -> Operand {
    match plane {
        Operand::Pose(def) => match def.as_ref() {
            PoseDef::Plane { face } => face.clone(),
            other => panic!("the floor is read as a plane: {other:?}"),
        },
        other => panic!("the floor is read as a plane: {other:?}"),
    }
}

/// Where `placement`'s copy stands, or the fold's fault.
fn at(doc: &ProfileDoc, placement: RecipeNodeId) -> Result<Frame, String> {
    let poses = solve(doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(fault) => Err(format!("{fault:?}")),
        None => poses
            .relative(placement)
            .ok_or_else(|| "unsolved".to_owned()),
    }
}

/// **Claims 2 and 3: a read slide beside a rest sums with a carried
/// slide, and refuses typed once an edit tilts the plane out from under
/// it.** On the level floor the copy stands at
/// `0.3·(1, 1, 0)/√2 + (0.2, 0, 0)`; the floor tilted 0.3 rad about `x`
/// leaves a plane the read normal runs across, and the fold names it
/// `ReadSlidesDoNotSpan` rather than re-reading the direction into the
/// plane.
#[test]
fn r1_a_read_slide_beside_a_rest_sums_then_refuses_once_its_plane_tilts() {
    let (doc, floor, copy, _) = rested(0.0);
    let frame = at(&doc, copy).expect("the level rest places");
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let want = [0.3 * h + 0.2, 0.3 * h, 0.0];
    assert!(
        frame
            .translation
            .iter()
            .zip(want)
            .all(|(g, w)| (g - w).abs() < 1e-12),
        "the copy stands at {:?}, not {want:?}",
        frame.translation
    );
    let tilt = fixture::value_bump(&doc, floor, Coord::Spin(Some(TargetAxis::X)), 0.3);
    let tilted = doc
        .apply(&tilt, Tol::witness())
        .unwrap_or_else(|e| panic!("the floor's tilt is admitted: {e}"))
        .doc;
    let fault = at(&tilted, copy).expect_err("the read runs across the tilted plane");
    assert!(
        fault.contains("ReadSlidesDoNotSpan"),
        "the fault names the read slides: {fault}"
    );
}

/// **Claim 3: the mated slide refuses beside a plane mate**, which
/// fixes it: the door names it.
#[test]
fn r1_the_mated_slide_refuses_beside_a_rest() {
    let (doc, _, copy, rest) = rested(0.0);
    let authored = Placed {
        mates: vec![rest],
        values: vec![
            (Coord::SLIDE, len(0.1)),
            (Coord::slide(TargetAxis::X), len(0.0)),
            (Coord::SPIN, fixture::ang(0.0)),
        ],
    };
    let fault = fixture::bundle_refusal(&doc, copy, authored.bundle());
    assert!(
        matches!(fault, MateFault::Overconstrained { .. }),
        "the mated slide is named: {fault:?}"
    );
}

/// **`read_back` of a chain that turns about a read axis off the
/// origin**: the wedge spun a quarter turn about its vertical edge
/// through `(1, 0)`, its three slides written `0`, stands with its
/// origin at `(1, ±1, 0)`. Read back through the chart that placed it,
/// each slide should read the `0` that places it there; `read_back`
/// answers the copy's origin instead, which written back would stand the
/// copy a second `(1, ±1, 0)` off. RED at `a18ea48` by design.
#[test]
fn r1_read_back_of_a_chain_about_an_off_origin_read_axis_reads_the_written_slides() {
    let doc = ProfileDoc::empty(
        editor_core::DocumentId::derive("review-placing-dual-6394-r1-read-back"),
        Tol::witness(),
    );
    let (doc, w) = wedge(doc);
    let plane = |normal| {
        Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(Operand::Node(w), vec![face_along(&doc, w, normal)]),
        }))
    };
    let edge = Operand::Pose(Box::new(PoseDef::Meet {
        a: RoleRead::plain(plane([0.0, 1.0, 0.0])),
        b: RoleRead::plain(plane([1.0, 1.0, 0.0])),
    }));
    let placed = Placed::unmated(vec![(
        Coord::SpinAbout(RoleRead::plain(edge)),
        fixture::ang(core::f64::consts::FRAC_PI_2),
    )]);
    let coords: Vec<Coord<Operand>> = placed.values.iter().map(|(c, _)| c.clone()).collect();
    let space = fixture::space(&doc, w);
    let (doc, copy) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(w)),
            Operand::Var(space),
            placed.bundle(),
        ),
    );
    let frame = at(&doc, copy).expect("the bundle places");
    let c = frame.columns;
    let affine = Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(c[0][0], c[0][1], c[0][2]),
            Vec3::new(c[1][0], c[1][1], c[1][2]),
            Vec3::new(c[2][0], c[2][1], c[2][2]),
        ),
        Vec3::new(
            frame.translation[0],
            frame.translation[1],
            frame.translation[2],
        ),
    );
    let read = editor_core::mate::read_back(None, &affine, &coords, fixture::band());
    let slides: Vec<Option<f64>> = coords
        .iter()
        .zip(&read)
        .filter(|(c, _)| c.is_slide())
        .map(|(_, r)| *r)
        .collect();
    assert!(
        slides.iter().all(|r| r.is_none_or(|v| v.abs() < 1e-12)),
        "the slides written 0 read back {slides:?}, the copy's origin {:?}",
        frame.translation
    );
}

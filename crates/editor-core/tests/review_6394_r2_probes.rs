//! Review probes (PR 6394, lane placing-dual-6394-r2): a slide along a
//! read direction beside a PLANE mate (the PR's editor-core rows cover
//! only the bundle with no mate), the read's drift out of what the mate
//! leaves, and the mated slide beside a plane mate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, EvalOptions, Frame, MateFault, Node, Operand, Placed, ProfileDoc, RecipeNodeId,
    RoleSeg, TargetAxis,
};
use fixture::{head, insert, len, on_frame, solve};
use geom_core::Tol;

fn prism(doc: ProfileDoc, pts: Vec<(f64, f64)>, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(z),
        },
    )
}

/// The wedge (right triangle (0,0),(1,0),(0,1), a unit tall) and a
/// unit block, in one document.
fn scene() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(
        editor_core::DocumentId::derive("review-6394-r2"),
        Tol::witness(),
    );
    let (doc, wedge) = prism(doc, vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], 1.0);
    let (doc, block) = prism(
        doc,
        vec![(0.0, 0.0), (0.2, 0.0), (0.2, 0.2), (0.0, 0.2)],
        0.2,
    );
    (doc, wedge, block)
}

/// The wedge's face along `normal` (either sense), read as a plane.
fn wedge_plane(doc: &ProfileDoc, wedge: RecipeNodeId, normal: [f64; 3]) -> Operand {
    let ev = fixture::run(doc, &EvalOptions::default());
    let n = normal.iter().map(|c| c * c).sum::<f64>().sqrt();
    let name = editor_core::all_faces(&ev, wedge)
        .into_iter()
        .find(|name| {
            editor_core::face_frame(&ev, wedge, name).is_ok_and(|pose| {
                let a = [pose.axis.x, pose.axis.y, pose.axis.z];
                let along: f64 = a.iter().zip(normal).map(|(c, u)| c * u / n).sum();
                (along.abs() - 1.0).abs() < 1e-12
            })
        })
        .unwrap();
    Operand::Pose(Box::new(PoseDef::Plane {
        face: Operand::select(Operand::Node(wedge), vec![name]),
    }))
}

/// The slanted wall's normal, `(1, 1, 0)/√2`, turned by `tilt` about
/// the wedge's bottom edge along `x` (the meet of its bottom and its
/// `y = 0` wall).
fn slant_tilted(doc: &ProfileDoc, wedge: RecipeNodeId, tilt: f64) -> RoleRead<Operand> {
    let slant = RoleRead::through(wedge_plane(doc, wedge, [1.0, 1.0, 0.0]), PoseRole::Normal);
    if tilt == 0.0 {
        return slant;
    }
    let bottom = fixture::plane(head(fixture::fname(wedge, RoleSeg::Cap(CapEnd::Start))));
    let edge = Operand::Pose(Box::new(PoseDef::Meet {
        a: RoleRead::plain(bottom),
        b: RoleRead::plain(wedge_plane(doc, wedge, [0.0, 1.0, 0.0])),
    }));
    RoleRead::plain(Operand::Pose(Box::new(PoseDef::Spin {
        pose: slant,
        about: RoleRead::plain(edge),
        by: editor_core::test_support::ang(tilt),
    })))
}

/// The block rested by its bottom on the wedge's bottom face (flipped to
/// face up), in the wedge's space, by `values`.
fn rest(
    doc: ProfileDoc,
    wedge: RecipeNodeId,
    block: RecipeNodeId,
    values: Vec<(Coord<Operand>, editor_core::Formula)>,
) -> Result<Frame, String> {
    let space = fixture::space(&doc, wedge);
    let on = fixture::plane(head(fixture::fname(block, RoleSeg::Cap(CapEnd::Start))));
    // Both bottoms face `-z`: the rest equates them, unflipped.
    let to = fixture::plane(head(fixture::fname(wedge, RoleSeg::Cap(CapEnd::Start))));
    let placed = Placed {
        mates: vec![fixture::mate(on, to)],
        values,
    };
    let (doc, placement) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(block)),
            Operand::Var(space),
            placed.bundle(),
        ),
    );
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(fault) => Err(format!("{fault:?}")),
        None => poses.relative(placement).ok_or_else(|| "unsolved".into()),
    }
}

fn values(along: RoleRead<Operand>, by: f64) -> Vec<(Coord<Operand>, editor_core::Formula)> {
    vec![
        (Coord::SlideAlong(along), len(by)),
        (Coord::slide(TargetAxis::X), len(0.0)),
        (
            Coord::SPIN,
            editor_core::Formula::zero(editor_core::Dimension::Angle),
        ),
    ]
}

/// A read direction in the plane a plane mate leaves: the block stands
/// on the plane, its zero at the foot of the target's origin, and steps
/// exactly its value along the read.
#[test]
fn r2_a_read_slide_beside_a_plane_mate_steps_its_value_in_the_plane() {
    let (doc, wedge, block) = scene();
    let along = slant_tilted(&doc, wedge, 0.0);
    let a = rest(doc.clone(), wedge, block, values(along.clone(), 0.3)).expect("places");
    let b = rest(doc, wedge, block, values(along, 0.6)).expect("places");
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let want = [0.3 * h, 0.3 * h, 0.0];
    eprintln!("r2 plane read: a {:?} b {:?}", a.translation, b.translation);
    for i in 0..3 {
        assert!(
            (a.translation[i] - want[i]).abs() < 1e-12,
            "{:?}",
            a.translation
        );
        assert!((b.translation[i] - a.translation[i] - want[i]).abs() < 1e-12);
    }
}

/// The same read turned out of the plane by a tilt: refused typed,
/// never re-read along its shadow — for a gross tilt and a small one.
#[test]
fn r2_a_read_slide_tilted_out_of_the_plane_refuses_typed() {
    for tilt in [0.3, 1e-3, 1e-6] {
        let (doc, wedge, block) = scene();
        let along = slant_tilted(&doc, wedge, tilt);
        let out = rest(doc, wedge, block, values(along, 0.3));
        eprintln!("r2 tilt {tilt}: {out:?}");
        if tilt >= 1e-3 {
            let fault = out.expect_err("a read across the plane refuses");
            assert!(fault.contains("ReadSlidesDoNotSpan"), "{fault}");
        }
    }
}

/// The mated slide beside a plane mate is named at the door.
#[test]
fn r2_the_mated_slide_beside_a_plane_mate_refuses() {
    let (doc, wedge, block) = scene();
    let space = fixture::space(&doc, wedge);
    let (doc, placement) = insert(
        doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(block)),
            Operand::Var(space),
            Placed::identity().bundle(),
        ),
    );
    let on = fixture::plane(head(fixture::fname(block, RoleSeg::Cap(CapEnd::Start))));
    let to = fixture::plane(head(fixture::fname(wedge, RoleSeg::Cap(CapEnd::Start))));
    let bundle = Placed {
        mates: vec![fixture::mate(on, to)],
        values: vec![
            (Coord::SLIDE, len(0.1)),
            (Coord::slide(TargetAxis::X), len(0.0)),
            (Coord::slide(TargetAxis::Y), len(0.0)),
            (
                Coord::SPIN,
                editor_core::Formula::zero(editor_core::Dimension::Angle),
            ),
        ],
    }
    .bundle();
    let fault = fixture::bundle_refusal(&doc, placement, bundle);
    eprintln!("r2 mated slide beside plane: {fault:?}");
    assert!(
        matches!(fault, MateFault::Overconstrained { .. }),
        "{fault:?}"
    );
}

/// A read of the wrong kind at a value's seat: a plane read plainly at
/// a slide's direction, a frame's origin at it, and a direction at a
/// spin's axis. Each must be refused (at the door or typed at the fold),
/// never reach the fold's `unreachable!` in `heading_of`.
#[test]
fn r2_a_read_of_the_wrong_kind_at_a_values_seat_is_refused_not_a_panic() {
    let cases: Vec<(
        &str,
        Box<dyn Fn(&ProfileDoc, RecipeNodeId) -> Coord<Operand>>,
    )> = vec![
        (
            "plain plane at Along",
            Box::new(|doc, w| {
                Coord::SlideAlong(RoleRead::plain(wedge_plane(doc, w, [1.0, 1.0, 0.0])))
            }),
        ),
        (
            "plane's origin role at Along",
            Box::new(|doc, w| {
                Coord::SlideAlong(RoleRead::through(
                    wedge_plane(doc, w, [1.0, 1.0, 0.0]),
                    PoseRole::Origin,
                ))
            }),
        ),
        (
            "direction at About",
            Box::new(|doc, w| {
                Coord::SpinAbout(RoleRead::through(
                    wedge_plane(doc, w, [1.0, 1.0, 0.0]),
                    PoseRole::Normal,
                ))
            }),
        ),
    ];
    for (label, coord) in cases {
        let (doc, wedge, block) = scene();
        let space = fixture::space(&doc, wedge);
        let c = coord(&doc, wedge);
        let is_slide = c.is_slide();
        let mut moves = vec![(
            c,
            if is_slide {
                len(0.1)
            } else {
                editor_core::test_support::ang(0.1)
            },
        )];
        if is_slide {
            moves.push((Coord::slide(TargetAxis::Z), len(0.0)));
        }
        let placed = Placed::unmated(moves);
        let res = editor_core::apply(
            &doc,
            &editor_core::DocEdit::InsertNode {
                node: Box::new(Node::place(
                    editor_core::BodyRead::plain(Operand::Node(block)),
                    Operand::Var(space),
                    placed.bundle(),
                )),
                fresh: Vec::new(),
            },
            Tol::witness(),
        );
        match res {
            Err(e) => eprintln!("r2 wrong kind [{label}]: door refused: {e}"),
            Ok(applied) => {
                let poses = solve(&applied.doc, &EvalOptions::default(), Tol::witness());
                let p = *applied.doc.placements().last().unwrap();
                eprintln!(
                    "r2 wrong kind [{label}]: admitted; fold: {:?}",
                    poses.fault(p)
                );
            }
        }
    }
}

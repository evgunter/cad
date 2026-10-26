//! Review 2's probes of PR 5742 (the value chart reads a written
//! orientation). Each prints what it saw; the asserts are the claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocumentId, EvalOptions, Formula, Frame, MateFault, Node, Operand, Orientation,
    Placed, ProfileDoc, RecipeNodeId, RoleSeg, SignedAxis, StableName, TargetAxis,
};
use fixture::{head, head_at, insert, len, on_frame, solve};
use geom_core::Tol;

fn block(doc: ProfileDoc, x: f64, y: f64, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (x, 0.0), (x, y), (0.0, y)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(z),
        },
    )
}

fn face(doc: &ProfileDoc, body: RecipeNodeId, at: [f64; 3], normal: [f64; 3]) -> StableName {
    let read = editor_core::test_support::plane_on(doc, body, at, normal, Tol::witness()).read;
    let Operand::Pose(def) = read else {
        panic!("a plane read: {read:?}")
    };
    let PoseDef::Plane {
        face: Operand::Select { names, .. },
    } = *def
    else {
        panic!("a plane of one face")
    };
    names[0].clone()
}

fn ori(x: SignedAxis, z: SignedAxis) -> Orientation {
    Orientation::new(x, z).unwrap()
}

type Out = Result<Frame, String>;

fn run(doc: ProfileDoc, placement: RecipeNodeId, placed: Placed) -> Out {
    let mut doc = doc;
    for edit in fixture::bundle_edits(&doc, placement, placed.bundle()) {
        match editor_core::apply(&doc, &edit, Tol::witness()) {
            Ok(a) => doc = a.doc,
            Err(e) => return Err(format!("door: {e}")),
        }
    }
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(f) => Err(format!("fault: {f} [{f:?}]")),
        None => poses
            .placement(&doc, placement)
            .map_err(|e| format!("unplaced: {e:?}")),
    }
}

fn hung_fault(chain: Vec<(Coord, Formula)>, orient: Orientation) -> Result<Frame, MateFault> {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-hung"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let bottom = face(&doc, plate, [10.0, 5.0, 0.0], [0.0, 0.0, -1.0]);
    let (doc, b) = fixture::in_world_by(doc, plate, Placed::unmated(chain).values);
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(boss))),
    );
    let top = fixture::fname(boss, RoleSeg::Cap(CapEnd::End));
    let mut mate = fixture::rest(head(top), head_at(b, bottom));
    mate.orient = orient;
    let placed = Placed {
        mates: vec![mate],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(10.0)),
            (Coord::Slide(TargetAxis::Y), len(0.0)),
            (Coord::SPIN, fixture::ang(0.0)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(f) => Err(f.clone()),
        None => Ok(poses.placement(&doc, placement).expect("placed")),
    }
}

fn spin(axis: TargetAxis, t: f64) -> (Coord, Formula) {
    (Coord::Spin(Some(axis)), fixture::ang(t))
}

/// Band edges about the pole, and the copy near (outside) it.
#[test]
fn r2_band_edges_and_the_copy_just_outside() {
    let backwards = ori(SignedAxis::PlusX, SignedAxis::MinusZ);
    let mut first_placed = None;
    for about in [TargetAxis::X, TargetAxis::Y] {
        let mut placed_once = false;
        for e in -14..=-1 {
            let tilt = 10f64.powi(e);
            let got = hung_fault(vec![spin(about, tilt)], backwards);
            match &got {
                Ok(f) => {
                    println!("{about:?} tilt 1e{e}: placed at {:?}", f.translation);
                    placed_once = true;
                    if e == -6 {
                        first_placed.get_or_insert_with(Vec::new).push((about, f.clone()));
                    }
                }
                Err(MateFault::Backwards { write, .. }) => {
                    println!("{about:?} tilt 1e{e}: Backwards, write {write}");
                    assert!(!placed_once, "{about:?} 1e{e}: a refusal past a placing tilt");
                    assert_eq!(*write, Orientation::IDENTITY);
                }
                Err(other) => panic!("{about:?} 1e{e}: {other:?}"),
            }
        }
    }
    let rows = first_placed.unwrap();
    let (a, b) = (&rows[0].1, &rows[1].1);
    println!("1e-6 about X: {:?}\n1e-6 about Y: {:?}", a.translation, b.translation);
}

/// A near-pole target on no axis (a plate turned not quite a half turn
/// about x, then about z), and a generic tilted target far from it.
#[test]
fn r2_pole_on_a_tilted_non_axis_target() {
    for eps in [0.0, 1e-13, 1e-11] {
        let chain = vec![
            spin(TargetAxis::X, std::f64::consts::PI - eps),
            spin(TargetAxis::Z, 0.7),
        ];
        let got = hung_fault(chain.clone(), Orientation::IDENTITY);
        match got {
            Err(MateFault::Backwards { write, .. }) => {
                println!("eps {eps:e}: Backwards, write {write}");
                let again = hung_fault(chain, write);
                println!("  written: {again:?}");
                assert!(again.is_ok(), "eps {eps:e}: the named orientation places");
            }
            other => println!("eps {eps:e}: {other:?}"),
        }
    }
    for chain in [
        vec![spin(TargetAxis::X, 2.0), spin(TargetAxis::Y, 1.0)],
        vec![spin(TargetAxis::Y, 2.9), spin(TargetAxis::Z, 0.3)],
        vec![spin(TargetAxis::X, 3.0)],
    ] {
        let got = hung_fault(chain.clone(), Orientation::IDENTITY);
        println!("generic {:?}: {:?}", chain.len(), got.as_ref().map(|f| f.translation));
        assert!(got.is_ok(), "a target short of the pole places: {got:?}");
    }
}

/// The rest spelled every way, at one Spin value, on a level and a
/// tilted floor.
#[test]
fn r2_rest_spellings() {
    let s = 0.5_f64;
    let up = ori(SignedAxis::PlusX, SignedAxis::MinusZ);
    let at = |tilt: f64, how: &str, orient: Orientation| -> Out {
        let doc = ProfileDoc::empty(DocumentId::derive("r2-spell"), Tol::witness());
        let (doc, body) = block(doc, 1.0, 1.0, 1.0);
        let (doc, floor, _) = fixture::floor(doc, vec![spin(TargetAxis::X, tilt)]);
        let (doc, placement) = insert(
            doc,
            fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
        );
        let bottom = fixture::plane(head(fixture::fname(body, RoleSeg::Cap(CapEnd::Start))));
        let slid = |p: Operand| {
            Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(p.clone()),
                along: RoleRead::through(p, PoseRole::Normal),
                by: len(0.0),
            }))
        };
        let mut mate = match how {
            "copy" => fixture::mate(fixture::flip(bottom), floor),
            "target" => fixture::mate(bottom, fixture::flip(floor)),
            "target-slid" => fixture::mate(bottom, slid(fixture::flip(floor))),
            "copy-slid" => fixture::mate(slid(fixture::flip(bottom)), floor),
            "both" => fixture::mate(fixture::flip(bottom), fixture::flip(floor)),
            "dir-target" => {
                let n = |p: Operand| {
                    Operand::Pose(Box::new(PoseDef::Flip {
                        pose: RoleRead::through(p, PoseRole::Normal),
                    }))
                };
                fixture::mate(
                    Operand::Pose(Box::new(PoseDef::Slide {
                        pose: RoleRead::through(bottom.clone(), PoseRole::Normal),
                        along: RoleRead::through(bottom, PoseRole::Normal),
                        by: len(0.0),
                    })),
                    n(floor),
                )
            }
            _ => unreachable!(),
        };
        mate.orient = orient;
        let mut values = vec![
            (Coord::Slide(TargetAxis::X), len(3.0)),
            (Coord::Slide(TargetAxis::Y), len(4.0)),
            (Coord::SPIN, fixture::ang(s)),
        ];
        if how.starts_with("dir") {
            values.push((Coord::Slide(TargetAxis::Z), len(0.0)));
        }
        run(
            doc,
            placement,
            Placed {
                mates: vec![mate],
                values,
            },
        )
    };
    for tilt in [0.0, 0.3] {
        let base = at(tilt, "copy", Orientation::IDENTITY).unwrap();
        println!("tilt {tilt}: copy-side {base:?}");
        if tilt == 0.0 {
            let x = base.columns[0];
            assert!((x[0] - s.cos()).abs() < 1e-12 && (x[1] - s.sin()).abs() < 1e-12);
        }
        for (how, o) in [
            ("target", Orientation::IDENTITY),
            ("target-slid", Orientation::IDENTITY),
            ("copy-slid", Orientation::IDENTITY),
            ("both", Orientation::IDENTITY),
            ("both", up),
            ("dir-target", Orientation::IDENTITY),
        ] {
            let got = at(tilt, how, o);
            match &got {
                Ok(f) => {
                    let d = (0..3)
                        .map(|i| (f.translation[i] - base.translation[i]).abs())
                        .chain((0..3).flat_map(|c| {
                            (0..3).map(move |r| (f.columns[c][r] - base.columns[c][r]).abs())
                        }))
                        .fold(0.0, f64::max);
                    println!("tilt {tilt} {how} {o}: off the copy-side pose by {d:e}; x = {:?}", f.columns[0]);
                }
                Err(e) => println!("tilt {tilt} {how} {o}: {e}"),
            }
        }
    }
}

/// A rest against a wall (target normal +x) written through the
/// helpers' constructor, which fills the slides it leaves in axis order.
#[test]
fn r2_a_rest_against_a_wall_through_the_constructor() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-wall"), Tol::witness());
    let (doc, body) = block(doc, 1.0, 1.0, 1.0);
    let (doc, wall) = block(doc, 2.0, 10.0, 10.0);
    let wall_face = face(&doc, wall, [2.0, 5.0, 5.0], [1.0, 0.0, 0.0]);
    let (doc, w) = fixture::in_world_by(doc, wall, Placed::unmated(Vec::new()).values);
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
    );
    let side = face(&doc, body, [0.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    for orient in [
        Orientation::IDENTITY,
        ori(SignedAxis::PlusY, SignedAxis::PlusZ),
        ori(SignedAxis::PlusZ, SignedAxis::PlusX),
    ] {
        let mut mate = fixture::rest(head(side.clone()), head_at(w, wall_face.clone()));
        mate.orient = orient;
        let placed = Placed::of(&doc, vec![mate.clone()], Vec::new());
        println!(
            "{orient}: constructor wrote {:?}",
            placed.values.iter().map(|(c, _)| *c).collect::<Vec<_>>()
        );
        println!("   -> {:?}", run(doc.clone(), placement, placed));
        let explicit = Placed::of(
            &doc,
            vec![mate],
            vec![
                (Coord::Slide(TargetAxis::Y), len(0.0)),
                (Coord::Slide(TargetAxis::Z), len(0.0)),
            ],
        );
        println!("   y,z written -> {:?}", run(doc.clone(), placement, explicit));
    }
}

/// The same wall rest added to a stood copy through `DocEdit::AddConstraint`.
#[test]
fn r2_a_rest_against_a_wall_through_add_constraint() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-wall-add"), Tol::witness());
    let (doc, body) = block(doc, 1.0, 1.0, 1.0);
    let (doc, wall) = block(doc, 2.0, 10.0, 10.0);
    let wall_face = face(&doc, wall, [2.0, 5.0, 5.0], [1.0, 0.0, 0.0]);
    let (doc, w) = fixture::in_world_by(doc, wall, Placed::unmated(Vec::new()).values);
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
    );
    let side = face(&doc, body, [0.0, 0.5, 0.5], [-1.0, 0.0, 0.0]);
    let mate = fixture::rest(head(side), head_at(w, wall_face));
    let edit = editor_core::DocEdit::AddConstraint {
        placement,
        mate: Box::new(mate),
        fresh: Vec::new(),
    };
    let doc = match editor_core::apply(&doc, &edit, Tol::witness()) {
        Ok(a) => a.doc,
        Err(e) => panic!("door: {e}"),
    };
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    println!("add_constraint on a wall: {:?}", poses.fault(placement).map(ToString::to_string));
}

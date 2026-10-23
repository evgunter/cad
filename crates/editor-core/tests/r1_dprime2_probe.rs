//! R1 probe (review of D′2): the space chart near its antipode.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::PoseDef;
use editor_core::{
    BodyRead, CapEnd, ContactClass, Coord, DocumentId, EvalOptions, Node, Operand, Placed,
    ProfileDoc, RecipeNodeId, RoleSeg, StableName, TargetAxis,
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

/// A boss hung under the plate's bottom face, the plate turned `tilt`
/// about `axis`; `Slide(X) = 10`.
fn hung(axis: TargetAxis, tilt: f64, top: bool) -> Result<editor_core::Frame, String> {
    let doc = ProfileDoc::empty(DocumentId::derive("r1-antipode"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let side = if top {
        face(&doc, plate, [10.0, 5.0, 2.0], [0.0, 0.0, 1.0])
    } else {
        fixture::fname(plate, RoleSeg::Cap(CapEnd::Start))
    };
    let (doc, b) = fixture::in_world_by(
        doc,
        plate,
        Placed::unmated(vec![(Coord::Spin(axis), fixture::ang(tilt))]).values,
    );
    let target = head_at(b, side.in_copy(b));
    let (doc, placement) = insert(
        doc,
        fixture::stood(vec![BodyRead::plain(Operand::from(boss))]),
    );
    let cap = if top { CapEnd::Start } else { CapEnd::End };
    let start = fixture::fname(boss, RoleSeg::Cap(cap));
    let placed = Placed {
        mates: vec![fixture::rest(head(start), target, ContactClass::Rest)],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(10.0)),
            (Coord::Slide(TargetAxis::Y), len(0.0)),
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.0)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    if let Some(f) = poses.fault(placement) {
        return Err(format!("{f:?}"));
    }
    Ok(poses.placement(&doc, placement).expect("placed"))
}

#[test]
fn r1_probe_chart_near_its_antipode() {
    for top in [true, false] {
        for axis in [TargetAxis::X, TargetAxis::Y] {
            for tilt in [0.0, 1e-12, 1e-9, 1e-6, 1e-3, -1e-3, 0.1] {
                match hung(axis, tilt, top) {
                    Ok(f) => eprintln!(
                        "R1PROBE top={top} {axis:?} {tilt:e}: t={:?} x-col={:?}",
                        f.translation, f.columns[0]
                    ),
                    Err(e) => eprintln!("R1PROBE top={top} {axis:?} {tilt:e}: FAULT {e}"),
                }
            }
        }
    }
}

#[test]
fn r1_probe_old_wire_loads() {
    // Point R1_OLD at a directory of main-era documents (`git show
    // 5e22babe:<path>`) and hand-mutated new ones; absent, nothing runs.
    let Ok(dir) = std::env::var("R1_OLD") else {
        return;
    };
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    names.sort();
    for p in names {
        let text = std::fs::read_to_string(&p).unwrap();
        match editor_core::persist::load(&text, Tol::witness()) {
            Ok(_) => eprintln!("R1LOAD {}: LOADED", p.display()),
            Err(e) => eprintln!(
                "R1LOAD {}: refused {}",
                p.display(),
                format!("{e}").chars().take(300).collect::<String>()
            ),
        }
    }
}

fn coords(doc: &ProfileDoc, p: RecipeNodeId) -> Vec<String> {
    let Some(Node::Place { constraints, .. }) = doc.node(p) else {
        panic!("place")
    };
    constraints
        .iter()
        .map(|c| match c.value() {
            Some(v) => format!(
                "{:?}={:?}",
                v.coord,
                doc.var_name(v.by).map(|n| n.to_string())
            ),
            None => "mate".to_string(),
        })
        .collect()
}

/// Claim 4: an unmated copy whose spin about x reads the named `tilt`;
/// AddConstraint a rest, then RemoveConstraint it.
#[test]
fn r1_probe_rebundle_round_trip() {
    use editor_core::{Dimension, DocEdit, Formula, FreeVar, VarName};
    let doc = ProfileDoc::empty(DocumentId::derive("r1-rebundle"), Tol::witness());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("tilt"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Angle, 0.4)),
        },
    );
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let top = face(&doc, plate, [10.0, 5.0, 2.0], [0.0, 0.0, 1.0]);
    let (doc, b) = fixture::in_world_by(doc, plate, Placed::unmated(vec![]).values);
    let (doc, c) = fixture::in_world_by(
        doc,
        boss,
        vec![
            (Coord::Slide(TargetAxis::X), len(3.0)),
            (Coord::Slide(TargetAxis::Y), len(4.0)),
            (Coord::Slide(TargetAxis::Z), len(7.0)),
            (
                Coord::Spin(TargetAxis::X),
                Formula::named(VarName::from_static("tilt"), Dimension::Angle),
            ),
            (Coord::Spin(TargetAxis::Y), fixture::ang(0.0)),
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.2)),
        ],
    );
    eprintln!("R1RB before: {:?}", coords(&doc, c));
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
    let added = editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: c,
            mate: Box::new(fixture::rest(
                head(start),
                head_at(b, top.in_copy(b)),
                ContactClass::Rest,
            )),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("add");
    let doc2 = added.doc;
    eprintln!(
        "R1RB added: {:?} record minted={:?}",
        coords(&doc2, c),
        added.record.minted_constraints
    );
    eprintln!("R1RB tilt still declared: {:?}", doc2.var_named("tilt"));
    let Some(Node::Place { constraints, .. }) = doc2.node(c) else {
        panic!()
    };
    let mate = constraints.iter().find(|k| k.value().is_none()).unwrap().id;
    let removed = editor_core::apply(
        &doc2,
        &DocEdit::RemoveConstraint {
            placement: c,
            constraint: mate,
        },
        Tol::witness(),
    )
    .expect("remove");
    eprintln!("R1RB removed: {:?}", coords(&removed.doc, c));
    let p0 = solve(&doc, &EvalOptions::default(), Tol::witness()).placement(&doc, c);
    let p1 = solve(&doc2, &EvalOptions::default(), Tol::witness()).placement(&doc2, c);
    let p2 =
        solve(&removed.doc, &EvalOptions::default(), Tol::witness()).placement(&removed.doc, c);
    eprintln!("R1RB poses:\n {p0:?}\n {p1:?}\n {p2:?}");
}

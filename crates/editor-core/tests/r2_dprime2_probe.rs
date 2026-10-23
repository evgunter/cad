//! R2 review probes for D′2 (PR 5613). Printing probes: run with
//! `--nocapture` and read the output; the asserts mark what the review
//! claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    BodyRead, CapEnd, ContactClass, Coord, DocEdit, DocumentId, EvalOptions, Node, Operand,
    Placed, ProfileDoc, RecipeNodeId, RoleSeg, TargetAxis,
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

/// A boss hung under the bottom face (outward normal −z) of a plate
/// copy `B` placed by `moves`, `Slide(X) = 10`, the rest 0.
fn hung(moves: Vec<(Coord, editor_core::Formula)>) -> Result<[f64; 3], String> {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-antipode"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let bottom = fixture::fname(plate, RoleSeg::Cap(CapEnd::Start));
    let (doc, b) = fixture::in_world_by(doc, plate, Placed::unmated(moves).values);
    let target = head_at(b, bottom.in_copy(b));
    let (doc, placement) = insert(
        doc,
        fixture::stood(vec![BodyRead::plain(Operand::from(boss))]),
    );
    let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
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
    Ok(poses.placement(&doc, placement).unwrap().translation)
}

/// Claim 2: the space chart near the antipode. A boss hung under a
/// plate, the plate tilted by a hair about `x` or about `y`.
#[test]
fn r2_antipode_sensitivity() {
    for eps in [0.0, 1e-9, 1e-6, 1e-3, 1e-2] {
        for axis in [TargetAxis::X, TargetAxis::Y] {
            let got = hung(vec![(Coord::Spin(axis), fixture::ang(eps))]);
            println!("tilt {eps:e} about {axis:?}: boss at {got:?}");
        }
    }
}

fn values(doc: &ProfileDoc, placement: RecipeNodeId) -> Vec<String> {
    fixture::bundle(doc, placement)
        .iter()
        .map(|c| match c.value() {
            Some(v) => format!("{:?}={:?}", v.coord, fixture::read_var(doc, v.by)),
            None => "mate".to_string(),
        })
        .collect()
}

/// Claim 4: a written chain under AddConstraint then RemoveConstraint.
#[test]
fn r2_rebundle_keeps_what() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-rebundle"), Tol::witness());
    let (doc, floor) = block(doc, 20.0, 20.0, 1.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let top = fixture::fname(floor, RoleSeg::Cap(CapEnd::End));
    let (doc, f) = fixture::in_world_by(doc, floor, Placed::unmated(vec![]).values);
    for chain in [
        vec![
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.1)),
            (Coord::Spin(TargetAxis::X), fixture::ang(0.2)),
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.3)),
        ],
        vec![
            (Coord::Spin(TargetAxis::X), fixture::ang(0.2)),
            (Coord::Spin(TargetAxis::Z), fixture::ang(0.3)),
        ],
    ] {
        let mut moves = chain;
        moves.push((Coord::Slide(TargetAxis::X), len(3.0)));
        moves.push((Coord::Slide(TargetAxis::Z), len(7.0)));
        let (doc, copy) = fixture::in_world_by(doc.clone(), boss, Placed::unmated(moves).values);
        println!("before: {:?}", values(&doc, copy));
        let start = fixture::fname(boss, RoleSeg::Cap(CapEnd::Start));
        let mate = fixture::rest(head(start), head_at(f, top.in_copy(f)), ContactClass::Rest);
        let applied = doc
            .apply(
                &DocEdit::AddConstraint {
                    placement: copy,
                    mate: Box::new(mate),
                    fresh: Vec::new(),
                },
                Tol::witness(),
            )
            .map_err(|e| format!("{e}"));
        let Ok(applied) = applied else {
            println!("add refused: {applied:?}");
            continue;
        };
        let doc = applied.doc;
        println!("mated: {:?}", values(&doc, copy));
        let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
        println!("  fault {:?} at {:?}", poses.fault(copy), poses.placement(&doc, copy).map(|p| p.translation));
        let mate_id = fixture::bundle(&doc, copy)
            .iter()
            .find(|c| c.mate().is_some())
            .unwrap()
            .id;
        let doc = doc
            .apply(
                &DocEdit::RemoveConstraint {
                    placement: copy,
                    constraint: mate_id,
                },
                Tol::witness(),
            )
            .unwrap()
            .doc;
        println!("unmated: {:?}", values(&doc, copy));
    }
}

/// Claim 7: main's committed documents against this head's load door.
#[test]
fn r2_mains_documents_load() {
    let Ok(dir) = std::env::var("R2_MAIN_DOCS") else {
        return;
    };
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let text = std::fs::read_to_string(&path).unwrap();
        let got = editor_core::persist::load(&text, Tol::witness());
        println!(
            "{}: {}",
            path.display(),
            match got {
                Ok(_) => "LOADED".to_string(),
                Err(e) => format!("refused: {e}"),
            }
        );
    }
}

/// Claim 4: `RemoveConstraint` of the middle link of a `Z, X, Z` chain.
/// Writing that link back to 0 means `Rz(c)·Rz(a) = Rz(a + c)`; the
/// door's rewrite must not drop `c`.
#[test]
fn r2_removing_a_chains_middle_link() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-middle"), Tol::witness());
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let (a, b, c) = (0.1, 0.2, 0.3);
    let moves = vec![
        (Coord::Spin(TargetAxis::Z), fixture::ang(a)),
        (Coord::Spin(TargetAxis::X), fixture::ang(b)),
        (Coord::Spin(TargetAxis::Z), fixture::ang(c)),
    ];
    let (doc, copy) = fixture::in_world_by(doc, boss, Placed::unmated(moves).values);
    let middle = fixture::bundle(&doc, copy)[1].id;
    let doc = doc
        .apply(
            &DocEdit::RemoveConstraint {
                placement: copy,
                constraint: middle,
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    println!("after: {:?}", values(&doc, copy));
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    let got = poses.placement(&doc, copy).unwrap();
    let want = a + c;
    let x = got.columns[0];
    let turned = x[1].atan2(x[0]);
    println!("x column {x:?}: turned {turned} about z; the link at 0 means {want}");
    assert!(
        (turned - want).abs() < 1e-12,
        "the third link survives: turned {turned}, want {want}"
    );
}

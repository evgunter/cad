//! R2 review probes for PR 4823 (`flush/touch-key`): outcomes printed,
//! run on base and head and diffed.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanInputs, BooleanOp, BooleanResult, CarriedRecord, OperandRecords, VvContact};

fn tol() -> Tol {
    Tol::witness()
}

fn bx(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("r2", brick(x, y, z, tol()), tol())
}

fn vertex_at(body: &Body<f64>, at: [f64; 3]) -> topo::VertexKey {
    body.vertex_points()
        .find(|(_, p)| [p.x, p.y, p.z] == at)
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("a vertex at {at:?}"))
}

fn show(tag: &str, r: &Result<BooleanResult<f64>, topo::BooleanError>) {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let mut pts: Vec<[f64; 3]> = b.body.vertex_points().map(|(_, p)| [p.x, p.y, p.z]).collect();
            pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let keys: std::collections::BTreeSet<_> = b.body.vertices().map(|(_, v)| v.point).collect();
            println!(
                "R2 {tag}: OK verts={} keys={} contacts={} pts={pts:?}",
                b.body.vertices().count(),
                keys.len(),
                b.contacts.len()
            );
        }
        Ok(BooleanResult::Empty) => println!("R2 {tag}: EMPTY"),
        Err(e) => println!("R2 {tag}: ERR {:?}", e.kind()),
    }
}

fn op(o: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>, i: &BooleanInputs) -> Result<BooleanResult<f64>, topo::BooleanError> {
    topo::boolean_op_with(o, a, b, i, topo::SweepStrategy::Realized, tol())
}

fn bogus(a: &AtRestBody<f64>, p: [f64; 3], q: [f64; 3]) -> BooleanInputs {
    BooleanInputs {
        records_a: OperandRecords {
            vv: vec![CarriedRecord {
                contact: VvContact { a: vertex_at(a, p), b: vertex_at(a, q) },
                record: 0,
            }],
            ..OperandRecords::default()
        },
        ..BooleanInputs::default()
    }
}

/// A carried vertex-vertex record between two corners a unit apart:
/// no decision backs it. Disjoint (assembly), touching (seamed), and
/// overlapping operands.
#[test]
fn r2_a_carried_record_between_distant_corners() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let far = bx((5.0, 6.0), (5.0, 6.0), (5.0, 6.0));
    let over = bx((0.5, 2.0), (0.5, 2.0), (0.5, 2.0));
    let face = bx((1.0, 2.0), (0.0, 1.0), (0.0, 1.0));
    for (tag, p, q) in [
        ("edge", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        ("diag", [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
    ] {
        let i = bogus(&a, p, q);
        for o in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            show(&format!("bogus {tag} far {o:?}"), &op(o, &a, &far, &i));
            show(&format!("bogus {tag} over {o:?}"), &op(o, &a, &over, &i));
            show(&format!("bogus {tag} face {o:?}"), &op(o, &a, &face, &i));
        }
    }
}

/// Records dropped along a chain of plain booleans: kiss at (1,1,1),
/// then a third cube touching at (0,0,0), at (1,1,1) and along an edge
/// through (1,1,1).
#[test]
fn r2_a_chain_that_drops_its_records() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = bx((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let kiss = match op(BooleanOp::Union, &a, &b, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    println!("R2 kiss: verts={} keys={} contacts={}", kiss.body.vertices().count(), kiss.body.vertices().map(|(_, v)| v.point).collect::<std::collections::BTreeSet<_>>().len(), kiss.contacts.len());
    let thirds = [
        ("corner000", bx((-1.0, 0.0), (-1.0, 0.0), (-1.0, 0.0))),
        ("pierce111", bx((0.5, 1.5), (0.5, 1.5), (0.5, 1.5))),
        ("edge111", bx((1.0, 2.0), (0.0, 1.0), (1.0, 2.0))),
        ("face", bx((-1.0, 3.0), (-1.0, 3.0), (1.0, 1.5))),
        ("over_b", bx((1.5, 2.5), (1.5, 2.5), (1.5, 2.5))),
    ];
    for (tag, c) in &thirds {
        for o in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            show(&format!("drop {tag} {o:?}"), &op(o, &kiss.body, c, &BooleanInputs::default()));
            let carried = BooleanInputs { records_a: kiss.contacts.carried(), ..BooleanInputs::default() };
            show(&format!("keep {tag} {o:?}"), &op(o, &kiss.body, c, &carried));
            show(&format!("dropB {tag} {o:?}"), &op(o, c, &kiss.body, &BooleanInputs::default()));
        }
    }
}

/// The offset door solves "the moved corners on one point" once
/// (`replace_face::group_by_point`) and states that two solids share no
/// vertex. Offset every face of a corner kiss's two solids together.
#[test]
fn r2_offset_both_solids_of_a_kiss() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = bx((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let kiss = match op(BooleanOp::Union, &a, &b, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    let band = geom_core::Band::linear(tol()).unwrap();
    for d in [-0.1, 0.1] {
        let mut body = Body::clone(&kiss.body);
        let moves: Vec<topo::ChartMove<f64>> = body
            .faces()
            .map(|(f, _)| topo::ChartMove { faces: vec![f], distance: d })
            .collect();
        match topo::offset_planes_together(&mut body, &moves, band, tol()) {
            Ok(_) => {
                let keys: std::collections::BTreeSet<_> = body.vertices().map(|(_, v)| v.point).collect();
                let v = topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>());
                println!("R2 offset_together {d}: OK verts={} keys={} at_rest={v:?}", body.vertices().count(), keys.len());
            }
            Err(e) => println!("R2 offset_together {d}: ERR {}", format!("{e:?}").chars().take(300).collect::<String>()),
        }
        let mut body = Body::clone(&kiss.body);
        let moves: Vec<topo::ChartMove<f64>> = body
            .faces()
            .map(|(f, _)| topo::ChartMove { faces: vec![f], distance: d })
            .collect();
        match topo::offset_charts_together(&mut body, &moves, band, tol()) {
            Ok(_) => println!("R2 offset_charts {d}: OK verts={}", body.vertices().count()),
            Err(e) => println!("R2 offset_charts {d}: ERR {}", format!("{e:?}").chars().take(300).collect::<String>()),
        }
    }
    for t in [0.1, -0.1] {
        match topo::shell(&kiss.body, t, tol()) {
            Ok(s) => println!("R2 shell {t}: OK verts={}", s.body.vertices().count()),
            Err(e) => println!("R2 shell {t}: ERR {}", format!("{e:?}").chars().take(300).collect::<String>()),
        }
    }
}

/// Other doors over a corner kiss's shared key: the general offset of
/// every face, `shell_open` through each face in turn, and a single
/// face offset of each face.
#[test]
fn r2_other_doors_over_a_kiss() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = bx((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let kiss = match op(BooleanOp::Union, &a, &b, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    let band = geom_core::Band::linear(tol()).unwrap();
    for d in [-0.1, 0.1] {
        let mut body = Body::clone(&kiss.body);
        let moves: Vec<topo::ChartMove<f64>> = body
            .faces()
            .map(|(f, _)| topo::ChartMove { faces: vec![f], distance: d })
            .collect();
        match topo::offset_surfaces_together(&mut body, &moves, band, tol()) {
            Ok(_) => println!("R2 offset_general {d}: OK verts={}", body.vertices().count()),
            Err(e) => println!("R2 offset_general {d}: ERR {}", format!("{e:?}").chars().take(160).collect::<String>()),
        }
    }
    let faces: Vec<topo::FaceKey> = kiss.body.faces().map(|(f, _)| f).collect();
    for (n, f) in faces.iter().enumerate() {
        match topo::shell_open(&kiss.body, 0.1, &[*f], tol()) {
            Ok(s) => println!("R2 shell_open face#{n}: OK verts={}", s.body.vertices().count()),
            Err(e) => println!("R2 shell_open face#{n}: ERR {}", format!("{e:?}").chars().take(160).collect::<String>()),
        }
        for d in [-0.1, 0.1] {
            let mut body = Body::clone(&kiss.body);
            match topo::replace_face_offset(&mut body, *f, d, tol()) {
                Ok(_) => println!("R2 replace_face_offset face#{n} {d}: OK verts={}", body.vertices().count()),
                Err(e) => println!("R2 replace_face_offset face#{n} {d}: ERR {}", format!("{e:?}").chars().take(160).collect::<String>()),
            }
        }
    }
}

/// The chain's tier-3′ verdicts with each result's own records.
#[test]
fn r2_chain_t3p() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = bx((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let kiss = match op(BooleanOp::Union, &a, &b, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    let thirds = [
        ("corner000", bx((-1.0, 0.0), (-1.0, 0.0), (-1.0, 0.0))),
        ("pierce111", bx((0.5, 1.5), (0.5, 1.5), (0.5, 1.5))),
        ("edge111", bx((1.0, 2.0), (0.0, 1.0), (1.0, 2.0))),
        ("face", bx((-1.0, 3.0), (-1.0, 3.0), (1.0, 1.5))),
        ("over_b", bx((1.5, 2.5), (1.5, 2.5), (1.5, 2.5))),
    ];
    let t3 = |r: &Result<BooleanResult<f64>, topo::BooleanError>| match r {
        Ok(BooleanResult::Body(bb)) => match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
            Ok(()) => "t3p=ok".to_string(),
            Err(es) => format!("t3p={:?}", es.iter().map(|e| format!("{e:?}").split([' ', '{', '(']).next().unwrap().to_string()).collect::<Vec<_>>()),
        },
        Ok(BooleanResult::Empty) => "EMPTY".into(),
        Err(e) => format!("ERR {:?}", e.kind()),
    };
    for (tag, c) in &thirds {
        for o in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let carried = BooleanInputs { records_a: kiss.contacts.carried(), ..BooleanInputs::default() };
            println!("R2T drop {tag} {o:?}: {}", t3(&op(o, &kiss.body, c, &BooleanInputs::default())));
            println!("R2T keep {tag} {o:?}: {}", t3(&op(o, &kiss.body, c, &carried)));
            println!("R2T dropB {tag} {o:?}: {}", t3(&op(o, c, &kiss.body, &BooleanInputs::default())));
        }
    }
}

/// `shell_open` through two faces at once, one of each solid, every
/// pair.
#[test]
fn r2_shell_open_pairs_over_a_kiss() {
    let a = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = bx((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let kiss = match op(BooleanOp::Union, &a, &b, &BooleanInputs::default()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    let faces: Vec<topo::FaceKey> = kiss.body.faces().map(|(f, _)| f).collect();
    let (mut ok, mut err) = (0, Vec::new());
    for (i, f) in faces.iter().enumerate() {
        for (j, g) in faces.iter().enumerate().skip(i + 1) {
            if kiss.body.solid_of_face(*f) == kiss.body.solid_of_face(*g) {
                continue;
            }
            match topo::shell_open(&kiss.body, 0.1, &[*f, *g], tol()) {
                Ok(_) => ok += 1,
                Err(e) => err.push(format!("{i},{j}: {}", format!("{e:?}").chars().take(100).collect::<String>())),
            }
        }
    }
    println!("R2S shell_open pairs ok={ok} err={}", err.len());
    for e in err {
        println!("R2S {e}");
    }
}

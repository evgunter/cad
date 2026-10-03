//! SCRATCH census-fold repros — copied into `crates/sweep/tests/` and
//! `mod`-ed from `all.rs` only while measuring; never committed there.
//! Run: `cargo nextest run --release -p sweep --test all census_fold_repros --no-capture`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanResult, ContactRecords};

fn tol() -> Tol {
    Tol::witness()
}

fn gates(label: &str, body: &Body<f64>) {
    let t3 = topo::validate_geometric(body, tol());
    let p = topo::validate_pseudomanifold(body, &ContactRecords::default(), tol());
    let kinds = |r: &Result<(), Vec<topo::ValidationError>>| match r {
        Ok(()) => "Ok".to_string(),
        Err(es) => {
            let mut v: Vec<String> = es
                .iter()
                .map(|e| format!("{e:?}").split([' ', '{', '(']).next().unwrap().to_string())
                .collect();
            v.sort();
            v.dedup();
            format!("Err[{}] {}", es.len(), v.join(","))
        }
    };
    println!(
        "REPRO {label}: solids={} shells={} faces={} | tier3={} | 3'(empty)={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        kinds(&t3),
        kinds(&p)
    );
    if let Err(es) = &p {
        for e in es.iter().take(3) {
            println!("REPRO {label}:   {}", format!("{e:?}").chars().take(300).collect::<String>());
        }
    }
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

fn rect_prism(x0: f64, x1: f64, y0: f64, y1: f64, h: f64) -> Body<f64> {
    let p = Profile::new(
        SketchPlane::xy(),
        vec![poly(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)])],
    )
    .validate(tol())
    .expect("rectangle validates");
    extrude(&p, Extrusion::Vector(Vec3::new(0.0, 0.0, h)), tol())
        .expect("extrudes")
        .body
}

fn vessel(pts: &[(f64, f64)]) -> Body<f64> {
    let p = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(
            pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect(),
        )],
    )
    .validate(tol())
    .expect("meridian validates");
    revolve(
        &p,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("revolves")
    .body
}

/// Class (i)/(ii) at the band: a prism whose width is a few ε.
#[test]
fn census_fold_repro_thin_prism() {
    let eps = tol().eps();
    for c in [0.5, 1.0, 1.5, 2.0, 5.0, 9.0, 10.0, 11.0, 20.0, 100.0] {
        let w = c * eps;
        let pr = Profile::new(
            SketchPlane::xy(),
            vec![poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, w), (0.0, w)])],
        )
        .validate(tol());
        let Ok(pr) = pr else {
            println!("REPRO thin prism w={c}eps: profile refused {:?}", pr.err().map(|e| format!("{e:?}").chars().take(120).collect::<String>()));
            continue;
        };
        match extrude(&pr, Extrusion::Vector(Vec3::new(0.0, 0.0, 1.0)), tol()) {
            Ok(b) => gates(&format!("thin prism w={c}eps"), &b.body),
            Err(e) => println!("REPRO thin prism w={c}eps: extrude refused {}", format!("{e:?}").chars().take(160).collect::<String>()),
        }
    }
}

/// The same, thin along the extrusion instead of in the profile.
#[test]
fn census_fold_repro_thin_slab() {
    let eps = tol().eps();
    for c in [0.5, 1.0, 1.5, 2.0, 5.0, 9.0, 10.0, 11.0, 20.0, 100.0] {
        let pr = Profile::new(SketchPlane::xy(), vec![poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])])
            .validate(tol())
            .unwrap();
        match extrude(&pr, Extrusion::Vector(Vec3::new(0.0, 0.0, c * eps)), tol()) {
            Ok(b) => gates(&format!("slab h={c}eps"), &b.body),
            Err(e) => println!("REPRO slab h={c}eps: extrude refused {}", format!("{e:?}").chars().take(160).collect::<String>()),
        }
    }
}

/// Class (ii): shell of a hollow curved body is two nested solids.
#[test]
fn census_fold_repro_nested_shell() {
    let v = vessel(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]);
    gates("vessel", &v);
    let hollow = topo::shell(&v, 0.2, tol()).expect("shells").body;
    gates("vessel shelled", &hollow);
    let again = topo::shell(&hollow, 0.05, tol()).expect("shells again").body;
    gates("hollow vessel shelled again", &again);
}

/// Class (ii)/(iii): a cylinder and a box, apart by `g`, unioned.
#[test]
fn census_fold_repro_disjoint_union() {
    let cyl = vessel(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    for g in [10.0, 1.0, 0.1, 1e-3] {
        let bx = rect_prism(1.0 + g, 2.0 + g, -0.5, 0.5, 1.0);
        gates(&format!("box alone g={g}"), &bx);
        match topo::union(&cyl, &bx, tol()) {
            Ok(BooleanResult::Body(bb)) => {
                println!("REPRO disjoint union g={g}: kind={:?} contacts_empty={}", bb.kind, bb.contacts == ContactRecords::default());
                gates(&format!("cyl ∪ box g={g}"), &bb.body);
            }
            other => println!("REPRO disjoint union g={g}: {}", format!("{other:?}").chars().take(200).collect::<String>()),
        }
        let bx2 = rect_prism(3.0 + g, 4.0 + g, -0.5, 0.5, 1.0);
        let planar = rect_prism(1.0, 3.0, -0.5, 0.5, 1.0);
        if let Ok(BooleanResult::Body(bb)) = topo::union(&planar, &bx2, tol()) {
            println!("REPRO planar disjoint union g={g}: kind={:?}", bb.kind);
            gates(&format!("box ∪ box g={g}"), &bb.body);
        }
    }
}

/// Class (iii): a corner-kiss union carries its contact in the
/// `BooleanBody`'s records; tier 3 on the bare body passes, the empty
/// 3′ refuses, and 3′ with the op's own records passes.
#[test]
fn census_fold_repro_corner_kiss() {
    let a = rect_prism(0.0, 1.0, 0.0, 1.0, 1.0);
    let b = {
        let p = Profile::new(SketchPlane::xy(), vec![poly(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)])])
            .validate(tol())
            .unwrap();
        // Extruded DOWN, so the boxes meet at the one point (1, 1, 0).
        extrude(&p, Extrusion::Vector(Vec3::new(0.0, 0.0, -1.0)), tol()).unwrap().body
    };
    match topo::union(&a, &b, tol()) {
        Ok(BooleanResult::Body(bb)) => {
            println!("REPRO corner kiss: kind={:?} vv={} vf={}", bb.kind, bb.contacts.vv.len(), bb.contacts.a_on_b.len() + bb.contacts.b_on_a.len());
            gates("corner kiss union", &bb.body);
            println!(
                "REPRO corner kiss union: 3'(own records)={:?}",
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).map_err(|e| e.len())
            );
        }
        other => println!("REPRO corner kiss: {}", format!("{other:?}").chars().take(200).collect::<String>()),
    }
}

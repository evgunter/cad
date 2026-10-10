//! A full-revolve tube's outer rim vertex (the lone vertex of a closed
//! circle edge) touching a large cube's face whose plane holds the rim
//! tangent and leans over the rim, so the touch is the one point.
//! Prints the union's tier-3′ refusal payload and its contact records.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use topo::test_support::mapped_cube;
use topo::{AtRestBody, Body, BooleanDeclarations};

fn tol() -> Tol {
    Tol::witness()
}

const SIDE: f64 = 10.0;

fn cube_beyond(v: [f64; 3], m: [f64; 3]) -> Body<f64> {
    // m lies in the xy plane here, so u = z and w = m × u.
    let u = [0.0, 0.0, 1.0];
    let w = [m[1] * u[2] - m[2] * u[1], m[2] * u[0] - m[0] * u[2], m[0] * u[1] - m[1] * u[0]];
    mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (SIDE * (x - 0.5), SIDE * (y - 0.5), SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

fn tube() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

#[test]
#[ignore = "measurement probe; cargo test --release -p sweep --test all -- --ignored --exact --nocapture contacthold_tube_rim_probe::tube_rim_union_payload"]
fn tube_rim_union_payload() {
    let d = BooleanDeclarations::default();
    let t = AtRestBody::validate(tube(), tol()).unwrap();
    for (rim, yv, k) in [("top", 1.0, 0usize), ("top", 1.0, 3), ("bot", 0.0, 18), ("bot", 0.0, 21)] {
        let th = (k as f64 + 0.37) * std::f64::consts::TAU / 24.0;
        let m = [th.cos(), th.sin(), 0.0];
        let c = AtRestBody::validate(cube_beyond([1.0, yv, 0.0], m), tol()).unwrap();
        for (order, p, q) in [("xy", &t, &c), ("yx", &c, &t)] {
            let r = topo::union_with(p, q, &d, tol());
            let tag = format!("PROBE {rim} k={k} {order}");
            let Ok(r) = r else {
                println!("{tag} ERR {r:?}");
                continue;
            };
            let Some(bb) = r.body() else {
                println!("{tag} EMPTY");
                continue;
            };
            let v = topo::mass_properties(&bb.body, tol()).map(|m| m.volume);
            println!("{tag} kind={:?} volume={v:?}", bb.kind);
            println!("{tag} contacts={:#?}", bb.contacts);
            match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                Ok(()) => println!("{tag} t3p=ok"),
                Err(es) => println!("{tag} t3p=ERR {es:#?}"),
            }
            for (vk, x) in bb.body.vertex_points() {
                println!("{tag} vertex {vk:?} at {x:?}");
            }
            for (fk, f) in bb.body.faces() {
                let kind = format!("{:?}", bb.body.get_surface(f.surface).unwrap());
                let kind: String = kind.chars().take(60).collect();
                let mut verts = std::collections::BTreeSet::new();
                for lk in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
                    match bb.body.get_loop(lk).unwrap().boundary {
                        topo::LoopBoundary::Cycle { first } => {
                            for he in bb.body.loop_cycle(first).unwrap() {
                                verts.insert(bb.body.get_half_edge(he).unwrap().start);
                            }
                        }
                        topo::LoopBoundary::Empty { vertex } => {
                            verts.insert(vertex);
                        }
                    }
                }
                println!("{tag} face {fk:?} sense={} verts={verts:?} {kind}", f.sense);
            }
        }
    }
}

/// The control: the same cube moved `gap` along `m`, off the rim, so the
/// union is two separated solids with no contact at all. The census
/// lines here name the same backstop pairs as the touching pose when
/// the cause is the backstop rather than the touch.
#[test]
#[ignore = "measurement probe; cargo test --release -p sweep --test all -- --ignored --exact --nocapture contacthold_tube_rim_probe::tube_rim_separated_control"]
fn tube_rim_separated_control() {
    let d = BooleanDeclarations::default();
    let t = AtRestBody::validate(tube(), tol()).unwrap();
    for (rim, yv, k) in [("top", 1.0, 0usize), ("top", 1.0, 3), ("bot", 0.0, 21)] {
        let th = (k as f64 + 0.37) * std::f64::consts::TAU / 24.0;
        let m = [th.cos(), th.sin(), 0.0];
        for gap in [0.05, 20.0] {
            let v = [1.0 + gap * m[0], yv + gap * m[1], 0.0];
            let c = AtRestBody::validate(cube_beyond(v, m), tol()).unwrap();
            let tag = format!("CONTROL {rim} k={k} gap={gap}");
            let r = topo::union_with(&t, &c, &d, tol());
            let Ok(Some(bb)) = r.as_ref().map(topo::BooleanResult::body) else {
                println!("{tag} {r:?}");
                continue;
            };
            println!("{tag} kind={:?} contacts={:?}", bb.kind, bb.contacts);
            match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                Ok(()) => println!("{tag} t3p=ok"),
                Err(es) => {
                    for e in es {
                        let s = format!("{e:?}");
                        println!("{tag} t3p=ERR {}", s.chars().take(90).collect::<String>());
                    }
                }
            }
        }
    }
}

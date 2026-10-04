//! PR 3773 review probes: `rim_of`'s structural walk swept over bodies
//! built through the public doors (extrude, revolve, boolean, fillet),
//! checking every circle edge with two side surfaces against an oracle
//! that reads the body independently of the door: the fixed-side
//! orientation read edge by edge, the geometric circle each member
//! lies on, and every other seed's answer.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::common::bead::bead;
use geom::Curve3;
use geom_core::{Tol, Vec3};
use profile::SketchPlane;
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    ball_poled_z, bored_block_of_arcs, boss_of_arcs, brick, circle_arcs_at_z, cube, disc_of_arcs,
    dome, finished, lantern, pocket_of_arcs, sphere_zone, waisted,
};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query::rim_of;
use topo::readback::edge_sides;
use topo::{
    Body, BooleanDeclarations, EdgeKey, HalfEdgeKey, RimError, SurfaceKey, validate_geometric,
};

fn tol() -> Tol {
    Tol::witness()
}

fn boolean(name: &str, op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let out = boolean_op_with(
        op,
        &finished(&format!("{name}: operand A"), a.clone(), tol()),
        &finished(&format!("{name}: operand B"), b.clone(), tol()),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("{name}: the boolean builds, got {e}"));
    out.body()
        .unwrap_or_else(|| panic!("{name}: a body"))
        .body
        .clone()
        .into_body()
}

fn pair(body: &Body<f64>, k: EdgeKey) -> (SurfaceKey, SurfaceKey) {
    let (a, b) = edge_sides(body, k).unwrap().surfaces();
    (a.min(b), a.max(b))
}

/// The half-edge of `k` on its pair's lower surface key.
fn lower_he(body: &Body<f64>, k: EdgeKey) -> HalfEdgeKey {
    let sides = edge_sides(body, k).unwrap();
    if sides.plus.surface == pair(body, k).0 {
        sides.plus.half_edge
    } else {
        sides.minus.half_edge
    }
}

fn circle(body: &Body<f64>, k: EdgeKey) -> Option<(Vec3<f64>, f64, Vec3<f64>)> {
    let e = body.get_edge(k)?;
    match body.get_curve_geom(e.curve)?.certified()?.carrier() {
        Curve3::Circle {
            center,
            radius,
            axis,
            ..
        } => Some((
            Vec3::new(center.x, center.y, center.z),
            *radius,
            Vec3::new(axis.x, axis.y, axis.z),
        )),
        _ => None,
    }
}

fn same_circle(a: (Vec3<f64>, f64, Vec3<f64>), b: (Vec3<f64>, f64, Vec3<f64>)) -> bool {
    let d = a.0 - b.0;
    let c = a.2.cross(b.2);
    d.dot(d).sqrt() < 1e-7 && (a.1 - b.1).abs() < 1e-7 && c.dot(c).sqrt() < 1e-7
}

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

#[derive(Default, Debug)]
struct Tally {
    rims: usize,
    max_arcs: usize,
    refused: BTreeMap<String, usize>,
    findings: Vec<String>,
}

/// Every circle edge with two side surfaces, through the door, against
/// the oracle.
fn audit(name: &str, body: &Body<f64>) -> Tally {
    let mut t = Tally::default();
    if let Err(e) = validate_geometric(body, tol()) {
        t.findings.push(format!(
            "{name}: SKIPPED, not valid: {} errors, first {:?}",
            e.len(),
            e.first()
        ));
        return t;
    }
    let seeds: Vec<EdgeKey> = body
        .edges()
        .map(|(k, _)| k)
        .filter(|&k| circle(body, k).is_some() && pair(body, k).0 != pair(body, k).1)
        .collect();
    let mut answers: BTreeMap<EdgeKey, Result<Vec<EdgeKey>, RimError>> = BTreeMap::new();
    for &k in &seeds {
        answers.insert(k, rim_of(body, k));
    }
    let mut classes: BTreeMap<Vec<EdgeKey>, Vec<EdgeKey>> = BTreeMap::new();
    for (&k, ans) in &answers {
        match ans {
            Ok(rim) => {
                if rim[0] != k {
                    t.findings
                        .push(format!("{name}: {k:?} does not lead its answer"));
                }
                let n = rim.len();
                for (i, &e) in rim.iter().enumerate() {
                    if pair(body, e) != pair(body, k) {
                        t.findings
                            .push(format!("{name}: {e:?} off the seed's pair"));
                    }
                    // Fixed side, read edge by edge: each lower-side
                    // half-edge ends where the next one starts.
                    let next = rim[(i + 1) % n];
                    let end = body.half_edge_end(lower_he(body, e)).unwrap();
                    let start = body.get_half_edge(lower_he(body, next)).unwrap().start;
                    if end != start {
                        t.findings.push(format!(
                            "{name}: seed {k:?}: lower side of {e:?} ends at {end:?}, next {next:?} starts at {start:?}"
                        ));
                    }
                    match circle(body, e) {
                        Some(c) if same_circle(c, circle(body, k).unwrap()) => {}
                        Some(_) => t.findings.push(format!(
                            "{name}: {e:?} is on another circle than seed {k:?}"
                        )),
                        None => t
                            .findings
                            .push(format!("{name}: {e:?} in {k:?}'s rim is not a circle")),
                    }
                    // A member that refuses while this seed answers.
                    if let Some(Err(err)) = answers.get(&e) {
                        t.findings.push(format!(
                            "{name}: {k:?} answers a rim holding {e:?}, which refuses: {err:?}"
                        ));
                    }
                }
                // Not split: every same-pair edge on the same circle is in.
                for &o in &seeds {
                    if !rim.contains(&o)
                        && pair(body, o) == pair(body, k)
                        && same_circle(circle(body, o).unwrap(), circle(body, k).unwrap())
                    {
                        t.findings.push(format!(
                            "{name}: {o:?} lies on {k:?}'s circle and pair but is not in its rim {rim:?}"
                        ));
                    }
                }
                let mut key = rim.clone();
                key.sort();
                if let Some(prev) = classes.get(&key) {
                    if !is_rotation(prev, rim) {
                        t.findings
                            .push(format!("{name}: {rim:?} is not a rotation of {prev:?}"));
                    }
                } else {
                    for other in classes.keys() {
                        if other.iter().any(|e| key.contains(e)) {
                            t.findings
                                .push(format!("{name}: rims {other:?} and {key:?} overlap"));
                        }
                    }
                    t.max_arcs = t.max_arcs.max(n);
                    classes.insert(key, rim.clone());
                }
            }
            Err(e) => {
                let tag = match e {
                    RimError::NotOneRim { how, .. } => format!("NotOneRim/{how:?}"),
                    other => format!("{other:?}")
                        .split(['(', ' '])
                        .next()
                        .unwrap()
                        .to_owned(),
                };
                *t.refused.entry(tag).or_default() += 1;
            }
        }
    }
    t.rims = classes.len();
    t
}

fn corpus() -> Vec<(String, Body<f64>)> {
    let mut v: Vec<(String, Body<f64>)> = vec![
        ("dome".into(), dome(1.0, tol())),
        ("lantern".into(), lantern(tol())),
        ("waisted".into(), waisted(tol())),
        (
            "sphere_zone full".into(),
            sphere_zone(0.5, Revolution::Full, tol()),
        ),
        (
            "bead xy".into(),
            bead(SketchPlane::xy(), 1.0, 0.5, Revolution::Full),
        ),
    ];
    for n in 2..=6 {
        v.push((
            format!("disc_of_arcs({n})"),
            disc_of_arcs(n, 0.5, 1.0, tol()),
        ));
        v.push((
            format!("bored_block_of_arcs({n})"),
            bored_block_of_arcs(n, 2.0, 1.0, 0.5, tol()),
        ));
        v.push((
            format!("boss_of_arcs({n})"),
            boss_of_arcs(n, 2.0, 0.5, 1.0, 2.0, tol()),
        ));
        v.push((
            format!("pocket_of_arcs({n})"),
            pocket_of_arcs(n, 2.0, 0.5, 1.5, tol()),
        ));
    }
    let far = brick::<f64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol());
    let ball = ball_poled_z(0.3, Vec3::new(0.5, 0.5, 1.0), tol());
    let mut bools: Vec<(String, BooleanOp, Body<f64>, Body<f64>)> = vec![
        (
            "cube - ball".into(),
            BooleanOp::Subtract,
            cube(1.0, tol()),
            ball.clone(),
        ),
        (
            "cube + ball".into(),
            BooleanOp::Union,
            cube(1.0, tol()),
            ball,
        ),
        (
            "two balls lens".into(),
            BooleanOp::Intersect,
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 1.2), tol()),
        ),
        (
            "two balls union".into(),
            BooleanOp::Union,
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 1.2), tol()),
        ),
        (
            "brick - 4-arc cylinder".into(),
            BooleanOp::Subtract,
            brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol()),
            sweep::test_support::cylinder_of_arcs_at(
                4,
                0.5,
                geom_core::Point2::new(0.1, 0.0),
                -1.0,
                3.0,
                tol(),
            ),
        ),
        (
            "3-arc disc cut by a brick".into(),
            BooleanOp::Subtract,
            disc_of_arcs(3, 0.5, 1.0, tol()),
            brick::<f64>((0.3, 1.0), (-1.0, 1.0), (-1.0, 2.0), tol()),
        ),
    ];
    for n in [3, 5] {
        // Re-key: a far brick first, so the disc's surfaces mint later.
        bools.push((
            format!("far brick + disc_of_arcs({n})"),
            BooleanOp::Union,
            far.clone(),
            disc_of_arcs(n, 0.5, 1.0, tol()),
        ));
        bools.push((
            format!("boss_of_arcs({n}) + far brick"),
            BooleanOp::Union,
            boss_of_arcs(n, 2.0, 0.5, 1.0, 2.0, tol()),
            far.clone(),
        ));
        bools.push((
            format!("far brick + pocket_of_arcs({n})"),
            BooleanOp::Union,
            far.clone(),
            pocket_of_arcs(n, 2.0, 0.5, 1.5, tol()),
        ));
    }
    for (name, op, a, b) in bools {
        let body = boolean(&name, op, &a, &b);
        v.push((name, body));
    }
    // Fillet outputs: the band's rims, on the closed-rim surgery.
    for n in [3, 4] {
        for (name, body, z, r) in [
            ("disc", disc_of_arcs(n, 0.5, 1.0, tol()), 1.0, 0.1),
            ("boss", boss_of_arcs(n, 2.0, 0.5, 1.0, 2.0, tol()), 2.0, 0.1),
            ("pocket", pocket_of_arcs(n, 2.0, 0.5, 1.5, tol()), 1.5, 0.1),
        ] {
            let rim = circle_arcs_at_z(&body, z);
            let out = fillet_edges(&body, &rim, r, tol())
                .unwrap_or_else(|e| panic!("filleted {name}({n}): carves, got {}", e.error));
            v.push((format!("filleted {name}_of_arcs({n})"), out.body));
        }
    }
    v
}

/// **Every circle edge of every corpus body answers, against the
/// oracle**: rim[0] is the seed; members are on the seed's pair and
/// geometric circle; the lower-side half-edges chain head to tail
/// (the fixed-side orientation, edge by edge); every same-pair edge on
/// the seed's circle is in its rim (not split); no member refuses while
/// another answers; every seed of a class answers a rotation; classes
/// are disjoint.
#[test]
fn every_circle_edge_of_the_public_door_corpus_answers_against_the_oracle() {
    let mut findings = Vec::new();
    let mut total = 0;
    for (name, body) in corpus() {
        let t = audit(&name, &body);
        println!(
            "REVIEW3773 {name}: rims={} max_arcs={} refused={:?}",
            t.rims, t.max_arcs, t.refused
        );
        if name == "3-arc disc cut by a brick" {
            // The cut interrupts both rims: every arc dangles.
            assert_eq!(t.rims, 0, "{name}: no closed rim survives the cut");
            assert_eq!(
                t.refused.get("NotOneRim/Dangles"),
                Some(&6),
                "{name}: each of the six arcs dangles"
            );
        }
        total += t.rims;
        findings.extend(t.findings);
    }
    for f in &findings {
        println!("REVIEW3773 FINDING {f}");
    }
    assert!(total > 30, "not vacuous: {total} rims");
    assert!(findings.is_empty(), "{} findings", findings.len());
}

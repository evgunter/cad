//! PROBE (phase 1) — where the reversed walk's finding sits.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

use core::f64::consts::TAU;

use geom_brep::Pcurve;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, HalfEdgeKey, ValidationError};

use super::common::latitude_seam::{door_cavity, two_arc_sphere};
use super::shell7_common::*;

fn discontinuities(body: &Body<f64>) -> Vec<HalfEdgeKey> {
    match topo::validate_geometric(body, tol()) {
        Ok(()) => vec![],
        Err(errs) => errs
            .iter()
            .filter_map(|e| match e {
                ValidationError::Pcurve {
                    finding: topo::PcurveMintError::LoopDiscontinuity { half_edge },
                } => Some(*half_edge),
                _ => None,
            })
            .collect(),
    }
}

fn describe(label: &str, body: &Body<f64>, he: HalfEdgeKey) {
    let h = body.get_half_edge(he).unwrap();
    let lp = body.get_loop(h.parent_loop).unwrap();
    let topo::LoopBoundary::Cycle { first } = lp.boundary else {
        panic!()
    };
    let cycle = body.loop_cycle(first).unwrap();
    let face = body.get_face(lp.face).unwrap();
    let surface = body.get_surface(face.surface).unwrap();
    println!(
        "[{label}] finding at {he:?}; loop {:?} on face {:?} ({surface:?}); first {first:?}; prev(first) {:?}; next(first) {:?}",
        h.parent_loop,
        lp.face,
        body.get_half_edge(first).unwrap().prev,
        body.get_half_edge(first).unwrap().next
    );
    for k in &cycle {
        let hk = body.get_half_edge(*k).unwrap();
        let cache = body.pcurve(*k);
        let plus = body.edges().any(|(_, e)| e.he_plus == *k);
        match cache {
            None => println!("  {k:?} plus={plus} NO ROW"),
            Some(c) => {
                let (t0, t1) = c.params();
                let (et, xt) = if plus { (t0, t1) } else { (t1, t0) };
                println!(
                    "  {k:?} plus={plus} entry {:?} exit {:?} start {:?} pcurve {:?}",
                    c.pcurve().eval(et),
                    c.pcurve().eval(xt),
                    hk.start,
                    c.pcurve()
                );
            }
        }
    }
}

fn fixtures() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("sphere", two_arc_sphere()),
        ("cavity", door_cavity(&two_arc_sphere(), 0.05)),
        ("torus", tube_torus(2.0, 0.5)),
        ("drum", drum(1.0, 2.0)),
        ("cone", polyline(&[(0.0, 0.0), (1.0, 0.0), (0.0, 2.0)], Revolution::Full)),
        ("revolved-torus", {
            revolved(
                ProfileLoop::new(vec![
                    ProfileVertex::new(p2(2.0, -0.5), 1.0),
                    ProfileVertex::new(p2(2.0, 0.5), 1.0),
                ]),
                Revolution::Full,
            )
        }),
        ("half-drum", polyline(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)], Revolution::Partial(core::f64::consts::PI))),
    ]
}

#[test]
fn probe_where_the_finding_sits() {
    for (name, body) in fixtures() {
        println!("== {name}: rows {}", body.pcurves().count());
        assert!(
            discontinuities(&body).is_empty(),
            "{name} source is continuous"
        );
        let reverted = body.revert().unwrap();
        let found = discontinuities(&reverted);
        let tier3 = topo::validate_geometric(&reverted, tol()).err().map(|v| {
            v.iter()
                .map(|e| format!("{e:?}").chars().take(70).collect::<String>())
                .collect::<Vec<_>>()
        });
        println!("== {name} reverted: findings {found:?}; tier3 {tier3:?}");
        for he in found {
            describe(&format!("{name} src"), &body, he);
            describe(&format!("{name} rev"), &reverted, he);
        }
    }
}

/// The brief's arithmetic re-park: is `(x + τ) − τ` the bits of `x`
/// on every row a periodic chart carries?
#[test]
fn probe_shift_arithmetic_involution() {
    let mut total = 0;
    let mut broken = 0;
    for (name, body) in fixtures() {
        for (he, c) in body.pcurves() {
            let x = match c.pcurve() {
                Pcurve::Harmonic { p0, .. }
                | Pcurve::IsoLine { p0, .. }
                | Pcurve::IsoArc { p0, .. } => p0.x,
                _ => continue,
            };
            total += 1;
            let up = ((x + TAU) - TAU).to_bits() != x.to_bits();
            let down = ((x - TAU) + TAU).to_bits() != x.to_bits();
            if up || down {
                broken += 1;
                println!("[{name}] {he:?} x={x:e} up-broken={up} down-broken={down}");
            }
        }
    }
    println!("rows {total}, shift-arithmetic non-involutions {broken}");
    let x = 1.0f64 + f64::EPSILON;
    println!(
        "generic: x=1+eps: ((x+tau)-tau)==x? {}",
        ((x + TAU) - TAU).to_bits() == x.to_bits()
    );
}

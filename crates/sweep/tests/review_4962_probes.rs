//! Review probes for PR 4962 (a fillet's overrun builds). Not a gate:
//! evidence for one review, each row labelled with what it measures.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::at_rest;
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, tol, volume_enclosure};
use crate::band_turn_overrun_chamfer::{Half, ends, halves_of, point_at};
use crate::band_turn_overrun_oracles::{
    HEX, Hex, Poly, TURN, a_clearance, hexahedron, leaning, non_trivalent, pinned,
};
use crate::common::cavity::brick;
use crate::common::operands::leaning_turn;
use geom_core::Point3;
use sweep::test_support::realized;
use topo::boolean::BooleanOp;

fn fillet(body: &Body<f64>, edges: &[EdgeKey], r: f64) -> Result<Body<f64>, BlendError> {
    let out = fillet_edges(&at_rest(body, tol()), edges, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).expect("tier 3");
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol()).expect("tier 3′");
    assert_eq!(non_trivalent(&out.body), 0, "every vertex trivalent");
    Ok(out.body)
}

fn v(p: [f64; 3]) -> String {
    format!("[{:.17e},{:.17e},{:.17e}]", p[0], p[1], p[2])
}

/// One JSON line per build, for the independent Monte Carlo oracle
/// (`review_4962_mc.py`): the polytope's half-spaces (the air's, for a
/// void), the two requested edges' ends, `r`, the kernel's `ΔV` and its
/// pad, and each edge filleted alone (a plain cut-off at each end, the
/// established path: `vol(B ∩ Wᵢ)` by itself).
fn dump(what: &str, halves: &[Half], body: &Body<f64>, edges: &[EdgeKey], r: f64, sign: f64) {
    let out = fillet(body, edges, r).unwrap_or_else(|e| panic!("{what}: builds, got {e}"));
    let (v0, p0) = volume_enclosure(body);
    let (v1, p1) = volume_enclosure(&out);
    let alone: Vec<String> = edges
        .iter()
        .map(|e| match fillet(body, &[*e], r) {
            Ok(one) => format!("{:.17e}", v0 - volume_enclosure(&one).0),
            Err(_) => "null".into(),
        })
        .collect();
    let halves: Vec<String> = halves
        .iter()
        .map(|(n, c)| format!("[{},{:.17e}]", v(*n), c))
        .collect();
    let reqs: Vec<String> = edges
        .iter()
        .map(|e| {
            let [a, b] = ends(body, *e).map(|x| point_at(body, x));
            format!("{{\"a\":{},\"b\":{}}}", v(a), v(b))
        })
        .collect();
    println!(
        "MC {{\"what\":\"{what}\",\"r\":{r},\"sign\":{sign},\"dv\":{:.17e},\"pad\":{:.3e},\"alone\":[{}],\"halves\":[{}],\"reqs\":[{}]}}",
        v0 - v1,
        p0 + p1,
        alone.join(","),
        halves.join(","),
        reqs.join(",")
    );
}

/// **P1** — builds at generic corners, both orders, convex and as a
/// void, dumped for the independent oracle.
#[test]
fn p1_dump_generic_overruns_for_the_independent_oracle() {
    let fixtures: Vec<(String, Poly)> = vec![
        ("hex HEX".into(), hexahedron(HEX)),
        (
            "hex yawed (0.4,0.2,0.3,0.8)".into(),
            hexahedron(Hex { c: 0.8, ..HEX }),
        ),
        (
            "hex (-0.6,-0.15,0.25,-0.7)".into(),
            hexahedron(Hex {
                s: -0.6,
                a: -0.15,
                b: 0.25,
                c: -0.7,
                ..HEX
            }),
        ),
        ("leaning 0.5".into(), leaning(0.5, 1.0, 1.0)),
        ("leaning -1".into(), leaning(-1.0, 1.0, 1.0)),
    ];
    for (name, p) in &fixtures {
        for r in [0.1, 0.15] {
            for order in [[0, 1], [1, 0]] {
                let edges = p.edges(&order.map(|i| TURN[i]));
                dump(
                    &format!("{name} r {r} {order:?}"),
                    &p.halves,
                    &p.body,
                    &edges,
                    r,
                    1.0,
                );
            }
        }
    }
    // As a void: the body gains the air's share.
    let p = hexahedron(Hex { c: 0.5, ..HEX });
    let block = brick(Point3::new(-5.0, -5.0, -5.0), Point3::new(5.0, 5.0, 5.0));
    let void = realized(BooleanOp::Subtract, &block, &p.body, tol());
    let edges: Vec<EdgeKey> = TURN
        .iter()
        .map(|q| {
            let (_, a, b) = p.edge(*q);
            crate::band_turn_overrun_oracles::edge_near(&void, a, b)
        })
        .collect();
    dump("void hex c 0.5 r 0.1", &p.halves, &void, &edges, 0.1, -1.0);
}

/// What one fillet attempt read as.
fn outcome(r: Result<Body<f64>, BlendError>) -> String {
    match r {
        Ok(_) => "built".into(),
        Err(BlendError::Escalated { decision, .. }) => format!("escalated {decision:?}"),
        Err(e) => format!("refused {e}"),
    }
}

/// **P2** — the `CapEllipse` window near a box corner: the leaning prism
/// at small leans `θ` (both signs), `r = d`. Claimed: the turn's own
/// readings escalate to ≈ 5e-6°, `CapEllipse` to ≈ 0.0256°
/// (`θ = √(2·escalate/r)`), then it builds, monotone.
#[test]
fn p2_the_cap_ellipse_window_is_monotone() {
    let degs = [
        1e-6, 3e-6, 1e-5, 1e-4, 1e-3, 0.01, 0.02, 0.024, 0.025, 0.0255, 0.0258, 0.026, 0.027, 0.03,
        0.05, 0.1, 1.0,
    ];
    let mut failures = Vec::new();
    for sign in [1.0, -1.0] {
        let mut built = false;
        for deg in degs {
            let s = sign * f64::to_radians(deg).tan();
            let (body, edges) = leaning_turn(s);
            let got = outcome(fillet(&body, &edges, D));
            println!("lean {sign}·{deg}°: {got}");
            if got == "built" {
                built = true;
            } else if built {
                failures.push(format!("lean {sign}·{deg}°: {got} after a build"));
            } else if !got.starts_with("escalated") {
                failures.push(format!("lean {sign}·{deg}°: {got}, not an escalation"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **P3** — round pins on the front face (`Fⱼ`) at the bracket's
/// `y = 0` corner (`leaning_turn(−1)`), around the fillet's overrun: in
/// `(x, z)` the far band's circle has radius `d` about
/// `c = (−1 + (1 + √2)·d, 1 − d)`, the overrun is the region outside it,
/// below `z = 1 − d` and right of `L` (`x + z = 0`). A pin of radius
/// `0.004` at distance `ρ` from `c`:
/// - `ρ = d + 0.002` straddles the arc, cut by the overrun: refuses.
/// - `ρ = 0.085`, `0.07` and `0.05`, inside the circle, below the
///   trimline and clear of `L`: nothing the fillet takes reaches it.
#[test]
fn p3_pins_around_the_fillets_overrun() {
    let c = [-1.0 + (1.0 + 2f64.sqrt()) * D, 1.0 - D];
    let at = |rho: f64, dir: [f64; 2]| {
        let n = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
        let (x, z) = (c[0] + rho * dir[0] / n, c[1] + rho * dir[1] / n);
        assert!(x + z > 0.0, "the pin's centre is inside the body");
        ([x, 0.0, z], [0.0, 1.0, 0.0])
    };
    let (plain, plain_edges) = leaning_turn(-1.0);
    let free = {
        let out = fillet(&plain, &plain_edges, D).expect("feature-free builds");
        volume_enclosure(&plain).0 - volume_enclosure(&out).0
    };
    // (what, ρ, direction from c, should build)
    let rows = [
        ("straddling the arc", D + 0.002, [-1.0, -0.6], false),
        (
            "straddling the arc lower down",
            D + 0.0015,
            [-1.0, -0.3],
            false,
        ),
        ("clear, 0.085 from c", 0.085, [-0.8, -0.6], true),
        ("clear, 0.07 from c", 0.07, [-0.6, -0.8], true),
        ("clear, 0.05 from c", 0.05, [-0.6, -0.8], true),
    ];
    let mut report = Vec::new();
    for (what, rho, dir, builds) in rows {
        let (p, n) = at(rho, dir);
        for (kind, boss, depth) in [("boss", true, 0.0), ("hole", false, 0.002)] {
            let (body, edges) = pinned(p, n, boss, depth);
            let got = fillet(&body, &edges, D);
            let line = match (&got, builds) {
                (Ok(out), true) => {
                    let dv = volume_enclosure(&body).0 - volume_enclosure(out).0;
                    format!("ok: built, ΔV − free = {:e}", dv - free)
                }
                (Err(e), false) if a_clearance(e) => format!("ok: refused {e}"),
                (Err(e), _) => format!("UNEXPECTED: refused {e}"),
                (Ok(_), false) => "UNEXPECTED: built where the overrun cuts it".into(),
            };
            report.push(format!("{kind} {what} at {p:?}: {line}"));
        }
    }
    println!("{}", report.join("\n"));
    assert!(
        report.iter().all(|l| !l.contains("UNEXPECTED")),
        "{}",
        report.join("\n")
    );
}

/// **P4** — the near-T needle (`band_turn_overrun_verdict`'s
/// `a_flattening_turns_needle_escalates_and_never_builds`, whose fillet
/// arm asserts only "not a mitre"), carried through the surgery: what
/// does the fillet actually do as the turn flattens? A build is dumped
/// for the oracle.
#[test]
fn p4_the_needle_through_the_fillet_surgery() {
    use crate::band_turn_overrun_probe::Cone;
    use crate::band_turn_overrun_verdict::corner;
    let mut report = Vec::new();
    for eps in [10.0, 3.0, 1.0, 0.3, 0.1, 0.03, 0.01] {
        let e = f64::to_radians(eps);
        let (body, edges) = corner(&Cone::of(
            [0.0, 0.5, 1.0],
            [1.0, 0.0, 0.0],
            [-e.cos(), e.sin(), 0.0],
        ));
        let got = fillet(&body, &edges, D);
        let ok = got.is_ok();
        let line = outcome(got);
        if ok {
            dump(
                &format!("needle {eps}°"),
                &halves_of(&body),
                &body,
                &edges,
                D,
                1.0,
            );
        }
        report.push(format!("needle {eps}°: {line}"));
    }
    println!("{}", report.join("\n"));
}

/// **P5** — the short near edge (`an-overruns-sliver-refuses-a-short-near-edge`,
/// filed for the chamfer) at the fillet: the step-2 hexahedron
/// (`c = 1`, `θ_S ≈ 136°`) with the top–wall edge shortened by moving
/// the back face in. What does the fillet do, and does the residue's
/// over-refusal carry over?
#[test]
fn p5_a_short_near_edge_at_the_fillet() {
    let base = Hex {
        c: 1.0,
        depth: 4.0,
        x1: 3.0,
        ..HEX
    };
    let v = hexahedron(base).edge(TURN[1]).1;
    let mut report = Vec::new();
    for len in [0.4, 0.3, 0.25, 0.22, 0.2, 0.18, 0.16, 0.14, 0.12] {
        let p = hexahedron(Hex {
            depth: len - v[1],
            ..base
        });
        let what = format!("near edge {len}");
        let edges = p.edges(&TURN);
        let got = fillet(&p.body, &edges, D);
        let ok = got.is_ok();
        let alone: Vec<String> = edges
            .iter()
            .map(|e| outcome(fillet(&p.body, &[*e], D)))
            .collect();
        report.push(format!("{what}: {} (alone: {alone:?})", outcome(got)));
        if ok {
            dump(&what, &p.halves, &p.body, &edges, D, 1.0);
        }
    }
    println!("{}", report.join("\n"));
}

/// **P6** — the turn recourse's "a larger setback" leg from the turn's
/// own escalation near a box corner, on the side whose overrun face is
/// oblique (lean `+`): `r = d` escalates `TurnFeetGap` at `3e-6°`; at a
/// larger radius the gap (linear in `r`) leaves the turn's band — does
/// the fillet then build, or land in `CapEllipse`'s window
/// (`√(2·escalate/r)`, `≈0.0256°` at `d`, `≈0.0128°` at `4d`)?
#[test]
fn p6_the_larger_setback_leg_near_a_box_corner() {
    let mut report = Vec::new();
    for (sign, deg) in [(1.0, 3e-6), (-1.0, 3e-6), (1.0, 1e-3)] {
        let s = sign * f64::to_radians(deg).tan();
        let (body, edges) = leaning_turn(s);
        for r in [D, 0.2, 0.4] {
            let got = outcome(fillet(&body, &edges, r));
            report.push(format!("lean {sign}·{deg}°, r {r}: {got}"));
        }
    }
    println!("{}", report.join("\n"));
}

//! Review probes for PR 3845 (lane reach-dual3845-r1). Each probe
//! prints a row `PROBE <name> | <outcome> | oracle <v>` and asserts
//! only "never a wrong body": a built body must match the closed-form
//! oracle (volume and point_in_solid samples), a refusal is recorded.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::circle_split;
use sweep::test_support::{brick, extruded, sketch_at};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, SolidContainment};

/// atan2(sqrt(1 - 1/d²), -1/d) at d = 2.5.
const RADIAL: f64 = 1.982_313_172_862_385;

fn tol() -> Tol {
    Tol::witness()
}

fn rod_at(c: (f64, f64), r: f64, z0: f64, h: f64, n: usize, phase: f64) -> Body<f64> {
    let rim = circle_split(Point2::new(c.0, c.1), r, n, phase, tol()).expect("rim");
    extruded(sketch_at(z0), vec![rim.into()], h, tol())
}

fn declared(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("detector");
    topo::flush::declare_all(&found)
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

/// Outcome of a union: Ok(body) or the refusal text.
fn union(a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    match topo::union_with(a, b, &declared(a, b), tol()) {
        Ok(BooleanResult::Body(bb)) => {
            let g = topo::validate_geometric(&bb.body, tol());
            let p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
            if g.is_err() || p.is_err() {
                return Err(format!("BUILT-INVALID tier3 {g:?} tier3' {p:?}"));
            }
            Ok(bb.body)
        }
        Ok(other) => Err(format!("non-body {other:?}")),
        Err(e) => Err(short(&e)),
    }
}

fn short(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    s.chars().take(160).collect()
}

/// Checks a built body against the oracle: volume within `vt`, and
/// each (point, inside?) sample. Returns a description of any
/// disagreement.
fn check(
    name: &str,
    out: &Result<Body<f64>, String>,
    oracle_v: f64,
    samples: &[(Point3<f64>, bool)],
) -> bool {
    match out {
        Err(e) => {
            println!("PROBE {name} | REFUSES {e} | oracle {oracle_v:.12}");
            !e.starts_with("BUILT-INVALID")
        }
        Ok(b) => {
            let v = volume(b);
            let band = Band::linear(tol()).unwrap();
            let mut bad = Vec::new();
            for (q, inside) in samples {
                let got = topo::point_in_solid(b, *q, band, tol());
                let ok = matches!(
                    (&got, inside),
                    (Ok(SolidContainment::In), true) | (Ok(SolidContainment::Out), false)
                );
                if !ok {
                    bad.push(format!("{q:?}:{got:?} want in={inside}"));
                }
            }
            let rel = (v - oracle_v).abs() / oracle_v.abs().max(1e-300);
            let good = rel <= 1e-9 && bad.is_empty();
            println!(
                "PROBE {name} | BUILDS faces {} vol {v:.12} rel {rel:.2e} | oracle {oracle_v:.12} | pis-bad {bad:?} | {}",
                b.faces().count(),
                if good { "OK" } else { "WRONG-BODY" }
            );
            good
        }
    }
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// Stack samples: inside both rods, outside beside, above, between.
fn stack_samples(r: f64, s: f64) -> Vec<(Point3<f64>, bool)> {
    vec![
        (p(0.1 * s, 0.2 * s, 0.5 * s), true),
        (p(0.3 * s, -0.4 * s, 1.5 * s), true),
        (p(-0.95 * r * s, 0.0, 1.0 * s + 0.01 * s), true),
        (p(0.0, 0.97 * r * s, 0.99 * s), true),
        (p(1.05 * r * s, 0.0, 1.0 * s), false),
        (p(0.0, 0.0, 2.1 * s), false),
        (p(0.0, -1.2 * r * s, 0.5 * s), false),
    ]
}

/// Claim 4: the aligned/turned stack at θ ∈ {0, 1e-7, 0.7, π/2, π−1e-7, π},
/// with 2-, 3- and 4-piece walls on each side, at three scales.
#[test]
fn probe_stack_matrix() {
    let mut wrong = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        for (na, nb) in [(2, 2), (2, 3), (3, 3), (2, 4), (4, 4), (3, 4)] {
            for th in [0.0, 1e-7, 0.7, FRAC_PI_2, PI - 1e-7, PI] {
                let name = format!("stack s={s} n=({na},{nb}) th={th:.3e}");
                let built = std::panic::catch_unwind(|| {
                    (rod_at((0.0, 0.0), s, 0.0, s, na, 0.0), rod_at((0.0, 0.0), s, s, s, nb, th))
                });
                let Ok((a, b)) = built else {
                    println!("PROBE {name} | FIXTURE-REFUSES (profile escalates at this eps)");
                    continue;
                };
                let out = union(&a, &b);
                if !check(&name, &out, 2.0 * PI * s * s * s, &stack_samples(1.0, s)) {
                    wrong.push(name.clone());
                }
                // reversed operand order
                let out2 = union(&b, &a);
                let name2 = format!("{name} (B∪A)");
                if !check(&name2, &out2, 2.0 * PI * s * s * s, &stack_samples(1.0, s)) {
                    wrong.push(name2);
                }
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

/// A rotated pose of the aligned stack: rotated about a skew axis and
/// translated, both operands; and the third and fourth rod reuse.
#[test]
fn probe_stack_rotated_and_reused() {
    let mut wrong = Vec::new();
    let axis = Vec3::new(1.0, 2.0, 3.0);
    let axis = axis * (1.0 / axis.norm());
    let m = Affine3::translation(Vec3::new(3.0, -1.0, 7.0))
        * Affine3::rotation_about_axis(p(0.0, 0.0, 0.0), axis, 0.9);
    for th in [0.0, PI, 0.7] {
        let a = topo::transform_rigid(&rod_at((0.0, 0.0), 1.0, 0.0, 1.0, 2, 0.0), &m, tol()).unwrap();
        let b = topo::transform_rigid(&rod_at((0.0, 0.0), 1.0, 1.0, 1.0, 2, th), &m, tol()).unwrap();
        let samples: Vec<_> = stack_samples(1.0, 1.0)
            .into_iter()
            .map(|(q, i)| (m.transform_point(q), i))
            .collect();
        let name = format!("rotated stack th={th}");
        if !check(&name, &union(&a, &b), 2.0 * PI, &samples) {
            wrong.push(name);
        }
    }
    // reuse: stack 4 rods, alternating phases.
    for phases in [[0.0, 0.0, 0.0, 0.0], [0.0, PI, 0.0, PI], [0.0, FRAC_PI_2, 0.0, 0.7]] {
        let mut acc = rod_at((0.0, 0.0), 1.0, 0.0, 1.0, 2, phases[0]);
        for (k, ph) in phases.iter().enumerate().skip(1) {
            let next = rod_at((0.0, 0.0), 1.0, k as f64, 1.0, 2, *ph);
            let name = format!("reuse {phases:?} rod {k}");
            let out = union(&acc, &next);
            let samples = vec![
                (p(0.2, 0.1, k as f64 + 0.5), true),
                (p(0.0, 0.0, k as f64 + 1.1), false),
                (p(0.98, 0.0, k as f64), true),
            ];
            if !check(&name, &out, PI * (k + 1) as f64, &samples) {
                wrong.push(name);
            }
            match out {
                Ok(b) => acc = b,
                Err(_) => break,
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

/// Unequal radii, coaxial, aligned seams: the small rod's circle lies
/// in the big rod's top face. Must refuse typed or build 5π.
#[test]
fn probe_unequal_radii() {
    let mut wrong = Vec::new();
    for (ra, rb) in [(2.0, 1.0), (1.0, 2.0), (1.0, 1.0 + 1e-3)] {
        for th in [0.0, PI, 0.7] {
            let a = rod_at((0.0, 0.0), ra, 0.0, 1.0, 2, 0.0);
            let b = rod_at((0.0, 0.0), rb, 1.0, 1.0, 2, th);
            let o = PI * (ra * ra + rb * rb);
            let rmin: f64 = ra.min(rb);
            let samples = vec![
                (p(0.0, 0.0, 0.5), true),
                (p(0.0, 0.0, 1.5), true),
                (p(0.98 * rmin, 0.0, 1.0), true),
                (p(0.0, 0.0, 2.2), false),
            ];
            let name = format!("unequal ra={ra} rb={rb} th={th}");
            if !check(&name, &union(&a, &b), o, &samples) {
                wrong.push(name);
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

/// A box resting on a rod covering x ≥ x0: the seam is one arc of the
/// rod's rim (a half turn at x0 = 0, more than a half turn at x0 < 0)
/// closed by a straight segment. Oracle: rod + box.
#[test]
fn probe_box_on_rod_partial_rim() {
    let mut wrong = Vec::new();
    for x0 in [0.3, 0.0, -0.3, -0.7] {
        for (n, ph) in [(2, 0.0), (2, FRAC_PI_2), (3, 0.0), (4, 0.3)] {
            let a = rod_at((0.0, 0.0), 1.0, 0.0, 1.0, n, ph);
            let b: Body<f64> = brick((x0, 3.0), (-2.0, 2.0), (1.0, 1.5), tol());
            let o = PI + (3.0 - x0) * 4.0 * 0.5;
            let samples = vec![
                (p(0.0, 0.0, 0.5), true),
                (p(2.0, 1.5, 1.25), true),
                (p(x0 - 0.05, 0.0, 1.25), false),
                (p(0.1, 0.0, 1.0 + 1e-3), x0 < 0.1),
                (p(0.9, 0.0, 1.0), true),
            ];
            let name = format!("box on rod x0={x0} n={n} ph={ph}");
            for (nm, out) in [(name.clone(), union(&a, &b)), (format!("{name} (B∪A)"), union(&b, &a))]
            {
                if !check(&nm, &out, o, &samples) {
                    wrong.push(nm);
                }
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

/// Several half-turn seam circles in ONE op: a plate on top of two
/// (or three) two-half rods that stand on a base plate. Every circle's
/// four germs reach the arc pass together.
#[test]
fn probe_plate_on_several_rods() {
    let mut wrong = Vec::new();
    let layouts: [&[(f64, f64)]; 4] = [
        &[(-2.0, 0.0), (2.0, 0.0)],
        &[(0.0, -2.0), (0.0, 2.0)],
        &[(-2.0, 0.0), (0.5, 2.3)],
        &[(-2.5, 0.0), (0.0, 0.0), (2.5, 0.0)],
    ];
    for centers in layouts {
        for (ph, r2) in [(0.0, 1.0), (FRAC_PI_2, 1.0), (0.0, 0.6)] {
            let mut base: Body<f64> = brick((-4.0, 4.0), (-4.0, 4.0), (-1.0, 0.0), tol());
            let mut o = 64.0;
            let mut ok = true;
            for (k, c) in centers.iter().enumerate() {
                let r = if k == 1 { r2 } else { 1.0 };
                let rod = rod_at(*c, r, 0.0, 1.0, 2, ph);
                match union(&base, &rod) {
                    Ok(b) => {
                        o += PI * r * r;
                        if (volume(&b) - o).abs() > 1e-9 {
                            println!("PROBE base+rod {centers:?} k={k} WRONG vol {}", volume(&b));
                            wrong.push(format!("base+rod {centers:?} {k}"));
                        }
                        base = b;
                    }
                    Err(e) => {
                        println!("PROBE base+rod {centers:?} ph={ph} k={k} | REFUSES {e}");
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                continue;
            }
            let top: Body<f64> = brick((-4.0, 4.0), (-4.0, 4.0), (1.0, 2.0), tol());
            let o2 = o + 64.0;
            let mut samples = vec![(p(3.5, 3.5, 1.5), true), (p(3.5, 3.5, 0.5), false)];
            for c in centers {
                samples.push((p(c.0, c.1 + 0.3, 0.5), true));
                samples.push((p(c.0 + 0.55, c.1, 0.999), true));
            }
            let name = format!("plate on rods {centers:?} ph={ph} r2={r2}");
            for (nm, out) in [(name.clone(), union(&base, &top)), (format!("{name} (B∪A)"), union(&top, &base))]
            {
                if !check(&nm, &out, o2, &samples) {
                    wrong.push(nm);
                }
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

fn chain(mut acc: Body<f64>, parts: Vec<Body<f64>>) -> Result<Body<f64>, String> {
    for b in parts {
        acc = union(&acc, &b)?;
    }
    Ok(acc)
}

/// Two (or three) half-turn seam circles in ONE op with null pairs on
/// both sides: A = base plate + rods (z 0..1), B = top plate + rods
/// (z 1..2) stacked rod on rod. Every circle's germs reach the arc
/// pass together. Phases per circle and radii varied.
#[test]
fn probe_two_columns() {
    let mut wrong = Vec::new();
    type Col = ((f64, f64), f64, f64, f64); // centre, r, phase low, phase up
    let layouts: Vec<Vec<Col>> = vec![
        vec![((-2.0, 0.0), 1.0, 0.0, 0.0), ((2.0, 0.0), 1.0, 0.0, 0.0)],
        vec![((-2.0, 0.0), 1.0, 0.0, PI), ((2.0, 0.0), 1.0, 0.0, 0.0)],
        vec![((0.0, -2.0), 1.0, 0.0, 0.0), ((0.0, 2.0), 1.0, 0.0, 0.0)],
        vec![((-2.0, 0.0), 1.0, FRAC_PI_2, FRAC_PI_2), ((2.0, 0.0), 1.0, 0.0, 0.0)],
        vec![((-2.0, 0.0), 1.0, 0.0, 0.0), ((0.5, 2.3), 0.6, 0.3, 0.3)],
        vec![((-2.0, -1.5), 1.0, 0.0, 0.0), ((1.0, 2.0), 1.0, 2.0, 2.0)],
        vec![((-2.5, 0.0), 1.0, 0.0, 0.0), ((0.0, 0.0), 1.0, 0.0, 0.0), ((2.5, 0.0), 1.0, 0.0, 0.0)],
        vec![((-2.0, 0.0), 1.0, 0.0, 0.0), ((2.0, 0.0), 1.0, 0.0, 0.7)],
        vec![((-2.0, 0.0), 1.0, 0.0, 0.0), ((2.0, 0.0), 0.5, 0.0, 0.0)],
        // Circle 2's sites sit where its tangent is RADIAL about circle
        // 1's centre (d = 2.5): germ j's sense in germ i's frame is zero.
        vec![((-1.25, 0.0), 1.0, 0.0, 0.0), ((1.25, 0.0), 1.0, RADIAL, RADIAL)],
        vec![((-1.25, 0.0), 1.0, PI - RADIAL, PI - RADIAL), ((1.25, 0.0), 1.0, 0.0, 0.0)],
    ];
    for cols in layouts {
        let base: Body<f64> = brick((-4.0, 4.0), (-4.0, 4.0), (-1.0, 0.0), tol());
        let top: Body<f64> = brick((-4.0, 4.0), (-4.0, 4.0), (2.0, 3.0), tol());
        let lows = cols.iter().map(|&(c, r, pl, _)| rod_at(c, r, 0.0, 1.0, 2, pl)).collect();
        let ups = cols.iter().map(|&(c, r, _, pu)| rod_at(c, r, 1.0, 1.0, 2, pu)).collect();
        let (a, b) = match (chain(base, lows), chain(top, ups)) {
            (Ok(a), Ok(b)) => (a, b),
            (ea, eb) => {
                println!("PROBE columns {cols:?} | SETUP-REFUSES {:?} {:?}", ea.err(), eb.err());
                continue;
            }
        };
        let o = 128.0 + cols.iter().map(|&(_, r, _, _)| 2.0 * PI * r * r).sum::<f64>();
        let mut samples = vec![(p(3.5, 3.5, 1.5), false), (p(3.5, 3.5, 2.5), true)];
        for &(c, r, _, _) in &cols {
            samples.push((p(c.0, c.1 + 0.3 * r, 1.5), true));
            samples.push((p(c.0 + 0.9 * r, c.1, 1.0), true));
            samples.push((p(c.0 + 1.1 * r, c.1, 1.0), false));
            samples.push((p(c.0, c.1 - 0.95 * r, 1.0 + 1e-3), true));
        }
        let name = format!("columns {cols:?}");
        for (nm, out) in [(name.clone(), union(&a, &b)), (format!("{name} (B∪A)"), union(&b, &a))] {
            if !check(&nm, &out, o, &samples) {
                wrong.push(nm);
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

/// A tube (annulus r_in..r_out) whose two rims are each split in two.
fn tube_at(rin: f64, rout: f64, z0: f64, n: usize, ph_out: f64, ph_in: f64) -> Body<f64> {
    let outer = circle_split(Point2::new(0.0, 0.0), rout, n, ph_out, tol()).expect("outer");
    let inner = circle_split(Point2::new(0.0, 0.0), rin, n, ph_in, tol()).expect("inner");
    let (o, i): (profile::ProfileLoop<f64>, profile::ProfileLoop<f64>) = (outer.into(), inner.into());
    extruded(sketch_at(z0), vec![o, i.reversed()], 1.0, tol())
}

/// Two concentric half-turn seam circles in ONE op: two stacked tubes.
/// A cross-circle pair here has a face bounded by both its ends (the
/// annular mating face), so a mispair is not caught by "no host face".
#[test]
fn probe_tube_stack() {
    let mut wrong = Vec::new();
    for (rin, rout) in [(1.0, 2.0), (0.5, 2.0), (1.9, 2.0)] {
        for (oa, ia, ob, ib) in [
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, PI, PI),
            (0.0, FRAC_PI_2, 0.0, FRAC_PI_2),
            (0.0, 0.0, 0.0, 0.7),
            (0.0, 0.0, 0.7, 0.0),
            (0.3, 0.0, 0.3, PI),
        ] {
            let a = tube_at(rin, rout, 0.0, 2, oa, ia);
            let b = tube_at(rin, rout, 1.0, 2, ob, ib);
            let o = 2.0 * PI * (rout * rout - rin * rin);
            let rm = 0.5 * (rin + rout);
            let samples = vec![
                (p(rm, 0.0, 0.5), true),
                (p(0.0, rm, 1.5), true),
                (p(-rm, 0.0, 1.0), true),
                (p(0.0, -rm, 1.0), true),
                (p(rm * 0.7071, rm * 0.7071, 1.0), true),
                (p(0.0, 0.0, 1.0), false),
                (p(0.5 * rin, 0.0, 1.0), false),
                (p(rout + 0.1, 0.0, 1.0), false),
            ];
            let name = format!("tubes rin={rin} rout={rout} phases=({oa},{ia})/({ob},{ib})");
            for (nm, out) in [(name.clone(), union(&a, &b)), (format!("{name} (B∪A)"), union(&b, &a))] {
                if !check(&nm, &out, o, &samples) {
                    wrong.push(nm);
                }
            }
        }
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

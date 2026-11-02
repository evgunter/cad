//! PROBE (read-only lane, not for merge): does a chain of faces each
//! within the Zero band of the next get merged into one face that
//! lies more than ε from where its members put them?
#![allow(clippy::all, clippy::pedantic, clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use geom_core::Tol;
use topo::test_support::{finished, prism_z};
use topo::{Body, BooleanResult, MemberInputs, NaryEnd, NaryMember, NaryOp, Surface};

fn h(k: usize, d: f64) -> f64 {
    let k = k as f64;
    1.0 + d * k * k / 2.0
}

/// Top faces (normal mostly +y), and the max distance of every top
/// vertex (y > 0.5) to each, over the body.
fn report(what: &str, body: &Body<f64>, eps: f64) {
    let tops: Vec<_> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) if normal.y.abs() > 0.9 => {
                Some((k, *origin, *normal))
            }
            _ => None,
        })
        .collect();
    let top_pts: Vec<_> = body
        .vertex_points()
        .filter(|(_, p)| p.y > 0.5)
        .map(|(_, p)| p)
        .collect();
    let mut worst = 0.0f64;
    if tops.len() == 1 {
        let (_, o, n) = tops[0];
        for p in &top_pts {
            let d = ((p.x - o.x) * n.x + (p.y - o.y) * n.y + (p.z - o.z) * n.z).abs();
            worst = worst.max(d);
        }
    }
    let t3 = topo::validate_geometric(body, Tol::witness());
    eprintln!(
        "PROBE {what}: faces {}, top faces {}, top verts {}, worst top-vertex off the one top plane = {:.2} eps, tier3 = {}",
        body.faces().count(),
        tops.len(),
        top_pts.len(),
        worst / eps,
        match &t3 { Ok(()) => "ok".to_string(), Err(e) => format!("ERR {:?}", e.iter().take(2).collect::<Vec<_>>()) }
    );
}

fn single(n: usize, d: f64, tol: Tol) {
    let eps = tol.eps();
    let mut profile = vec![(0.0, 0.0), (n as f64, 0.0)];
    for k in (0..=n).rev() {
        profile.push((k as f64, h(k, d * eps)));
    }
    let mut p = prism_z::<f64>(&profile, 0.0, 1.0, tol).body;
    let what = format!("merge n={n} d={d}eps drift={:.2}eps", d * ((n * n - n) as f64) / 2.0);
    match p.merge_coplanar_faces(tol) {
        Ok(out) => {
            eprintln!("PROBE {what}: Ok, {} groups, sizes {:?}", out.groups.len(), out.groups.iter().map(|g| 1 + g.absorbed.len()).collect::<Vec<_>>());
            report(&what, &p, eps);
        }
        Err(e) => eprintln!("PROBE {what}: Err {e}"),
    }
}

fn nary(n: usize, d: f64, tol: Tol) {
    let eps = tol.eps();
    let bodies: Vec<_> = (0..n)
        .map(|k| {
            let x0 = k as f64;
            let prof = [(x0, 0.0), (x0 + 1.0, 0.0), (x0 + 1.0, h(k + 1, d * eps)), (x0, h(k, d * eps))];
            finished("slab", prism_z::<f64>(&prof, 0.0, 1.0, tol).body, tol)
        })
        .collect();
    let inputs: Vec<MemberInputs> = (0..n).map(|r| MemberInputs { rank: r as u32, ..MemberInputs::default() }).collect();
    let members: Vec<NaryMember<'_, f64>> = bodies.iter().zip(&inputs).map(|(body, inputs)| NaryMember { body, inputs }).collect();
    let what = format!("nary n={n} d={d}eps drift={:.2}eps", d * ((n * n - n) as f64) / 2.0);
    let run = topo::nary_boolean(NaryOp::Union, &members, topo::SweepStrategy::Realized, tol);
    match &run.end {
        NaryEnd::Built => {
            let body = &run.result().unwrap().body;
            eprintln!("PROBE {what}: Built, {} judgements", run.judgements.len());
            for (i, s) in run.steps.iter().enumerate() {
                let tops = s.body.body.faces().filter(|(_, f)| matches!(s.body.body.get_surface(f.surface), Some(Surface::Plane{normal,..}) if normal.y.abs() > 0.9)).count();
                eprintln!("PROBE   step {i}: top faces {tops}, merge groups {:?}", s.body.naming.merge_groups.iter().map(|(_, a)| 1 + a.len()).collect::<Vec<_>>());
            }
            report(&what, body, eps);
        }
        other => eprintln!("PROBE {what}: {other:?}"),
    }
    // binary fold for comparison
    let mut acc = bodies[0].clone();
    for (k, b) in bodies.iter().enumerate().skip(1) {
        match topo::union(&acc, b, tol) {
            Ok(BooleanResult::Body(bb)) => acc = bb.body,
            Ok(_) => { eprintln!("PROBE fold {what}: empty at {k}"); return; }
            Err(e) => { eprintln!("PROBE fold {what}: refused at member {k}: {e}"); return; }
        }
    }
    report(&format!("fold {what}"), &acc, eps);
}

/// A flat unit block, a unit block whose top falls `s` per unit from
/// A's edge, and a long block whose top continues B's plane exactly.
fn lever(len: f64, s_eps: f64, tol: Tol) {
    let eps = tol.eps();
    let s = s_eps * eps;
    let z = |x: f64| if x <= 1.0 { 1.0 } else { 1.0 - s * (x - 1.0) };
    let slab = |x0: f64, x1: f64| {
        let prof = [(x0, 0.0), (x1, 0.0), (x1, z(x1)), (x0, z(x0))];
        finished("slab", prism_z::<f64>(&prof, 0.0, 1.0, tol).body, tol)
    };
    let bodies = [slab(0.0, 1.0), slab(1.0, 2.0), slab(2.0, 2.0 + len)];
    let inputs: Vec<MemberInputs> = (0..3).map(|r| MemberInputs { rank: r as u32, ..MemberInputs::default() }).collect();
    let members: Vec<NaryMember<'_, f64>> = bodies.iter().zip(&inputs).map(|(body, inputs)| NaryMember { body, inputs }).collect();
    let what = format!("lever len={len} s={s_eps}eps far-vertex-off-A={:.2}eps", s_eps * (1.0 + len));
    let run = topo::nary_boolean(NaryOp::Union, &members, topo::SweepStrategy::Realized, tol);
    eprintln!("PROBE {what}: judgements {:?}", run.judgements.iter().map(|j| (j.a, j.b)).collect::<Vec<_>>());
    match &run.end {
        NaryEnd::Built => report(&what, &run.result().unwrap().body, eps),
        other => eprintln!("PROBE {what}: {other:?}"),
    }
    // the same three by the binary fold, (A u B) u C
    match topo::union(&bodies[0], &bodies[1], tol) {
        Ok(BooleanResult::Body(ab)) => {
            let ab = ab.body;
            match topo::union(&ab, &bodies[2], tol) {
                Ok(BooleanResult::Body(abc)) => report(&format!("binary {what}"), &abc.body, eps),
                Ok(_) => eprintln!("PROBE binary {what}: empty"),
                Err(e) => eprintln!("PROBE binary {what}: (AB) u C refused: {e}"),
            }
        }
        Ok(_) => eprintln!("PROBE binary {what}: empty"),
        Err(e) => eprintln!("PROBE binary {what}: A u B refused: {e}"),
    }
}

#[test]
fn chain_drift_probe() {
    for &(len, s) in &[(1.0f64, -0.4f64), (3.0, -0.4), (100.0, -0.4), (100.0, 0.4)] {
        lever(len, s, Tol::witness());
    }
    let tol = Tol::witness();
    eprintln!("PROBE eps={} K={}", tol.eps(), tol.k());
    for &(n, d) in &[(3usize, 0.3f64), (4, 0.3), (6, 0.3), (12, 0.3), (24, 0.3)] {
        single(n, d, tol);
    }
    for &(n, d) in &[(3usize, 0.3f64), (4, 0.3), (6, 0.3), (12, 0.3)] {
        nary(n, d, tol);
    }
}

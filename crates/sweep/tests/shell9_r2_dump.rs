//! **SHELL-9 review lane R2 — the cache-row differential instrument.**
//! Applied uncommitted to BOTH trees (the merge base and the head) and
//! run with `--nocapture`; `grep '^\[r2rows\]' | sort` on each side,
//! diffed line by line. Every stored pcurve row of every body `shell`
//! and `shell_open` build over R2's own corpus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Tol};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, FaceKey};

use super::shell7_common::{drum, p2, polyline, revolved, tol, tube_torus, tube_torus_hollow};
use super::shell8_common::beside;
use super::verbs_shell::{boxy, tube, vessel};

fn rows(label: &str, body: &Body<f64>) {
    let mut n = 0;
    for (he, cache) in body.pcurves() {
        n += 1;
        println!(
            "[r2rows] {label}: he {he:?} params {:?} pcurve {:?}",
            cache.params(),
            cache.pcurve()
        );
    }
    println!("[r2rows] {label}: {n} rows");
    println!(
        "[r2rows] {label}: tier3 {:?}",
        topo::validate_geometric(body, tol())
    );
}

fn shelled(label: &str, body: &Body<f64>, t: f64, open: &[FaceKey]) {
    match topo::shell_open(body, t, open, tol()) {
        Ok(s) => rows(label, &s.body),
        Err(e) => println!("[r2rows] {label}: Err {e}"),
    }
}

fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

fn sphere_zone_vase(r: f64, h: f64) -> Body<f64> {
    let c = p2(0.0, h / 2.0);
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), bulge(p2(r, 0.0), p2(r, h), c)),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

fn cone_frustum(r0: f64, r1: f64, h: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r0, 0.0), 0.0),
            ProfileVertex::new(p2(r1, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

fn two_arc_sphere() -> Body<f64> {
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

fn cap_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.y - y).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.z.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn r2_dump_the_corpus() {
    let _ = Tol::witness();
    let sphere = two_arc_sphere();
    shelled("two-arc sphere", &sphere, 0.05, &[]);

    let vase = sphere_zone_vase(1.0, 1.5);
    shelled("sphere-zone vase", &vase, 0.1, &[]);
    shelled("sphere-zone vase opened", &vase, 0.1, &cap_at_y(&vase, 1.5));

    let frustum = cone_frustum(1.0, 0.5, 2.0);
    shelled("cone frustum", &frustum, 0.05, &[]);
    shelled(
        "cone frustum opened",
        &frustum,
        0.05,
        &cap_at_y(&frustum, 2.0),
    );

    let d = drum(1.0, 2.0);
    shelled("drum", &d, 0.1, &[]);

    let tt = tube_torus(2.0, 0.5);
    shelled("tube torus solid", &tt, 0.1, &[]);
    let tth = tube_torus_hollow(2.0, 0.5, 0.1);
    shelled("tube torus hollow", &tth, 0.02, &[]);

    let b = boxy(2.0, 3.0, 4.0);
    shelled("box", &b, 0.25, &[]);
    let v = vessel(1.0, 2.0);
    shelled("vessel", &v, 0.2, &[]);
    shelled("vessel opened", &v, 0.2, &cap_at_y(&v, 2.0));
    let u = tube(0.6, 1.0, 2.0);
    shelled("tube", &u, 0.1, &[]);
    shelled("tube opened", &u, 0.1, &cap_at_y(&u, 2.0));
    let hollow = topo::shell(&v, 0.2, tol()).expect("hollows").body;
    shelled("hollow vessel again", &hollow, 0.05, &[]);

    let pair = beside(&boxy(2.0, 2.0, 2.0), &vessel(1.0, 2.0), 6.0);
    shelled("box beside vessel", &pair, 0.2, &[]);
    shelled(
        "box beside vessel opened",
        &pair,
        0.2,
        &cap_at_y(&pair, 2.0),
    );

    let dsplit = {
        let mut d2 = drum(1.0, 2.0);
        let edges: Vec<_> = d2.edges().map(|(k, _)| k).collect();
        let seam = edges
            .into_iter()
            .find(|e| super::shell7_common::same_surface(&d2, *e));
        if let Some(e) = seam {
            let _ = super::shell7_common::split_mid(&mut d2, e);
        }
        d2
    };
    println!(
        "[r2rows] drum seam split operand: tier3 {:?}",
        topo::validate_geometric(&dsplit, tol())
    );
    shelled("drum seam split unminted", &dsplit, 0.1, &[]);
}

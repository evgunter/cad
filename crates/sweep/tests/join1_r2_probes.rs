//! JOIN-1 review (lane r2) probes: poses the acceptance rows do not
//! cover, each op in both operand orders. A pose may refuse; a pose
//! that builds must pass tiers 2, 3′ and the certificate at its closed
//! form. Refusals are printed (`--nocapture`), wrong bodies fail.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, SQRT_2};

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

fn prism(origin: Point3<f64>, x: Vec3<f64>, y: Vec3<f64>, pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let n = x.cross(y);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(x, y, n),
        Vec3::new(origin.x, origin.y, origin.z),
    ));
    let lp = bulge_loop(pts.iter().map(|&(a, b)| (Point2::new(a, b), 0.0)).collect());
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(&p, Extrusion::Distance(h), tol()).unwrap().body
}

fn zprism(pts: &[(f64, f64)], z: (f64, f64)) -> Body<f64> {
    prism(
        Point3::new(0.0, 0.0, z.0),
        Vec3::unit_x(),
        Vec3::unit_y(),
        pts,
        z.1 - z.0,
    )
}

const R: f64 = 0.5;
fn rod() -> Body<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), R, tol()).unwrap();
    let p = Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .unwrap();
    extrude(&p, Extrusion::Distance(4.0), tol()).unwrap().body
}

fn seg(d: f64) -> f64 {
    R * R * (d / R).acos() - d * (R * R - d * d).sqrt()
}

fn bbox(b: &Body<f64>) -> String {
    let mut lo = [f64::MAX; 3];
    let mut hi = [f64::MIN; 3];
    for (_, p) in b.points() {
        for (i, c) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[i] = lo[i].min(c);
            hi[i] = hi[i].max(c);
        }
    }
    format!("{lo:?}..{hi:?}")
}

/// Runs ∪ ∖ ∩ in both orders. Returns (wrong bodies, outcomes).
fn probe(
    name: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    va: f64,
    vb: f64,
    ov: f64,
    abs: f64,
) -> Vec<String> {
    let mut wrong = Vec::new();
    for (ord, x, y, vx, vy) in [("A·B", a, b, va, vb), ("B·A", b, a, vb, va)] {
        for (op, r, want) in [
            ("∪", topo::union(x, y, tol()), vx + vy - ov),
            ("∖", topo::subtract(x, y, tol()), vx - ov),
            ("∩", topo::intersect(x, y, tol()), ov),
        ] {
            let what = format!("{name} {ord} {op}");
            match r {
                Err(e) => println!("[r2] {what}: REFUSES {e:?}"),
                Ok(r) => {
                    let Some(bb) = r.body() else {
                        println!("[r2] {what}: EMPTY");
                        if want.abs() > abs {
                            wrong.push(format!("{what}: empty against {want}"));
                        }
                        continue;
                    };
                    let mut bad = Vec::new();
                    if let Err(e) = topo::validate_closed(&bb.body) {
                        bad.push(format!("tier 2 {e:?}"));
                    }
                    if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                        bad.push(format!("tier 3′ {e:?}"));
                    }
                    if let Err(e) = topo::validate_geometric_certificate(&bb.body, tol()) {
                        bad.push(format!("cert {e:?}"));
                    }
                    match topo::mass_properties(&bb.body, tol()) {
                        Ok(m) if (m.volume - want).abs() <= abs => {}
                        Ok(m) => bad.push(format!("volume {} against {want}", m.volume)),
                        Err(e) => bad.push(format!("volume unmeasured {e:?}")),
                    }
                    println!(
                        "[r2] {what}: BUILDS {}",
                        if bad.is_empty() {
                            "ok".into()
                        } else {
                            bad.join("; ")
                        }
                    );
                    if !bad.is_empty() {
                        wrong.push(format!("{what}: {}", bad.join("; ")));
                    }
                }
            }
        }
    }
    wrong
}

#[test]
fn r2_probes() {
    let mut wrong = Vec::new();
    // 1. The axis lap with the operand order swapped and the cutter on either side.
    for y in [(0.0, 1.0), (-1.0, 0.0)] {
        let cutter = brick((-1.0, 1.0), y, (3.0, 4.5), tol());
        wrong.extend(probe(
            &format!("lap y∈{y:?}"),
            &rod(),
            &cutter,
            PI * R * R * 4.0,
            3.0,
            PI * R * R / 2.0,
            1e-9,
        ));
    }
    // 2. Edge-edge: a wedge prism whose corner edge lies along the box's edge x = y = 2.
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol());
    for (zr, olen) in [((1.0, 3.0), 1.0), ((-1.0, 3.0), 2.0), ((0.5, 1.5), 1.0)] {
        let inner = zprism(&[(2.0, 2.0), (1.0, 1.5), (1.5, 1.0)], zr);
        wrong.extend(probe(
            &format!("edge-edge inner z∈{zr:?}"),
            &a,
            &inner,
            8.0,
            0.375 * (zr.1 - zr.0),
            0.375 * olen,
            1e-9,
        ));
        let part = zprism(&[(2.0, 2.0), (1.0, 2.5), (1.5, 1.0)], zr);
        wrong.extend(probe(
            &format!("edge-edge partial z∈{zr:?}"),
            &a,
            &part,
            8.0,
            0.625 * (zr.1 - zr.0),
            (5.0 / 12.0) * olen,
            1e-9,
        ));
    }
    // 3. An in-face edge along the top face's diagonal, both ends vertex-vertex sites.
    let u = Vec3::new(1.0, -1.0, 0.0).normalize();
    let diag = prism(
        Point3::new(2.0, 2.0, 2.0),
        u,
        Vec3::unit_z(),
        &[(0.0, 0.0), (0.5, -0.5), (0.5, 0.5)],
        2.0 * SQRT_2,
    );
    println!("[r2] diag bbox {}", bbox(&diag));
    wrong.extend(probe(
        "diagonal",
        &a,
        &diag,
        8.0,
        0.25 * 2.0 * SQRT_2,
        2.0 * SQRT_2 / 8.0 - 1.0 / 12.0,
        1e-9,
    ));
    // 4. A straight cutter edge lying on the rod's wall (a ruling, not a rod edge), and
    //    the control where the cutter's face meets the wall in a ruling that is no edge.
    for (z, len) in [((-1.0, 5.0), 4.0), ((3.0, 4.5), 1.0)] {
        let on = brick((0.3, 2.0), (-2.0, 0.4), z, tol());
        let vb = 1.7 * 2.4 * (z.1 - z.0);
        wrong.extend(probe(
            &format!("ruling-is-edge z∈{z:?}"),
            &rod(),
            &on,
            PI * R * R * 4.0,
            vb,
            seg(0.3) * len,
            1e-9,
        ));
        let off = brick((0.3, 2.0), (-2.0, 0.2), z, tol());
        let vb = 1.7 * 2.2 * (z.1 - z.0);
        wrong.extend(probe(
            &format!("ruling-not-edge z∈{z:?}"),
            &rod(),
            &off,
            PI * R * R * 4.0,
            vb,
            0.093_177_264_958_195_95 * len,
            1e-9,
        ));
    }
    // 5. The seam of a one-face cylindrical wall along a cutter edge (edge-edge, both
    //    flanks of the seam are ONE face).
    let tube = revolve(
        &validated(vec![ProfileLoop::polygon([
            Point2::new(0.5, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 4.0),
            Point2::new(0.5, 4.0),
        ])]),
        axis_y(),
        Revolution::Full,
        tol(),
    )
    .unwrap()
    .body;
    println!("[r2] tube bbox {}", bbox(&tube));
    for (y0, y1, olen) in [(1.0, 3.0, 2.0), (-1.0, 5.0, 4.0)] {
        for sx in [1.0, -1.0] {
            // (x, z) triangle, mirrored by sx in case the seam is at x = −1.
            let pts: Vec<(f64, f64)> = [(1.0, 0.0), (0.7, 0.09), (1.25, -0.5)]
                .iter()
                .map(|&(x, z)| (sx * x, z))
                .collect();
            let pts = if sx < 0.0 {
                pts.into_iter().rev().collect::<Vec<_>>()
            } else {
                pts
            };
            let w = prism(
                Point3::new(0.0, y1, 0.0),
                Vec3::unit_x(),
                Vec3::unit_z(),
                &pts,
                y1 - y0,
            );
            println!("[r2] seam wedge bbox {}", bbox(&w));
            wrong.extend(probe(
                &format!("seam sx={sx} y∈[{y0},{y1}]"),
                &tube,
                &w,
                PI * 0.75 * 4.0,
                0.06375 * (y1 - y0),
                0.032_998_647_756_249_28 * olen,
                1e-9,
            ));
        }
    }
    assert!(wrong.is_empty(), "WRONG BODIES:\n{}", wrong.join("\n"));
}

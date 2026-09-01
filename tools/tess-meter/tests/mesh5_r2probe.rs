//! R2 review probes for MESH-5 (PR 1507, issue 685). ADDITIVE ONLY —
//! nothing in the PR depends on this file.
//!
//! Three green reporters in the `r2_bytes.rs` shape (run at two
//! revisions and diff the printed tables):
//!
//! 1. `r2_wedge_bytes` — FNV hash over the emitted mesh of the π/6
//!    cone wedge at the measurement's nine δ plus the apex-free
//!    frustum wedge, i.e. a two-build BYTE instrument whose bodies
//!    actually REACH the decided `nu == 1` cone branch. (The adopted
//!    tour bodies in `mesh/tests/r2_bytes.rs` never do: its `cone()` is
//!    a full revolve, `nu >= 8`.)
//! 2. `r2_overflow_edge_needle_cone` — the disclosed behavior edge
//!    executed through the public door: a revolve-minted needle cone
//!    (half-angle ≈ 5e-8 rad) whose skipped `ceil_count(vspan, ρ_max·hu)`
//!    is ≈ 2.5e7 ≥ 2^24. At the merge base this REFUSES
//!    `ResolutionOverflow`; at the PR head it serves — the probe prints
//!    which, and when served asserts the mesh still passes
//!    `check_mesh` (the per-triangle certificate already gated it
//!    inside `tessellate`).
//! 3. `r2_cone_shape_sweep` — tall / squat / near-degenerate cone
//!    wedges at their own `nu == 1` δ, printing triangles, watertight,
//!    densely sampled max deviation, AND the rim-chord sagitta
//!    prediction `ρ_rim·(1 − cos(chord_span/2))·cos(half_angle)` —
//!    if measured max_dev matches the prediction, the binding
//!    deviation is the rim chord's azimuthal sagitta and v-rows cannot
//!    buy any of it. Diff this table against a build with the
//!    "honour the schedule" mutant to see whether any shape's rows
//!    move the deviation.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec2};
use mesh::validate::check_mesh;
use profile::{Profile, ProfileLoop, RawLoop as _, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn revolve_polygon(pts: &[Point2<f64>], theta: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().copied());
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Partial(theta), Tol::witness())
        .unwrap()
        .body
}

fn fnv(bytes: &[u8], h: &mut u64) {
    for b in bytes {
        *h ^= u64::from(*b);
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

fn mesh_hash(m: &mesh::Mesh) -> (usize, usize, u64) {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for p in &m.positions {
        for c in [p.x, p.y, p.z] {
            fnv(&c.to_bits().to_le_bytes(), &mut h);
        }
    }
    let mut nt = 0usize;
    for fp in &m.patches {
        for t in &fp.triangles {
            nt += 1;
            for i in t {
                fnv(&i.to_le_bytes(), &mut h);
            }
        }
    }
    for pl in &m.boundaries {
        for i in &pl.points {
            fnv(&i.to_le_bytes(), &mut h);
        }
    }
    (m.positions.len(), nt, h)
}

/// Exact distance to the complete cone locus (both nappes; apex
/// fallback) — the acceptance suite's oracle, inlined as in
/// `mesh5_probe.rs`.
fn dist_to_cone(
    apex: Point3<f64>,
    axis: geom_core::Vec3<f64>,
    half_angle: f64,
    p: Point3<f64>,
) -> f64 {
    let w = p - apex;
    let h = w.dot(axis);
    let rho = (w - axis * h).norm();
    let (s, c) = half_angle.sin_cos();
    let mut best = f64::INFINITY;
    for hh in [h, -h] {
        if rho * s + hh * c >= 0.0 {
            best = best.min((rho * c - hh * s).abs());
        }
    }
    best.min(w.norm())
}

/// (cone-patch triangles, densely sampled max |S − Π| over the cone
/// patch, n = 8 barycentric divisions → 45 samples/triangle).
fn cone_patch_report(body: &Body<f64>, mesh: &mesh::Mesh) -> (usize, f64) {
    let mut tris = 0usize;
    let mut max_dev = 0.0_f64;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).unwrap();
        let Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } = *body.get_surface(face.surface).unwrap()
        else {
            continue;
        };
        tris += patch.triangles.len();
        for t in &patch.triangles {
            let [a, b, c] = [
                mesh.positions[t[0] as usize],
                mesh.positions[t[1] as usize],
                mesh.positions[t[2] as usize],
            ];
            let n = 8u32;
            for i in 0..=n {
                for j in 0..=(n - i) {
                    let k = n - i - j;
                    let (li, lj, lk) = (
                        f64::from(i) / f64::from(n),
                        f64::from(j) / f64::from(n),
                        f64::from(k) / f64::from(n),
                    );
                    let p = Point3::origin()
                        + (a - Point3::origin()) * li
                        + (b - Point3::origin()) * lj
                        + (c - Point3::origin()) * lk;
                    max_dev = max_dev.max(dist_to_cone(apex, axis, half_angle, p));
                }
            }
        }
    }
    (tris, max_dev)
}

/// Probe 1: byte hashes of the bodies that REACH the decided branch.
#[test]
fn r2_wedge_bytes() {
    let theta = core::f64::consts::FRAC_PI_6;
    let wedge = revolve_polygon(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
        ],
        theta,
    );
    let frustum = revolve_polygon(
        &[
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(1.0, 1.0),
        ],
        theta,
    );
    println!("R2 WEDGE HASHES (nu == 1 branch reached at the coarse deltas)");
    for (name, body) in [("cone_wedge_pi6", &wedge), ("frustum_wedge_pi6", &frustum)] {
        for d in [0.25, 0.1, 0.07, 0.0682, 0.05, 0.025, 0.01, 0.004, 0.001] {
            let m = mesh::tessellate(body, d, Tol::witness()).unwrap();
            let (n, t, h) = mesh_hash(&m);
            println!("{name} d={d} n={n} t={t} => {h:016x}");
        }
    }
}

/// Probe 2: the overflow behavior edge, through the public door.
#[test]
fn r2_overflow_edge_needle_cone() {
    // Slant (0,0) → (5e-7, 10): dr = 5e-7 (500× ambient ε — decidedly
    // a cone, not a cylinder), dz = 10, half-angle = atan(5e-8).
    // ρ_max = 5e-7, so at δ = 0.1 (δ_s = 0.05 ≥ ρ_max) hu caps at π/4
    // and the merge base's v-schedule count is
    // ceil(vspan/(ρ_max·hu)) ≈ ceil(10/(5e-7·π/4)) ≈ 2.55e7 ≥ 2^24.
    // A Partial(0.5) wedge keeps nu = ceil(0.5/(π/4)) = 1.
    let needle = revolve_polygon(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(5e-7, 10.0),
            Point2::new(0.0, 10.0),
        ],
        0.5,
    );
    let half_angle = needle
        .faces()
        .find_map(|(_, f)| match *needle.get_surface(f.surface).unwrap() {
            Surface::Cone { half_angle, .. } => Some(half_angle),
            _ => None,
        })
        .expect("the revolve minted a cone face");
    println!("R2 NEEDLE CONE: public-door half_angle = {half_angle:.3e} rad");
    match mesh::tessellate(&needle, 0.1, Tol::witness()) {
        Ok(m) => {
            let wt = check_mesh(&m).is_ok();
            let (tris, max_dev) = cone_patch_report(&needle, &m);
            println!(
                "R2 NEEDLE CONE d=0.1: SERVED cone_tris={tris} watertight={wt} \
                 max_dev={max_dev:.3e}"
            );
            assert!(wt, "a served needle-cone mesh must be watertight");
            assert!(
                max_dev <= 0.1,
                "served mesh out of chordal tolerance: {max_dev}"
            );
        }
        Err(e) => println!("R2 NEEDLE CONE d=0.1: REFUSED {e:?}"),
    }
}

/// Probe 3: does any cone SHAPE let v-rows buy deviation at `nu == 1`?
#[test]
fn r2_cone_shape_sweep() {
    let theta = core::f64::consts::FRAC_PI_6;
    // (name, profile polygon, deltas at which nu == 1).
    let shapes: [(&str, [Point2<f64>; 3], &[f64]); 4] = [
        (
            "tall a~0.10",
            [
                Point2::new(0.0, 0.0),
                Point2::new(0.1, 0.0),
                Point2::new(0.0, 1.0),
            ],
            &[0.25, 0.1],
        ),
        (
            "ref a=pi/4",
            [
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(0.0, 1.0),
            ],
            &[0.25, 0.1, 0.07, 0.0682],
        ),
        (
            "squat a~1.47",
            [
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(0.0, 0.1),
            ],
            &[0.25, 0.1],
        ),
        (
            "degen a~1.5698",
            [
                Point2::new(0.0, 0.0),
                Point2::new(1.0, 0.0),
                Point2::new(0.0, 0.001),
            ],
            &[0.25, 0.1],
        ),
    ];
    println!(
        "R2 CONE SHAPE SWEEP (pi/6 wedges): tris / watertight / measured \
         max_dev vs rim-chord sagitta prediction"
    );
    for (name, poly, deltas) in shapes {
        let body = revolve_polygon(&poly, theta);
        let (rho_rim, half_angle) = body
            .faces()
            .find_map(|(_, f)| match *body.get_surface(f.surface).unwrap() {
                Surface::Cone { half_angle, .. } => Some((poly[1].x, half_angle)),
                _ => None,
            })
            .expect("cone face");
        for &d in deltas {
            match mesh::tessellate(&body, d, Tol::witness()) {
                Ok(m) => {
                    let wt = check_mesh(&m).is_ok();
                    let (tris, max_dev) = cone_patch_report(&body, &m);
                    // Rim boundary chord count from the boundary pass is
                    // not public; infer the chord span from the widest
                    // gap between adjacent rim vertices? Cheaper and
                    // sufficient here: predict with the FULL wedge span
                    // (one chord) and let a mismatch show as
                    // prediction > measured.
                    let sag = rho_rim * (1.0 - (theta / 2.0).cos()) * half_angle.cos();
                    println!(
                        "{name:>16} d={d:<7} tris={tris:<4} watertight={wt} \
                         max_dev={max_dev:.4e} one_chord_sagitta={sag:.4e}"
                    );
                }
                Err(e) => println!("{name:>16} d={d:<7} REFUSED {e:?}"),
            }
        }
    }
}

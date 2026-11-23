Probe source for the two-hole MAJOR, as asked. Source only.

TREE: detached at 553d4d6310a82bca1a7b1954c1b40f0c5e2c5ead (frozen head), `crates/mesh/src` untouched for this run. Two local additions, neither committed: the file below as `crates/mesh/tests/rv_probe.rs`, and two lines appended to `crates/mesh/tests/all.rs`:
```
#[path = "rv_probe.rs"]
mod rv_probe;
```
COMMAND (from the repo root):
`cargo nextest run -p mesh --test all --profile default -E 'test(rv_probe::rv_two_holes)' --no-capture --ignore-default-filter`
ENVIRONMENT: debug (`test` profile), default features, no env switch; `CAD_TOLERANCE_EPS` unset, so ε is the default 1e-9 (`reach_probes::tol()` = `Tol::witness()`). Toolchain 1.97.0, 4 vCPUs. (At `CAD_TOLERANCE_EPS=1e-6` the probe panics in a helper before reaching the mesher, so it says nothing there; 1e-12 not yet read.)

SOURCE (`rv_probe.rs`, the part the two-hole probe uses; every helper not on the head is here, the rest are `reach_probes::{geometry, op_of, posed, rod, tol}` and `sweep::test_support::{brick, finished}`):
```rust
//! Reviewer probe (not committed): walls the suites do not hold.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::cast_precision_loss)]

use geom::SurfaceKind;
use geom_core::{Affine3, Point3, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body};

use crate::reach_probes::{geometry, op_of, posed, rod, tol};

fn stub(x: (f64, f64), w: f64, z: (f64, f64), turn: f64) -> AtRestBody<f64> {
    posed(
        "the bar",
        brick(x, (-w, w), z, tol()),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
    )
}

fn cyl_tris(body: &Body<f64>, mesh: &mesh::Mesh) -> usize {
    mesh.patches
        .iter()
        .filter(|p| body.face_surface(p.face).unwrap().kind() == SurfaceKind::Cylinder)
        .map(|p| p.triangles.len())
        .sum()
}

fn outcome(body: &Body<f64>, delta: f64) -> String {
    match mesh::tessellate(body, delta, tol()) {
        Ok(m) => {
            let c = mesh::validate::check_mesh(&m).map_err(|e| format!("{e:?}"));
            let g = geometry(body, &m, delta);
            match (c, g) {
                (Ok(()), Ok(())) => format!("ok {}", cyl_tris(body, &m)),
                (c, g) => format!("MESHED-BAD check={c:?} geom={g:?}"),
            }
        }
        Err(e) => {
            let s = format!("{e:?}");
            format!("REFUSED {}", &s[..s.len().min(160)])
        }
    }
}

/// The rim's chord count on the plain rod at the chart lane's target.
/// (The rod's rim is two half-circle edges; this is one edge's count.)
fn rim_chords(r: f64, delta: f64) -> usize {
    let b = rod(r, 0.0, 1.0);
    let m = mesh::tessellate(&b, delta * 0.5, tol()).unwrap();
    m.boundaries.iter().map(|b| b.points.len()).max().unwrap() - 1
}

/// Two small holes at mid-height, `gap` columns of free wall between them.
#[test]
fn rv_two_holes() {
    let mut bad = 0;
    for (scale, delta_rel) in [(1.0f64, 1e-2f64), (1.0, 1e-3), (1e-3, 1e-2), (10.0, 1e-2), (1.0, 1e-1)] {
        let r = scale;
        let delta = delta_rel * scale;
        let n = rim_chords(r, delta);
        let a = std::f64::consts::PI / n as f64;
        println!("delta {delta:e} r {r}: rim chords {n}, column {a:.5} rad");
        for hmul in [1.0f64, 1.33, 1.9] {
            let h = hmul * r;
            let pipe = finished("the pipe", rod(r, -0.5 * h, h), tol());
            for gap in [1.4f64, 1.5, 1.6] {
                for phase in [0.0f64, 0.25, 0.5, 0.75] {
                    let w = 0.04 * a * r; // half-width of a hole
                    let c = (9.0 + phase) * a; // centre of the free wall
                    let half = 0.5 * gap * a + w / r;
                    let d = 0.02 * h;
                    let s1 = stub((0.5 * r, 1.1 * r), w, (-d, d), c - half);
                    let s2 = stub((0.5 * r, 1.1 * r), w, (-d, d), c + half);
                    let one = finished("one hole", op_of(&pipe, &s1, "A ∖ B"), tol());
                    let two = op_of(&one, &s2, "A ∖ B");
                    let got = outcome(&two, delta);
                    if !got.starts_with("ok") {
                        bad += 1;
                    }
                    println!("  h {hmul} gap {gap} phase {phase}: {got}");
                }
            }
        }
    }
    println!("rv_two_holes: {bad} not ok");
}
```
One caveat on the source: the run whose output is below had `(1e3, 1e-2)` where the list now reads `(10.0, 1e-2)`; at scale 1e3 a helper (`reach_probes::posed`, line 442) panics, after the first three scale rows had printed. The r=10 and δ=0.1 rows of the list as written have run inside the suite runs but I have not read their output yet.

RAW OUTPUT, every line that is not `ok` (all other cells of the three scale rows printed `ok 68` or their own count):
```
delta 1e-2 r 1: rim chords 16, column 0.19635 rad
  h 1 gap 1.5 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 0.010823478187066082, requested: 0.01 }
  h 1 gap 1.6 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 0.012311646773133789, requested: 0.01 }
  h 1.9 gap 1.5 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 0.010823478187066082, requested: 0.01 }
delta 1e-3 r 1: rim chords 50, column 0.06283 rad
  h 1.33 gap 1.5 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 0.0011101249133922408, requested: 0.001 }
  h 1.33 gap 1.6 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 0.0012630432610423181, requested: 0.001 }
delta 1e-5 r 0.001: rim chords 16, column 0.19635 rad
  h 1 gap 1.5 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 1.0823478187066199e-5, requested: 1e-5 }
  h 1 gap 1.6 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 1.231164677313384e-5, requested: 1e-5 }
  h 1.9 gap 1.5 phase 0.5: REFUSED CertificateExceeded { face: FaceKey(3v1), bound: 1.0823478187066199e-5, requested: 1e-5 }
delta 1e1 r 1000: rim chords 16, column 0.19635 rad
thread 'rv_probe::rv_two_holes' panicked at crates/mesh/tests/reach_probes.rs:442:5
```
Still pending and not to be assumed: the same bodies on main's mesher (queued behind the head's `sweep` run).

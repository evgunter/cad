//! Review of PR 6002: probes of `chord_join::arc_leaving`'s two Zero
//! arms on a needle cone cut near its apex. Prints one `PROBE` line per
//! pose; the reviewer reads the lines rather than asserting. The review
//! read each `arc_leaving` reading (datum, its cosine with the conic's
//! tangent, the lever, the sign) through a local, uncommitted print at
//! that function: every split Section Zero had cosine 1 (the short lever,
//! not an off-tangent datum), and the boolean's germ arm never read Zero.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::needle;
use geom_core::{Point3, Tol, Vec3};

fn head(s: &str) -> String {
    s.chars().take(260).collect()
}

/// The foot is `kε` down the ruling at azimuth `az` (0 is the seam).
fn run(
    body: &topo::AtRestBody<f64>,
    deg: f64,
    az: f64,
    k: f64,
    tilt: f64,
    spin: f64,
    tol: Tol,
) -> String {
    let a = deg.to_radians();
    let apex = Point3::new(0.0, 1.0, 0.0);
    let g = Vec3::new(a.sin() * az.cos(), -a.cos(), a.sin() * az.sin());
    let n = Vec3::new(tilt.sin() * spin.cos(), tilt.cos(), tilt.sin() * spin.sin());
    let plane = topo::test_support::split_plane(apex + g * (k * tol.eps()), n, tol);
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        topo::splitting::split(body, &plane, tol)
    })) {
        Err(_) => "PANIC".into(),
        Ok(Ok(r)) => format!(
            "Ok above {} below {}",
            r.above.body().is_some(),
            r.below.body().is_some()
        ),
        Ok(Err(e)) => head(&format!("{e:?}")),
    }
}

#[test]
#[ignore = "review probe: prints PROBE lines"]
fn review_6002_the_prs_poses() {
    let tol = Tol::witness();
    for deg in [1.0, 3.0] {
        let body = needle(deg, None, tol);
        for (tilt, spin) in [(1.5, 0.0), (1.2, 2.0)] {
            eprintln!(
                "PROBE α {deg} k 1e3 tilt {tilt} spin {spin}: {}",
                run(&body, deg, 0.0, 1e3, tilt, spin, tol)
            );
        }
    }
}

/// Planes whose foot is within tens of ε of the apex, nearly along the
/// axis, at and off the seam: the apex site's germ arm.
#[test]
#[ignore = "review probe: prints PROBE lines"]
fn review_6002_germ_arm_hunt() {
    let tol = Tol::witness();
    for deg in [1.0f64, 3.0, 5.0] {
        for window in [None, Some(4.0)] {
            let body = needle(deg, window, tol);
            for az in [0.0f64, 0.7, std::f64::consts::PI] {
                for k in [
                    0.1f64, 0.5, 1.0, 2.0, 4.0, 8.0, 15.0, 25.0, 40.0, 60.0, 100.0,
                ] {
                    for tilt in [0.4f64, 0.9, 1.2, 1.4, 1.5, 1.53] {
                        for spin in [0.0f64, 0.7, 2.0, std::f64::consts::PI, 4.5] {
                            let got = run(&body, deg, az, k, tilt, spin, tol);
                            eprintln!(
                                "PROBE α {deg} w {window:?} az {az:.2} k {k} tilt {tilt} spin {spin}: {got}"
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The same planes as a block's top face, so the boolean's germ datum
/// reaches the chord: needle ∩ and ∖ the block.
#[test]
#[ignore = "review probe: prints PROBE lines"]
fn review_6002_boolean_germ_hunt() {
    use geom_core::{Band, UnitVec3, linalg::RigidMotion};
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for deg in [1.0f64, 3.0] {
        let a = deg.to_radians();
        let body = needle(deg, None, tol);
        for k in [2.0f64, 10.0, 40.0, 1e3, 1e4] {
            for tilt in [0.4f64, 1.2, 1.5] {
                for spin in [0.0f64, 2.0] {
                    let apex = Point3::new(0.0, 1.0, 0.0);
                    let g = Vec3::new(a.sin(), -a.cos(), 0.0);
                    let foot = apex + g * (k * tol.eps());
                    let n = Vec3::new(tilt.sin() * spin.cos(), tilt.cos(), tilt.sin() * spin.sin());
                    let axis = Vec3::new(n.z, 0.0, -n.x);
                    let bx = sweep::test_support::brick((-2.0, 2.0), (-3.0, 0.0), (-2.0, 2.0), tol);
                    let turned = topo::transform_rigid(
                        &bx,
                        &RigidMotion::rotation_then_translation(
                            UnitVec3::new(axis, "axis", band).unwrap(),
                            tilt,
                            foot - Point3::origin(),
                        )
                        .unwrap(),
                        tol,
                    );
                    let Ok(turned) = turned else {
                        eprintln!(
                            "PROBE bool α {deg} k {k} tilt {tilt} spin {spin}: transform {turned:?}"
                        );
                        continue;
                    };
                    let bx = sweep::test_support::finished("block", turned, tol);
                    for (op, name) in [(0, "∩"), (1, "∖")] {
                        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            if op == 0 {
                                topo::intersect(&body, &bx, tol)
                            } else {
                                topo::subtract(&body, &bx, tol)
                            }
                        }));
                        let got = match res {
                            Err(_) => "PANIC".to_string(),
                            Ok(Ok(r)) => format!("Ok {}", r.body().is_some()),
                            Ok(Err(e)) => head(&format!("{e:?}")),
                        };
                        eprintln!("PROBE bool{name} α {deg} k {k} tilt {tilt} spin {spin}: {got}");
                    }
                }
            }
        }
    }
}

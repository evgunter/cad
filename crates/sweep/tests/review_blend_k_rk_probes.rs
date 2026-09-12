//! **Review probes, BLEND unit K** — the cap-rim crossover varied on
//! both sides of `K* = √φ`, by the tilt's direction against the short
//! chord, and by the sketch plane's pose; each row re-execs this binary
//! at a chosen `CAD_AMBIGUITY_K` (the `fillet_h6_cap_rim` pattern) and
//! reads the outcome back off `RKPROBE` lines.
//!
//! The algebra the rows pin: the worst admitted vector `(ε, 0, K·ε)`
//! tilts a line wall whose chord is PERPENDICULAR to the in-plane
//! component by `sin θ = K/√(K² + 1)`; the wedge margin on a rim of
//! chord `f·K·ε` is `f·K·sin θ` (in ε), Smooth at `≤ 1`, in band below
//! `K`, transverse at `≥ K`. A chord PARALLEL to the in-plane
//! component has `sin θ = 1` and reads the arm alone.
//!
//! The in-plane component is built at [`IN_PLANE`]·ε rather than at ε
//! exactly: `extrusion_obliquity`'s band is OPEN at ε, so a vector one
//! ulp over the threshold escalates rather than being admitted, and on
//! a tilted sketch plane — whose frame axes are irrational — building
//! `u·ε + n·K·ε` and reading its in-plane part back rounds. It came
//! back one ulp high at ε = 1e-12, and the gate refused it, correctly.
//! These rows want a vector both gates ADMIT, so they sit a billionth
//! inside the threshold instead of exactly on it; every margin above
//! is unchanged to twelve digits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cap_rims::chart_counts;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::validate_geometric;

/// The in-plane component's size as a fraction of ε (module docs): as
/// close to the obliquity threshold as a rotated frame can be read
/// back from, and below it on every frame and every ε row.
const IN_PLANE: f64 = 1.0 - 1e-9;

fn rect(plane: SketchPlane<f64>, sx: f64, sy: f64) -> ValidatedProfile<f64> {
    let p2 = Point2::<f64>::new;
    Profile::new(
        plane,
        vec![ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(sx, 0.0),
            p2(sx, sy),
            p2(0.0, sy),
        ])],
    )
    .validate(Tol::witness())
    .unwrap()
}

/// The sketch plane under `RK_PLANE`: `xy` (default) or `tilted` (the
/// frame `fillet_h6_cap_rim`'s tilted row uses).
fn plane_under_env() -> SketchPlane<f64> {
    match std::env::var("RK_PLANE").as_deref() {
        Ok("tilted") => SketchPlane::from_frame(
            Point3::new(0.3, -0.2, 0.7),
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(-1.0, 1.0, 2.0).normalize(),
        ),
        _ => SketchPlane::xy(),
    }
}

/// Prints the body at the run's K: `rect(2, f·K·ε)` on the chosen
/// plane, extruded by `u·ε + n·K·ε` (`RK_TILT=x`, perpendicular to the
/// short chord) or `v·ε + n·K·ε` (`RK_TILT=y`, along it).
#[test]
#[ignore]
fn print_rk_probe() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    println!("RKPROBE k={k}");
    let f: f64 = std::env::var("H6_ARM_FACTOR")
        .expect("H6_ARM_FACTOR")
        .parse()
        .expect("a float");
    let plane = plane_under_env();
    let place = plane.placement;
    let (u, v, n) = (place.linear.c0, place.linear.c1, place.linear.c2);
    let tilt = std::env::var("RK_TILT").unwrap_or_else(|_| "x".to_string());
    let in_plane = match tilt.as_str() {
        "y" => v,
        _ => u,
    };
    let w = in_plane * (IN_PLANE * eps) + n * (k * eps);
    let short = f * k * eps;
    match extrude(&rect(plane, 2.0, short), Extrusion::Vector(w), tol) {
        Ok(built) => {
            println!("RKPROBE extrude=Ok");
            let (conventional, wall_chart, cap_chart) = chart_counts(&built);
            println!("RKPROBE non_intersection_rims={conventional}");
            println!("RKPROBE wall_chart_rims={wall_chart}");
            println!("RKPROBE cap_chart_rims={cap_chart}");
            match validate_geometric(&built.body, tol) {
                Ok(()) => println!("RKPROBE tier3=Ok"),
                Err(errs) => {
                    println!("RKPROBE tier3=Err n={}", errs.len());
                    for e in &errs {
                        println!("RKPROBE tier3_error={e:?}");
                    }
                }
            }
        }
        Err(e) => println!("RKPROBE extrude=Err {e:?}"),
    }
}

fn at(k: &str, f: &str, tilt: &str, plane: &str) -> String {
    let exe = std::env::current_exe().expect("this test binary's own path");
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::print_rk_probe"),
        None => "print_rk_probe".to_string(),
    };
    let out = std::process::Command::new(&exe)
        .args([probe.as_str(), "--ignored", "--exact", "--nocapture"])
        .env("CAD_AMBIGUITY_K", k)
        .env("H6_ARM_FACTOR", f)
        .env("RK_TILT", tilt)
        .env("RK_PLANE", plane)
        .output()
        .expect("the re-exec runs");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        text.contains(&format!("RKPROBE k={k}")),
        "K = {k} did not reach the child process:\n{text}",
    );
    eprintln!("--- K={k} f={f} tilt={tilt} plane={plane}\n{text}");
    text
}

fn assert_smooth_built_and_refused_at_rest(text: &str, label: &str) {
    for needle in [
        "RKPROBE extrude=Ok",
        "RKPROBE non_intersection_rims=4",
        "RKPROBE wall_chart_rims=4",
        "RKPROBE cap_chart_rims=0",
        "RKPROBE tier3=Err n=4",
    ] {
        assert!(
            text.contains(needle),
            "{label}: expected {needle:?}:\n{text}"
        );
    }
    assert_eq!(
        text.matches("material_wedge_side").count(),
        4,
        "{label}:\n{text}"
    );
    assert_eq!(
        text.matches("SliverDihedral").count(),
        4,
        "{label}:\n{text}"
    );
}

fn assert_all_transverse_and_valid(text: &str, label: &str) {
    for needle in [
        "RKPROBE extrude=Ok",
        "RKPROBE non_intersection_rims=0",
        "RKPROBE tier3=Ok",
    ] {
        assert!(
            text.contains(needle),
            "{label}: expected {needle:?}:\n{text}"
        );
    }
}

fn assert_in_band_rim(text: &str, label: &str) {
    assert!(
        text.contains("RKPROBE extrude=Err SliverRim") && text.contains("dihedral_wedge"),
        "{label}: expected the typed in-band rim refusal under dihedral_wedge:\n{text}",
    );
}

/// **Either side of `K* = √φ ≈ 1.272`, at the tightest admitted arm.**
/// `f = 1.002`: K = 1.1 → 0.814, K = 1.2 → 0.924, K = 1.25 → 0.978
/// (Smooth: built, refused at rest); K = 1.28 → 1.011, K = 1.3 → 1.033
/// (in band: refused at the door as `SliverRim`). Above the crossover
/// no admitted `f ≥ 1` reaches Smooth, since `K·sin θ > 1` there.
#[test]
fn the_crossover_sits_where_the_algebra_says() {
    for k in ["1.1", "1.2", "1.25"] {
        assert_smooth_built_and_refused_at_rest(&at(k, "1.002", "x", "xy"), k);
    }
    for k in ["1.28", "1.3"] {
        assert_in_band_rim(&at(k, "1.002", "x", "xy"), k);
    }
    // A comfortable arm clears the band on both sides: K = 1.1 → 1.22
    // ≥ 1.1; K = 1.3 → 1.545 ≥ 1.3.
    for k in ["1.1", "1.3"] {
        assert_all_transverse_and_valid(&at(k, "1.5", "x", "xy"), k);
    }
}

/// **The tilt along the short chord reaches no smooth rim at any K.**
/// With `w = (0, ε, K·ε)` the short walls' normal is exactly the sketch
/// x-axis (`sin θ = 1`, margin `f·K·ε` — transverse by the arm's own
/// 0.2 % clearance) and the LONG rims carry the tilt against a 2 m arm.
#[test]
fn the_tilt_along_the_short_chord_leaves_every_rim_transverse() {
    for k in ["1.1", "1.2", "1.3"] {
        assert_all_transverse_and_valid(&at(k, "1.002", "y", "xy"), k);
    }
}

/// **On a tilted sketch plane the same rows read the same**: the wall
/// chart certifies the smooth rim below the crossover and the at-rest
/// gate refuses the body, exactly as on `xy`.
#[test]
fn a_tilted_sketch_plane_moves_nothing() {
    assert_smooth_built_and_refused_at_rest(&at("1.1", "1.002", "x", "tilted"), "tilted K=1.1");
    assert_in_band_rim(&at("1.3", "1.002", "x", "tilted"), "tilted K=1.3");
    assert_all_transverse_and_valid(&at("1.1", "1.002", "y", "tilted"), "tilted y K=1.1");
}

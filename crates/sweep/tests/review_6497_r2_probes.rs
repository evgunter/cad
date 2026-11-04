//! Review R2 probes for PR 6497 (not for merge).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, revolve};

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The football's arc drawn as two halves meeting at the widest point.
fn football2(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let h = (r * r - big * big).sqrt();
    let v_star = (-big / r).acos();
    let b = (v_star / 4.0).tan();
    bulge_loop(vec![
        (Point2::new(0.0, -h), b),
        (Point2::new(big + r, 0.0), b),
        (Point2::new(0.0, h), 0.0),
    ])
}

fn football(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let h = (r * r - big * big).sqrt();
    let v_star = (-big / r).acos();
    bulge_loop(vec![
        (Point2::new(0.0, -h), (v_star / 2.0).tan()),
        (Point2::new(0.0, h), 0.0),
    ])
}

/// Pappus for the apple side (R > 0), as the PR's oracle.
fn pappus_apple(big: f64, r: f64) -> (f64, f64) {
    let d = big;
    let alpha = (d / r).acos();
    let segment = r * r * alpha - d * (r * r - d * d).sqrt();
    let centroid = 2.0 * (r * r - d * d).powf(1.5) / (3.0 * segment);
    let moment = PI * r * r * big - segment * (big - centroid);
    let rest = PI - alpha;
    let arc = 2.0 * r * rest;
    (
        2.0 * PI * moment,
        2.0 * PI * arc * (big + r * alpha.sin() / rest),
    )
}

/// Report every stage of one profile; returns a summary string.
fn run(what: &str, lp: profile::ProfileLoop<f64>, exact: Option<(f64, f64)>) -> String {
    let tol = Tol::witness();
    let vp = match profile::Profile::new(profile::SketchPlane::xy(), vec![lp]).validate(tol) {
        Ok(v) => v,
        Err(e) => return format!("{what}: profile refuses: {e:?}"),
    };
    let built = match revolve(&vp, y_axis(), Revolution::Full, tol) {
        Ok(b) => b,
        Err(e) => return format!("{what}: revolve refuses: {e}"),
    };
    let mut out = String::new();
    for (_, f) in built.body.faces() {
        if let Some(geom::Surface::Spindle {
            major_radius,
            minor_radius,
            ..
        }) = built.body.get_surface(f.surface)
        {
            out += &format!(" sheet(R={major_radius:e}, r={minor_radius})");
        }
    }
    for (_, v) in built.body.vertex_points() {
        out += &format!(" v({:e},{:e},{:e})", v.x, v.y, v.z);
    }
    match mesh::tessellate(&built.body, 1e-2, tol) {
        Ok(m) => {
            let ok = mesh::validate::check_mesh(&m).is_ok();
            out += &format!(" | mesh ok={ok} vol={}", mesh::validate::signed_volume(&m));
        }
        Err(e) => out += &format!(" | mesh refuses {e:?}"),
    }
    match topo::AtRestBody::validate(built.body, tol) {
        Err(e) => out += &format!(" | tier3 refuses {e:?}"),
        Ok(body) => {
            match topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)) {
                Err(e) => out += &format!(" | props refuses {e:?}"),
                Ok(p) => {
                    let g = p.enclosure;
                    out += &format!(
                        " | props V={} (pad {:e}) A={} (pad {:e})",
                        g.volume, g.volume_pad, g.surface_area, g.area_pad
                    );
                    if let Some((v, a)) = exact {
                        out += &format!(
                            " exact V={v} A={a} dV={:e} dA={:e}",
                            g.volume - v,
                            g.surface_area - a
                        );
                    }
                }
            }
        }
    }
    format!("{what}:{out}")
}

/// Claim 1: an apple drawn apex to apex whose reach `rc − r` is inside
/// the band's zero. The snap stores a horn, whose one cusp is the
/// centre's foot, √(2r|d|) from each drawn apex vertex.
#[test]
fn probe_near_horn_apple_apex_to_apex() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let z = band.zero();
    eprintln!("band zero {z:e} escalate {:e}", band.escalate());
    for f in [0.5, 0.1, 0.01] {
        let d = -f * z;
        let big = 1.0 + d;
        eprintln!(
            "{}",
            run(
                &format!("apple R=1{d:+e}"),
                football2(big, 1.0),
                Some(pappus_apple(big, 1.0))
            )
        );
    }
    // Control: plainly an apple.
    eprintln!(
        "{}",
        run(
            "apple R=0.9",
            football(0.9, 1.0),
            Some(pappus_apple(0.9, 1.0))
        )
    );
    eprintln!(
        "{}",
        run(
            "apple R=0.9 halves",
            football2(0.9, 1.0),
            Some(pappus_apple(0.9, 1.0))
        )
    );
    eprintln!(
        "{}",
        run(
            "apple R=0.999 halves",
            football2(0.999, 1.0),
            Some(pappus_apple(0.999, 1.0))
        )
    );
    eprintln!(
        "{}",
        run(
            "apple R=1-1e-7 halves",
            football2(1.0 - 1e-7, 1.0),
            Some(pappus_apple(1.0 - 1e-7, 1.0))
        )
    );
}

/// Claim 3: the football's π-bands read `Unencoded`, so the bit is
/// their one encoding. Flip one sheet face's bit, and every face's;
/// tier 3 must refuse, or props must stay right.
#[test]
fn probe_flipped_sense_on_a_football_band() {
    let tol = Tol::witness();
    for (what, big) in [("lemon", -0.5), ("apple", 0.4)] {
        let built = revolve(
            &validated(vec![football(big, 1.0)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let faces: Vec<_> = built.body.faces().map(|(k, _)| k).collect();
        eprintln!("{what}: {} faces", faces.len());
        for (i, &f) in faces.iter().enumerate() {
            let flipped = built.body.flipped_face_sense_for_tests(f).unwrap();
            let r = match topo::AtRestBody::validate(flipped, tol) {
                Err(e) => format!("tier3 refuses {e:?}"),
                Ok(b) => {
                    match topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol))
                    {
                        Ok(p) => format!(
                            "PASSES tier3; V={} A={}",
                            p.enclosure.volume, p.enclosure.surface_area
                        ),
                        Err(e) => format!("passes tier3, props refuses {e:?}"),
                    }
                }
            };
            eprintln!("{what}: flip face {i}: {}", &r[..r.len().min(300)]);
        }
        let all = built.body.revert().unwrap();
        let r = match topo::AtRestBody::validate(all, tol) {
            Err(e) => format!("tier3 refuses {e:?}"),
            Ok(b) => {
                match topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol)) {
                    Ok(p) => format!(
                        "PASSES tier3; V={} A={}",
                        p.enclosure.volume, p.enclosure.surface_area
                    ),
                    Err(e) => format!("passes tier3, props refuses {e:?}"),
                }
            }
        };
        eprintln!("{what}: revert all: {}", &r[..r.len().min(300)]);
    }
}

/// Claim 5: which refusal the wrong-sheet row sees (it asserts only
/// `is_err`).
#[test]
fn probe_other_sheet_refusal_names() {
    let tol = Tol::witness();
    for (what, big) in [("lemon", -0.5), ("apple", 0.4)] {
        let built = revolve(
            &validated(vec![football(big, 1.0)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let mut body = built.body;
        let keys: Vec<_> = body
            .surfaces()
            .filter(|(_, s)| matches!(s, geom::Surface::Spindle { .. }))
            .map(|(k, _)| k)
            .collect();
        for key in keys {
            let Some(geom::Surface::Spindle {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }) = body.get_surface(key).cloned()
            else {
                unreachable!()
            };
            body = body
                .with_surface_moved_for_tests(
                    key,
                    geom::Surface::Spindle {
                        center,
                        axis,
                        major_radius: -major_radius,
                        minor_radius,
                        u_ref,
                    },
                )
                .unwrap();
        }
        let e = topo::AtRestBody::validate(body, tol).err();
        let s = format!("{e:?}");
        eprintln!("{what}: {}", &s[..s.len().min(1500)]);
    }
}

/// Claim 8: the horn needle mirrored onto the horn's lower side
/// (`−π ≤ v ≤ −π/2`), which the PR says reads a turn off. Is the
/// outcome loud?
#[test]
fn probe_lower_horn_needle() {
    let exact = (
        PI * (5.0 / 3.0 - PI / 2.0),
        2.0 * PI * (PI / 2.0 - 1.0) + PI,
    );
    let upper = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    eprintln!("{}", run("upper needle", upper, Some(exact)));
    let lower = bulge_loop(vec![
        (Point2::new(0.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), -(PI / 8.0).tan()),
        (Point2::new(0.0, 0.0), 0.0),
    ]);
    eprintln!("{}", run("lower needle", lower, Some(exact)));
}

/// Claim 3: flip BOTH sheet faces' bits (loops kept). Each edge's two
/// uses stay consistent with each other; the material sign reads
/// `Unencoded` and no seam region reads a sheet.
#[test]
fn probe_both_bits_flipped() {
    let tol = Tol::witness();
    for (what, big, r) in [("lemon", -0.5, 1.0), ("apple", 0.4, 1.0)] {
        let built = revolve(
            &validated(vec![football(big, r)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let mut body = built.body;
        let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
        for f in faces {
            body = body.flipped_face_sense_for_tests(f).unwrap();
        }
        let r = match topo::AtRestBody::validate(body, tol) {
            Err(e) => format!("tier3 refuses {e:?}"),
            Ok(b) => {
                match topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol)) {
                    Ok(p) => format!(
                        "PASSES tier3; V={} A={}",
                        p.enclosure.volume, p.enclosure.surface_area
                    ),
                    Err(e) => format!("passes tier3, props refuses {e:?}"),
                }
            }
        };
        eprintln!("{what}: flip both bits: {}", &r[..r.len().min(400)]);
    }
}

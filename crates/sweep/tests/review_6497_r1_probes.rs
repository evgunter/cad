//! Review 6497 (r1) probes: bodies on a spindle sheet's apex beyond the
//! PR's rows, each measured against a closed form of revolution
//! (`V = π∮ρ² dz`, `A = 2π∫ρ ds`) written here. Run with
//! `R1_STRICT=1` to make a refusal red on rows expected to build
//! (the mutation runs did).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, revolve};

fn axis(origin: (f64, f64), dir: (f64, f64)) -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(origin.0, origin.1),
        dir: Vec2::new(dir.0, dir.1),
    }
}

/// Outcome of a full build: Err(stage: message) or Ok((V, A, pads)).
fn run(
    lp: profile::ProfileLoop<f64>,
    ax: RevolveAxis<f64>,
) -> Result<(f64, f64, f64, f64), String> {
    let tol = Tol::witness();
    let built = revolve(&validated(vec![lp]), ax, Revolution::Full, tol)
        .map_err(|e| format!("revolve: {e}"))?;
    let delta = 1e-2;
    let mesh = mesh::tessellate(&built.body, delta, tol).map_err(|e| format!("mesh: {e:?}"))?;
    mesh::validate::check_mesh(&mesh).map_err(|e| format!("check_mesh: {e:?}"))?;
    let mv = mesh::validate::signed_volume(&mesh);
    let body = topo::AtRestBody::validate(built.body, tol).map_err(|e| format!("tier3: {e:?}"))?;
    let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .map_err(|e| format!("props: {e:?}"))?;
    let g = props.enclosure;
    eprintln!("    mesh volume {mv}");
    Ok((g.volume, g.surface_area, g.volume_pad, g.area_pad))
}

/// `(V, A)` of a sheet arc `v ∈ [v0, v1]` of the circle `(R, r)`
/// (ρ = R + r cos v, z = r sin v), by quadrature, plus flat ends whose
/// area the caller adds.
fn arc_terms(big: f64, r: f64, v0: f64, v1: f64) -> (f64, f64) {
    let n = 20_000;
    let h = (v1 - v0) / f64::from(n);
    let (mut vol, mut area) = (0.0, 0.0);
    for i in 0..=n {
        let v = v0 + h * f64::from(i);
        let w = if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        let rho = big + r * v.cos();
        vol += w * rho * rho * r * v.cos();
        area += w * rho * r;
    }
    (PI * vol * h / 3.0, 2.0 * PI * area * h / 3.0)
}

fn check(what: &str, got: Result<(f64, f64, f64, f64), String>, (v, a): (f64, f64)) -> bool {
    match got {
        Err(e) => {
            eprintln!("{what}: REFUSED {e}");
            // A refusal is reported, and red only where the row expects a build.
            !(std::env::var("R1_STRICT").is_ok_and(|v| v == "1")
                && ["bullet", "cavity", "split", "tilted", "sheet+sphere"]
                    .iter()
                    .any(|k| what.contains(k)))
        }
        Ok((gv, ga, vp, ap)) => {
            let okv = (gv - v).abs() <= 1e-9 * v.abs() + vp;
            let oka = (ga - a).abs() <= 1e-9 * a.abs() + ap;
            eprintln!(
                "{what}: V {gv} vs {v} (pad {vp:e}, err {:e}) {}; A {ga} vs {a} (pad {ap:e}, err {:e}) {}",
                gv - v,
                if okv { "ok" } else { "WRONG" },
                ga - a,
                if oka { "ok" } else { "WRONG" }
            );
            okv && oka
        }
    }
}

fn y() -> RevolveAxis<f64> {
    axis((0.0, 0.0), (0.0, 1.0))
}

/// Half a football: the lower (or upper) half of the sheet arc and a
/// disc at the equator.
fn bullet(big: f64, r: f64, upper: bool) -> (profile::ProfileLoop<f64>, (f64, f64)) {
    let vs = (-big / r).acos();
    let h = r * vs.sin();
    let w = big + r;
    let b = (vs / 4.0).tan();
    let (v, a) = if upper {
        arc_terms(big, r, 0.0, vs)
    } else {
        arc_terms(big, r, -vs, 0.0)
    };
    let lp = if upper {
        bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(w, 0.0), b),
            (Point2::new(0.0, h), 0.0),
        ])
    } else {
        bulge_loop(vec![
            (Point2::new(0.0, -h), b),
            (Point2::new(w, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ])
    };
    (lp, (v, a + PI * w * w))
}

#[test]
fn r1_bullets() {
    let mut bad = vec![];
    for (big, r) in [(-0.5_f64, 1.0_f64), (-0.9, 1.0), (0.4, 1.0), (0.3, 0.7)] {
        for upper in [false, true] {
            let what = format!("bullet R {big} r {r} upper {upper}");
            let (lp, want) = bullet(big, r, upper);
            if !check(&what, run(lp, y()), want) {
                bad.push(what);
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

#[test]
fn r1_split_football_and_mixed() {
    let mut bad = vec![];
    for (big, r) in [(-0.5_f64, 1.0_f64), (0.4, 1.0)] {
        let vs = (-big / r).acos();
        let h = r * vs.sin();
        let b = (vs / 4.0).tan();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -h), b),
            (Point2::new(big + r, 0.0), b),
            (Point2::new(0.0, h), 0.0),
        ]);
        let what = format!("split football R {big}");
        if !check(&what, run(lp, y()), arc_terms(big, r, -vs, vs)) {
            bad.push(what);
        }
        // Lower half the sheet, upper a hemisphere of radius w.
        let w = big + r;
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -h), b),
            (Point2::new(w, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, w), 0.0),
        ]);
        let (v, a) = arc_terms(big, r, -vs, 0.0);
        let what = format!("sheet+sphere R {big}");
        if !check(
            &what,
            run(lp, y()),
            (v + 2.0 / 3.0 * PI * w.powi(3), a + 2.0 * PI * w * w),
        ) {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

#[test]
fn r1_tilted_axis_football() {
    let mut bad = vec![];
    let (o, d) = ((0.3, -0.2), (0.6, 0.8));
    let n = (d.1, -d.0);
    let at = |rho: f64, z: f64| Point2::new(o.0 + rho * n.0 + z * d.0, o.1 + rho * n.1 + z * d.1);
    for (big, r) in [(-0.5_f64, 1.0_f64), (0.4, 1.0)] {
        let vs = (-big / r).acos();
        let h = r * vs.sin();
        let lp = bulge_loop(vec![(at(0.0, -h), (vs / 2.0).tan()), (at(0.0, h), 0.0)]);
        let what = format!("tilted football R {big}");
        if !check(&what, run(lp, axis(o, d)), arc_terms(big, r, -vs, vs)) {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

#[test]
fn r1_lower_needle_and_wide_apples() {
    let mut bad = vec![];
    // The lower needle: horn (1, 0) r 1, cusp at the origin, arc to (1, −1).
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), -(PI / 8.0).tan()),
        (Point2::new(0.0, 0.0), 0.0),
    ]);
    let what = "lower needle".to_string();
    if !check(
        &what,
        run(lp, y()),
        (
            PI * (5.0 / 3.0 - PI / 2.0),
            2.0 * PI * (PI / 2.0 - 1.0) + PI,
        ),
    ) {
        bad.push(what);
    }
    for (big, r) in [
        (0.6_f64, 1.0_f64),
        (0.7, 1.0),
        (0.8, 1.0),
        (0.9, 1.0),
        (0.99, 1.0),
        (1.0 - 1e-6, 1.0),
        (-1e-3, 1.0),
        (1e-3, 1.0),
        (-0.99, 1.0),
    ] {
        let vs = (-big / r).acos();
        let h = r * vs.sin();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -h), (vs / 2.0).tan()),
            (Point2::new(0.0, h), 0.0),
        ]);
        let what = format!("football R {big} r {r}");
        if !check(&what, run(lp, y()), arc_terms(big, r, -vs, vs)) {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// A sheet face whose sense is flipped must not pass tier 3 (its π-band
/// reading encodes no side).
#[test]
fn r1_flipped_sheet_face() {
    let tol = Tol::witness();
    let mut passed = vec![];
    for (big, r) in [(-0.5_f64, 1.0_f64), (0.4, 1.0)] {
        let vs = (-big / r).acos();
        let h = r * vs.sin();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -h), (vs / 2.0).tan()),
            (Point2::new(0.0, h), 0.0),
        ]);
        let built = revolve(&validated(vec![lp]), y(), Revolution::Full, tol).unwrap();
        let sheet: Vec<_> = built
            .body
            .faces()
            .filter(|(_, f)| {
                matches!(
                    built.body.get_surface(f.surface),
                    Some(geom::Surface::Spindle { .. })
                )
            })
            .map(|(k, _)| k)
            .collect();
        for (label, keys) in [("one", vec![sheet[0]]), ("both", sheet.clone())] {
            let mut body = built.body.clone();
            for k in keys {
                body = body.flipped_face_sense_for_tests(k).unwrap();
            }
            match topo::AtRestBody::validate(body, tol) {
                Ok(b) => {
                    let p =
                        topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol));
                    eprintln!(
                        "R {big} flip {label}: PASSES tier 3; props {:?}",
                        p.map(|p| p.enclosure.volume)
                    );
                    passed.push(format!("{big} {label}"));
                }
                Err(e) => eprintln!("R {big} flip {label}: refused {e:?}"),
            }
        }
    }
    assert!(passed.is_empty(), "{passed:?}");
}

/// What the other-sheet row is refused for.
#[test]
fn r1_other_sheet_reason() {
    let tol = Tol::witness();
    for (big, r) in [(-0.5_f64, 1.0_f64), (0.4, 1.0)] {
        let vs = (-big / r).acos();
        let h = r * vs.sin();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -h), (vs / 2.0).tan()),
            (Point2::new(0.0, h), 0.0),
        ]);
        let mut body = revolve(&validated(vec![lp]), y(), Revolution::Full, tol)
            .unwrap()
            .body;
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
        eprintln!("R {big}: {:?}", topo::AtRestBody::validate(body, tol).err());
    }
}

/// The bodies the mesh refuses (lower needle, wide apples): tier 3 and
/// props alone.
#[test]
fn r1_unmeshed_bodies_measure() {
    let tol = Tol::witness();
    let mut rows: Vec<(String, profile::ProfileLoop<f64>, (f64, f64))> = vec![(
        "lower needle".into(),
        bulge_loop(vec![
            (Point2::new(0.0, -1.0), 0.0),
            (Point2::new(1.0, -1.0), -(PI / 8.0).tan()),
            (Point2::new(0.0, 0.0), 0.0),
        ]),
        (
            PI * (5.0 / 3.0 - PI / 2.0),
            2.0 * PI * (PI / 2.0 - 1.0) + PI,
        ),
    )];
    for big in [0.7_f64, 0.8, 0.9, 0.99, 1.0 - 1e-6] {
        let vs = (-big).acos();
        let h = vs.sin();
        rows.push((
            format!("football R {big}"),
            bulge_loop(vec![
                (Point2::new(0.0, -h), (vs / 2.0).tan()),
                (Point2::new(0.0, h), 0.0),
            ]),
            arc_terms(big, 1.0, -vs, vs),
        ));
    }
    let mut bad = vec![];
    for (what, lp, want) in rows {
        let got = revolve(&validated(vec![lp]), y(), Revolution::Full, tol)
            .map_err(|e| format!("revolve: {e}"))
            .and_then(|b| {
                topo::AtRestBody::validate(b.body, tol).map_err(|e| format!("tier3: {e:?}"))
            })
            .and_then(|b| {
                topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol))
                    .map_err(|e| format!("props: {e:?}"))
            })
            .map(|p| {
                (
                    p.enclosure.volume,
                    p.enclosure.surface_area,
                    p.enclosure.volume_pad,
                    p.enclosure.area_pad,
                )
            });
        if !check(&what, got, want) {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// What each sheet face's own boundary encodes (the PR says the
/// football's π-bands encode no side, the needle's band one).
#[test]
fn r1_sheet_material_signs() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut rows = vec![];
    for big in [-0.5_f64, 0.4] {
        let vs = (-big).acos();
        let h = vs.sin();
        rows.push((
            format!("football R {big}"),
            bulge_loop(vec![
                (Point2::new(0.0, -h), (vs / 2.0).tan()),
                (Point2::new(0.0, h), 0.0),
            ]),
        ));
        rows.push((format!("bullet R {big}"), bullet(big, 1.0, false).0));
    }
    rows.push((
        "needle".into(),
        bulge_loop(vec![
            (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ]),
    ));
    for (what, lp) in rows {
        let body = revolve(&validated(vec![lp]), y(), Revolution::Full, tol)
            .unwrap()
            .body;
        for (_, f) in body.faces() {
            let s = body.get_surface(f.surface).unwrap();
            if !matches!(s, geom::Surface::Spindle { .. }) {
                continue;
            }
            let (edges, _) = topo::props::loop_edges(&body, f.outer).unwrap();
            let got = geom_brep::props::boundary_material_sign_loops(s, &[&edges], band);
            eprintln!("{what}: sense {} reads {got:?}", f.sense);
        }
    }
}

/// A cylinder (radius 1.5, height 3) with a football cavity on its
/// axis: the sheet faces are π-bands with the material outside (sense
/// false), whose boundary encodes no side.
#[test]
fn r1_football_cavity() {
    let mut bad = vec![];
    for big in [-0.5_f64, 0.4] {
        let vs = (-big).acos();
        let h = vs.sin();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -1.5), 0.0),
            (Point2::new(1.5, -1.5), 0.0),
            (Point2::new(1.5, 1.5), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
            (Point2::new(0.0, h), -(vs / 2.0).tan()),
            (Point2::new(0.0, -h), 0.0),
        ]);
        let (v, a) = arc_terms(big, 1.0, -vs, vs);
        let cyl_v = PI * 2.25 * 3.0;
        let cyl_a = 2.0 * PI * 2.25 + 2.0 * PI * 1.5 * 3.0;
        let what = format!("cavity R {big}");
        if !check(&what, run(lp, y()), (cyl_v - v, cyl_a + a)) {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

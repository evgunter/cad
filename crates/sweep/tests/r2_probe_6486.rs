//! Review probes for PR 6486 (reviewer r2): the revolve's arc
//! classification and the sheet's readers on profiles the PR's oracle
//! does not build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

fn axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

fn kinds(body: &topo::Body<f64>) -> Vec<String> {
    body.faces()
        .map(|(_, f)| match body.get_surface(f.surface).unwrap() {
            geom::Surface::Spindle { major_radius, .. } => format!("spindle({major_radius})"),
            geom::Surface::Torus { major_radius, .. } => format!("torus({major_radius})"),
            geom::Surface::Sphere { .. } => "sphere".into(),
            s => format!("{:?}", s.kind()),
        })
        .collect()
}

/// The sheet band's `∮πρ²dz` from `v0` to `v1` (in the arc's direction)
/// and its `∫2πρ ds`.
fn band(big: f64, r: f64, v0: f64, v1: f64) -> (f64, f64) {
    let vol = |v: f64| {
        let (s, c) = v.sin_cos();
        PI * r * (big * big * s + big * r * (v + s * c) + r * r * (s - s.powi(3) / 3.0))
    };
    let area = |v: f64| 2.0 * PI * r * (big * v + r * v.sin());
    (vol(v1) - vol(v0), (area(v1) - area(v0)).abs())
}

/// A cylinder of radius `outer` with a sheet-shaped hole: the arc is
/// traversed clockwise about its centre, the other turn sense from the
/// football's, and its face's material lies outside the tube. Short
/// vertical runs at the rim radius to `y = ±cap` keep an apple's arc
/// (which dips below its rim's height past π/2) clear of the caps.
fn concave(
    big: f64,
    r: f64,
    v1: f64,
    outer: f64,
) -> Result<(topo::Body<f64>, f64, f64), RevolveError> {
    let (rho, z) = (big + r * v1.cos(), r * v1.sin());
    let cap = 1.1 * r;
    let lp = bulge_loop(vec![
        (Point2::new(rho, -z), 0.0),
        (Point2::new(rho, -cap), 0.0),
        (Point2::new(outer, -cap), 0.0),
        (Point2::new(outer, cap), 0.0),
        (Point2::new(rho, cap), 0.0),
        (Point2::new(rho, z), -(v1 / 2.0).tan()),
    ]);
    let built = revolve(
        &validated(vec![lp]),
        axis(),
        Revolution::Full,
        Tol::witness(),
    )?;
    let (bv, ba) = band(big, r, v1, -v1);
    let volume = PI * outer * outer * 2.0 * cap - 2.0 * PI * rho * rho * (cap - z) + bv;
    let area = 2.0 * PI * outer * 2.0 * cap
        + 2.0 * 2.0 * PI * rho * (cap - z)
        + 2.0 * PI * (outer * outer - rho * rho)
        + ba;
    Ok((built.body, volume, area))
}

#[test]
fn r2_a_concave_sheet_wall_builds_and_measures_exact() {
    let tol = Tol::witness();
    for (what, big, r, v1, outer) in [
        ("concave lemon", -0.5, 1.0, 0.8, 1.0),
        ("concave apple", 0.4, 1.0, 1.6, 2.0),
    ] {
        let (body, volume, area) =
            concave(big, r, v1, outer).unwrap_or_else(|e| panic!("{what}: {e}"));
        let k = kinds(&body);
        eprintln!("{what}: {k:?}");
        assert!(
            k.iter().any(|s| s == &format!("spindle({big})")),
            "{what}: {k:?}"
        );
        let delta = 1e-2;
        let mesh = mesh::tessellate(&body, delta, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: mesh {e:?}"));
        let mv = mesh::validate::signed_volume(&mesh);
        eprintln!("{what}: mesh volume {mv} exact {volume}");
        assert!(
            (mv - volume).abs() <= delta * area,
            "{what}: mesh {mv} vs {volume}"
        );
        let body = topo::AtRestBody::validate(body, tol)
            .unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
        let p = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: props {e:?}"));
        let g = p.enclosure;
        eprintln!(
            "{what}: V {} vs {volume}, A {} vs {area}",
            g.volume, g.surface_area
        );
        assert!(
            (g.volume - volume).abs() <= 1e-10 * volume + g.volume_pad,
            "{what}: V {} vs {volume}",
            g.volume
        );
        assert!(
            (g.surface_area - area).abs() <= 1e-10 * area + g.area_pad,
            "{what}: A {} vs {area}",
            g.surface_area
        );
    }
}

/// An arc on a sheet's circle whose interior dips across the axis, its
/// ends clear of it, refuses `ArcCrossesAxis` (lemon and apple).
#[test]
fn r2_a_sheet_arc_dipping_across_the_axis_refuses() {
    for (what, big, v) in [("apple", 0.5, 1.5), ("lemon", -0.5, 0.5)] {
        let r = 1.0;
        let (rho, z) = (big + r * f64::cos(v), r * f64::sin(v));
        // From (ρ, z) counter-clockwise the long way through `v = π` to
        // (ρ, −z): turn 2π − 2v.
        let turn = 2.0 * PI - 2.0 * v;
        let lp = bulge_loop(vec![
            (Point2::new(rho, z), (turn / 4.0).tan()),
            (Point2::new(rho, -z), 0.0),
            (Point2::new(3.0, -z), 0.0),
            (Point2::new(3.0, z), 0.0),
        ]);
        let got = revolve(
            &validated(vec![lp]),
            axis(),
            Revolution::Full,
            Tol::witness(),
        );
        assert!(
            matches!(got, Err(RevolveError::ArcCrossesAxis { .. })),
            "{what}: {:?}",
            got.as_ref().map(|b| kinds(&b.body))
        );
    }
}

/// The football profile at `rc = r + d`: what each offset classifies as.
#[test]
fn r2_near_horn_ladder() {
    let tol = Tol::witness();
    eprintln!("eps {}", tol.eps());
    let r = 1.0;
    for d in [
        -1e-3, -1e-6, -3e-9, 3e-9, -1e-8, -1e-9, -1e-10, -1e-12, -1e-14, 0.0, 1e-14, 1e-12, 1e-10,
        1e-9, 1e-8, 1e-6, 1e-3,
    ] {
        let big = r + d;
        let v1: f64 = 2.0;
        let (rho, z) = (big + r * v1.cos(), r * v1.sin());
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -1.5), 0.0),
            (Point2::new(rho, -z), (v1 / 2.0).tan()),
            (Point2::new(rho, z), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
        ]);
        match revolve(&validated(vec![lp]), axis(), Revolution::Full, tol) {
            Err(e) => eprintln!("d {d:e}: refused {e:?}"),
            Ok(b) => {
                let k = kinds(&b.body);
                let mesh =
                    mesh::tessellate(&b.body, 1e-2, tol).map(|m| mesh::validate::signed_volume(&m));
                let v = topo::AtRestBody::validate(b.body, tol)
                    .map_err(|e| format!("{e:?}"))
                    .and_then(|body| {
                        topo::mass_properties(
                            &body,
                            tol,
                            topo::test_support::pinned_resolution(tol),
                        )
                        .map(|p| p.enclosure.volume)
                        .map_err(|e| format!("{e:?}"))
                    });
                let cones = {
                    let f = |(r0, z0): (f64, f64), (r1, z1): (f64, f64)| {
                        PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
                    };
                    f((0.0, -1.5), (rho, -z)) + f((rho, z), (0.0, 1.5))
                };
                let exact = cones + band(big, r, -v1, v1).0;
                eprintln!("d {d:e}: {k:?} mesh {mesh:?} props {v:?} exact {exact}");
            }
        }
    }
}

/// The football's bands revolved part way: meridian edges on the sheet
/// and plane caps. Volume and area against `θ/2π` of the full solid's
/// plus the two caps; tier 3; mesh.
#[test]
fn r2_partial_footballs_measure_exact() {
    let tol = Tol::witness();
    for (what, big, r, v1, tip, theta, concave_wall) in [
        ("lemon 1 rad", -0.5, 1.0, 0.8, 1.2, 1.0, false),
        ("apple 1 rad", 0.4, 1.0, 1.6, 1.6, 1.0, false),
        ("lemon 4 rad", -0.5, 1.0, 0.8, 1.2, 4.0, false),
        ("apple 4 rad", 0.4, 1.0, 1.6, 1.6, 4.0, false),
        ("concave lemon 2 rad", -0.5, 1.0, 0.8, 0.0, 2.0, true),
    ] {
        let (rho, z) = (big + r * f64::cos(v1), r * f64::sin(v1));
        let (lp, full_v, lateral, section) = if concave_wall {
            let outer = 1.0;
            let cap = 1.1 * r;
            let lp = bulge_loop(vec![
                (Point2::new(rho, -z), 0.0),
                (Point2::new(rho, -cap), 0.0),
                (Point2::new(outer, -cap), 0.0),
                (Point2::new(outer, cap), 0.0),
                (Point2::new(rho, cap), 0.0),
                (Point2::new(rho, z), -(v1 / 2.0).tan()),
            ]);
            let (bv, ba) = band(big, r, v1, -v1);
            let volume = PI * outer * outer * 2.0 * cap - 2.0 * PI * rho * rho * (cap - z) + bv;
            let lateral = 2.0 * PI * outer * 2.0 * cap
                + 2.0 * 2.0 * PI * rho * (cap - z)
                + 2.0 * PI * (outer * outer - rho * rho)
                + ba;
            let theta_arc = 2.0 * v1;
            let segment = r * r / 2.0 * (theta_arc - theta_arc.sin());
            let section = (outer - rho) * 2.0 * cap - segment;
            (lp, volume, lateral, section)
        } else {
            let lp = bulge_loop(vec![
                (Point2::new(0.0, -tip), 0.0),
                (Point2::new(rho, -z), (v1 / 2.0).tan()),
                (Point2::new(rho, z), 0.0),
                (Point2::new(0.0, tip), 0.0),
            ]);
            let frustum = |(r0, z0): (f64, f64), (r1, z1): (f64, f64)| {
                let slant = ((r1 - r0).powi(2) + (z1 - z0).powi(2)).sqrt();
                (
                    PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0,
                    PI * (r0 + r1) * slant,
                )
            };
            let (lo, la) = frustum((0.0, -tip), (rho, -z));
            let (hi, ha) = frustum((rho, z), (0.0, tip));
            let (bv, ba) = band(big, r, -v1, v1);
            let theta_arc = 2.0 * v1;
            let segment = r * r / 2.0 * (theta_arc - theta_arc.sin());
            (lp, lo + bv + hi, la + ha + ba, rho * (tip + z) + segment)
        };
        let built = revolve(
            &validated(vec![lp]),
            axis(),
            Revolution::Partial(theta),
            tol,
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let volume = full_v * theta / (2.0 * PI);
        let area = lateral * theta / (2.0 * PI) + 2.0 * section;
        eprintln!("{what}: {:?}", kinds(&built.body));
        let mesh = mesh::tessellate(&built.body, 1e-2, tol).map(|m| {
            let ok = mesh::validate::check_mesh(&m).is_ok();
            (mesh::validate::signed_volume(&m), ok)
        });
        eprintln!("{what}: mesh {mesh:?} exact {volume}");
        let body = topo::AtRestBody::validate(built.body, tol)
            .unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
        let p = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: props {e:?}"));
        let g = p.enclosure;
        eprintln!(
            "{what}: V {} vs {volume}, A {} vs {area}",
            g.volume, g.surface_area
        );
        assert!(
            (g.volume - volume).abs() <= 1e-9 * volume + g.volume_pad,
            "{what}: V {} vs {volume}",
            g.volume
        );
        assert!(
            (g.surface_area - area).abs() <= 1e-9 * area + g.area_pad,
            "{what}: A {} vs {area}",
            g.surface_area
        );
        let (mv, ok) = mesh.unwrap_or_else(|e| panic!("{what}: mesh {e:?}"));
        assert!(ok, "{what}: check_mesh");
        assert!(
            (mv - volume).abs() <= 1e-2 * area,
            "{what}: mesh {mv} vs {volume}"
        );
    }
}

/// The PR's wide-apple evidence: `R = 0.8, |v| ≤ 2.4` builds, passes
/// tier 3 and measures exact; the mesh refuses typed.
#[test]
fn r2_the_wide_apple_evidence() {
    let tol = Tol::witness();
    let (big, r, v1, tip): (f64, f64, f64, f64) = (0.8, 1.0, 2.4, 1.2);
    let (rho, z) = (big + r * v1.cos(), r * v1.sin());
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -tip), 0.0),
        (Point2::new(rho, -z), (v1 / 2.0).tan()),
        (Point2::new(rho, z), 0.0),
        (Point2::new(0.0, tip), 0.0),
    ]);
    let built = revolve(&validated(vec![lp]), axis(), Revolution::Full, tol).unwrap();
    let mesh = mesh::tessellate(&built.body, 1e-2, tol).map(|m| mesh::validate::signed_volume(&m));
    let body = topo::AtRestBody::validate(built.body, tol).map_err(|e| format!("{e:?}"));
    let props = body.and_then(|b| {
        topo::mass_properties(&b, tol, topo::test_support::pinned_resolution(tol))
            .map(|p| (p.enclosure.volume, p.enclosure.surface_area))
            .map_err(|e| format!("{e:?}"))
    });
    let frustum = |(r0, z0): (f64, f64), (r1, z1): (f64, f64)| {
        PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
    };
    let exact =
        frustum((0.0, -tip), (rho, -z)) + band(big, r, -v1, v1).0 + frustum((rho, z), (0.0, tip));
    eprintln!("wide apple: mesh {mesh:?} props {props:?} exact {exact}");
}

/// How wide the oracle's admitted pad is: the enclosure's volume and
/// area pads on the lemon and apple, against the `1e-10` relative term.
#[test]
fn r2_the_oracles_pads() {
    let tol = Tol::witness();
    for (what, big, r, v1, tip) in [
        ("lemon", -0.5, 1.0, 0.8_f64, 1.2),
        ("apple", 0.4, 1.0, 1.6, 1.6),
    ] {
        let (rho, z) = (big + r * v1.cos(), r * v1.sin());
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -tip), 0.0),
            (Point2::new(rho, -z), (v1 / 2.0).tan()),
            (Point2::new(rho, z), 0.0),
            (Point2::new(0.0, tip), 0.0),
        ]);
        let built = revolve(&validated(vec![lp]), axis(), Revolution::Full, tol).unwrap();
        let body = topo::AtRestBody::validate(built.body, tol).unwrap();
        let p =
            topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        let g = p.enclosure;
        eprintln!(
            "{what}: V {} pad {:e} (rel {:e}); A {} pad {:e} (rel {:e})",
            g.volume,
            g.volume_pad,
            g.volume_pad / g.volume,
            g.surface_area,
            g.area_pad,
            g.area_pad / g.surface_area
        );
    }
}

//! Review probes (R1, PR 6486): the revolve's arc classification at its
//! edges, and conical-ended bands beyond the PR's four.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use core::f64::consts::{PI, TAU};
use geom_core::{Arc2, Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop, Segment};
use sweep::{Revolution, RevolveError as E, revolve};

fn circle_at(cx: f64, cy: f64, r: f64, phase: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r * phase.cos(), cy + r * phase.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep,
        }),
    )])
}

fn run(lp: ProfileLoop<f64>) -> Result<sweep::Revolved<f64>, E> {
    revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(1.0), Tol::witness())
}

fn kind(r: &Result<sweep::Revolved<f64>, E>) -> String {
    match r {
        Ok(out) => {
            let kinds: Vec<String> = out
                .body
                .faces()
                .filter_map(|(_, f)| out.body.get_surface(f.surface))
                .map(|s| match s {
                    geom::Surface::Spindle { major_radius, minor_radius, .. } => {
                        format!("Spindle(R={major_radius}, r={minor_radius})")
                    }
                    other => format!("{:?}", other.kind()),
                })
                .collect();
            format!("Ok {kinds:?}")
        }
        Err(e) => format!("Err {e:?}"),
    }
}

/// P1: a full circle wholly across the axis, touching it at its one
/// vertex (`R = −r`): no sheet exists (`−r < R` fails), and its interior
/// lies across the axis. The ratified rule reads the interior first.
#[test]
fn r1_p1_a_circle_wholly_across_the_axis_touching_at_its_vertex() {
    let got = run(circle_at(-1.0, 0.0, 1.0, 0.0, TAU));
    eprintln!("P1: {}", kind(&got));
    assert!(matches!(got, Err(E::ArcCrossesAxis { .. })), "P1: {}", kind(&got));
}

/// P2: a full lemon circle whose vertex is an apex: its interior crosses
/// the axis (it holds `m`), and it ends on the axis.
#[test]
fn r1_p2_a_full_lemon_circle_from_an_apex() {
    let got = run(circle_at(-0.5, 0.0, 1.0, PI / 3.0, TAU));
    eprintln!("P2: {}", kind(&got));
    assert!(matches!(got, Err(E::ArcCrossesAxis { .. })), "P2: {}", kind(&got));
}

/// P3: the lemon's long arc between its apexes, through `x < 0`, closed
/// along the axis: its interior crosses the axis and it ends on it.
#[test]
fn r1_p3_the_lemons_long_arc_between_its_apexes() {
    let h = 0.75_f64.sqrt();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -h), -(PI / 3.0).tan()),
        (Point2::new(0.0, h), 0.0),
    ]);
    let got = run(lp);
    eprintln!("P3: {}", kind(&got));
    assert!(matches!(got, Err(E::ArcCrossesAxis { .. })), "P3: {}", kind(&got));
}

/// A conical-ended band (the PR's oracle, re-derived here).
#[derive(Clone, Copy, Debug)]
struct Band {
    major: f64,
    minor: f64,
    half: f64,
    tip: f64,
}

impl Band {
    fn rim(self) -> (f64, f64) {
        (self.major + self.minor * self.half.cos(), self.minor * self.half.sin())
    }
    fn profile(self) -> ProfileLoop<f64> {
        let (rho, z) = self.rim();
        bulge_loop(vec![
            (Point2::new(0.0, -self.tip), 0.0),
            (Point2::new(rho, -z), (self.half / 2.0).tan()),
            (Point2::new(rho, z), 0.0),
            (Point2::new(0.0, self.tip), 0.0),
        ])
    }
    /// Volume and area by a dense midpoint quadrature of the meridian
    /// loop (independent of the closed form): `∮ πρ² dz`, `∮ 2πρ ds`.
    fn quadrature(self) -> (f64, f64) {
        let (rho, z) = self.rim();
        let n = 400_000;
        let mut vol = 0.0;
        let mut area = 0.0;
        let mut seg = |p: &dyn Fn(f64) -> (f64, f64)| {
            for i in 0..n {
                let (a, b) = (p(i as f64 / n as f64), p((i + 1) as f64 / n as f64));
                let mid = 0.5 * (a.0 + b.0);
                let dz = b.1 - a.1;
                let ds = ((b.0 - a.0).powi(2) + dz.powi(2)).sqrt();
                // Exact for a line piece: ∫πρ²dz over a frustum.
                vol += PI * dz * (a.0 * a.0 + a.0 * b.0 + b.0 * b.0) / 3.0;
                area += 2.0 * PI * mid * ds;
            }
        };
        let tip = self.tip;
        seg(&|t| (rho * t, -tip + t * (tip - z)));
        let (big, r, v1) = (self.major, self.minor, self.half);
        seg(&|t| {
            let v = -v1 + 2.0 * v1 * t;
            (big + r * v.cos(), r * v.sin())
        });
        seg(&|t| (rho * (1.0 - t), z + t * (tip - z)));
        (vol.abs(), area)
    }
}

fn band_report(what: &str, band: Band) -> Result<String, String> {
    let tol = Tol::witness();
    let built = revolve(&validated(vec![band.profile()]), axis_y(), Revolution::Full, tol)
        .map_err(|e| format!("{what}: revolve: {e:?}"))?;
    let sheets: Vec<f64> = built
        .body
        .faces()
        .filter_map(|(_, f)| match built.body.get_surface(f.surface) {
            Some(geom::Surface::Spindle { major_radius, .. }) => Some(*major_radius),
            _ => None,
        })
        .collect();
    let mesh = mesh::tessellate(&built.body, 1e-2, tol)
        .map_err(|e| format!("{what}: tessellate: {e:?}"))?;
    mesh::validate::check_mesh(&mesh).map_err(|e| format!("{what}: check_mesh: {e:?}"))?;
    let mesh_vol = mesh::validate::signed_volume(&mesh);
    let body = topo::AtRestBody::validate(built.body, tol)
        .map_err(|e| format!("{what}: tier 3: {e:?}"))?;
    let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .map_err(|e| format!("{what}: props: {e:?}"))?;
    let (vq, aq) = band.quadrature();
    let got = props.enclosure;
    let (dv, da) = ((got.volume - vq).abs() / vq, (got.surface_area - aq).abs() / aq);
    let line = format!(
        "{what}: sheets {sheets:?}; V {} vs quad {vq} (rel {dv:.2e}, pad {:.2e}); A {} vs quad {aq} \
         (rel {da:.2e}, pad {:.2e}); mesh V {mesh_vol} (deficit {:.2e}, bound δ·A {:.2e})",
        got.volume,
        got.volume_pad,
        got.surface_area,
        got.area_pad,
        (mesh_vol - vq).abs(),
        1e-2 * aq
    );
    if dv > 1e-8 || da > 1e-8 {
        return Err(format!("MISMATCH {line}"));
    }
    if sheets.iter().any(|&m| (m - band.major).abs() > 1e-12) {
        return Err(format!("WRONG SHEET {line}"));
    }
    Ok(line)
}

/// P4: conical-ended bands beyond the PR's four: the exact horn
/// (`R = r`), a wide-ish apple below the window cut, a thin lemon near
/// `R = −r`, an apple at small `R`, an apple band on one side of the
/// equator only. Volume and area against an independent quadrature.
#[test]
fn r1_p4_more_bands() {
    let bands = [
        ("horn R = r", Band { major: 1.0, minor: 1.0, half: 1.2, tip: 1.5 }),
        ("apple R=0.9 v=2.0", Band { major: 0.9, minor: 1.0, half: 2.0, tip: 1.5 }),
        ("apple R=0.05 v=1.5", Band { major: 0.05, minor: 1.0, half: 1.5, tip: 1.5 }),
        ("lemon R=-0.9 v=0.4", Band { major: -0.9, minor: 1.0, half: 0.4, tip: 0.6 }),
        ("lemon R=-0.05 v=1.5", Band { major: -0.05, minor: 1.0, half: 1.5, tip: 1.5 }),
        ("the PR's lemon", Band { major: -0.5, minor: 1.0, half: 0.8, tip: 1.2 }),
        ("the PR's apple", Band { major: 0.4, minor: 1.0, half: 1.6, tip: 1.6 }),
    ];
    let mut failures = Vec::new();
    for (what, band) in bands {
        match band_report(what, band) {
            Ok(line) => eprintln!("P4 ok {line}"),
            Err(line) => {
                eprintln!("P4 FAIL {line}");
                failures.push(line);
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// P5: the near-horn band and a sphere-class centre in band.
#[test]
fn r1_p5_bands_at_the_horn_and_the_sphere() {
    for (what, big) in [
        ("horn - 1e-13", 1.0 - 1e-13),
        ("horn + 1e-13", 1.0 + 1e-13),
        ("horn - 1e-9", 1.0 - 1e-9),
        ("horn + 1e-9", 1.0 + 1e-9),
    ] {
        // An arc on the circle centred `big` out, radius 1, its band
        // |v| ≤ 1.2, closed by cones to the axis.
        let band = Band { major: big, minor: 1.0, half: 1.2, tip: 1.5 };
        let got = revolve(&validated(vec![band.profile()]), axis_y(), Revolution::Full, Tol::witness());
        eprintln!("P5 {what}: {}", kind(&got));
    }
    for (what, c) in [("centre 1e-13", 1e-13), ("centre -1e-13", -1e-13), ("centre 1e-9", 1e-9), ("centre -1e-9", -1e-9)] {
        // A sphere-class arc (its centre `c` off the axis) between
        // cones, as the bands are built.
        let band = Band { major: c, minor: 1.0, half: 1.2, tip: 1.5 };
        let got = revolve(&validated(vec![band.profile()]), axis_y(), Revolution::Full, Tol::witness());
        eprintln!("P5 sphere {what}: {}", kind(&got));
        if c.abs() < 1e-12 {
            if let Ok(out) = got {
                let body = topo::AtRestBody::validate(out.body, Tol::witness());
                eprintln!("P5 sphere {what}: tier 3 {:?}", body.as_ref().err());
            }
        }
    }
    for (what, big) in [("horn + 1e-13", 1.0 + 1e-13), ("horn - 1e-13", 1.0 - 1e-13)] {
        let band = Band { major: big, minor: 1.0, half: 1.2, tip: 1.5 };
        eprintln!("P5 full {what}: {:?}", band_report(what, band));
    }
}

/// P6: bands ever nearer their apexes (the rim's radius falling toward
/// zero): each must build and measure exactly, or refuse typed.
#[test]
fn r1_p6_bands_at_their_apexes() {
    let mut bad = Vec::new();
    for (big, tip) in [(-0.5, 1.2), (0.4, 1.6), (1.0, 1.5)] {
        let v_star = (-big / 1.0_f64).acos();
        for eps in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7] {
            // A horn's cusp is at v* = π: stop short of it the same way.
            let band = Band { major: big, minor: 1.0, half: v_star * (1.0 - eps), tip };
            let what = format!("R={big} half=v*(1-{eps:e}) rim rho={:.3e}", band.rim().0);
            match band_report(&what, band) {
                Ok(line) => eprintln!("P6 ok {line}"),
                Err(line) => {
                    eprintln!("P6 refused/failed {line}");
                    if line.starts_with("MISMATCH") || line.starts_with("WRONG") {
                        bad.push(line);
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

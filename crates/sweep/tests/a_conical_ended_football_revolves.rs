//! **The football with conical ends revolves.** A band `|v| ≤ v₁` of
//! one spindle-torus sheet between two cones, built from its profile
//! through the revolve door: the arc on the circle of radius `r` centred
//! `R` from the axis, its ends joined to the axis by lines and closed
//! along it. `R < 0` is the lemon, `0 < R < r` the apple. Each is
//! measured against its exact volume and area of revolution, meshed
//! closed, and passes tier 3; `stl`'s acceptance bodies carry the lemon
//! to the external admesh gate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

/// A conical-ended band: the sheet's circle `(R, r)`, its band's half
/// angle `v₁`, the cones' apex height `z_t`, all about the axis through
/// `at` along the sketch `y`.
#[derive(Clone, Copy, Debug)]
struct Band {
    major: f64,
    minor: f64,
    half: f64,
    tip: f64,
    at: (f64, f64),
}

impl Band {
    fn rim(self) -> (f64, f64) {
        (
            self.major + self.minor * self.half.cos(),
            self.minor * self.half.sin(),
        )
    }

    fn profile(self) -> profile::ProfileLoop<f64> {
        let (rho, z) = self.rim();
        let (x0, y0) = self.at;
        let p = |x: f64, y: f64| Point2::new(x0 + x, y0 + y);
        bulge_loop(vec![
            (p(0.0, -self.tip), 0.0),
            (p(rho, -z), (self.half / 2.0).tan()),
            (p(rho, z), 0.0),
            (p(0.0, self.tip), 0.0),
        ])
    }

    fn axis(self) -> RevolveAxis<f64> {
        RevolveAxis {
            origin: Point2::new(self.at.0, self.at.1),
            dir: Vec2::new(0.0, 1.0),
        }
    }
}

impl Band {
    /// The exact volume and area of the revolved profile: `∮ π ρ² dz`
    /// and `∮ 2π ρ ds` round the meridian loop, each segment in closed
    /// form (a line's frustum, the arc's band integrated in its tube
    /// angle; the axis segment adds nothing).
    fn exact(self) -> (f64, f64) {
        use core::f64::consts::PI;
        let (big, r, v1) = (self.major, self.minor, self.half);
        let (rho, z) = self.rim();
        let frustum = |(r0, z0): (f64, f64), (r1, z1): (f64, f64)| {
            let slant = ((r1 - r0).powi(2) + (z1 - z0).powi(2)).sqrt();
            (
                PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0,
                PI * (r0 + r1) * slant,
            )
        };
        let band = |v: f64| {
            let (s, c) = v.sin_cos();
            PI * r * (big * big * s + big * r * (v + s * c) + r * r * (s - s.powi(3) / 3.0))
        };
        let (low, low_area) = frustum((0.0, -self.tip), (rho, -z));
        let (high, high_area) = frustum((rho, z), (0.0, self.tip));
        let volume = low + (band(v1) - band(-v1)) + high;
        let area = low_area + high_area + 2.0 * PI * r * (2.0 * big * v1 + 2.0 * r * v1.sin());
        (volume.abs(), area)
    }
}

const LEMON: Band = Band {
    major: -0.5,
    minor: 1.0,
    half: 0.8,
    tip: 1.2,
    at: (0.0, 0.0),
};

const APPLE: Band = Band {
    major: 0.4,
    minor: 1.0,
    half: 1.6,
    tip: 1.6,
    at: (0.0, 0.0),
};

/// The sheet's apex height `√(r² − R²)` over its tube angle `v*`.
fn apex_angle(band: Band) -> f64 {
    (-band.major / band.minor).acos()
}

#[test]
fn a_conical_ended_football_revolves() {
    let tol = Tol::witness();
    let near_apex = Band {
        half: 0.97 * apex_angle(LEMON),
        tip: 1.2,
        ..LEMON
    };
    for (what, band) in [
        ("the lemon", LEMON),
        ("the apple", APPLE),
        ("the lemon near its apexes", near_apex),
        // Far out, but inside the 1 km session box: at (1e3, 1e3) it
        // reached about 1.41 km, past it, and the finished-body gate
        // refuses that (`OutsideSessionBox`).
        (
            "the lemon 920 m out",
            Band {
                at: (650.0, 650.0),
                ..LEMON
            },
        ),
    ] {
        let built = revolve(
            &validated(vec![band.profile()]),
            band.axis(),
            Revolution::Full,
            tol,
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        // The stored datum: every sheet face's `R` is the profile's
        // signed centre distance. This records the datum rather than
        // guarding its sign: a revolve that minted the mirror sheet
        // would refuse at its own pcurve mint, the edges being off it.
        // What a code flip of the sheet's SIDE would break is the
        // mesh's winding, which `assert_meshed_geometry` reads below
        // (`assert_outward`, sheet faces included).
        let sheets: Vec<f64> = built
            .body
            .faces()
            .filter_map(|(_, f)| match built.body.get_surface(f.surface) {
                Some(geom::Surface::Spindle { major_radius, .. }) => Some(*major_radius),
                _ => None,
            })
            .collect();
        assert!(
            !sheets.is_empty() && sheets.iter().all(|&m| (m - band.major).abs() < 1e-12),
            "{what}: the sheet faces' major radii {sheets:?}, not {}",
            band.major
        );
        // A closed mesh, its volume within the chordal deficit of the
        // body's.
        let delta = 1e-2;
        let mesh = mesh::tessellate(&built.body, delta, tol)
            .unwrap_or_else(|e| panic!("{what}: tessellate: {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: the mesh: {e:?}"));
        crate::common::differential::assert_meshed_geometry(what, &built.body, &mesh, delta);
        let body = built.body;
        let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let (volume, area) = band.exact();
        let got = props.enclosure;
        assert!(
            (got.volume - volume).abs() <= 1e-10 * volume + got.volume_pad,
            "{what}: volume {} against {volume} (pad {})",
            got.volume,
            got.volume_pad
        );
        assert!(
            (got.surface_area - area).abs() <= 1e-10 * area + got.area_pad,
            "{what}: area {} against {area} (pad {})",
            got.surface_area,
            got.area_pad
        );
    }
}

/// The surface kinds of a body's faces, a sheet or torus with its `R`.
fn kinds(body: &topo::Body<f64>) -> Vec<String> {
    body.faces()
        .map(|(_, f)| match body.get_surface(f.surface).unwrap() {
            geom::Surface::Spindle { major_radius, .. } => format!("spindle({major_radius})"),
            geom::Surface::Torus { major_radius, .. } => format!("torus({major_radius})"),
            s => format!("{:?}", s.kind()),
        })
        .collect()
}

/// A sheet band's `∮ πρ² dz` from `v0` to `v1` (in the arc's direction),
/// and its `∫ 2πρ ds`, in closed form.
fn band_integrals(big: f64, r: f64, v0: f64, v1: f64) -> (f64, f64) {
    let vol = |v: f64| {
        let (s, c) = v.sin_cos();
        PI * r * (big * big * s + big * r * (v + s * c) + r * r * (s - s.powi(3) / 3.0))
    };
    let area = |v: f64| 2.0 * PI * r * (big * v + r * v.sin());
    (vol(v1) - vol(v0), (area(v1) - area(v0)).abs())
}

/// `∫ πρ² dz` and `∫ 2πρ ds` of a line from `(r0, z0)` to `(r1, z1)`.
fn frustum((r0, z0): (f64, f64), (r1, z1): (f64, f64)) -> (f64, f64) {
    let slant = ((r1 - r0).powi(2) + (z1 - z0).powi(2)).sqrt();
    (
        PI * (z1 - z0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0,
        PI * (r0 + r1) * slant,
    )
}

/// A built body's tier 3, props against `(volume, area)` within `rel`
/// plus the enclosure's pads, and its mesh: closed, wound outward on
/// every curved face, its volume within the chordal deficit.
fn holds_exact(what: &str, body: topo::AtRestBody<f64>, (volume, area): (f64, f64), rel: f64) {
    let tol = Tol::witness();
    let delta = 1e-2;
    let mesh =
        mesh::tessellate(&body, delta, tol).unwrap_or_else(|e| panic!("{what}: tessellate: {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: the mesh: {e:?}"));
    crate::common::differential::assert_outward(what, &body, &mesh);
    let mv = mesh::validate::signed_volume(&mesh);
    assert!(
        (mv - volume).abs() <= delta * area,
        "{what}: mesh volume {mv} against {volume}"
    );
    let p = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{what}: props: {e:?}"));
    let g = p.enclosure;
    assert!(
        (g.volume - volume).abs() <= rel * volume + g.volume_pad,
        "{what}: volume {} against {volume} (pad {})",
        g.volume,
        g.volume_pad
    );
    assert!(
        (g.surface_area - area).abs() <= rel * area + g.area_pad,
        "{what}: area {} against {area} (pad {})",
        g.surface_area,
        g.area_pad
    );
}

/// A cylinder of radius `outer` with a sheet-shaped hole: the arc runs
/// clockwise about its centre (the other turn sense from the football's),
/// so the sheet face's material lies outside its tube. Short runs at the
/// rim radius out to `y = ±1.1r` keep an apple's arc, which dips below its
/// rim's height past `π/2`, clear of the caps. The loop, the full
/// solid's volume and lateral area, and the meridian section's area.
fn concave(big: f64, r: f64, v1: f64, outer: f64) -> (profile::ProfileLoop<f64>, f64, f64, f64) {
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
    let (bv, ba) = band_integrals(big, r, v1, -v1);
    let volume = PI * outer * outer * 2.0 * cap - 2.0 * PI * rho * rho * (cap - z) + bv;
    let lateral = 2.0 * PI * outer * 2.0 * cap
        + 2.0 * 2.0 * PI * rho * (cap - z)
        + 2.0 * PI * (outer * outer - rho * rho)
        + ba;
    let segment = r * r / 2.0 * (2.0 * v1 - (2.0 * v1).sin());
    (lp, volume, lateral, (outer - rho) * 2.0 * cap - segment)
}

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// **A concave sheet wall builds and measures exact**: a lemon and an
/// apple hole through a cylinder (`concave`), each a sheet face with its
/// material outside the tube. The sheet is the profile's (`R` signed as
/// its centre lies), tier 3 passes, the mesh closes and is wound outward,
/// and volume and area meet the closed form.
#[test]
fn a_concave_sheet_wall_builds_and_measures_exact() {
    for (what, big, r, v1, outer) in [
        ("concave lemon", -0.5, 1.0, 0.8, 1.0),
        ("concave apple", 0.4, 1.0, 1.6, 2.0),
    ] {
        let (lp, volume, area, _) = concave(big, r, v1, outer);
        let built = revolve(
            &validated(vec![lp]),
            y_axis(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let k = kinds(&built.body);
        assert!(k.contains(&format!("spindle({big})")), "{what}: {k:?}");
        holds_exact(what, built.body, (volume, area), 1e-10);
    }
}

/// **A football revolved part way measures exact**: the lemon and the
/// apple at one and four radians, and the concave lemon at two, so the
/// sheet faces carry meridian edges and plane caps close them. Volume
/// against `θ/2π` of the full solid's, area against that of the lateral
/// surface plus the two caps.
#[test]
fn a_football_revolved_part_way_measures_exact() {
    for (what, band, theta) in [
        ("lemon 1 rad", LEMON, 1.0),
        ("apple 1 rad", APPLE, 1.0),
        ("lemon 4 rad", LEMON, 4.0),
        ("apple 4 rad", APPLE, 4.0),
    ] {
        let (full_v, _) = band.exact();
        let (rho, z) = band.rim();
        let (_, low) = frustum((0.0, -band.tip), (rho, -z));
        let (_, high) = frustum((rho, z), (0.0, band.tip));
        let (_, side) = band_integrals(band.major, band.minor, -band.half, band.half);
        let segment = band.minor.powi(2) / 2.0 * (2.0 * band.half - (2.0 * band.half).sin());
        let section = rho * (band.tip + z) + segment;
        let built = revolve(
            &validated(vec![band.profile()]),
            band.axis(),
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let area = (low + high + side) * theta / (2.0 * PI) + 2.0 * section;
        holds_exact(what, built.body, (full_v * theta / (2.0 * PI), area), 1e-9);
    }
    let (lp, volume, lateral, section) = concave(-0.5, 1.0, 0.8, 1.0);
    let theta = 2.0;
    let built = revolve(
        &validated(vec![lp]),
        y_axis(),
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("concave lemon 2 rad: {e}"));
    holds_exact(
        "concave lemon 2 rad",
        built.body,
        (
            volume * theta / (2.0 * PI),
            lateral * theta / (2.0 * PI) + 2.0 * section,
        ),
        1e-9,
    );
}

/// **The wide apple builds and measures, and its mesh refuses typed**:
/// `R = 0.8, |v| ≤ 2.4`, past the latitude window's cut
/// (`work/spiricarm/a-wide-apples-tube-angle-passes-the-latitude-windows-cut.md`).
/// Tier 3 passes and props meets the closed form; `tessellate` reads the
/// rim at `v = −2.4` a turn off and refuses `UnsupportedCurvedDomain`.
/// The row moves when that window does.
#[test]
fn the_wide_apple_measures_and_its_mesh_refuses_typed() {
    let tol = Tol::witness();
    let band = Band {
        major: 0.8,
        minor: 1.0,
        half: 2.4,
        tip: 1.2,
        at: (0.0, 0.0),
    };
    let built = revolve(
        &validated(vec![band.profile()]),
        band.axis(),
        Revolution::Full,
        tol,
    )
    .unwrap_or_else(|e| panic!("the wide apple: {e}"));
    let mesh = mesh::tessellate(&built.body, 1e-2, tol);
    assert!(
        matches!(
            mesh,
            Err(mesh::TessellateError::UnsupportedCurvedDomain { .. })
        ),
        "the wide apple's mesh: {:?}",
        mesh.map(|_| "meshed")
    );
    let body = built.body;
    let g = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the wide apple: props: {e:?}"))
        .enclosure;
    let (volume, area) = band.exact();
    assert!(
        (g.volume - volume).abs() <= 1e-10 * volume + g.volume_pad,
        "the wide apple: volume {} against {volume}",
        g.volume
    );
    assert!(
        (g.surface_area - area).abs() <= 1e-10 * area + g.area_pad,
        "the wide apple: area {} against {area}",
        g.surface_area
    );
}

/// The band's volume and area by a dense quadrature of its meridian
/// loop, independent of [`Band::exact`]'s closed form: `∮ πρ² dz` exact
/// on each of 400 000 chords, `∮ 2πρ ds` at their midpoints.
fn quadrature(band: Band) -> (f64, f64) {
    let (rho, z) = band.rim();
    let n = 400_000;
    let (mut vol, mut area) = (0.0, 0.0);
    let mut seg = |p: &dyn Fn(f64) -> (f64, f64)| {
        for i in 0..n {
            let (a, b) = (
                p(f64::from(i) / f64::from(n)),
                p(f64::from(i + 1) / f64::from(n)),
            );
            let dz = b.1 - a.1;
            vol += PI * dz * (a.0 * a.0 + a.0 * b.0 + b.0 * b.0) / 3.0;
            area += PI * (a.0 + b.0) * ((b.0 - a.0).powi(2) + dz * dz).sqrt();
        }
    };
    let tip = band.tip;
    seg(&|t| (rho * t, -tip + t * (tip - z)));
    let (big, r, v1) = (band.major, band.minor, band.half);
    seg(&|t| {
        let v = -v1 + 2.0 * v1 * t;
        (big + r * v.cos(), r * v.sin())
    });
    seg(&|t| (rho * (1.0 - t), z + t * (tip - z)));
    (vol.abs(), area)
}

/// **Props meets an independent quadrature on bands of every
/// proportion**: the exact horn (`R = r`), an apple below the window's
/// cut, apples and lemons at small `|R|`, a thin lemon near `R = −r`,
/// and the suite's lemon and apple. Volume and area against
/// [`quadrature`], not the closed form the other rows read, within
/// `1e-8` relative (the quadrature's own error is below it).
#[test]
fn props_meets_an_independent_quadrature_on_every_band() {
    let tol = Tol::witness();
    let band = |major, half, tip| Band {
        major,
        minor: 1.0,
        half,
        tip,
        at: (0.0, 0.0),
    };
    for (what, b) in [
        ("the horn R = r", band(1.0, 1.2, 1.5)),
        ("an apple R = 0.9, v = 2", band(0.9, 2.0, 1.5)),
        ("an apple R = 0.05", band(0.05, 1.5, 1.5)),
        ("a lemon R = −0.9", band(-0.9, 0.4, 0.6)),
        ("a lemon R = −0.05", band(-0.05, 1.5, 1.5)),
        ("the lemon", LEMON),
        ("the apple", APPLE),
    ] {
        let built = revolve(
            &validated(vec![b.profile()]),
            b.axis(),
            Revolution::Full,
            tol,
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let body = built.body;
        let g = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("{what}: props: {e:?}"))
            .enclosure;
        let (vq, aq) = quadrature(b);
        assert!(
            (g.volume - vq).abs() <= 1e-8 * vq,
            "{what}: volume {} against the quadrature's {vq}",
            g.volume
        );
        assert!(
            (g.surface_area - aq).abs() <= 1e-8 * aq,
            "{what}: area {} against the quadrature's {aq}",
            g.surface_area
        );
    }
}

/// **The near-horn ladder**: the football's band `|v| ≤ 2` on the
/// circle of radius 1 centred `1 + d` from the axis, `d` read against
/// the run's band (zero `z`, escalate `e`), so the row holds at every ε.
///
/// - `|d|` well inside the band's zero (`±z/2`, and `d = 0`): a
///   horn-class sheet, with `R = 1 + d` up to a hair past `r`, which
///   builds, passes tier 3 and measures exact (piece 1's clamped apex
///   radicand closes it at one cusp).
/// - `d` in the band past its zero (`±3z`): the near-horn sliver,
///   `SliverAxisClearance` on `ArcReach`.
/// - `d` past the escalate (`±2e`) and far out (`±1e-3`): a lemon-side
///   sheet below, a ring torus above, each measuring exact.
#[test]
fn the_near_horn_ladder_reads_its_band() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let (z, e) = (band.zero(), band.escalate());
    let at = |d: f64| Band {
        major: 1.0 + d,
        minor: 1.0,
        half: 2.0,
        tip: 1.5,
        at: (0.0, 0.0),
    };
    for (what, d, kind) in [
        ("far inside", -1e-3, "spindle"),
        ("past the escalate, inside", -2.0 * e, "spindle"),
        ("half the zero, inside", -0.5 * z, "spindle"),
        ("the horn", 0.0, "spindle"),
        ("half the zero, outside", 0.5 * z, "spindle"),
        ("past the escalate, outside", 2.0 * e, "torus"),
        ("far outside", 1e-3, "torus"),
    ] {
        let b = at(d);
        let built = revolve(
            &validated(vec![b.profile()]),
            b.axis(),
            Revolution::Full,
            tol,
        )
        .unwrap_or_else(|e| panic!("{what} (d = {d:e}): {e}"));
        let k = kinds(&built.body);
        assert!(
            k.iter().filter(|s| s.starts_with(kind)).count() == 2,
            "{what} (d = {d:e}): {k:?}"
        );
        holds_exact(what, built.body, b.exact(), 1e-10);
    }
    for d in [-3.0 * z, 3.0 * z] {
        let b = at(d);
        let got = revolve(
            &validated(vec![b.profile()]),
            b.axis(),
            Revolution::Full,
            tol,
        );
        assert!(
            matches!(
                got,
                Err(RevolveError::SliverAxisClearance {
                    decision: sweep::ClearanceDecision::ArcReach,
                    ..
                })
            ),
            "d = {d:e}: {:?}",
            got.map(|out| kinds(&out.body))
        );
    }
}

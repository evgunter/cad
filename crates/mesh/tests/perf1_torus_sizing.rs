//! **The torus chart sizing's pin**: `mesh::sizing::torus_grid_steps`'
//! two per-direction steps, swept over `R/r` from 1.2 to 50 and three
//! decades of δ, against the certifier (`mesh::cert::cert_torus`)
//! recomputed from the emitted mesh, the exact torus distance, the
//! grid's own count, and the boundary polylines' chord counts.
//!
//! Three claims, each of which a drift would break in a different
//! direction:
//!
//! 1. **Sound**: every emitted torus triangle's sampled deviation is
//!    within its certificate, and every certificate within δ — the
//!    lane refuses typed above δ, so this reads the same worst the
//!    lane accepted.
//! 2. **Tight**: the worst certificate over a body is at least 25 % of
//!    δ. The sizing targets δ_s = δ/2 and spends it with no slack in
//!    the bound's constant; the remaining gap is the sups `A`, `B` not
//!    being attained at one φ plus `ceil` rounding. A bound ten times
//!    too loose — the defect this unit removed — passes claim 1 and
//!    fails this.
//! 3. **On the ideal**: each torus patch carries EXACTLY
//!    `2·⌈U/h_u⌉·⌈V/h_v⌉` triangles — the grid the derivation's optimum
//!    names, with `ceil` its only cost — and every circle edge on a
//!    torus carries `⌈span/h⌉` chords for the step of ITS direction (a
//!    rim against `h_u`, a meridian against `h_v`), which is what
//!    makes the boundary rows coincide with the grid's.
//!
//! The steps are re-derived here from the formula rather than read
//! from the crate, so the row is a second statement of the bound, not
//! a tautology over one.

use crate::common::{axis_y, dist_to_surface, eps, p2, validated};
use core::f64::consts::{FRAC_PI_4, PI, TAU};
use geom::{Curve3, Surface};
use geom_core::{Point3, Tol};
use mesh::cert::cert_torus;
use mesh::tessellate;
use mesh::validate::check_mesh;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::{Revolution, revolve};
use topo::Body;
use topo::chart::Chart;
use topo::chart_iso::unwrap_near;

/// The unit's minor radius; `R/r` and `δ/r` are the sweep's axes.
const MINOR: f64 = 1.0;

/// A tube of minor radius [`MINOR`] about a circle of radius `major`,
/// revolved by `sweep` about the y axis: two half-tube torus faces
/// (`V = π` each), the full circle's two rim vertex circles at
/// `φ = ±π/2`, and on a partial sweep two planar caps whose edges are
/// meridian semicircles.
fn tube(major: f64, sweep: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(major, -MINOR), 1.0),
        ProfileVertex::new(p2(major, MINOR), 1.0),
    ]);
    revolve(&validated(vec![lp]), axis_y(), sweep, Tol::witness())
        .unwrap()
        .body
}

/// The derivation's steps, re-derived: `h_u = √(4δ_s/((1+β)(R+r)))`,
/// `h_v = √(4δ_s/((1+β)r))`, `β = √(r/(R+r))`.
fn steps(delta: f64, major: f64) -> (f64, f64) {
    let delta_s = delta / 2.0;
    let a = major + MINOR;
    let beta = (MINOR / a).sqrt();
    let share = 4.0 * delta_s / (1.0 + beta);
    ((share / a).sqrt(), (share / MINOR).sqrt())
}

fn cap(h: f64) -> f64 {
    h.min(FRAC_PI_4)
}

fn count(span: f64, h: f64) -> usize {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (span / h).ceil() as usize;
    n.max(1)
}

/// The three UV corners of an emitted triangle, inverted through the
/// chart and unwrapped onto one branch about the first corner (both
/// coordinates are periodic on a torus).
fn triangle_uv(chart: &Chart, tri: [Point3<f64>; 3]) -> [[f64; 2]; 3] {
    let raw = tri.map(|p| [chart.u_of(p), chart.v_of(p)]);
    [
        raw[0],
        [
            unwrap_near(raw[1][0], raw[0][0]),
            unwrap_near(raw[1][1], raw[0][1]),
        ],
        [
            unwrap_near(raw[2][0], raw[0][0]),
            unwrap_near(raw[2][1], raw[0][1]),
        ],
    ]
}

/// Largest sampled distance from the affine triangle to the surface
/// (barycentric grid, `n = 6` ⇒ 28 points).
fn sampled_deviation(surface: &Surface<f64>, tri: [Point3<f64>; 3]) -> f64 {
    let n = 6u32;
    let mut worst: f64 = 0.0;
    for i in 0..=n {
        for j in 0..=(n - i) {
            let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
            let lk = 1.0 - li - lj;
            let p = Point3::new(
                tri[0].x * li + tri[1].x * lj + tri[2].x * lk,
                tri[0].y * li + tri[1].y * lj + tri[2].y * lk,
                tri[0].z * li + tri[1].z * lj + tri[2].z * lk,
            );
            worst = worst.max(dist_to_surface(surface, p));
        }
    }
    worst
}

/// One sweep row: the three claims over one body.
fn sweep_row(major: f64, delta: f64, sweep: Revolution<f64>) {
    let body = tube(major, sweep);
    let uspan = match sweep {
        Revolution::Full => TAU,
        Revolution::Partial(theta) => theta.abs(),
    };
    let mesh = tessellate(&body, delta, Tol::witness()).unwrap();
    assert_eq!(check_mesh(&mesh), Ok(()), "R/r {major} delta {delta}");
    let (hu, hv) = steps(delta, major);
    let (nu, nv) = (count(uspan, cap(hu)), count(PI, cap(hv)));
    let label = format!("R/r {major} delta {delta} sweep {uspan:.3} (nu {nu}, nv {nv})");

    let mut worst: f64 = 0.0;
    let mut torus_patches = 0;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).unwrap();
        let surface = body.get_surface(face.surface).unwrap();
        let Surface::Torus { .. } = surface else {
            continue;
        };
        torus_patches += 1;
        let chart = Chart::of(surface).unwrap();
        // Claim 3, the grid: exactly the optimum's cells, twice.
        assert_eq!(
            patch.triangles.len(),
            2 * nu * nv,
            "{label}: a torus patch is not the 2·nu·nv grid"
        );
        for tri in &patch.triangles {
            let pts = tri.map(|i| mesh.positions[i as usize]);
            let cert = cert_torus(major, MINOR, triangle_uv(&chart, pts));
            let dev = sampled_deviation(surface, pts);
            // Claim 1: sampled deviation within the certificate (the
            // boundary corners sit within ε of the surface), and the
            // certificate within δ.
            assert!(
                dev <= cert * (1.0 + 1e-9) + eps() + 1e-12,
                "{label}: deviation {dev} exceeds certificate {cert}"
            );
            assert!(cert <= delta, "{label}: certificate {cert} exceeds delta");
            worst = worst.max(cert);
        }
    }
    assert_eq!(torus_patches, 2, "{label}: two half-tube faces");
    // Claim 2: at least a quarter of δ is spent on the worst cell.
    assert!(
        worst >= 0.25 * delta,
        "{label}: the worst certificate {worst} is under a quarter of delta — the sizing has \
         gone loose"
    );
    // Claim 3, the cost against the ideal `2·U·V/(h_u·h_v)` per face:
    // the ceil, and nothing else.
    let ideal = 2.0 * uspan * PI / (hu * hv);
    #[allow(clippy::cast_precision_loss)]
    let actual = (2 * nu * nv) as f64;
    let rounding = (1.0 + 1.0 / nu as f64) * (1.0 + 1.0 / nv as f64);
    assert!(
        actual >= ideal && actual <= ideal * rounding && actual <= 1.5 * ideal,
        "{label}: {actual} triangles per face against the ideal {ideal}"
    );

    // Claim 3, the boundary: every circle edge on a torus carries the
    // chord count of its own direction's step.
    let mut rims = 0;
    let mut meridians = 0;
    for polyline in &mesh.boundaries {
        let edge = body.get_edge(polyline.edge).unwrap();
        let curve = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap();
        let Curve3::Circle { axis, .. } = *curve.carrier() else {
            continue;
        };
        let mut on_torus = None;
        for hek in [edge.he_plus, edge.he_minus] {
            let lp = body.get_half_edge(hek).unwrap().parent_loop;
            let fk = body.get_loop(lp).unwrap().face;
            let surface = body
                .get_surface(body.get_face(fk).unwrap().surface)
                .unwrap();
            if let Surface::Torus { axis: taxis, .. } = *surface {
                on_torus = Some(taxis);
            }
        }
        let Some(taxis) = on_torus else {
            continue;
        };
        let (t0, t1) = curve.params();
        let span = t1 - t0;
        let h = if axis.dot(taxis).abs() > 0.5 {
            rims += 1;
            hu
        } else {
            meridians += 1;
            hv
        };
        assert_eq!(
            polyline.points.len() - 1,
            count(span, h),
            "{label}: a circle edge of span {span} is not chorded at its direction's step"
        );
    }
    match sweep {
        Revolution::Full => {
            assert_eq!(
                (rims, meridians),
                (2, 2),
                "{label}: two rims, two seam meridians"
            );
        }
        Revolution::Partial(_) => {
            assert_eq!(
                (rims, meridians),
                (2, 4),
                "{label}: two rims, four cap meridians"
            );
        }
    }
}

/// `R/r` from 1.2 to 50, δ over three decades, on a quarter-turn
/// wedge (caps: meridian edges) and, where the count allows, the full
/// tube (seams and rims). Sizes stay in the tens of thousands of
/// triangles per row.
#[test]
fn torus_sizing_sweep_is_sound_tight_and_on_the_ideal() {
    for major in [1.2, 2.0, 30.0 / 7.0, 10.0, 50.0] {
        for delta in [3e-2, 3e-3, 3e-4] {
            sweep_row(major, delta, Revolution::Partial(PI / 4.0));
        }
    }
    for major in [1.2, 30.0 / 7.0] {
        for delta in [3e-2, 3e-3] {
            sweep_row(major, delta, Revolution::Full);
        }
    }
}

//! **Issue 723's import-door reproduction, committed: the sphere
//! polar extent through `import_step`.**
//!
//! The fixtures (re-derived from the issue's text by
//! `fixtures/halfcap/gen_halfcap.py`; the originals died with their
//! machine) are literally half of a spherical cap — R = 10 mm above
//! latitude 0.5 rad, cut by a plane through the axis — whose sphere
//! face's meridian side is ONE pole-crossing great-circle arc.
//!
//! * `halfcap.step` splits that arc with one ORDINARY vertex
//!   (3 V / 4 E / 3 F, χ = 2). Before the span-derived extent this
//!   imported, passed tier 3 and certified **−47.187%** of the true
//!   volume at `pad = 0.0`: the endpoint fold saw latitudes
//!   `{sin 0.5, sin 1.0}` and never the pole in the arc's interior.
//! * `halfcap_nosplit.step` is the identical solid with the arc as
//!   one edge (2 V / 3 E / 3 F). Its endpoint latitudes coincide, so
//!   the endpoint fold refused it `DegenerateFace` — the alarm shape:
//!   one vertex of pure topology flipping a refusal into a wrong
//!   certified number.
//!
//! With the extent taken from each arc's stored span, the two twins
//! are what they geometrically are — the SAME solid — and both must
//! certify the SAME exact closed-form volume, pad 0. The no-split
//! twin's old refusal was an artifact of the endpoint fold, not a
//! fact about the geometry: the face is an honest half-cap with
//! positive extent, so measuring it exactly is the honest answer and
//! the refusal legitimately retires.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::halfcap_fixture as fixture;

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// The generator's parameters, SI: R = 10 mm, base latitude 0.5 rad.
const R: f64 = 0.010;
const B: f64 = 0.5;

/// Half of the spherical-cap volume `π·h²·(3R − h)/3`, `h = R(1 − sin B)`
/// — 3.518158565e-7 m³ at these parameters (issue 723's exact figure).
fn exact_volume() -> f64 {
    let h = R * (1.0 - B.sin());
    core::f64::consts::PI * h * h * (3.0 * R - h) / 6.0
}

fn certifies_exactly(name: &str) {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture(name), &ImportOptions::default(), Tol::witness())
    else {
        panic!("{name} must import as a solid");
    };
    topo::validate_geometric(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must pass tier 3: {e:?}"));
    let mp = topo::mass_properties(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} must measure: {e:?}"));
    assert_eq!(
        mp.volume_pad, 0.0,
        "{name}: the closed-form lane's pad is 0"
    );
    let exact = exact_volume();
    let rel = (mp.volume - exact).abs() / exact;
    assert!(
        rel < 1e-12,
        "{name}: volume {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
        mp.volume
    );
}

/// **The split twin** — the executed −47% door — certifies the exact
/// closed-form volume.
#[test]
fn the_split_half_cap_certifies_the_exact_volume() {
    certifies_exactly("halfcap.step");
}

/// **The no-split twin** — the old `DegenerateFace` refusal —
/// certifies the same exact volume: the twins are one solid, and the
/// answer no longer depends on a vertex that moves no geometry.
#[test]
fn the_no_split_twin_certifies_the_same_volume() {
    certifies_exactly("halfcap_nosplit.step");
}

/// **The near-pole split twins** — the split vertex 1e-6 / 1e-7 rad
/// off the pole (10 nm / 1 nm of arc), landing the pole-membership
/// margin inside or beside the default band. Reviewer-executed red:
/// before the indeterminate outcome folded, both flipped
/// certify-exactly into
/// `TierInvalid { VolumeUncomputable { Escalated { .. "props_meridian_pole" } } }`
/// — an import refusal of a solid whose exact area is not in doubt
/// (the fold choices differ by ~band²/2). Both must certify exactly,
/// like every other authoring of the same solid.
#[test]
fn a_split_vertex_a_hair_off_the_pole_certifies_through_the_door() {
    certifies_exactly("halfcap_eps6.step");
    certifies_exactly("halfcap_eps7.step");
}

/// The imported solid of fixture `name`, at the run's tolerance.
fn imported(name: &str) -> topo::Body<f64> {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture(name), &ImportOptions::default(), Tol::witness())
    else {
        panic!("{name} must import as a solid");
    };
    body
}

/// **The join door refuses whole** (`Body::join_edges`).
/// `halfcap_nosplit`'s meridian is one arc through the pole; split it
/// once at an ordinary point (a joinable vertex, first in arena order)
/// and once inside the band from the pole (a vertex whose distance from
/// the chart's singular set reads in the band). The door refuses
/// `JoinUndecided` before any kill, so the ordinary vertex is not
/// joined either and the body is exactly as it was.
#[test]
fn a_join_refused_in_the_band_leaves_the_body_as_found() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut body = imported("halfcap_nosplit.step");
    let (sphere_centre, sphere_axis) = body
        .faces()
        .find_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Sphere { center, axis, .. }) => Some((*center, *axis)),
            _ => None,
        })
        .expect("a sphere face");
    // The arc through the pole: a circle edge whose span holds the pole
    // in its interior; `t_pole` is the pole's parameter on it.
    let (arc, (t0, t1), t_pole, radius) = body
        .edges()
        .find_map(|(e, d)| {
            let c = body.get_curve_geom(d.curve)?.certified()?;
            let geom::Curve3::Circle { radius, .. } = c.carrier() else {
                return None;
            };
            let (t0, t1) = c.params();
            [1.0, -1.0].into_iter().find_map(|s| {
                let pole = sphere_centre + sphere_axis * (s * radius);
                let t = c.carrier().param_near(pole, 0.5 * (t0 + t1))?;
                let on = (c.carrier().eval(t) - pole).norm() < 1e-12 && t0 < t && t < t1;
                on.then_some((e, (t0, t1), t, *radius))
            })
        })
        .expect("the meridian arc through the pole");
    // An ordinary point halfway from the arc's start to the pole, then a
    // point a mid-band distance past the pole.
    let ordinary = body.split_edge(arc, 0.5 * (t0 + t_pole), tol).unwrap();
    let near = 0.5 * (band.zero() + band.escalate());
    let tail = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            body.get_curve_geom(body.get_edge(e).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| {
                    let (a, b) = c.params();
                    a < t_pole && t_pole < b && b <= t1
                })
        })
        .expect("the piece through the pole");
    body.split_edge(tail, t_pole + near / radius, tol).unwrap();
    assert_eq!(
        topo::joinable_vertices(&body, geom_core::Band::new(1e-300, 2e-300).unwrap())
            .unwrap()
            .first(),
        Some(&ordinary.vertex),
        "the ordinary vertex comes first in arena order"
    );
    let before = format!("{body:?}");
    assert!(matches!(
        body.join_edges(band, tol),
        Err(topo::BooleanError::JoinUndecided(_))
    ));
    assert_eq!(format!("{body:?}"), before, "refused, so untouched");
}

/// **A curved join leaves no row on a dead cell.** `halfcap.step`
/// splits the meridian arc with one ordinary vertex; the join door
/// takes it back into one arc of the sphere's chart, and every pcurve
/// and joint row the body carries afterwards is keyed by a live
/// half-edge, the body passing tier 3.
#[test]
fn a_curved_join_leaves_no_row_on_a_dead_cell() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut body = imported("halfcap.step");
    assert!(body.pcurves().next().is_some(), "the import carries rows");
    let joins = body.join_edges(band, tol).unwrap();
    assert_eq!(joins.len(), 1, "the ordinary split vertex is joined");
    assert!(body.get_vertex(joins[0].vertex).is_none());
    for (h, _) in body.pcurves() {
        assert!(
            body.get_half_edge(h).is_some(),
            "a pcurve row on dead {h:?}"
        );
    }
    for (h, _) in body.joints() {
        assert!(body.get_half_edge(h).is_some(), "a joint row on dead {h:?}");
    }
    topo::validate_geometric(&body, tol).unwrap();
}

//! **The same-surface latitude-seam fixtures** and the readers their
//! rows share. Two bodies: the drum whose top cap carries a collinear
//! profile vertex (one plane in four faces, a latitude ring between
//! them — the ring's two half-circles are the plane's only `Chart`
//! images with a non-zero `v` channel), and the sphere authored as two
//! cocircular arcs (one sphere in four faces, a seam at `v = π/4`).
//! With them: the axial door's cavity of either, the meter the void
//! door's graft runs over a body, the evidence `shell` hands that
//! door, and a body's plane-chart images.
//!
//! Body authoring routes here ([`super`]'s rule). The three readers
//! ride with the bodies rather than beside [`super::orient`] because
//! none evaluates a surface — each reads a stored description, or
//! runs the kernel's own meter over one — and every suite that builds
//! one of these bodies reads it through them: `shell9_probe`,
//! `shell9_r1_probes`, `shell9_r2_probes`, `shell9_r2_dump` and
//! `revert_plane_charts`. The unit suites and their review probes
//! build THE SAME BODY or their rows are not about each other
//! ([`super::cavity`]'s rule), which is what a shared fixture buys.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `shell7_common`'s profile vocabulary (`polyline`, `revolved`,
//!   `hollow_moves`, `tol`, `point`), which this module reaches
//!   through `crate::shell7_common` rather than copying — that tree
//!   is the SHELL-7 suites' own and is not a `common::` module;
//! - `shell9_r1_probes::multi_arc_sphere` and its `two_arc_sphere`
//!   — the reviewer's own derivation of the same body from a bulge
//!   computed off the arc's geometry, kept apart under
//!   [`super::oracles`]'s rule (an independent derivation is what a
//!   probe is for), with the torus family beside it;
//! - `shell7_seam_corner`'s inline drum, whose `(r, h, t)` are that
//!   row's own closed-form inputs and read beside its oracle.

use geom::Surface;
use geom_brep::{CertifyError, EdgeCurve, Pcurve};
use sweep::Revolution;
use topo::{Body, EdgeKey, VoidContainment, VoidEvidence};

use crate::shell7_common::{hollow_moves, p2, point, polyline, revolved, tol};

/// The drum's radius.
pub const DRUM_R: f64 = 1.0;
/// The drum's height.
pub const DRUM_H: f64 = 2.0;

/// The collinear-cap drum: a cylinder of radius [`DRUM_R`] and height
/// [`DRUM_H`] whose top cap's profile carries a vertex at mid-radius,
/// so the cap is one plane in four faces with a latitude ring at
/// `r/2` between them.
pub fn collinear_cap_drum() -> Body<f64> {
    polyline(
        &[
            (0.0, 0.0),
            (DRUM_R, 0.0),
            (DRUM_R, DRUM_H),
            (DRUM_R / 2.0, DRUM_H),
            (0.0, DRUM_H),
        ],
        Revolution::Full,
    )
}

/// The unit sphere authored as two cocircular arcs meeting at the
/// latitude `π/4`: one sphere in four faces, a same-surface seam.
pub fn two_arc_sphere() -> Body<f64> {
    use core::f64::consts::{FRAC_PI_2, PI};
    use profile::test_support::bulge_loop;
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    revolved(
        bulge_loop(vec![
            (p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            (p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            (p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

/// The axial door's cavity of `body` at wall thickness `t` — every
/// chart moved inward through `offset_charts_together` — asserted
/// tier-3 valid, which is the door's own contract and the premise of
/// every row that reverts or grafts the cavity.
pub fn door_cavity(body: &Body<f64>, t: f64) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol())
        .expect("the door takes it");
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "cavity tier 3"
    );
    cavity
}

/// Every edge of `body` re-certified exactly as the void door's graft
/// does (`combine.rs`'s recertify arm: the description with its image
/// verbatim, carrier and interval verbatim, endpoints from `he_plus`,
/// surfaces from the body itself), and the refusals. A `Scaffold`
/// edge is skipped, as the graft skips it.
///
/// A MIRROR of that arm, and it can drift from it: if the graft's
/// meter changes, this reads a different meter. The rows that also
/// call `insert_voids` on the same cavity (the door's own verdict)
/// are what catch a drift — this reader's refusals would disagree
/// with a door that no longer refuses, or refuses more.
pub fn graft_recertify_failures(body: &Body<f64>) -> Vec<(EdgeKey, CertifyError)> {
    let band = geom_core::Band::linear(tol()).expect("band");
    body.edges()
        .filter_map(|(ek, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            if matches!(curve.description(), geom_brep::EdgeDescription::Scaffold(_)) {
                return None;
            }
            let start_v = body.get_half_edge(e.he_plus)?.start;
            let end_v = body.half_edge_end(e.he_plus)?;
            EdgeCurve::certify(
                curve.restated_spec(),
                point(body, start_v),
                point(body, end_v),
                |sk| body.get_surface(sk).cloned(),
                band,
            )
            .err()
            .map(|err| (ek, err))
        })
        .collect()
}

/// The evidence `shell` hands the void door — every cavity shell
/// `Carried { Positive }` (`shell.rs`, "The evidence"). A restatement,
/// and it can drift: a door that starts demanding a different
/// certificate refuses these rows at `insert_voids` while `shell`'s
/// own rows (`shell7_seam_corner`) keep passing, which is the signal.
pub fn void_evidence(cavity: &Body<f64>) -> VoidEvidence {
    VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}

/// Every edge described as a `Chart` image on a PLANE, with its image,
/// in the body's edge order.
pub fn plane_images(body: &Body<f64>) -> Vec<(EdgeKey, Pcurve<f64>)> {
    body.edges()
        .filter_map(|(k, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            let c = curve.description().chart()?;
            matches!(body.get_surface(c.surface), Some(Surface::Plane { .. }))
                .then(|| (k, c.pcurve.clone()))
        })
        .collect()
}

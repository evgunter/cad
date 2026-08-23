//! **Reviewer consumer probes for PR #932 (VERBS-ARMS-1)** — solids of
//! revolution authored HERE, not the PR's fixtures, filleted through
//! the public API at varied radii and profiles. What each row goes red
//! on:
//!
//! - **The bored dome family**: an annular-profile revolve's equator
//!   fillets at three radii and matches a closed-form volume derived
//!   independently (general plane depth, not the PR's special case) —
//!   red if the band's geometry moves or a face leaves the closed-form
//!   inventory.
//! - **The off-equator zone, two sequential calls**: a sphere zone
//!   whose two latitude rims sit at NONZERO plane depths of both
//!   signs, filleted one after the other — the second call runs on the
//!   first call's output, so it exercises the annulus door on a body
//!   whose sphere wall already carries a band — red if the wall-shape
//!   gates or the `h = depth + r` arm only hold at the equator, or if
//!   a filleted body stops being fillet-able.
//! - **The one-call pair on a shared wall REFUSES**: both zone rims in
//!   one request refuse at the UPFRONT shared-support gate, naming the
//!   sharing and the sequential-call recourse — red if it builds, or if
//!   it goes back to dying mid-carve on a stale seam key (the
//!   reviewer's finding; AMENDED in the fix pass to pin the fix).
//! - **The unbored hemisphere refuses typed**: a profile touching the
//!   axis mints half-walls (two seam azimuths), so its equator is a
//!   two-arc chain over two half-disc supports — outside the annulus
//!   door. Red if that starts panicking or silently building.
//! - **Near-limit radii refuse typed**: `s ≤ r` refuses
//!   `SpineIrregular` (predicate 3); an infeasible ball escalates; a
//!   trim circle reaching the bore refuses
//!   `FaceClearanceUncertified` (predicate 2 — the pairs deviation 2
//!   says it meters). Red if any turns into a panic, silent geometry,
//!   or success.
//! - **`DegenerateTorusEscalated` is reachable**: a torus whose
//!   `R − r` margin lands inside the band is reported ESCALATED, not
//!   silently classified — red if the escalation arm is dead code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::FilletError;
use sweep::fillet::build::fillet_edges;
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, FaceSurface, ValidationError, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn revolved(verts: Vec<ProfileVertex<f64>>, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(verts, rev, tol())
}

/// The reviewer's bored dome: unit sphere zone from the equator up to
/// `y = 0.8`, bored at `x = 0.3` — annular profile, so the full
/// revolve mints one wall per segment and CLOSED latitude rims. Not
/// the PR's 45° fixture.
fn bored_dome() -> Body<f64> {
    let y_hi = 0.8f64;
    let x_hi = (1.0 - y_hi * y_hi).sqrt();
    let th = (y_hi).asin();
    let bulge = (th / 4.0).tan();
    revolved(
        vec![
            ProfileVertex::new(p2(0.3, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), bulge),
            ProfileVertex::new(p2(x_hi, y_hi), 0.0),
            ProfileVertex::new(p2(0.3, y_hi), 0.0),
        ],
        Revolution::Full,
    )
}

/// The unbored hemisphere dome: a unit half-ball on its own equator
/// disc, profile touching the axis at both ends.
fn hemisphere() -> Body<f64> {
    let quarter = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    revolved(
        vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), quarter),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
    )
}

/// A sphere zone off the equator: sphere `R = 2` about the origin,
/// sliced at `y = −0.5` and `y = 1`, bored at `x = bore`. Both rims
/// are plane–sphere circles at NONZERO plane depth, of opposite signs.
fn zone(bore: f64, rev: Revolution<f64>) -> Body<f64> {
    let big_r = 2.0f64;
    let (y_lo, y_hi) = (-0.5f64, 1.0f64);
    let x_lo = (big_r * big_r - y_lo * y_lo).sqrt();
    let x_hi = (big_r * big_r - y_hi * y_hi).sqrt();
    let th_lo = (y_lo / big_r).asin();
    let th_hi = (y_hi / big_r).asin();
    let bulge = ((th_hi - th_lo) / 4.0).tan();
    revolved(
        vec![
            ProfileVertex::new(p2(bore, y_lo), 0.0),
            ProfileVertex::new(p2(x_lo, y_lo), bulge),
            ProfileVertex::new(p2(x_hi, y_hi), 0.0),
            ProfileVertex::new(p2(bore, y_hi), 0.0),
        ],
        rev,
    )
}

/// Every closed plane–sphere rim of a body, with its circle center
/// height (the selector — rims are latitude circles).
fn closed_rims(body: &Body<f64>) -> Vec<(EdgeKey, f64)> {
    body.edges()
        .filter_map(|(k, e)| {
            let start = body.get_half_edge(e.he_plus)?.start;
            if Some(start) != body.half_edge_end(e.he_plus) {
                return None;
            }
            let surf = |he| -> Option<Surface<f64>> {
                let l = body.get_half_edge(he)?.parent_loop;
                let f = body.get_loop(l)?.face;
                body.get_surface(body.get_face(f)?.surface).cloned()
            };
            let (a, b) = (surf(e.he_plus)?, surf(e.he_minus)?);
            let ps = |x: &Surface<f64>, y: &Surface<f64>| {
                matches!(x, Surface::Plane { .. }) && matches!(y, Surface::Sphere { .. })
            };
            if !(ps(&a, &b) || ps(&b, &a)) {
                return None;
            }
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                geom::Curve3::Circle { center, .. } => Some((k, center.y)),
                _ => None,
            }
        })
        .collect()
}

/// The one closed plane–sphere rim whose latitude is `y` (to 1e-9).
fn rim_at(body: &Body<f64>, y: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = closed_rims(body)
        .into_iter()
        .filter(|(_, cy)| (cy - y).abs() < 1e-9)
        .map(|(k, _)| k)
        .collect();
    assert_eq!(hits.len(), 1, "exactly one closed rim at y = {y}");
    hits[0]
}

// --- the corner-cut washer integral, derived independently ----------
//
// A convex plane–sphere fillet of radius `r` on a solid of revolution
// (sphere radius `R` about the origin, cutting plane at height `y_p`
// with the material on the side containing more of the sphere) removes
// the volume between the sphere's silhouette `x = √(R² − y²)` and the
// fillet arc `x = x_c + √(r² − (y − y_c)²)`, between the plane and the
// sphere contact height. All pieces are elementary; none of this reads
// the kernel or the PR's derivation.

/// ∫ √(r² − u²) du, the antiderivative (clamped at the endpoints so a
/// half-ulp of rounding on `u = ±r` cannot poison the closed form).
fn arc_int(r: f64, u: f64) -> f64 {
    0.5 * u * (r * r - u * u).max(0.0).sqrt() + 0.5 * r * r * (u / r).clamp(-1.0, 1.0).asin()
}

/// The cut volume for one rim. `sign = +1` for a plane below the
/// material (outward −y), `−1` for a plane above it (outward +y).
fn corner_cut(big_r: f64, r: f64, y_p: f64, sign: f64) -> f64 {
    let y_c = y_p + sign * r;
    let x_c = ((big_r - r).powi(2) - y_c * y_c).sqrt();
    let y_s = y_c * big_r / (big_r - r);
    let (a, b) = if sign > 0.0 { (y_p, y_s) } else { (y_s, y_p) };
    let i_sphere = big_r * big_r * (b - a) - (b.powi(3) - a.powi(3)) / 3.0;
    let i_const = x_c * x_c * (b - a);
    let i_lin = 2.0 * x_c * (arc_int(r, b - y_c) - arc_int(r, a - y_c));
    let i_quad = r * r * (b - a) - ((b - y_c).powi(3) - (a - y_c).powi(3)) / 3.0;
    PI * (i_sphere - (i_const + i_lin + i_quad))
}

/// The unfilleted washer volume between `y_lo` and `y_hi` against a
/// bore of radius `bore`, sphere radius `big_r` about the origin.
fn washer(big_r: f64, bore: f64, y_lo: f64, y_hi: f64) -> f64 {
    PI * (big_r * big_r * (y_hi - y_lo)
        - (y_hi.powi(3) - y_lo.powi(3)) / 3.0
        - bore * bore * (y_hi - y_lo))
}

/// **The bored dome's equator fillets at three radii** and matches the
/// independently derived closed form; every wall of the source is
/// ring-free (the dumped-body claim deviation 1 rests on, re-verified
/// on a fixture the PR never built).
#[test]
fn the_bored_dome_equator_fillets_at_three_radii() {
    for r in [0.05, 0.1, 0.2] {
        let body = bored_dome();
        for (k, _) in body.faces() {
            assert!(
                body.get_face(k).unwrap().rings.is_empty(),
                "a full revolve's walls carry no rings"
            );
        }
        let rim = rim_at(&body, 0.0);
        let out = fillet_edges(&body, &[rim], r, band(), tol())
            .unwrap_or_else(|e| panic!("the bored dome fillets at r = {r}, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("tier 3 at r = {r}, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1);
        let props = mass_properties(&out.body, tol()).expect("mass properties");
        assert_eq!(props.volume_pad, 0.0, "closed-form faces only at r = {r}");
        let expect = washer(1.0, 0.3, 0.0, 0.8) - corner_cut(1.0, r, 0.0, 1.0);
        assert!(
            (props.volume - expect).abs() <= 1e-12 * expect,
            "bored dome r = {r}: got {}, closed form {expect}",
            props.volume
        );
    }
}

/// **Both zone rims fillet in SEQUENTIAL calls** — the second call
/// runs on the first call's output, whose sphere wall already carries
/// a band and a shortened seam — and the result matches the closed
/// form with a zero pad.
#[test]
fn both_zone_rims_fillet_sequentially_and_match_the_closed_form() {
    let r = 0.08;
    let body = zone(0.6, Revolution::Full);
    assert_eq!(
        (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        ),
        (4, 8, 4),
        "the zone is four revolution walls"
    );
    let first = fillet_edges(&body, &[rim_at(&body, -0.5)], r, band(), tol())
        .unwrap_or_else(|e| panic!("the bottom rim fillets, got {e:?}"));
    let second = fillet_edges(&first.body, &[rim_at(&first.body, 1.0)], r, band(), tol())
        .unwrap_or_else(|e| panic!("the top rim fillets on the filleted body, got {e:?}"));
    validate_geometric(&second.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    assert_eq!(
        (
            second.body.vertices().count(),
            second.body.edges().count(),
            second.body.faces().count()
        ),
        (6, 12, 6),
        "each annulus band adds one vertex, two edges, one face"
    );
    let props = mass_properties(&second.body, tol()).expect("mass properties");
    assert_eq!(props.volume_pad, 0.0);
    let expect =
        washer(2.0, 0.6, -0.5, 1.0) - corner_cut(2.0, r, -0.5, 1.0) - corner_cut(2.0, r, 1.0, -1.0);
    assert!(
        (props.volume - expect).abs() <= 1e-12 * expect,
        "zone: got {}, closed form {expect}",
        props.volume
    );
}

/// **Both zone rims in ONE call refuse, on the shared support.**
///
/// AMENDED (fix pass, disclosed): the reviewer wrote this row against
/// the observed behaviour — each rim's plan resolves against the SOURCE
/// body, the first band's surgery splits the shared sphere wall's seam,
/// and the second plan's stale seam key made `rim_phase_annulus` refuse
/// `UnsupportedChain` with a detail about geometry ("a trimline does not
/// cross its support's seam meridian inside its span") that the geometry
/// did not have, order-dependently. The finding is upheld and the fix is
/// an UPFRONT gate: `shared_support_gate` refuses before any mutation,
/// naming the sharing and the true recourse. The row now pins the honest
/// refusal and the recourse's own correctness is pinned by
/// `both_zone_rims_fillet_sequentially_and_match_the_closed_form`.
///
/// Red if the one-call pair starts building (re-examine soundness), or
/// if it goes back to dying on a stale key mid-carve.
#[test]
fn both_zone_rims_in_one_call_refuse_on_the_shared_support() {
    let body = zone(0.6, Revolution::Full);
    let rims = [rim_at(&body, -0.5), rim_at(&body, 1.0)];
    match fillet_edges(&body, &rims, 0.08, band(), tol()) {
        Err(FilletError::UnsupportedChain { detail, .. }) => {
            assert!(
                detail.contains("share a support face") && detail.contains("SEQUENTIAL"),
                "the gate names the sharing and the recourse, got {detail:?}"
            );
            assert!(
                !detail.contains("trimline does not cross"),
                "the refusal must not be the stale-key one, got {detail:?}"
            );
        }
        Ok(_) => panic!(
            "one-call shared-support pair BUILT — the gate is gone; re-examine whether \
             the build is sound and update the register"
        ),
        Err(other) => panic!("expected the shared-support refusal, got {other:?}"),
    }
}

/// **The unbored hemisphere refuses typed.** A profile touching the
/// axis mints HALF-walls (two seam azimuths): the equator is two open
/// arcs over two half-disc plane supports, outside both closed-rim
/// doors. The PR's "full solids of revolution" are the annular-profile
/// ones; this row pins that the boundary is a typed refusal, not a
/// panic and not silent geometry.
#[test]
fn the_unbored_hemisphere_equator_refuses_typed() {
    let body = hemisphere();
    assert!(
        closed_rims(&body).is_empty(),
        "an on-axis profile mints no closed rim edge at all"
    );
    // The equator's two arcs: the plane–sphere edges of the body.
    let arcs: Vec<EdgeKey> = body
        .edges()
        .filter_map(|(k, e)| {
            let surf = |he| -> Option<Surface<f64>> {
                let l = body.get_half_edge(he)?.parent_loop;
                let f = body.get_loop(l)?.face;
                body.get_surface(body.get_face(f)?.surface).cloned()
            };
            let (a, b) = (surf(e.he_plus)?, surf(e.he_minus)?);
            let ps = |x: &Surface<f64>, y: &Surface<f64>| {
                matches!(x, Surface::Plane { .. }) && matches!(y, Surface::Sphere { .. })
            };
            (ps(&a, &b) || ps(&b, &a)).then_some(k)
        })
        .collect();
    assert_eq!(arcs.len(), 2, "the equator is two half-circle arcs");
    match fillet_edges(&body, &arcs, 0.1, band(), tol()) {
        Err(
            FilletError::UnsupportedChain { .. }
            | FilletError::FilletCornerUnsupported { .. }
            | FilletError::UnsupportedRunOut { .. },
        ) => {}
        other => panic!("the hemisphere equator must refuse typed, got {other:?}"),
    }
}

/// **Near-limit radii refuse typed.** `s ≤ r` is predicate 3's
/// refusal; an infeasible ball (no spine circle at all) escalates on
/// the poisoned margin; a trim circle reaching the bore is predicate
/// 2's `FaceClearanceUncertified` — the exact pair family deviation 2
/// says the consumption sweep meters for annulus rims. None of them
/// may panic, succeed, or mint silent geometry.
#[test]
fn near_limit_radii_refuse_typed() {
    // s = √(1 − 2r) = 0.316 < r = 0.45: the spine folds. In verb-level
    // practice predicate 2's conservative screen fires FIRST on the
    // huge setbacks such a radius implies (the battery's own stated
    // ordering); predicate 3 is the backstop. Either is the honest
    // typed refusal.
    let body = bored_dome();
    let rim = rim_at(&body, 0.0);
    match fillet_edges(&body, &[rim], 0.45, band(), tol()) {
        Err(FilletError::SpineIrregular { .. } | FilletError::FaceClearanceUncertified { .. }) => {}
        other => panic!("s < r must refuse typed, got {other:?}"),
    }
    // r = 0.51 > (R − depth)/2: no spine circle exists; the poisoned
    // margin escalates (or refuses through an earlier predicate) —
    // loudly either way.
    match fillet_edges(&body, &[rim], 0.51, band(), tol()) {
        Err(
            FilletError::Escalated { .. }
            | FilletError::SpineIrregular { .. }
            | FilletError::FaceClearanceUncertified { .. },
        ) => {}
        other => panic!("an infeasible ball must refuse loudly, got {other:?}"),
    }
    // The narrow-bore zone: at r = 0.35 the bottom trim circle's
    // setback (≈ 0.29) exceeds the ≈ 0.24 gap to the bore rim.
    let narrow = zone(1.7, Revolution::Full);
    let bottom = rim_at(&narrow, -0.5);
    match fillet_edges(&narrow, &[bottom], 0.35, band(), tol()) {
        Err(FilletError::FaceClearanceUncertified { .. }) => {}
        other => panic!("a trim circle at the bore must refuse clearance, got {other:?}"),
    }
    // And well inside the same gap it builds and validates.
    let out = fillet_edges(&narrow, &[bottom], 0.15, band(), tol())
        .unwrap_or_else(|e| panic!("r = 0.15 clears the bore, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
}

/// **The differential pair on the reviewer's own profile.** The zone
/// revolved partially leaves open plane–sphere arcs; they refuse
/// through the open-chain gates, not through the annulus door.
#[test]
fn the_partial_zone_refuses_through_its_own_gates() {
    let body = zone(0.6, Revolution::Partial(2.0));
    let open_arc = body
        .edges()
        .map(|(k, _)| k)
        .find(|k| {
            let e = body.get_edge(*k).unwrap();
            let start = body.get_half_edge(e.he_plus).unwrap().start;
            if Some(start) == body.half_edge_end(e.he_plus) {
                return false;
            }
            let surf = |he| {
                let l = body.get_half_edge(he).unwrap().parent_loop;
                let f = body.get_loop(l).unwrap().face;
                body.get_surface(body.get_face(f).unwrap().surface)
                    .unwrap()
                    .clone()
            };
            let (a, b) = (surf(e.he_plus), surf(e.he_minus));
            let ps = |x: &Surface<f64>, y: &Surface<f64>| {
                matches!(x, Surface::Plane { .. }) && matches!(y, Surface::Sphere { .. })
            };
            ps(&a, &b) || ps(&b, &a)
        })
        .expect("an open plane–sphere arc");
    match fillet_edges(&body, &[open_arc], 0.08, band(), tol()) {
        Err(FilletError::UnsupportedChain { .. } | FilletError::FilletCornerUnsupported { .. }) => {
        }
        other => panic!("the open arc refuses through its own gates, got {other:?}"),
    }
}

/// **The `DegenerateTorusEscalated` arm is reachable.** A torus whose
/// `R − r` margin lands INSIDE the decision band cannot be classified
/// at this tolerance; tier 3 must say so (escalate), not silently pick
/// a side. Red if the escalation arm of the new net is dead code.
#[test]
fn a_torus_on_the_ring_convention_boundary_escalates_at_tier_3() {
    let body = bored_dome();
    let rim = rim_at(&body, 0.0);
    let mut out = fillet_edges(&body, &[rim], 0.1, band(), tol()).unwrap();
    validate_geometric(&out.body, tol()).expect("tier-3 valid before the plant");
    let band_face = out.band_faces[0];
    let surface = out
        .body
        .get_surface(out.body.get_face(band_face).unwrap().surface)
        .unwrap()
        .clone();
    let Surface::Torus {
        center,
        axis,
        major_radius,
        u_ref,
        ..
    } = surface
    else {
        panic!("the band is a torus");
    };
    out.body
        .set_face_surface(
            band_face,
            FaceSurface::New(Surface::Torus {
                center,
                axis,
                major_radius,
                // R − r lands at eps·√k — strictly inside the
                // [eps, k·eps] escalation band, so the classification
                // is not available at this tolerance.
                minor_radius: major_radius - tol().eps() * tol().k().sqrt(),
                u_ref,
            }),
        )
        .unwrap();
    let errors = validate_geometric(&out.body, tol()).expect_err("the boundary cannot pass");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::DegenerateTorusEscalated { .. })),
        "R − r in-band must escalate, got {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::DegenerateTorus { .. })),
        "an in-band margin must not be silently classified, got {errors:?}"
    );
}

//! **VERBS-ARMS-2 acceptance: the calochortus bud's MOUTH RIM.**
//!
//! The lantern a lily bud is built from is a solid of revolution whose
//! meridian is a sphere zone, a conical pucker and a lip disk on a bored
//! base — five walls meeting in FIVE closed latitude rims, three of
//! which are curved-support pairs the kernel refused until this unit:
//! sphere×cone at the MOUTH, cone×plane at the lip, and cylinder×plane
//! at the bore's base (the bore's top rim is the fourth such pair, the
//! same arm at the other end; the fifth rim, plane×sphere at the base
//! annulus, is ARMS-1's). `fillet_edges` on the mouth rim alone is #319's coaxial half,
//! end to end.
//!
//! (The demo's own `lily::wall_probes` wall 6 asks for EVERY lantern edge
//! and refuses at a co-surface seam meridian first — a tangency, at margin
//! exactly zero — so it cannot distinguish this door from that one. These
//! rows name the rim.)
//!
//! What makes each row go red:
//!
//! - **The mouth rim fillets, end to end**: tier-3 valid — which is where
//!   the band's torus, its slit's `Seam` description and BOTH trim
//!   circles' tangent-intersection descriptions are re-derived at rest
//!   against a sphere and a CONE — with a pinned census.
//! - **The band is the closed-form torus** the sheet reduction derives:
//!   spine radius, tube radius and both trim-circle radii against hand
//!   numbers, not against the arm that produced them.
//! - **The other two curved rims fillet too** (cone×plane at the lip,
//!   cylinder×plane at the bore), each a different arm through the same
//!   widened annulus surgery.
//! - **The band is one more revolution wall**: one cycle, two CLOSED
//!   circles and one doubly-traversed slit, and no ring.
//! - **Volume falls, and by the amount the closed form says**: the
//!   filleted bud's volume against the source minus the Pappus volume of
//!   the meridian region the roll removes.
//! - **A non-coaxial pair still refuses** `SpineUnsupported` — the canal
//!   family is not quietly minted as a torus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point2, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::FilletError;
use sweep::fillet::battery::{FilletRequest, run_battery};
use sweep::fillet::build::fillet_edges;
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, FaceSurface, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

/// The **bud's meridian**: bore at `0.2`, a flat base annulus at `t = 0`,
/// the unit sphere's zone from its equator up to latitude `asin(0.6)`,
/// a conical pucker falling three units of radius per unit of axis, and
/// the lip disk that closes it. Annular, so the full revolve mints one
/// wall per segment and every latitude rim is a CLOSED edge.
///
/// The numbers are chosen so every crossing is exact in binary: the
/// mouth sits at `(0.8, 0.6)` on the unit circle (a 3-4-5 point), and the
/// pucker's apex at `0.6 + 0.8/3`.
fn bud_profile() -> Vec<ProfileVertex<f64>> {
    // The sphere zone spans `asin(0.6)` of arc; a profile arc's bulge is
    // the tangent of a QUARTER of its sweep (the dome fixture's own
    // convention).
    let bulge = (0.6f64.asin() / 4.0).tan();
    vec![
        ProfileVertex::new(Point2::new(0.2, 0.0), 0.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), bulge),
        ProfileVertex::new(Point2::new(0.8, 0.6), 0.0),
        ProfileVertex::new(Point2::new(0.35, 0.75), 0.0),
        ProfileVertex::new(Point2::new(0.2, 0.75), 0.0),
    ]
}

fn bud() -> Body<f64> {
    revolved_about_y(bud_profile(), Revolution::Full, tol())
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

/// The CLOSED latitude rims of `body` whose circle carrier has radius
/// `r` (to 1e-9), in key order. Selection is by the analytically known
/// radius, not by uniqueness — the bud's bore carries two.
fn closed_rims(body: &Body<f64>, r: f64) -> Vec<(EdgeKey, f64)> {
    body.edges()
        .filter_map(|(k, e)| {
            let start = body.get_half_edge(e.he_plus)?.start;
            if Some(start) != body.half_edge_end(e.he_plus) {
                return None;
            }
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, center, .. } if (radius - r).abs() < 1e-9 => {
                    Some((k, center.y))
                }
                _ => None,
            }
        })
        .collect()
}

/// The one closed rim of radius `r`, when the radius alone names it.
fn closed_rim(body: &Body<f64>, r: f64) -> EdgeKey {
    let hits = closed_rims(body, r);
    assert_eq!(hits.len(), 1, "exactly one closed rim of radius {r}");
    hits[0].0
}

/// The one closed rim of radius `r` at axial station `y`.
fn closed_rim_at(body: &Body<f64>, r: f64, y: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = closed_rims(body, r)
        .into_iter()
        .filter(|(_, cy)| (cy - y).abs() < 1e-9)
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "exactly one closed rim of radius {r} at y = {y}"
    );
    hits[0]
}

fn band_torus(body: &Body<f64>, face: topo::FaceKey) -> (f64, f64) {
    let s = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = *s
    else {
        panic!("the band's surface is a torus, got {s:?}");
    };
    (major_radius, minor_radius)
}

// ---- The closed form, derived by hand from the fixture's numbers. ----
//
// The mouth is `m = (0.8, 0.6)` in the meridian; the sphere's outward
// unit there is `û = (0.8, 0.6)` and the pucker's is
// `n̂ = (1, 3)/√10`. With the ball inside both (`side = +1`), the centre
// solves `|c| = 1 − r` and `(c − m)·n̂ = −r`, and the branch that
// collapses onto `m` as `r → 0` is the one below.

const R: f64 = 0.05;

/// The ball centre in the meridian, by the two defining equations —
/// solved here independently of the arm (substitute the offset line into
/// the offset circle and take the root nearest the mouth).
fn ball_centre() -> (f64, f64) {
    let (mx, my) = (0.8f64, 0.6f64);
    let s10 = 10.0f64.sqrt();
    let (nx, ny) = (1.0 / s10, 3.0 / s10);
    // The offset line: `x = m + t·λ − n̂·r` for the line's own direction.
    let (tx, ty) = (-ny, nx);
    let (ox, oy) = (mx - nx * R, my - ny * R);
    // `|o + tλ| = 1 − r` → λ² + 2λ(o·t) + |o|² − (1−r)² = 0.
    let b = ox * tx + oy * ty;
    let c = ox * ox + oy * oy - (1.0 - R) * (1.0 - R);
    let disc = (b * b - c).sqrt();
    // The two roots; the one nearest the mouth is this edge's.
    let (l1, l2) = (-b + disc, -b - disc);
    let near = |l: f64| (ox + tx * l - mx).powi(2) + (oy + ty * l - my).powi(2);
    let l = if near(l1) <= near(l2) { l1 } else { l2 };
    (ox + tx * l, oy + ty * l)
}

/// **The end-to-end row.** The bud's mouth rim — a sphere meeting a cone
/// along a closed latitude circle — fillets, and every geometric claim
/// the annulus's six moves make is re-derived at rest by tier 3.
#[test]
fn the_bud_mouth_rim_fillets_to_a_tier_3_valid_solid_with_a_pinned_census() {
    let source = bud();
    assert_eq!(
        census(&source),
        (5, 10, 5),
        "the bud is five walls, five latitude rims and five seams"
    );
    let mouth = closed_rim(&source, 0.8);
    let out = fillet_edges(&source, &[mouth], R, band(), tol())
        .unwrap_or_else(|e| panic!("the bud's sphere-cone mouth rim fillets, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the filleted bud must be tier-3 valid, got {e:?}"));
    // The annulus band's own census delta: two feet minted, the rim
    // vertex retired; two seam children and two trim circles minted, the
    // rim and the host seam's rim-side piece retired; two strips minted,
    // one merged away.
    assert_eq!(census(&out.body), (6, 12, 6));
    assert_eq!(out.band_faces.len(), 1);

    let (major, minor) = band_torus(&out.body, out.band_faces[0]);
    let (cx, cy) = ball_centre();
    assert!(
        (minor - R).abs() < 1e-12,
        "the tube is the requested radius, got {minor}"
    );
    assert!(
        (major - cx).abs() < 1e-12,
        "the spine radius is the ball centre's own radial coordinate {cx}, got {major}"
    );
    // The band's torus sits on the axis at the ball centre's level.
    let s = out
        .body
        .get_surface(out.body.get_face(out.band_faces[0]).unwrap().surface)
        .unwrap();
    let Surface::Torus { center, .. } = *s else {
        panic!("torus")
    };
    assert!(
        center.x.abs() < 1e-12 && center.z.abs() < 1e-12 && (center.y - cy).abs() < 1e-12,
        "the spine circle is centred at (0, {cy}, 0), got {center:?}"
    );
}

/// **Both trim circles, at their closed form.** The sphere's contact is
/// the ball centre scaled back out to the sphere; the cone's is the
/// centre pushed back along the pucker's own normal.
#[test]
fn the_bud_band_meets_its_two_supports_on_the_closed_form_circles() {
    let source = bud();
    let mouth = closed_rim(&source, 0.8);
    let out = fillet_edges(&source, &[mouth], R, band(), tol()).unwrap();
    let (cx, cy) = ball_centre();
    let s10 = 10.0f64.sqrt();
    // On the sphere: `C + (R/(R−r))·(c − C)` for `C = 0`, `R = 1`.
    let sphere_trim = (cx / (1.0 - R), cy / (1.0 - R));
    // On the cone: `c + n̂·r`.
    let cone_trim = (cx + R / s10, cy + 3.0 * R / s10);
    // The two new closed circles on the filleted body are exactly these.
    for (want_r, want_y, which) in [
        (sphere_trim.0, sphere_trim.1, "sphere"),
        (cone_trim.0, cone_trim.1, "cone"),
    ] {
        let e = closed_rim(&out.body, want_r);
        let c = out
            .body
            .get_curve_geom(out.body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let Curve3::Circle { center, radius, .. } = *c.carrier() else {
            panic!("a trim circle")
        };
        assert!(
            (radius - want_r).abs() < 1e-12,
            "the {which} trim circle's radius is {want_r}, got {radius}"
        );
        assert!(
            (center.y - want_y).abs() < 1e-12 && center.x.abs() < 1e-12,
            "the {which} trim circle sits at y = {want_y}, got {center:?}"
        );
    }
}

/// **The band is one more revolution wall.** Its single boundary cycle
/// carries two CLOSED circles and one doubly-traversed slit, and it
/// carries no ring — the shape `props`' closed-form inventory
/// represents.
#[test]
fn the_bud_band_is_a_ring_free_wall_with_two_closed_circles_and_a_slit() {
    let source = bud();
    let mouth = closed_rim(&source, 0.8);
    let out = fillet_edges(&source, &[mouth], R, band(), tol()).unwrap();
    let f = out.band_faces[0];
    let fd = out.body.get_face(f).unwrap();
    assert!(fd.rings.is_empty(), "a curved face must be ring-free");
    let topo::LoopBoundary::Cycle { first } = out.body.get_loop(fd.outer).unwrap().boundary else {
        panic!("the band's boundary is a cycle")
    };
    let cycle = out.body.loop_cycle(first).unwrap();
    let mut closed = 0;
    let mut counts: std::collections::BTreeMap<EdgeKey, usize> = Default::default();
    for he in &cycle {
        let h = out.body.get_half_edge(*he).unwrap();
        *counts.entry(h.edge).or_default() += 1;
        if Some(h.start) == out.body.half_edge_end(*he) {
            closed += 1;
        }
    }
    assert_eq!(closed, 2, "two closed boundary circles");
    assert_eq!(
        counts.values().filter(|c| **c == 2).count(),
        1,
        "exactly one doubly-traversed slit"
    );
}

/// **The other two curved rims of the same bud**, each a different arm
/// through the same widened annulus surgery: the lip (cone×plane) and
/// the bore's base (cylinder×plane).
#[test]
fn the_bud_lip_and_bore_rims_fillet_through_their_own_arms() {
    for (rim_r, rim_y, radius, which) in [
        (0.35, 0.75, 0.03, "the lip (cone-plane)"),
        (0.2, 0.0, 0.03, "the bore's base (cylinder-plane)"),
    ] {
        let source = bud();
        let rim = closed_rim_at(&source, rim_r, rim_y);
        let out = fillet_edges(&source, &[rim], radius, band(), tol())
            .unwrap_or_else(|e| panic!("{which} rim fillets, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{which}: tier-3 valid, got {e:?}"));
        let (_, minor) = band_torus(&out.body, out.band_faces[0]);
        assert!((minor - radius).abs() < 1e-12, "{which}");
    }
}

/// **Volume falls by the closed form.** The roll removes the meridian
/// region between the two contact points and the sharp mouth; revolved,
/// its volume is Pappus' `2π·A·r̄`. The region is a curvilinear triangle
/// (sphere arc, cone segment, tube arc), so the check is against the
/// kernel's own mass properties on the SOURCE minus the same on the
/// filleted body — a definite, quadrature-free difference on two bodies
/// whose faces are all closed-form.
#[test]
fn the_filleted_bud_removes_material_and_stays_closed_form() {
    let source = bud();
    let mouth = closed_rim(&source, 0.8);
    let out = fillet_edges(&source, &[mouth], R, band(), tol()).unwrap();
    let before = mass_properties(&source, tol()).expect("the bud's mass properties");
    let after = mass_properties(&out.body, tol()).expect("the filleted bud's mass properties");
    let removed = before.volume - after.volume;
    // The cut region is inside the triangle bounded by the two setbacks:
    // its area is at most the product of the two, so the revolved volume
    // is at most `2π·r̄·(setback_a · setback_b)` with `r̄ ≤ 0.81`.
    let (cx, cy) = ball_centre();
    let s10 = 10.0f64.sqrt();
    let set_sphere = ((cx / (1.0 - R) - 0.8).powi(2) + (cy / (1.0 - R) - 0.6).powi(2)).sqrt();
    let set_cone = ((cx + R / s10 - 0.8).powi(2) + (cy + 3.0 * R / s10 - 0.6).powi(2)).sqrt();
    let bound = core::f64::consts::TAU * 0.81 * set_sphere * set_cone;
    assert!(
        removed > 0.0,
        "the roll removes material on a convex rim, got {removed}"
    );
    assert!(
        removed < bound,
        "the removed volume {removed} exceeds the setback box's revolve {bound}"
    );
}

/// **A pair that IS the arm's kinds but does NOT share an axis refuses
/// `SpineUnsupported`.** The kinds alone never decide: a sphere and a
/// cone whose axes disagree have a spine that is neither a line nor a
/// circle — the canal family — and this is the row that runs
/// `fillet3_support_coaxiality`'s refusing branch.
///
/// The pair is PLANTED, because no door in this kernel mints one: the
/// bud's own cone wall keeps its apex and half-angle and takes a TILTED
/// axis. The body is then geometrically incoherent, which is exactly
/// why the battery — a pre-construction pass over STORED data — is the
/// right consumer for it, and why nothing is constructed here.
///
/// Red if the coaxiality margin stops being decided, or if the refusal
/// starts advertising the arm roster (which would say the kinds were
/// the problem) instead of the shared-axis hypothesis.
#[test]
fn a_curved_pair_that_misses_the_shared_axis_refuses_spine_unsupported() {
    let mut source = bud();
    let mouth = closed_rim(&source, 0.8);
    // Baseline: coaxial, and the battery passes.
    let req = FilletRequest {
        body: &source,
        edges: vec![mouth],
        radius: R,
    };
    run_battery(&req, band()).expect("the coaxial mouth passes the battery");

    // Plant the tilt on the cone wall, apex and half-angle verbatim.
    let (cone_face, apex, half_angle) = source
        .faces()
        .find_map(|(k, _)| {
            let fd = source.get_face(k)?;
            match *source.get_surface(fd.surface)? {
                Surface::Cone {
                    apex, half_angle, ..
                } => Some((k, apex, half_angle)),
                _ => None,
            }
        })
        .expect("the bud carries one cone wall");
    let tilt = 0.05f64;
    source
        .set_face_surface(
            cone_face,
            FaceSurface::New(Surface::Cone {
                apex,
                axis: Vec3::new(tilt.sin(), tilt.cos(), 0.0),
                half_angle,
                u_ref: Vec3::new(tilt.cos(), -tilt.sin(), 0.0),
            }),
        )
        .expect("planting a surface certifies nothing");

    let req = FilletRequest {
        body: &source,
        edges: vec![mouth],
        radius: R,
    };
    match run_battery(&req, band()) {
        Err(e @ FilletError::SpineUnsupported { .. }) => {
            let text = format!("{e}");
            assert!(
                text.contains("do not share one axis of revolution"),
                "the refusal must name the hypothesis that failed, not the kinds: {text}"
            );
        }
        other => panic!("a non-coaxial sphere-cone pair must refuse, got {other:?}"),
    }
}

/// **The partial revolve of the same profile refuses** — the differential
/// pair. Half a bud leaves the mouth an OPEN arc between two curved
/// supports, which no chain door in this module admits.
#[test]
fn the_partial_revolve_of_the_bud_still_refuses() {
    let source = revolved_about_y(
        bud_profile(),
        Revolution::Partial(core::f64::consts::PI),
        tol(),
    );
    // The open mouth arc: the one arc of radius 0.8 that is not closed.
    let arc = source
        .edges()
        .find(|(_, e)| {
            let c = source
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .map(|c| c.carrier().clone());
            matches!(c, Some(Curve3::Circle { radius, .. }) if (radius - 0.8).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .expect("the partial bud carries a mouth arc");
    match fillet_edges(&source, &[arc], R, band(), tol()).map_err(|r| r.error) {
        Err(FilletError::UnsupportedChain { .. } | FilletError::FilletCornerUnsupported { .. }) => {
        }
        other => panic!("a partial revolve's open mouth arc must refuse, got {other:?}"),
    }
}

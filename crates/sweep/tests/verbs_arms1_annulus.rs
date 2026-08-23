//! **VERBS-ARMS-1 acceptance: the one-edge closed rim's ANNULUS band.**
//!
//! A full solid of revolution's latitude rim is ONE closed edge, so the
//! band replacing it has two closed boundary circles and no strut-and-
//! `kef` ladder to walk. These rows are the evidence that the annulus
//! surgery builds it, and what makes each go red:
//!
//! - **End to end**: the dome's equator fillets, the result is tier-3
//!   valid (which is where the band's torus, its slit's `Seam`
//!   description and both trim circles' tangent-intersection
//!   descriptions are all re-derived at rest) and its census is pinned
//!   — red on any change to what the six moves mint.
//! - **The band is one more revolution wall**: its boundary is one
//!   cycle carrying two CLOSED circles and one doubly-traversed slit,
//!   and it carries no ring — red if the annulus ever closes with a
//!   ring, which `props`' closed-form inventory does not represent.
//! - **Mass properties vs closed form**: the filleted dome's volume
//!   against a hand derivation (washer integral of the cut region),
//!   with a ZERO quadrature pad — red if the band is not the exact
//!   torus the derivation assumes, or if any face leaves the
//!   closed-form inventory.
//! - **The differential pair**: the SAME profile revolved partially
//!   leaves the rim an open plane–sphere arc, which refuses through its
//!   own gate — red if the annulus path ever widens to open chains.
//! - **The naming totality** — every output entity of the annulus band
//!   is a recorded mint or a survivor, and every retirement names a
//!   SOURCE key (the conditional dead-edge push's only evidence).
//! - **The wrap-around G1**: a self-closed link registers no junction,
//!   so predicate 4 reaches a one-edge rim only through the explicit
//!   wrap-around site — vacuous on a circle, live on a kink.
//! - **The planted horn**: the band's torus re-stated with
//!   `minor >= major` is reported by tier 3 (#889's net), on a body
//!   every other check passes — red if that net goes away.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::Surface;
use geom_core::{Band, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::FilletError;
use sweep::fillet::battery::chain_g1;
use sweep::fillet::build::fillet_edges;
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, FaceSurface, ValidationError, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

/// The dome's profile, verbatim from the R1 probe suite: a sphere zone
/// from the equator up 45°, on a flat base annulus, bored on-axis so the
/// profile stays annular and the full revolve mints CLOSED latitude
/// rims.
fn dome_profile(r: f64) -> Vec<ProfileVertex<f64>> {
    sweep::test_support::dome_profile(r)
}

fn revolved(verts: Vec<ProfileVertex<f64>>, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(verts, rev, tol())
}

fn dome(r: f64) -> Body<f64> {
    revolved(dome_profile(r), Revolution::Full)
}

/// The closed plane–sphere rim of radius `rim_r` (to 1e-6).
fn closed_rim_of_radius(body: &Body<f64>, rim_r: f64) -> EdgeKey {
    sweep::test_support::closed_plane_sphere_rim(body, rim_r)
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

/// **The end-to-end row.** The equator of a full solid of revolution
/// fillets, and every geometric claim the six moves make is re-derived
/// at rest by tier 3.
#[test]
fn the_dome_equator_fillets_to_a_tier_3_valid_solid_with_a_pinned_census() {
    let source = dome(1.0);
    assert_eq!(
        census(&source),
        (4, 8, 4),
        "the dome is four walls, four latitude rims and four seams"
    );
    let rim = closed_rim_of_radius(&source, 1.0);
    let out = fillet_edges(&source, &[rim], 0.05, band(), tol())
        .unwrap_or_else(|e| panic!("the dome's one-edge rim fillets, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the filleted dome must be tier-3 valid, got {e:?}"));
    // Two feet minted, the rim vertex retired; two seam children and
    // two trim circles minted, the rim and the plane seam's rim-side
    // piece retired; two strips minted, one merged away.
    assert_eq!(census(&out.body), (5, 10, 5));
    assert_eq!(out.band_faces.len(), 1);
    let surface = out
        .body
        .get_surface(out.body.get_face(out.band_faces[0]).unwrap().surface)
        .unwrap();
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = *surface
    else {
        panic!("the band's surface is a torus, got {surface:?}");
    };
    assert!(
        (minor_radius - 0.05).abs() < 1e-12,
        "the tube is the requested radius, got {minor_radius}"
    );
    // The ball rests at height r inside a 90° corner of the unit
    // sphere: |centre| = 1 − r, so the spine radius is √((1−r)² − r²).
    let spine = ((1.0f64 - 0.05).powi(2) - 0.05f64.powi(2)).sqrt();
    assert!(
        (major_radius - spine).abs() < 1e-12,
        "the spine radius is {spine}, got {major_radius}"
    );
}

/// **Every output entity of an annulus band is a recorded mint or a
/// survivor, both directions.** The ladder rim's totality identity has
/// had this row since M6-5; the annulus mints a DIFFERENT record block
/// (a rim foot from a seam split rather than a strut `mev`, two
/// remnants rather than one, and a dead-edge push that is CONDITIONAL
/// on which child of the plane-seam split carried the source key), and
/// none of it had red-when-broken evidence.
///
/// Red if a mint goes unrecorded, if a record names a key that already
/// existed, if a source entity is destroyed without a retirement row,
/// or if the conditional push starts retiring a key the source never
/// had.
#[test]
fn every_annulus_output_entity_is_a_recorded_mint_or_a_survivor() {
    let source = dome(1.0);
    let rim = closed_rim_of_radius(&source, 1.0);
    let out = fillet_edges(&source, &[rim], 0.05, band(), tol()).unwrap();
    let rec = out.naming.as_ref().expect("the surgery keeps its records");

    // The annulus's own shape, pinned: one band over one source edge,
    // one foot, one seam split, two remnants (one per support), two
    // trims (one per side), one slit.
    assert_eq!(rec.bands.len(), 1);
    assert_eq!(
        rec.bands[0].1,
        vec![rim],
        "the band names its one source edge"
    );
    assert_eq!(rec.rim_feet.len(), 1);
    assert_eq!(rec.meridian_splits.len(), 1);
    assert_eq!(rec.meridian_remnants.len(), 2, "one remnant per support");
    assert_eq!(rec.rim_trims.len(), 2, "one trim circle per side");
    assert_eq!(rec.slits.len(), 1, "one slit per band");
    assert!(
        rec.blends.is_empty() && rec.corners.is_empty() && rec.trims.is_empty(),
        "a lone closed rim fills no open-chain record"
    );

    let mut minted_f: Vec<_> = rec.bands.iter().map(|(f, _)| *f).collect();
    let mut minted_e: Vec<_> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    let mut minted_v: Vec<_> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .collect();
    minted_e.sort_unstable();
    minted_e.dedup();
    minted_v.sort_unstable();
    minted_v.dedup();
    minted_f.sort_unstable();
    minted_f.dedup();

    // No mint is a survivor — except the two record kinds that are
    // documented to name a SURVIVING fragment (a split's remaining
    // piece keeps the parent's key when the parent is that piece).
    for f in &minted_f {
        assert!(source.get_face(*f).is_none(), "a minted face reused a key");
    }
    let fragments: Vec<EdgeKey> = rec
        .meridian_remnants
        .iter()
        .chain(rec.slits.iter())
        .map(|(e, _)| *e)
        .collect();
    for e in &minted_e {
        if !fragments.contains(e) {
            assert!(source.get_edge(*e).is_none(), "a minted edge reused a key");
        }
    }
    for v in &minted_v {
        assert!(
            source.get_vertex(*v).is_none(),
            "a minted vertex reused a key"
        );
    }

    // output ⊆ source ⊎ minted.
    for (f, _) in out.body.faces() {
        assert!(
            minted_f.contains(&f) || source.get_face(f).is_some(),
            "an output face is neither minted nor a survivor"
        );
    }
    for (e, _) in out.body.edges() {
        assert!(
            minted_e.contains(&e) || source.get_edge(e).is_some(),
            "an output edge is neither minted nor a survivor"
        );
    }
    for (v, _) in out.body.vertices() {
        assert!(
            minted_v.contains(&v) || source.get_vertex(v).is_some(),
            "an output vertex is neither minted nor a survivor"
        );
    }

    // (source − retired) ⊆ output, and every retirement names a SOURCE
    // key — which is exactly what the conditional dead-edge push has to
    // get right.
    for e in &rec.dead.edges {
        assert!(
            source.get_edge(*e).is_some(),
            "a retirement names a key the source never had"
        );
        assert!(out.body.get_edge(*e).is_none(), "a retired edge survived");
    }
    for v in &rec.dead.vertices {
        assert!(
            source.get_vertex(*v).is_some(),
            "a retirement names a non-source vertex"
        );
        assert!(
            out.body.get_vertex(*v).is_none(),
            "a retired vertex survived"
        );
    }
    for (e, _) in source.edges() {
        assert!(
            rec.dead.edges.contains(&e) || out.body.get_edge(e).is_some(),
            "a source edge vanished without a retirement record"
        );
    }
    for (v, _) in source.vertices() {
        assert!(
            rec.dead.vertices.contains(&v) || out.body.get_vertex(v).is_some(),
            "a source vertex vanished without a retirement record"
        );
    }
    for (f, _) in source.faces() {
        assert!(
            out.body.get_face(f).is_some(),
            "a source face vanished; supports shrink, they do not die"
        );
    }
}

/// **The wrap-around G1 site.** A self-closed link registers no
/// junction, so predicate 4 reaches a one-edge rim only through the
/// explicit wrap-around check on the link's own carrier endpoints. The
/// check is real — the same `fillet3_chain_g1` margin every junction
/// takes — and it is VACUOUS for a `Curve3::Circle` by construction:
/// the tangent arriving at `t₁` and the one leaving at `t₀` are the
/// same vector to rounding, so the margin decides `Zero`.
///
/// **What this row cannot reach:** a closed carrier with a real kink at
/// its own seam. No door in this tree mints one (every closed rim is a
/// circle), so the non-vacuous half is exercised against the predicate
/// directly rather than through a body.
#[test]
fn the_wrap_around_g1_is_vacuous_on_a_circle_and_live_on_a_kink() {
    let body = dome(1.0);
    let rim = closed_rim_of_radius(&body, 1.0);
    let e = body.get_edge(rim).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    let vertex = body.get_half_edge(e.he_plus).unwrap().start;
    let arm = 2.0;
    chain_g1(
        c.carrier().deriv(t1),
        c.carrier().deriv(t0),
        arm,
        vertex,
        band(),
    )
    .expect("a circle closes on itself tangentially by construction");
    // The same site with a genuine kink: the tangent leaving the vertex
    // rotated by a degree.
    let tau_in = c.carrier().deriv(t1);
    let off = tau_in
        .cross(geom_core::Vec3::new(1.0, 2.0, 3.0))
        .normalize();
    let kinked = tau_in + off * (0.02 * tau_in.norm());
    match chain_g1(tau_in, kinked, arm, vertex, band()) {
        Err(FilletError::ChainNotG1 { .. }) => {}
        other => panic!("a kinked wrap-around must refuse, got {other:?}"),
    }
}

/// **The band is one more revolution wall.** Two closed boundary
/// circles and a doubly-traversed slit in ONE cycle; no ring, which is
/// what `props`' closed-form inventory requires of a curved face.
#[test]
fn the_annulus_band_carries_two_closed_circles_and_a_doubly_traversed_slit() {
    let source = dome(1.0);
    let rim = closed_rim_of_radius(&source, 1.0);
    let out = fillet_edges(&source, &[rim], 0.05, band(), tol()).unwrap();
    let band_face = out.band_faces[0];
    let fd = out.body.get_face(band_face).unwrap();
    assert!(fd.rings.is_empty(), "a curved face carries no ring");
    let topo::LoopBoundary::Cycle { first } = out.body.get_loop(fd.outer).unwrap().boundary else {
        panic!("the band's boundary is a cycle");
    };
    let cycle = out.body.loop_cycle(first).unwrap();
    assert_eq!(cycle.len(), 4, "two circles and a slit traversed twice");
    let mut closed = 0;
    let mut doubled = 0;
    let edges: Vec<EdgeKey> = cycle
        .iter()
        .map(|he| out.body.get_half_edge(*he).unwrap().edge)
        .collect();
    for (i, e) in edges.iter().enumerate() {
        let ed = out.body.get_edge(*e).unwrap();
        let start = out.body.get_half_edge(ed.he_plus).unwrap().start;
        if Some(start) == out.body.half_edge_end(ed.he_plus) {
            closed += 1;
        }
        if edges.iter().skip(i + 1).any(|k| k == e) {
            doubled += 1;
        }
    }
    assert_eq!(closed, 2, "the two trim circles are closed edges");
    assert_eq!(doubled, 1, "exactly one edge is traversed twice: the slit");
}

/// **Mass properties against a hand derivation.** The filleted dome is
/// still a solid of revolution, so its volume is the washer integral of
/// its profile region: the unfilleted dome minus the corner cut between
/// the sphere on the right and the fillet arc on the left. Both halves
/// are elementary and derived here independently of the kernel.
#[test]
fn the_filleted_dome_matches_its_closed_form_volume_with_no_quadrature_pad() {
    let r = 0.05f64;
    let source = dome(1.0);
    let rim = closed_rim_of_radius(&source, 1.0);
    let out = fillet_edges(&source, &[rim], r, band(), tol()).unwrap();
    let props = mass_properties(&out.body, tol()).expect("mass properties must compute");
    assert_eq!(
        props.volume_pad, 0.0,
        "every face of the filleted dome is closed-form"
    );

    // The unfilleted dome: a washer between the bore (x = 1/2) and the
    // unit sphere, from y = 0 to y = 1/√2.
    let a45 = FRAC_1_SQRT_2;
    let dome_volume = core::f64::consts::PI * (0.75 * a45 - a45.powi(3) / 3.0);

    // The cut: the rolling ball rests at (x_c, r) with |centre| = 1 − r.
    let x_c = ((1.0 - r).powi(2) - r * r).sqrt();
    // The sphere contact is the centre projected out to the unit
    // sphere; the cut runs from y = 0 up to its height.
    let h = r / (1.0 - r);
    // ∫₀ʰ π (x_right² − x_left²) dy with x_right = √(1 − y²) and
    // x_left = x_c + √(r² − (y − r)²).
    let i_right = h - h.powi(3) / 3.0;
    let i_s2 = r * r * h - ((h - r).powi(3) + r.powi(3)) / 3.0;
    let arc = |u: f64| 0.5 * u * (r * r - u * u).sqrt() + 0.5 * r * r * (u / r).asin();
    let i_s = arc(h - r) - arc(-r);
    let cut = core::f64::consts::PI * (i_right - (x_c * x_c * h + 2.0 * x_c * i_s + i_s2));

    let expect = dome_volume - cut;
    assert!(
        (props.volume - expect).abs() <= 1e-12 * expect.abs().max(1.0),
        "filleted dome volume: got {}, closed form {expect}",
        props.volume
    );
}

/// **The differential pair.** The same profile revolved PARTIALLY
/// leaves the equator an open plane–sphere arc; the annulus band is a
/// closed-rim construction and that request refuses through its own
/// gate, unchanged.
#[test]
fn the_partial_revolve_of_the_same_profile_still_refuses() {
    let body = revolved(dome_profile(1.0), Revolution::Partial(2.0));
    let arcs: Vec<EdgeKey> = body
        .edges()
        .map(|(k, _)| k)
        .filter(|k| {
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
        .collect();
    assert!(
        !arcs.is_empty(),
        "the partial revolve leaves open plane–sphere arcs"
    );
    match fillet_edges(&body, &arcs[..1], 0.05, band(), tol()) {
        Err(FilletError::UnsupportedChain { .. } | FilletError::FilletCornerUnsupported { .. }) => {
        }
        other => panic!("expected the open plane–sphere arc's own refusal, got {other:?}"),
    }
}

/// **#889's net, planted.** The band's own torus re-stated with
/// `minor >= major` is a horn/spindle: tier 3 reports it, on a body
/// that was tier-3 valid one line above. This is the second net behind
/// the poisoned-spine refusal and the only one that covers the import
/// door, which reads `TOROIDAL_SURFACE`'s two radii verbatim.
#[test]
fn a_planted_horn_torus_is_reported_by_tier_3() {
    let source = dome(1.0);
    let rim = closed_rim_of_radius(&source, 1.0);
    let mut out = fillet_edges(&source, &[rim], 0.05, band(), tol()).unwrap();
    validate_geometric(&out.body, tol()).expect("the filleted dome is tier-3 valid");
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
        panic!("the band's surface is a torus");
    };
    out.body
        .set_face_surface(
            band_face,
            FaceSurface::New(Surface::Torus {
                center,
                axis,
                major_radius,
                // A spindle: the tube swallows the axis.
                minor_radius: major_radius * 2.0,
                u_ref,
            }),
        )
        .unwrap();
    let errors = validate_geometric(&out.body, tol())
        .expect_err("a spindle torus must not pass tier 3")
        .into_iter()
        .filter(|e| matches!(e, ValidationError::DegenerateTorus { .. }))
        .count();
    assert_eq!(errors, 1, "the ring-torus convention is checked per face");
}

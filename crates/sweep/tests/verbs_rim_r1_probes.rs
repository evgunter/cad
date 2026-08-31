//! **Review probes for the closed-rim lever arm** — an independent
//! derivation of what the VERBS-RIM unit claims, written against the
//! public battery/build API rather than against the diff.
//!
//! The rows, and what makes each go RED:
//! - **Straight edges meter bit-identically to the endpoint chord**:
//!   the reduction property is asserted at the bit level on randomized
//!   boxes — red the moment the lever functional moves a plane–plane
//!   path at all.
//! - **Closed circular rims meter their diameter, and never more than
//!   arc length**: the honesty and soundness halves of the ruling, on
//!   randomized radii — red if the lever collapses again OR if it ever
//!   over-reports.
//! - **The #554 pair at randomized geometry**: full and partial
//!   revolves of one profile DECIDE the same dihedral — red if the
//!   closed form regresses to `TangentialEdge`. Past that decision the
//!   closed rim's band is built and the open arc reaches its
//!   unimplemented seam-meridian corner.
//! - **The detector differential**: a co-surface seam meridian still
//!   refuses `TangentialEdge` at margin exactly 0.0 — red if the fix
//!   removed the detector rather than the false positive.
//! - **A closed one-edge chain has no junctions**: the structural fact
//!   the explicit wrap-around G1 site rests on — red if the walk
//!   starts minting junctions for self-closed links.
//! - **The newly reachable frontier builds**: a convex plane–sphere
//!   closed rim PASSES the battery (it used to refuse `TangentialEdge`
//!   before any construction) and `fillet_edges` mints its annulus
//!   band — red if a one-edge rim stops reaching, or stops building
//!   at, the surgery's closed-chain door.
//! - **An open arc just under a full period decides at an honest
//!   lever**: sweep 2π − 0.0032 leaves an endpoint chord of ~0.003·r
//!   (the old lever) while the max pairwise chord meters ~2r; a
//!   near-tangent corner (kink ~1e-6 rad) then sits IN the ambiguity
//!   band at the old lever and decides definitely at the new one —
//!   red (an `Escalated`, or a tangency, instead of the decided corner
//!   refusal) if the lever regresses to the endpoint chord on open
//!   arcs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::battery::{BlendRequest, run_battery};
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{BlendError, ChainClosure, Convexity};
use sweep::test_support::{cube, revolved_about_y};
use test_utils::fuzz;
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Revolve a closed sketch loop about the sketch y-axis.
fn revolved(verts: Vec<ProfileVertex<f64>>, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(verts, rev, tol())
}

/// The surface kind on each side of an edge, plus whether the edge is
/// closed (start vertex == end vertex).
fn edge_sides(body: &Body<f64>, edge: EdgeKey) -> (Surface<f64>, Surface<f64>, bool) {
    let e = body.get_edge(edge).unwrap();
    let surf = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        let f = body.get_loop(l).unwrap().face;
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let start = body.get_half_edge(e.he_plus).unwrap().start;
    let end = body.half_edge_end(e.he_plus).unwrap();
    (surf(e.he_plus), surf(e.he_minus), start == end)
}

/// Every edge matching a two-sided support predicate and a closedness
/// requirement.
fn find_edges(
    body: &Body<f64>,
    closed: bool,
    pair: impl Fn(&Surface<f64>, &Surface<f64>) -> bool,
) -> Vec<EdgeKey> {
    body.edges()
        .map(|(k, _)| k)
        .filter(|k| {
            let (a, b, c) = edge_sides(body, *k);
            c == closed && (pair(&a, &b) || pair(&b, &a))
        })
        .collect()
}

/// The endpoint chord of an edge's carrier — the OLD lever functional,
/// restated as this suite's oracle for the reduction claim only (on a
/// straight edge the ruling requires the new lever to equal it at the
/// bit level).
fn endpoint_chord(body: &Body<f64>, edge: EdgeKey) -> f64 {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    (c.carrier().eval(t1) - c.carrier().eval(t0)).norm()
}

/// A neck-and-flare ring at neck radius `a`: cylinder wall meeting a
/// cone wall at a 30° dihedral on a latitude rim of radius `a`. The
/// profile is annular (off the axis) so the full revolve mints CLOSED
/// latitude rims.
fn neck_flare(a: f64, rev: Revolution<f64>) -> Body<f64> {
    let t30 = (30.0f64).to_radians().tan();
    revolved(
        vec![
            ProfileVertex::new(p2(0.2 * a, 0.0), 0.0),
            ProfileVertex::new(p2(a, 0.0), 0.0),
            ProfileVertex::new(p2(a, 1.0), 0.0),
            ProfileVertex::new(p2(a - t30, 2.0), 0.0),
            ProfileVertex::new(p2(0.2 * a, 2.0), 0.0),
        ],
        rev,
    )
}

/// A dome ring at sphere radius `r`: a sphere zone from the equator up
/// 45°, on a flat base annulus, bored through so the profile stays
/// annular. Its equator rim is ONE closed plane–sphere edge of radius
/// `r`, and the battery PASSES it (Convex) — the fixture that reads
/// `Link::arm_len` back out of a verdict.
fn dome(r: f64) -> Body<f64> {
    sweep::test_support::dome(r, tol())
}

/// A spherical boss of radius `r` rising out of a plate's top face —
/// the concave closed-rim fixture (plate top y = r/2 cuts the sphere
/// at rim radius r·√3/2; bored on-axis so the profile stays annular).
fn boss(r: f64) -> Body<f64> {
    let c = 0.5 * r;
    let rim_r = r * (3.0f64).sqrt() / 2.0;
    let b = 0.2 * r;
    let bore_y = (r * r - b * b).sqrt();
    let sweep = (0.2f64).acos() - core::f64::consts::FRAC_PI_6;
    let bulge = (sweep / 4.0).tan();
    revolved(
        vec![
            ProfileVertex::new(p2(b, 0.0), 0.0),
            ProfileVertex::new(p2(2.0 * r, 0.0), 0.0),
            ProfileVertex::new(p2(2.0 * r, c), 0.0),
            ProfileVertex::new(p2(rim_r, c), bulge),
            ProfileVertex::new(p2(b, bore_y), 0.0),
        ],
        Revolution::Full,
    )
}

/// The closed plane–sphere rim whose circle carrier has radius
/// `rim_r` (to 1e-6) — the dome ring carries TWO closed plane–sphere
/// rims (equator and top edge), so selection is by the analytically
/// known radius, not by uniqueness.
fn closed_rim_of_radius(body: &Body<f64>, rim_r: f64) -> EdgeKey {
    sweep::test_support::closed_plane_sphere_rim(body, rim_r)
}

fn is_cone_cylinder(a: &Surface<f64>, b: &Surface<f64>) -> bool {
    matches!(a, Surface::Cone { .. }) && matches!(b, Surface::Cylinder { .. })
}

/// **The reduction property, at the bit level.** On a straight edge
/// the ruled lever (max pairwise chord over `{t0, mid, t1}`) must
/// equal the endpoint chord exactly — not approximately: the
/// output-stability rule is what licensed swapping the functional
/// under every already-certified plane–plane path. Randomized box
/// sizes; all twelve edges resolve through `run_battery` and each
/// link's `arm_len` is compared to the chord bit-for-bit.
#[test]
fn straight_edges_meter_bit_identically_to_the_endpoint_chord() {
    let mut rng = fuzz::start("rim-r1 straight-edge bit identity");
    for _ in 0..fuzz::scaled(4) {
        let l = rng.range(0.4, 3.0);
        let body = cube(l, tol());
        let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
        assert_eq!(edges.len(), 12, "a box has twelve edges");
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.1 * l,
        };
        let verdict = run_battery(&req, band())
            .unwrap_or_else(|e| panic!("a box passes the battery, got {e:?}; {}", fuzz::replay()));
        for chain in &verdict.chains {
            for link in chain.links() {
                let chord = endpoint_chord(&body, link.edge);
                assert_eq!(
                    link.arm_len.to_bits(),
                    chord.to_bits(),
                    "straight edge {:?} at l = {l}: arm {} != endpoint chord {} at the bit \
                     level; {}",
                    link.edge,
                    link.arm_len,
                    chord,
                    fuzz::replay()
                );
            }
        }
    }
}

/// **Honesty and soundness on closed circular rims**, at randomized
/// radii. The dome's equator (Convex) and the boss's root ring
/// (Concave) each carry ONE closed plane–sphere rim whose carrier is
/// a circle of analytically known radius: `arm_len` must be the
/// diameter to 1e-9 (honest — the old lever read ~0 here) and must
/// never exceed the rim's arc length (sound: a chord lower-bounds
/// arc length).
#[test]
fn closed_rims_meter_their_diameter_and_never_exceed_arc_length() {
    let mut rng = fuzz::start("rim-r1 closed-rim honesty");
    for _ in 0..fuzz::scaled(4) {
        let r = rng.range(0.7, 1.6);
        for (body, expect_convexity, rim_r) in [
            (dome(r), Convexity::Convex, r),
            (boss(r), Convexity::Concave, r * (3.0f64).sqrt() / 2.0),
        ] {
            let rim = closed_rim_of_radius(&body, rim_r);
            let req = BlendRequest {
                body: &body,
                edges: vec![rim],
                size: 0.05 * r,
            };
            let verdict = run_battery(&req, band()).unwrap_or_else(|e| {
                panic!(
                    "a plane–sphere closed rim resolves, got {e:?}; {}",
                    fuzz::replay()
                )
            });
            let link = verdict.chains[0].first();
            assert_eq!(link.convexity, expect_convexity, "{}", fuzz::replay());
            assert!(
                (link.arm_len - 2.0 * rim_r).abs() < 1e-9,
                "closed rim of radius {rim_r}: arm {} is not the diameter; {}",
                link.arm_len,
                fuzz::replay()
            );
            assert!(
                link.arm_len <= core::f64::consts::TAU * rim_r,
                "arm {} exceeds the rim's arc length; {}",
                link.arm_len,
                fuzz::replay()
            );
        }
    }
}

/// **The #554 pair, at randomized neck radii and sweep angles.** The
/// same cone×cylinder corner DECIDES its dihedral — never reporting
/// tangency on a transverse corner — whether its rim is CLOSED (full
/// revolve) or open (partial at any sweep). Red if the closed form
/// regresses to the false `TangentialEdge`.
///
/// Past that decision the forms part: the closed rim's band is built by
/// the cylinder×cone arm, the open arc terminates at the revolve's
/// seam-meridian vertices, whose corner configuration is unimplemented.
#[test]
fn the_554_pair_decides_its_dihedral_at_any_neck_radius() {
    let mut rng = fuzz::start("rim-r1 #554 pair");
    for _ in 0..fuzz::scaled(4) {
        // Keep `a − tan 30°` clear of the 0.2a bore: below a ≈ 0.72
        // the flare wall crosses the bore and profile validation
        // refuses `NonSimple` before any revolve exists.
        let a = rng.range(0.75, 1.5);
        let sweep = rng.range(0.5, 6.0);
        for (rev, closed) in [
            (Revolution::Full, true),
            (Revolution::Partial(sweep), false),
        ] {
            let body = neck_flare(a, rev);
            let rims = find_edges(&body, closed, is_cone_cylinder);
            assert!(
                !rims.is_empty(),
                "a cone×cylinder corner exists at a = {a}; {}",
                fuzz::replay()
            );
            let v = fillet_edges(&body, &rims[..1], 0.05 * a, band(), tol()).map_err(|r| r.error);
            assert!(
                !matches!(v, Err(BlendError::TangentialEdge { .. })),
                "neck {a}, closed = {closed}: a transverse corner is not a tangency, \
                 got {v:?}; {}",
                fuzz::replay()
            );
            let expected = if closed {
                v.is_ok()
            } else {
                matches!(v, Err(BlendError::UnsupportedCorner { .. }))
            };
            assert!(
                expected,
                "neck {a}, closed = {closed}: a closed rim builds its band and an open \
                 arc reaches the unimplemented seam-meridian corner, got {v:?}; {}",
                fuzz::replay()
            );
        }
    }
}

/// **The detector differential**: a full ball's seam meridian is a
/// co-surface edge (one sphere on both sides, dihedral sine exactly
/// zero) and still refuses `TangentialEdge` at margin exactly 0.0 —
/// the fix removed the collapsed lever, not the tangency detector.
#[test]
fn a_co_surface_seam_still_refuses_tangential_at_exactly_zero_margin() {
    let mut rng = fuzz::start("rim-r1 seam differential");
    for _ in 0..fuzz::scaled(4) {
        let r = rng.range(0.5, 2.0);
        let ball = revolved(
            vec![
                ProfileVertex::new(p2(0.0, -r), 1.0),
                ProfileVertex::new(p2(0.0, r), 0.0),
            ],
            Revolution::Full,
        );
        let seams = find_edges(&ball, false, |a, b| {
            matches!(a, Surface::Sphere { .. }) && matches!(b, Surface::Sphere { .. })
        });
        assert!(!seams.is_empty(), "a full ball carries a seam meridian");
        match fillet_edges(&ball, &seams[..1], 0.05 * r, band(), tol()).map_err(|r| r.error) {
            Err(BlendError::TangentialEdge { margin, .. }) => {
                assert_eq!(
                    margin,
                    0.0,
                    "a co-surface seam's sine is structurally zero; {}",
                    fuzz::replay()
                );
            }
            other => panic!(
                "expected TangentialEdge at 0.0, got {other:?}; {}",
                fuzz::replay()
            ),
        }
    }
}

/// **A closed one-edge chain mints no junctions** — the structural
/// fact the wrap-around G1 check rests on: `walk_chains` registers a
/// self-closed link's one vertex once, so the junction loop has
/// nothing to walk and predicate 4 reaches that chain only through the
/// explicit wrap-around site on the link's own carrier endpoints. Red
/// if the walk starts recording a wrap-around junction for a
/// self-closed single link, which would meter it twice.
#[test]
fn a_closed_one_edge_chain_has_no_junctions_to_fold_the_arm_at() {
    let body = dome(1.0);
    let rim = closed_rim_of_radius(&body, 1.0);
    let req = BlendRequest {
        body: &body,
        edges: vec![rim],
        size: 0.05,
    };
    let verdict = run_battery(&req, band()).expect("the dome rim resolves");
    assert_eq!(verdict.chains.len(), 1);
    let chain = &verdict.chains[0];
    assert_eq!(
        chain.link_count(),
        1,
        "one closed edge seeds one one-link chain"
    );
    assert!(matches!(chain.closure, ChainClosure::Closed));
    assert!(
        chain.junctions.is_empty(),
        "a self-closed single link has no junction for predicate 4 to fold at"
    );
}

/// **The newly reachable frontier BUILDS.** Before the lever fix, every
/// closed edge refused `TangentialEdge` inside the battery, so the
/// surgery's closed-chain door had never seen a one-link closed chain;
/// the fix made it live and this row pinned the typed refusal that met
/// it. The annulus band is the construction that door now reaches: a
/// convex plane–sphere closed rim of a full solid of revolution fillets
/// end to end, minting exactly one band face.
#[test]
fn a_passing_closed_rim_reaches_the_surgery_and_builds_its_annulus_band() {
    let body = dome(1.0);
    let rims = [closed_rim_of_radius(&body, 1.0)];
    let out = fillet_edges(&body, &rims[..1], 0.05, band(), tol())
        .unwrap_or_else(|e| panic!("the dome's one-edge rim fillets, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one closed rim mints one band");
    assert!(
        out.blend_faces.is_empty() && out.corner_faces.is_empty(),
        "a lone closed rim mints no open blend and no corner patch"
    );
}

/// **An open arc just under a full period decides at an honest
/// lever.** The 30° neck-and-flare corner revolved 2π − 0.0032 rad
/// leaves the corner rim an OPEN arc whose endpoint chord is
/// ~0.0032·a — the collapsed old lever — while the honest lever
/// meters ~the rim's diameter; the dihedral decides, and the refusal
/// that follows is the OPEN arc's own — its seam-meridian
/// terminations, whose corner configuration is unimplemented — on an
/// arc the endpoint chord would have starved before either door.
///
/// The fixture's kink is the same healthy 30° as the #554 pair, ON
/// PURPOSE: this row first tried a ~1e-6..1e-4 rad kink so the
/// dihedral itself would sit in-band at the collapsed lever, and
/// that fixture cannot exist across the ε matrix — small kinks land
/// the profile validator's `chord_side` margin (~kink·a) in the
/// ε = 1e-6 band, while every kink small enough for that
/// counterfactual makes the wall a near-degenerate cone (apex
/// ~a/kink away) whose revolve chart residuals
/// (`pcurve_map_residual`, `pcurve_loop_continuity`) land in the
/// ε = 1e-12 band. The endpoint-chord regression tripwire this row
/// carried lives instead in the R2 suite's
/// `open_arcs_approaching_a_full_turn_do_not_collapse`, which pins
/// `Link::arm_len` floors on this same class directly and goes red
/// under a collapsed lever at every ε row.
#[test]
fn a_near_full_period_open_arc_decides_its_sign_at_the_honest_lever() {
    let a = 1.0;
    let body = neck_flare(a, Revolution::Partial(core::f64::consts::TAU - 3.2e-3));
    let corner = find_edges(&body, false, is_cone_cylinder);
    assert!(!corner.is_empty(), "the near-full-period corner rim exists");
    // The collapsing-regime witness: the requested rim really is the
    // arc whose endpoint chord has collapsed.
    let chord = endpoint_chord(&body, corner[0]);
    assert!(
        chord < 0.01 * a,
        "the fixture must be in the collapsing regime (endpoint chord {chord})"
    );
    let v = fillet_edges(&body, &corner[..1], 0.05, band(), tol()).map_err(|r| r.error);
    assert!(
        !matches!(v, Err(BlendError::TangentialEdge { .. })),
        "the dihedral must decide at the honest lever, not starve: {v:?}"
    );
    assert!(
        matches!(v, Err(BlendError::UnsupportedCorner { .. })),
        "expected the decided corner refusal at the honest lever, got {v:?}"
    );
}

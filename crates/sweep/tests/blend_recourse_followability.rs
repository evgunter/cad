//! **Every `sweep::blend` recourse sentence, followed to its promised
//! outcome — or its gap named.**
//!
//! A recourse constant is a claim about a SECOND request. Asserting
//! that the sentence renders, or that some old refusal class is gone,
//! says nothing about whether the request it endorses is one the
//! kernel answers: the dead-recourse class was caught twice by
//! reviewers who executed the sentence and watched it re-refuse. So
//! every row here is COMPOSED — it reaches the refusal that carries
//! the constant, asserts the typed variant and the rendered sentence
//! (spelled against the constant, never a phrase of it), and then
//! EXECUTES the request the sentence names and asserts the outcome it
//! promises.
//!
//! **Where no fixture HERE reaches the refusal, the row says exactly
//! that.** A `*_has_no_witness_in_this_suite` row is a measurement of
//! this file, not an invariant of the door: on the bodies named in the
//! row, the constant's variant is not reached through
//! `fillet_edges`/`chamfer_edges`, and what the caller meets at the
//! nearest site is a DIFFERENT refusal carrying a different recourse.
//! Each such row names the PREMISE its fixture fixes — the property
//! that excludes the failing mode — because that premise is where the
//! next witness will come from. Two of these rows were wrong in
//! exactly that way and are gone: `FILLET3_GEOMETRY_RECOURSE` and
//! `FILLET3_RING_RECOURSE` are both front-door reachable, on a
//! non-circular ring and off the clearance screen's sample lattice
//! respectively, and `review_fillet_e2_probes.rs` holds both witnesses
//! (`work/fillet/geometry-recourse-dead-at-line-ring.md`,
//! `work/fillet/ring-clearance-reaches-front-door-off-lattice.md`).
//! Wording a fixture's reach as a door's reach is what hid them.
//!
//! Three constants are held composed elsewhere and are not duplicated
//! here — duplicating them would buy a second copy of the same
//! evidence at full fixture cost:
//! `review_blend1_r2_probes::the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires`
//! (`FILLET3_SEAM_VERTEX_RECOURSE`),
//! `blend_tworims::colliding_bands_on_a_shared_wall_refuse_upfront`
//! (`FILLET3_CLEARANCE_SPLIT_RECOURSE`) and
//! `review_fillet_e2_probes::the_ring_recourse_reaches_the_front_door_off_the_sample_lattice_and_is_followable`
//! (`FILLET3_RING_RECOURSE`).
//!
//! **A recourse constant is not the only place a recourse lives.** Two
//! refusals map to `Recourse::None` in the recourse table — which
//! decides which named CONSTANT is appended, and appends none — and
//! then end their own `Display` arm with advice anyway. Those two are
//! rowed here on the same terms as the named constants
//! ([`a_nonpositive_size_gives_advice_the_recourse_table_says_it_has_none_of`],
//! [`a_repeated_edge_gives_advice_the_recourse_table_says_it_has_none_of`]):
//! no named constant is appended, the advice IS given, and the request
//! it names is executed. The table is not changed — what it routes is
//! which CONSTANT gets appended, and both rows assert that routing is
//! honoured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::blend::{
    ALL_RECOURSES, BlendError, CHAMFER_ARM_RECOURSE, CornerConfig, FILLET3_ASSEMBLY_RECOURSE,
    FILLET3_BODY_RECOURSE, FILLET3_CHAIN_RECOURSE, FILLET3_CLEARANCE_RECOURSE,
    FILLET3_CONVEXITY_RECOURSE, FILLET3_CORNER_RECOURSE, FILLET3_GEOMETRY_RECOURSE,
    FILLET3_RADIUS_RECOURSE, FILLET3_RING_RECOURSE, FILLET3_SPINE_KIND_RECOURSE,
    FILLET3_SPINE_RECOURSE, FILLET3_TANGENTIAL_RECOURSE,
};
use sweep::test_support::{
    closed_plane_sphere_rim, cube, dome, prism, rim_arcs_at, spool, waisted,
};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, EdgeKey, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(p2(x, y), bulge)
}

/// `a ∖ b`, the one boolean these rows use.
fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the subtraction runs")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone()
}

/// A radius-0.3 ball centred at `c`. Same body the review probe dimples
/// its turned prism with, so the two fixtures differ in the ONE thing
/// their rows are about: whether the box is axis-aligned.
fn ball_at(c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![v(0.0, -0.3, 1.0), v(0.0, 0.3, 0.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let b = revolve(&vp, axis, Revolution::Full, tol()).unwrap().body;
    topo::transform_rigid(&b, &Affine3::translation(c), tol()).unwrap()
}

/// The edges of `body` on LINE carriers — on a dimpled or pocketed box,
/// the box's own twelve.
fn line_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|k| {
            body.get_edge(*k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(*c.carrier(), geom::Curve3::Line { .. }))
        })
        .collect()
}

/// The twelve edges of the OUTER unit box: line carriers whose midpoint
/// sits on `x = 0/1` or `y = 0/1`. A pocket's own rim is interior to the
/// top face, so this is what separates the request from the ring.
fn outer_box_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let on = |c: f64| c.abs() < 1e-9 || (c - 1.0).abs() < 1e-9;
    line_edges(body)
        .into_iter()
        .filter(|k| {
            let Some(g) = body
                .get_edge(*k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
            else {
                return false;
            };
            let (t0, t1) = g.params();
            let m = g.carrier().eval((t0 + t1) / 2.0);
            on(m.x) || on(m.y)
        })
        .collect()
}

/// The edges of `body` whose two supports are the SAME surface — a
/// chart seam, whose dihedral is tangential at margin zero. The
/// counterpart of [`rim_arcs_at`]'s second half, which excludes
/// exactly these.
fn seam_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let surface_of = |he| -> Option<topo::SurfaceKey> {
        let l = body.get_half_edge(he)?.parent_loop;
        Some(body.get_face(body.get_loop(l)?.face)?.surface)
    };
    body.edges()
        .filter_map(|(k, e)| (surface_of(e.he_plus)? == surface_of(e.he_minus)?).then_some(k))
        .collect()
}

/// The three edges meeting at `body`'s first vertex.
fn edges_at_first_vertex(body: &Body<f64>) -> Vec<EdgeKey> {
    let v0 = body.vertices().next().unwrap().0;
    query::all_edges(body)
        .into_iter()
        .filter(|k| {
            let e = body.get_edge(*k).unwrap();
            let s = body.get_half_edge(e.he_plus).unwrap().start;
            s == v0 || body.half_edge_end(e.he_plus) == Some(v0)
        })
        .collect()
}

/// The refusal a request meets, or a panic naming what built instead.
fn refusal(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str, chamfer: bool) -> BlendError {
    let out = if chamfer {
        chamfer_edges(body, edges, r, tol())
    } else {
        fillet_edges(body, edges, r, tol())
    };
    match out {
        Err(e) => e.error,
        Ok(_) => panic!("{what}: expected a refusal, the request built"),
    }
}

/// Execute the second request a recourse names and assert the outcome
/// it promises: a body that builds and passes tier-3 validation.
fn builds(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) {
    let out = fillet_edges(body, edges, r, tol())
        .unwrap_or_else(|e| panic!("{what}: the recourse's own request must build, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: and the result must be tier-3 valid, got {e:?}"));
}

/// [`builds`] for the chamfer verb.
fn chamfers(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) {
    let out = chamfer_edges(body, edges, r, tol())
        .unwrap_or_else(|e| panic!("{what}: the recourse's own request must build, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: and the result must be tier-3 valid, got {e:?}"));
}

/// The refusal carries exactly the recourse under test.
fn carries(err: &BlendError, recourse: &str, what: &str) {
    let shown = err.to_string();
    assert!(
        shown.contains(recourse),
        "{what}: the refusal must carry its own recourse.\n  got: {shown}"
    );
}

// ------------------------------------------------------------------
// Followable: the second request the sentence names is answered.
// ------------------------------------------------------------------

/// **`FILLET3_RADIUS_RECOURSE` — "reduce the fillet radius".**
///
/// The dome's equator is a plane–sphere rim with a unit sphere's
/// curvature headroom; `r = 2` exhausts it. The reduced radius the
/// sentence endorses then builds on the SAME request.
///
/// The sentence's other clause — "blend a support with more curvature
/// headroom" — names a different body, not a second request against
/// this one, and is not pinned here.
#[test]
fn the_radius_recourse_reduces_to_a_radius_that_builds() {
    let body = dome(1.0, tol());
    let rim = [closed_plane_sphere_rim(&body, 1.0)];
    let err = refusal(&body, &rim, 2.0, "r = 2 on a unit sphere's rim", false);
    assert!(
        matches!(err, BlendError::RadiusHeadroom { .. }),
        "the curvature headroom is what refuses, got {err:?}"
    );
    carries(&err, FILLET3_RADIUS_RECOURSE, "radius headroom");
    builds(&body, &rim, 0.1, "the reduced radius");
}

/// **`FILLET3_CLEARANCE_RECOURSE` — "reduce the blend size".**
///
/// The waisted revolve's base rim at `r = 0.5` sets back further than
/// the cone wall is wide, so the screen cannot certify the wall
/// survives; the two setbacks are ONE chain's, so this is the
/// non-splittable sentence. The reduced size builds.
///
/// The sentence's other clause — "enlarge the support face" — is a
/// different body, and is not pinned here.
#[test]
fn the_clearance_recourse_reduces_to_a_blend_size_that_builds() {
    let body = waisted(tol());
    let rim = rim_arcs_at(&body, 1.0, 0.0);
    assert_eq!(rim.len(), 2, "the base rim is seam-split");
    let err = refusal(&body, &rim, 0.5, "r = 0.5 on the base rim", false);
    assert!(
        matches!(
            err,
            BlendError::FaceClearanceUncertified {
                cross_chain: false,
                ..
            }
        ),
        "one chain's two setbacks: the non-splittable clearance sentence, got {err:?}"
    );
    carries(&err, FILLET3_CLEARANCE_RECOURSE, "face clearance");
    builds(&body, &rim, 0.05, "the reduced blend size");
}

/// **`FILLET3_TANGENTIAL_RECOURSE` — "blend an edge whose supports meet
/// at a definite angle".**
///
/// A chart seam's two supports are one surface, so its dihedral decides
/// Zero at any lever. The definite-angle edge the sentence names is on
/// the SAME body: the dome's equator, where the base plane meets the
/// sphere.
#[test]
fn the_tangential_recourse_names_a_definite_angle_edge_that_builds() {
    let body = dome(1.0, tol());
    let seams = seam_edges(&body);
    assert!(!seams.is_empty(), "the full revolve mints chart seams");
    let err = refusal(&body, &seams[..1], 0.05, "a chart seam", false);
    assert!(
        matches!(err, BlendError::TangentialEdge { margin, .. } if margin == 0.0),
        "a co-surface seam is the zero-margin wedge, got {err:?}"
    );
    carries(&err, FILLET3_TANGENTIAL_RECOURSE, "tangential edge");
    builds(
        &body,
        &[closed_plane_sphere_rim(&body, 1.0)],
        0.1,
        "the definite-angle edge",
    );
}

/// **`FILLET3_CHAIN_RECOURSE` — all three of its clauses, on one cube.**
///
/// Two adjacent cube edges are connected but not tangent-continuous.
/// The sentence then says three things, and each is executed:
///
/// - splitting at a CORNER "refuses again as a run-out" — the
///   sentence's own named alternative refusal, asserted as such;
/// - "request every edge of EVERY corner the chain terminates at" —
///   the whole cube, which builds. The clause carried no `EVERY`
///   before this unit: read literally it endorsed the three edges at
///   the shared corner, and that request re-refuses as a run-out at
///   the three FAR corners, which the row also pins.
#[test]
fn the_chain_recourse_is_followed_by_requesting_every_terminating_corner() {
    let body = cube(1.0, tol());
    let edges = query::all_edges(&body);

    let err = refusal(&body, &edges[..2], 0.1, "two adjacent edges", false);
    assert!(
        matches!(err, BlendError::ChainNotG1 { .. }),
        "adjacent cube edges break tangency at their shared corner, got {err:?}"
    );
    carries(&err, FILLET3_CHAIN_RECOURSE, "chain not G1");

    // Clause: splitting at a corner refuses again as a run-out.
    for half in [&edges[..1], &edges[1..2]] {
        let split = refusal(&body, half, 0.1, "one half of the split", false);
        assert!(
            matches!(split, BlendError::UnsupportedRunOut { .. }),
            "the sentence names this outcome for the split, got {split:?}"
        );
    }

    // Clause: one corner's three edges is NOT enough — the far corners
    // are then the partly-requested ones. This is why the clause is
    // scoped to every corner the chain terminates at.
    let corner = edges_at_first_vertex(&body);
    assert_eq!(corner.len(), 3, "a cube corner is trivalent");
    let partial = refusal(&body, &corner, 0.1, "one corner's three edges", false);
    assert!(
        matches!(partial, BlendError::UnsupportedRunOut { .. }),
        "three edges at ONE corner still run out at the far ones, got {partial:?}"
    );

    assert!(
        FILLET3_CHAIN_RECOURSE.contains("EVERY corner the chain terminates at"),
        "the clause is scoped to every terminating corner: {FILLET3_CHAIN_RECOURSE}"
    );
    builds(&body, &edges, 0.1, "every edge of every terminating corner");
}

/// **`FILLET3_CORNER_RECOURSE` — "a chain that terminates only in FULLY
/// REQUESTED trivalent vertices whose three edges are all convex".**
///
/// One cube edge leaves both its corners partly requested. The
/// sentence's positive clause is then executed: the whole cube, whose
/// every corner is a fully-requested all-convex trihedron over
/// plane–plane supports, builds.
///
/// The `FULLY REQUESTED` condition is this unit's: without it the
/// sentence endorsed a chain terminating in an all-convex trivalent
/// vertex, which is exactly what the three-edges-at-one-corner request
/// is — and that request refuses with this same variant (pinned in the
/// chain row above). Its sibling `FILLET3_ASSEMBLY_RECOURSE` carried
/// the condition already.
///
/// The negative clause ("mixed-convexity corners and general run-outs
/// are not implemented") endorses no request and is unpinnable.
#[test]
fn the_corner_recourse_names_a_fully_requested_uniform_corner_that_builds() {
    let body = cube(1.0, tol());
    let edges = query::all_edges(&body);
    let err = refusal(&body, &edges[..1], 0.1, "one cube edge", false);
    assert!(
        matches!(err, BlendError::UnsupportedRunOut { .. }),
        "one edge leaves its corners partly requested, got {err:?}"
    );
    carries(&err, FILLET3_CORNER_RECOURSE, "run-out");
    assert!(
        FILLET3_CORNER_RECOURSE.contains("FULLY REQUESTED"),
        "the endorsed corner is conditioned on being wholly requested: \
         {FILLET3_CORNER_RECOURSE}"
    );
    builds(&body, &edges, 0.1, "every corner fully requested");
}

/// **`FILLET3_ASSEMBLY_RECOURSE` — the refusal it rides carries it, and
/// every door it names is executed.**
///
/// The refusal: the REPAIRED lantern (`merge_coplanar_faces` fuses each
/// pole cap's two half-disks into one face, as every boolean consumer
/// must), whose neck rim then has both arcs on ONE plane face, routes to
/// the ladder and refuses on its ring gate — exactly the exception the
/// closed clause states ("where each support face carries one arc of
/// the rim"), so the sentence is true at the site that carries it. The
/// sentence names three requests that carve, and each is built here:
/// open plane–plane links ending at fully-requested trivalent corners
/// (the cube), a closed circular plane–sphere rim (the dome's equator),
/// and — the "either material side" half — a CONCAVE closed rim (the
/// waisted revolve's waist, whose band adds material).
///
/// What is NOT pinned: the open-chain clause says "on either material
/// side", and the concave side would need an all-plane concave
/// trivalent corner, which no fixture here builds.
#[test]
fn the_assembly_recourse_names_two_doors_that_both_carve() {
    let mut repaired = sweep::test_support::lantern(tol());
    repaired
        .merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    let neck = rim_arcs_at(&repaired, 1.0, 0.0);
    assert_eq!(neck.len(), 2, "the repaired neck rim is still two arcs");
    let err = refusal(&repaired, &neck, 0.05, "the repaired neck rim", false);
    assert!(
        matches!(&err, BlendError::UnsupportedChain { detail, .. } if detail.contains("ring")),
        "one plane face hosting both arcs routes to the ladder, whose ring gate refuses: {err:?}"
    );
    carries(&err, FILLET3_ASSEMBLY_RECOURSE, "unsupported chain");

    let boxy = cube(1.0, tol());
    builds(
        &boxy,
        &query::all_edges(&boxy),
        0.1,
        "single plane–plane links at fully-requested corners",
    );
    let d = dome(1.0, tol());
    builds(
        &d,
        &[closed_plane_sphere_rim(&d, 1.0)],
        0.1,
        "a closed circular plane–sphere rim",
    );
    let body = waisted(tol());
    let waist = rim_arcs_at(&body, 0.5, 0.5);
    assert_eq!(waist.len(), 2, "the waist rim is seam-split");
    builds(
        &body,
        &waist,
        0.05,
        "a concave closed rim, on the other material side",
    );
}

/// **`FILLET3_BODY_RECOURSE` — "a body that is a single solid with a
/// single shell".**
///
/// Two cubes grafted into one body are valid input the in-place
/// surgery is not built for. The single-solid body the sentence names
/// is the same cube, and it builds.
#[test]
fn the_body_recourse_names_a_single_solid_that_builds() {
    let mut two = cube(1.0, tol());
    let other = cube(1.0, tol());
    topo::instance::graft_disjoint_all(&mut two, &other, tol()).expect("a disjoint graft");
    let e = query::all_edges(&two);
    let err = refusal(&two, &e[..1], 0.1, "a two-solid body", false);
    assert!(
        matches!(err, BlendError::UnsupportedBody { solids: 2, .. }),
        "the body inventory is what refuses, got {err:?}"
    );
    carries(&err, FILLET3_BODY_RECOURSE, "two-solid body");

    let one = cube(1.0, tol());
    builds(
        &one,
        &query::all_edges(&one),
        0.1,
        "the single-solid single-shell body",
    );
}

/// **`FILLET3_SPINE_KIND_RECOURSE` — "a chain whose support pairs have
/// analytic blend arms (plane–plane or plane–sphere)".**
///
/// The spool's outer wall is a torus, whose pairs the arm table does
/// not carry. The plane–plane chain the sentence names is built here
/// and carves.
///
/// The sentence names a NARROWER set than the table now holds — the
/// refusal's own payload rosters cylinder and cone pairs too — so
/// following it succeeds while under-describing the door. That
/// mismatch is a finding for the door inventory, not a dead recourse,
/// and is left as measured.
#[test]
fn the_spine_kind_recourse_names_an_analytic_pair_that_builds() {
    let s = spool(Revolution::Full, tol());
    let torus_edges: Vec<EdgeKey> = query::all_edges(&s)
        .into_iter()
        .filter(|k| {
            matches!(
                fillet_edges(&s, &[*k], 0.05, tol()).map_err(|r| r.error),
                Err(BlendError::SpineUnsupported { .. })
            )
        })
        .collect();
    assert!(
        !torus_edges.is_empty(),
        "the spool's torus wall has no analytic arm"
    );
    let err = refusal(&s, &torus_edges[..1], 0.05, "a torus-supported edge", false);
    carries(&err, FILLET3_SPINE_KIND_RECOURSE, "spine unsupported");

    let boxy = cube(1.0, tol());
    builds(
        &boxy,
        &query::all_edges(&boxy),
        0.1,
        "a plane–plane chain (the pair the sentence names)",
    );
}

/// **`CHAMFER_ARM_RECOURSE` — "chamfer edges whose two supports are
/// both planes".**
///
/// The dome's equator is plane–sphere: the ruled strip has no arm over
/// it. The both-planes request the sentence names is the cube, and it
/// chamfers.
#[test]
fn the_chamfer_arm_recourse_names_a_plane_plane_pair_that_chamfers() {
    let d = dome(1.0, tol());
    let err = refusal(
        &d,
        &[closed_plane_sphere_rim(&d, 1.0)],
        0.1,
        "chamfering a plane–sphere rim",
        true,
    );
    assert!(
        matches!(err, BlendError::ChamferArmUnsupported { .. }),
        "the chamfer's own arm table refuses, got {err:?}"
    );
    carries(&err, CHAMFER_ARM_RECOURSE, "chamfer arm unsupported");

    let boxy = cube(1.0, tol());
    chamfers(
        &boxy,
        &query::all_edges(&boxy),
        0.1,
        "a both-planes chamfer request",
    );
}

// ------------------------------------------------------------------
// Advice that is NOT a named constant. `Recourse::None` routes these
// two away from every constant above and they advise anyway.
// ------------------------------------------------------------------

/// The refusal appends no named recourse constant — what the table
/// routes as `Recourse::None` — and still says something.
///
/// Reads [`ALL_RECOURSES`], the crate's one home for that list, rather
/// than a copy: these rows' whole content is that NONE of the fifteen
/// appears, so a copy that fell behind would weaken them silently, and
/// the copy this suite used to keep is exactly the failure the home's
/// own doc records.
fn carries_no_named_recourse(err: &BlendError, what: &str) {
    let shown = err.to_string();
    for (name, sentence) in ALL_RECOURSES {
        assert!(
            !shown.contains(sentence),
            "{what}: the `{name}` constant was appended, which the recourse table \
             routes away from this variant.\n  got: {shown}"
        );
    }
}

/// **`NonpositiveSize` advises "supply a positive radius or setback",
/// and the request it names builds.**
///
/// The recourse TABLE routes this variant to `Recourse::None`, and the
/// table's own doc calls it a variant with no advice to give — yet the
/// Display arm ends in a second request, which is a recourse by every
/// working definition this unit uses. So the sentence is followed here
/// on the same terms as a named one: the positive setback chamfers and
/// validates.
///
/// Reached through the CHAMFER door. The check is no longer only
/// there — FILLET-E1 put the shared size gate in `build.rs`, so the
/// fillet door answers `NonpositiveSize` too — and this row stays on
/// the chamfer because that is where the SETBACK half of the sentence
/// ("a positive radius or setback") is followable. The fillet door's
/// own row is E1's.
#[test]
fn a_nonpositive_size_gives_advice_the_recourse_table_says_it_has_none_of() {
    let body = cube(1.0, tol());
    let edges = query::all_edges(&body);
    for size in [0.0, -0.1] {
        let err = refusal(&body, &edges, size, "a nonpositive setback", true);
        assert!(
            matches!(err, BlendError::NonpositiveSize { .. }),
            "a nonpositive size is refused at the door, got {err:?}"
        );
        carries_no_named_recourse(&err, "a nonpositive setback");
        assert!(
            err.to_string()
                .contains("supply a positive radius or setback"),
            "and it advises anyway: {err}"
        );
    }
    chamfers(&body, &edges, 0.1, "the positive setback it names");
}

/// **`RepeatedEdge` advises "request each edge once", and the request
/// it names builds.**
///
/// Same shape as the row above and the same table row: no constant is
/// appended, advice is given, and the deduplicated request carves.
#[test]
fn a_repeated_edge_gives_advice_the_recourse_table_says_it_has_none_of() {
    let body = cube(1.0, tol());
    let edges = query::all_edges(&body);
    let mut repeated = edges.clone();
    repeated.push(edges[0]);
    let err = refusal(&body, &repeated, 0.1, "a repeated edge", false);
    assert!(
        matches!(err, BlendError::RepeatedEdge { edge } if edge == edges[0]),
        "the repeat is named, got {err:?}"
    );
    carries_no_named_recourse(&err, "a repeated edge");
    assert!(
        err.to_string().contains("request each edge once"),
        "and it advises anyway: {err}"
    );
    builds(&body, &edges, 0.1, "each edge requested once");
}

// ------------------------------------------------------------------
// No fixture HERE reaches the constant. Each row pins what the caller
// meets instead AND the premise its fixture fixes — never "the door
// cannot be reached", which is a claim no fixture can support and
// which two rows in this file got wrong.
// ------------------------------------------------------------------

/// **No fixture in this suite reaches `FILLET3_SPINE_RECOURSE`.**
///
/// The sentence endorses "a radius below the spine's own curvature
/// radius", which presumes the caller was told the spine folded. On the
/// one closed-form curved spine these fixtures can build — a
/// plane–sphere rim — the clearance screen answers at every radius from
/// the one that builds to the one that poisons, so `SpineIrregular` is
/// not what this caller reads.
///
/// **The premise, stated as one:** on a plane–sphere rim the spine's
/// curvature limit and the face's clearance limit are the same
/// geometry, so the screen cannot be outrun. A body whose spine folds
/// while clearance stays ample would be handed the sentence; this row
/// does not claim there is none, and the suite builds no such body.
/// That is the shape of witness to look for, and the two flipped
/// verdicts above are what makes the distinction worth writing down.
///
/// The nearest existing pin, `verbs_arms1_r1_probes::near_limit_radii_
/// refuse_typed`, accepts either variant and so does not decide this.
///
/// Red when a radius on this ladder starts refusing at the spine gate:
/// that is when the composed row is owed.
#[test]
fn the_spine_recourse_has_no_witness_in_this_suite_the_clearance_screen_answers_first() {
    let body = dome(1.0, tol());
    let rim = [closed_plane_sphere_rim(&body, 1.0)];
    let (mut built, mut clearance) = (0, 0);
    for r in [0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.7] {
        match fillet_edges(&body, &rim, r, tol()).map_err(|e| e.error) {
            Ok(_) => built += 1,
            Err(BlendError::SpineIrregular { .. }) => panic!(
                "r = {r} reaches the spine gate — {FILLET3_SPINE_RECOURSE} is followable now"
            ),
            Err(BlendError::FaceClearanceUncertified { .. } | BlendError::Escalated { .. }) => {
                clearance += 1;
            }
            Err(other) => panic!("r = {r}: unexpected outcome {other}"),
        }
    }
    assert!(
        built >= 1 && clearance >= 5,
        "the ladder must cross from building to refusing: {built} built, \
         {clearance} met the clearance screen"
    );
}

/// **No fixture in this suite reaches `FILLET3_CONVEXITY_RECOURSE`.**
///
/// The sentence endorses splitting a chain at a convexity flip, which
/// presumes a G1 chain whose links disagree in sign.
///
/// **The premise, stated as one:** every G1 chain this kernel's doors
/// can express TODAY is a rim, and a rim's convexity is uniform, so a
/// body that mixes convexity mixes it at a CORNER and the corner tag
/// answers first. That is a property of the doors, not a theorem about
/// blends — a chain door admitting a non-rim G1 run is exactly where
/// the witness would come from, and this row goes red there.
///
/// The L prism is that body: its reflex edge makes one vertex
/// mixed-convexity, and the whole-body request meets
/// `UnsupportedCorner`, never `ConvexitySignFlip`. The adversarial
/// corpus agrees — `review_d2_adv_probes::d2_reached_variants` reaches
/// ten refusal classes over hundreds of requests and this is not one
/// of them.
#[test]
fn the_convexity_recourse_has_no_witness_in_this_suite() {
    let l = prism(
        vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.0),
            v(2.0, 1.0, 0.0),
            v(1.0, 1.0, 0.0),
            v(1.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ],
        1.0,
        tol(),
    );
    let err = refusal(
        &l,
        &query::all_edges(&l),
        0.1,
        "every edge of an L prism",
        false,
    );
    assert!(
        matches!(
            err,
            BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { .. },
                ..
            }
        ),
        "mixed convexity is met at the CORNER, not along a chain, got {err:?}"
    );
    assert!(
        !err.to_string().contains(FILLET3_CONVEXITY_RECOURSE),
        "and the caller is handed the corner recourse, not the convexity one"
    );
}

/// **`FILLET3_GEOMETRY_RECOURSE` names a ring and an order that
/// builds.**
///
/// The refusal is reached at a support face's non-circular RING:
/// `review_fillet_e2_probes::the_geometry_recourse_reaches_the_front_door_at_a_line_ring_and_cannot_be_followed`
/// is the witness — a square pocket through a cube's top face, the
/// twelve outer edges refused at every radius, because `ring_circle`
/// reads circle rings only.
///
/// This row follows the sentence. The old wording described only the
/// REQUEST ("blend edges whose supports are planes … carriers are lines
/// and circles"), which the twelve requested edges already satisfied —
/// a dead recourse of issue 1278's class. The sentence now says the
/// offending shape need not be one you requested and gives the lever
/// that exists: **cut the feature that leaves the ring AFTER the blend
/// rather than before it.** Executed here, on the same body, at the
/// same radius the pocketed body refuses.
///
/// Red if that order stops working, or if the sentence stops naming
/// the ring — either way the caller is back to advice they cannot act
/// on.
#[test]
fn the_geometry_recourse_names_a_ring_and_an_order_that_builds() {
    let pocket = topo::transform_rigid(
        &cube(0.3, tol()),
        &Affine3::translation(Vec3::new(0.35, 0.35, 0.8)),
        tol(),
    )
    .unwrap();
    let pocketed = subtract(&cube(1.0, tol()), &pocket);
    let outer = outer_box_edges(&pocketed);
    assert_eq!(outer.len(), 12, "the outer box's twelve edges");

    // The refusal, and that it is about the RING rather than anything
    // the caller named.
    let err = refusal(
        &pocketed,
        &outer,
        0.1,
        "the outer edges of a pocketed box",
        false,
    );
    assert!(
        matches!(err, BlendError::UnsupportedGeometry { .. }),
        "the ring's line carriers are what refuse, got {err:?}"
    );
    let shown = err.to_string();
    assert!(
        shown.contains(FILLET3_GEOMETRY_RECOURSE),
        "the caller is handed the geometry recourse: {shown}"
    );
    assert!(
        FILLET3_GEOMETRY_RECOURSE.contains("support face's own ring")
            && FILLET3_GEOMETRY_RECOURSE.contains("AFTER the blend"),
        "the sentence names the ring the refusal is about, and the order that \
         answers it: {FILLET3_GEOMETRY_RECOURSE}"
    );

    // Followed: blend first, cut the pocket second.
    let blended = fillet_edges(
        &cube(1.0, tol()),
        &outer_box_edges(&cube(1.0, tol())),
        0.1,
        tol(),
    )
    .expect("the bare cube's twelve edges blend")
    .body;
    let after = subtract(&blended, &pocket);
    validate_geometric(&after, tol())
        .expect("and cutting the pocket into the blended cube leaves a tier-3 valid body");
}

/// **On a LATTICE-ALIGNED dimple the clearance screen answers before
/// the ring check — a property of this fixture, not of the door.**
///
/// `FILLET3_RING_RECOURSE` is front-door reachable and followable;
/// `review_fillet_e2_probes::the_ring_recourse_reaches_the_front_door_off_the_sample_lattice_and_is_followable`
/// is the composed pin, and this row is not a second copy of it.
///
/// What is measured here is the ORDER, and its premise. The battery's
/// screen samples each boundary edge at `CHAIN_SAMPLES = 9` places; a
/// sampled gap is never smaller than the true one, so which check
/// answers first depends on whether the ring's closest approach to a
/// requested edge lands ON a sample. On an axis-aligned dimpled cube it
/// does, the two margins agree, and the screen answers — the caller
/// reads the CLEARANCE recourse. Turn the same body 30° and it does
/// not, and the exact ring check answers instead.
///
/// That premise — axis alignment — is the whole content of the row, and
/// stating it as a property of the door is what made this suite file
/// the ring recourse unreachable
/// (`work/fillet/ring-clearance-reaches-front-door-off-lattice.md`).
#[test]
fn the_ring_recourse_is_screened_first_on_a_lattice_aligned_dimple() {
    let dimpled = subtract(&cube(1.0, tol()), &ball_at(Vec3::new(0.5, 0.5, 1.1)));
    let box_edges = line_edges(&dimpled);
    assert_eq!(
        box_edges.len(),
        12,
        "the dimple leaves the twelve box edges"
    );

    builds(&dimpled, &box_edges, 0.2, "a setback that clears the ring");
    let err = refusal(
        &dimpled,
        &box_edges,
        0.25,
        "a setback that reaches it",
        false,
    );
    assert!(
        matches!(err, BlendError::FaceClearanceUncertified { .. }),
        "on THIS fixture the samples land on the closest approach, so the screen \
         meters the ring gap first, got {err:?}"
    );
    assert!(
        !err.to_string().contains(FILLET3_RING_RECOURSE),
        "and the caller reads the clearance recourse here, not the ring one"
    );
}

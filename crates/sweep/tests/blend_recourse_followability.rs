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
//! **Where the recourse cannot be followed the row says so in its own
//! name.** A `*_has_no_front_door_witness` row pins the CURRENT
//! outcome: the constant's variant is not reachable through
//! `fillet_edges`/`chamfer_edges` on any body this file can build, and
//! what the caller meets at the nearest site instead is a DIFFERENT
//! refusal carrying a different recourse. Those rows go red the day
//! the variant becomes reachable, which is when its own composed row
//! is owed.
//!
//! Two constants are already held composed elsewhere and are not
//! duplicated here — duplicating them would buy a second copy of the
//! same evidence at full fixture cost:
//! `review_blend1_r2_probes::the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires`
//! (`FILLET3_SEAM_VERTEX_RECOURSE`) and
//! `blend_tworims::colliding_bands_on_a_shared_wall_refuse_upfront`
//! (`FILLET3_CLEARANCE_SPLIT_RECOURSE`).
//!
//! **A recourse constant is not the only place a recourse lives.** Two
//! refusals map to `Recourse::None` in the recourse table — the row
//! that says an invalid-input variant "has no fillet advice to give" —
//! and then end their own sentence with advice anyway. Those two are
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
    BlendError, CHAMFER_ARM_RECOURSE, CornerConfig, FILLET3_ASSEMBLY_RECOURSE,
    FILLET3_BODY_RECOURSE, FILLET3_CHAIN_RECOURSE, FILLET3_CLEARANCE_RECOURSE,
    FILLET3_CLEARANCE_SPLIT_RECOURSE, FILLET3_CONVEXITY_RECOURSE, FILLET3_CORNER_RECOURSE,
    FILLET3_GEOMETRY_RECOURSE, FILLET3_RADIUS_RECOURSE, FILLET3_RING_RECOURSE,
    FILLET3_SEAM_VERTEX_RECOURSE, FILLET3_SPINE_KIND_RECOURSE, FILLET3_SPINE_RECOURSE,
    FILLET3_TANGENTIAL_RECOURSE,
};
use sweep::test_support::{closed_plane_sphere_rim, cube, dome, revolved_about_y, rim_arcs_at};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
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

/// A closed sketch loop extruded `h` along +z.
fn prism(verts: Vec<ProfileVertex<f64>>, h: f64) -> Body<f64> {
    let pf = Profile::new(SketchPlane::xy(), vec![ProfileLoop::new(verts)])
        .validate(tol())
        .unwrap();
    extrude(&pf, Extrusion::Distance(h), tol()).unwrap().body
}

/// The waisted revolve: two cones meeting at radius 0.5, so the waist
/// rim is CONCAVE and the base rim CONVEX — the cheapest body that
/// carries both a clearance-refusing rim and a material-side refusal.
fn waisted() -> Body<f64> {
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(0.5, 0.5, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// A toroidal spool: an off-axis meridian arc revolved, so its outer
/// wall is a TORUS — a support pair with no analytic blend arm.
fn spool() -> Body<f64> {
    let bulge = (core::f64::consts::FRAC_PI_6 / 2.0).tan();
    let (ex, ey) = (1.75, 0.25 * 3.0f64.sqrt());
    revolved_about_y(
        vec![
            v(0.5, 0.0, 0.0),
            v(2.0, 0.0, bulge),
            v(ex, ey, 0.0),
            v(0.5, ey, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
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
    let body = waisted();
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

/// **`FILLET3_ASSEMBLY_RECOURSE` — both doors it names, both executed.**
///
/// The waisted revolve's CONCAVE waist rim is the closed chain the
/// material-side gate refuses. The sentence names two requests that do
/// carve, and each is built here: open plane–plane links ending at
/// fully-requested trivalent corners (the cube), and — for a fillet —
/// a closed circular plane–sphere rim (the dome's equator).
///
/// What is NOT pinned: the open-chain clause says "on either material
/// side", and the concave side would need an all-plane concave
/// trivalent corner, which no fixture here builds.
#[test]
fn the_assembly_recourse_names_two_doors_that_both_carve() {
    let body = waisted();
    let waist = rim_arcs_at(&body, 0.5, 0.5);
    assert_eq!(waist.len(), 2, "the waist rim is seam-split");
    let err = refusal(&body, &waist, 0.05, "the concave waist rim", false);
    assert!(
        matches!(err, BlendError::UnsupportedChain { .. }),
        "a concave closed band is the chain-shape frontier, got {err:?}"
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
    let s = spool();
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

/// Every recourse constant this module can append. Restated here
/// rather than imported as the crate's own private array, exactly as
/// `review_d2_recourse_at_the_site.rs` restates it: a constant dropped
/// from that array must not silently weaken the two rows below, whose
/// whole content is that NONE of these appears.
const EVERY_NAMED_RECOURSE: [(&str, &str); 15] = [
    ("radius", FILLET3_RADIUS_RECOURSE),
    ("clearance", FILLET3_CLEARANCE_RECOURSE),
    ("clearance-split", FILLET3_CLEARANCE_SPLIT_RECOURSE),
    ("tangential", FILLET3_TANGENTIAL_RECOURSE),
    ("spine", FILLET3_SPINE_RECOURSE),
    ("chain", FILLET3_CHAIN_RECOURSE),
    ("convexity", FILLET3_CONVEXITY_RECOURSE),
    ("corner", FILLET3_CORNER_RECOURSE),
    ("seam-vertex", FILLET3_SEAM_VERTEX_RECOURSE),
    ("assembly", FILLET3_ASSEMBLY_RECOURSE),
    ("body", FILLET3_BODY_RECOURSE),
    ("geometry", FILLET3_GEOMETRY_RECOURSE),
    ("ring", FILLET3_RING_RECOURSE),
    ("spine-kind", FILLET3_SPINE_KIND_RECOURSE),
    ("chamfer-arm", CHAMFER_ARM_RECOURSE),
];

/// The refusal appends no named recourse constant — what the table
/// routes as `Recourse::None` — and still says something.
fn carries_no_named_recourse(err: &BlendError, what: &str) {
    let shown = err.to_string();
    for (name, sentence) in EVERY_NAMED_RECOURSE {
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
/// Reached through the CHAMFER door, which is where the check lives.
/// Whether the fillet door grows one is a separate unit's business and
/// this row does not assert either way.
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
// Not followable: no request reaches the constant. The row pins what
// the caller meets instead, so the gap is a measured fact.
// ------------------------------------------------------------------

/// **`FILLET3_SPINE_RECOURSE` has no front-door witness.**
///
/// The sentence endorses "a radius below the spine's own curvature
/// radius", which presumes the caller was told the spine folded. On a
/// plane–sphere rim — the one closed-form curved spine the surgery
/// carries — the clearance screen answers at every radius from the one
/// that builds to the one that poisons: `SpineIrregular` never reaches
/// the caller, so its sentence names a lever nobody was handed.
///
/// The nearest existing pin, `verbs_arms1_r1_probes::near_limit_radii_
/// refuse_typed`, accepts either variant and so does not decide this.
///
/// Red when a radius on this ladder starts refusing at the spine gate:
/// that is when the composed row is owed.
#[test]
fn the_spine_recourse_has_no_front_door_witness_the_clearance_screen_answers_first() {
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

/// **`FILLET3_CONVEXITY_RECOURSE` has no front-door witness.**
///
/// The sentence endorses splitting a chain at a convexity flip, which
/// presumes a G1 chain whose links disagree in sign. Every G1 chain
/// this kernel's doors can express is a rim, and a rim's convexity is
/// uniform; a body that mixes convexity mixes it at a CORNER, and the
/// corner tag answers first with a different recourse.
///
/// The L prism is that body: its reflex edge makes one vertex
/// mixed-convexity, and the whole-body request meets
/// `UnsupportedCorner`, never `ConvexitySignFlip`. The adversarial
/// corpus agrees — `review_d2_adv_probes::d2_reached_variants` reaches
/// ten refusal classes over hundreds of requests and this is not one
/// of them.
#[test]
fn the_convexity_recourse_has_no_front_door_witness() {
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

/// **`FILLET3_GEOMETRY_RECOURSE` has no front-door witness.**
///
/// The sentence endorses supports that are planes (or, for a rim, a
/// sphere cap) with line and circle carriers. Reaching the refusal
/// needs a corner whose support is neither, fully requested — but the
/// chain-shape gate reads the support pair FIRST and answers with the
/// assembly recourse, so the geometry frontier stays behind it.
///
/// The arc-sided prism is the cheapest such body: one wall is a
/// cylinder, and every-edge request meets `UnsupportedChain`.
#[test]
fn the_geometry_recourse_has_no_front_door_witness() {
    let bulge = (core::f64::consts::FRAC_PI_4 / 2.0).tan();
    let a = prism(
        vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, bulge),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ],
        1.0,
    );
    let err = refusal(
        &a,
        &query::all_edges(&a),
        0.1,
        "every edge of an arc-sided prism",
        false,
    );
    assert!(
        matches!(err, BlendError::UnsupportedChain { .. }),
        "the chain-shape gate reads the support pair first, got {err:?}"
    );
    assert!(
        !err.to_string().contains(FILLET3_GEOMETRY_RECOURSE),
        "the caller is handed the assembly recourse, not the geometry one"
    );
}

/// **`FILLET3_RING_RECOURSE` has no front-door witness.**
///
/// The sentence endorses reducing the blend size or moving the feature
/// whose ring the trimline would consume. Both levers are real, but the
/// battery's clearance screen meters the same gap BEFORE the surgery's
/// ring carry-through check runs, and it is the screen that answers: a
/// dimpled cube builds while the dimple clears the setback and refuses
/// `FaceClearanceUncertified` — naming the CLEARANCE recourse — as soon
/// as it does not.
///
/// So the ring sentence is written for a caller nobody becomes. Its one
/// pinned producer is the predicate called directly
/// (`m6_surgery::ring_clearance_trio_definite_pass_definite_refuse_in_band_escalate`).
#[test]
fn the_ring_recourse_has_no_front_door_witness_the_clearance_screen_answers_first() {
    let ball = {
        let lp = ProfileLoop::new(vec![v(0.0, -0.3, 1.0), v(0.0, 0.3, 0.0)]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .unwrap();
        let axis = RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        let b = revolve(&vp, axis, Revolution::Full, tol()).unwrap().body;
        topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.5, 0.5, 1.1)), tol()).unwrap()
    };
    let dimpled = boolean_op_with(
        BooleanOp::Subtract,
        &cube(1.0, tol()),
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the dimple subtracts")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone();

    // The straight edges are the cube's twelve; the dimple's rim is the
    // ring the carry-through check exists for.
    let box_edges: Vec<EdgeKey> = query::all_edges(&dimpled)
        .into_iter()
        .filter(|k| {
            dimpled
                .get_edge(*k)
                .and_then(|e| dimpled.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(*c.carrier(), geom::Curve3::Line { .. }))
        })
        .collect();
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
        "the screen meters the ring gap first, got {err:?}"
    );
    assert!(
        !err.to_string().contains(FILLET3_RING_RECOURSE),
        "the caller is handed the clearance recourse, not the ring one"
    );
}

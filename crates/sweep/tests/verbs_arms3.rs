//! **VERBS-ARMS-3: the general sphere×sphere arm, and what a chain
//! that stops at a chart seam actually meets.**
//!
//! Two halves, and they share a fixture family — solids of revolution,
//! authored here, whose rims are the two shapes a revolve can give a
//! latitude circle:
//!
//! - an **annular** profile revolves to ONE self-closed rim edge per
//!   segment boundary, which is the closed-rim (annulus) band's own
//!   input, and
//! - a **pole-touching** profile revolves to walls split into two
//!   half-bands, so every latitude rim is TWO arcs meeting at two
//!   chart-seam vertices — the shape #319's second finding met.
//!
//! What makes each row go red:
//!
//! - **The equator of a lentil fillets** — two spheres on distinct
//!   centres meeting in a convex circular rim, end to end through the
//!   annulus door, tier-3 valid, with the band's torus against a hand
//!   number the arm never computes.
//! - **The arm is the sphere–sphere one**, and its coaxiality departure
//!   is exactly zero: two spheres always meet in a circle whose axis is
//!   the line through their centres, so this arm's shared-axis
//!   hypothesis is free rather than checked-and-hoped.
//! - **Both material configurations** put the ball centre where the
//!   symmetric closed form `s = √((R ∓ r)² − c²)` says, at every
//!   combination of the two stored sense bits.
//! - **A chain stopping at a seam vertex refuses `SeamVertex`**, with no
//!   run-out policy named, and its recourse is the seam one rather than
//!   the corner one. The vertex's own structure is asserted beside it:
//!   valence four, two co-surface seam meridians (one surface on both
//!   sides — dihedral zero by construction, and each refuses
//!   `TangentialEdge` at margin exactly zero in its own right), and two
//!   rim arcs carrying ONE support pair between them.
//! - **Asking for the rim whole gets past the seam** — no corner
//!   refusal at all — and carves: one annulus band over both arcs, the
//!   walk carrying through the seam vertices rather than stopping at
//!   them, which is what makes the tag's recourse true.
//! - **A one-edge rim registers no seam vertex**, so the new tag cannot
//!   fire on a rim the charts did not split.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::battery::{BlendRequest, run_battery};
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{BlendArm, BlendError, CornerConfig};
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, SurfaceKey, VertexKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// The two spheres' radius, and their centres' half-separation: the
/// lentil's own numbers, and the only ones the closed forms below use.
const SPHERE_R: f64 = 1.0;
const HALF_SEP: f64 = 0.6;
/// The equator rim's radius, `√(R² − c²)` — exact in binary (3-4-5).
const RIM_R: f64 = 0.8;

/// **The lentil**: the solid between two unit spheres centred at
/// `(0, ∓0.6)`, bored on-axis at `0.6` so the profile stays ANNULAR and
/// every latitude rim is one closed edge.
///
/// Its equator at radius `0.8` is the sphere×sphere rim — CONVEX, which
/// is the configuration the composition surgery carves (a concave rim's
/// blend adds material, and that door is elsewhere). Every crossing is
/// exact in binary: both `(0.6, ±0.2)` and `(0.8, 0)` are 3-4-5 points
/// of their own sphere.
fn lentil() -> Body<f64> {
    // A profile arc's bulge is the tangent of a QUARTER of its sweep,
    // and the sweep here is the angle between two exact unit vectors —
    // `(0.6, −0.8)` and `(0.8, −0.6)`, whose cosine is exactly `0.96`.
    let bulge = (0.96f64.acos() / 4.0).tan();
    revolved_about_y(
        vec![v(0.6, -0.2, bulge), v(RIM_R, 0.0, bulge), v(0.6, 0.2, 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// **The seam-split lantern**: the same shapes on a POLE-TOUCHING
/// profile, whose full revolve splits every wall into two half-bands.
///
/// That is the body #319's second finding was probed on: each latitude
/// rim is two arcs, and the two vertices where they meet are the points
/// at which the charts' seam meridians cross the rim.
fn lantern() -> Body<f64> {
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(0.8, 0.6, 0.0),
            v(0.35, 0.75, 0.0),
            v(0.0, 0.75, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The unordered pair of surfaces an edge's two supports carry.
fn supports(body: &Body<f64>, edge: EdgeKey) -> (SurfaceKey, SurfaceKey) {
    let e = body.get_edge(edge).unwrap();
    let face = |he| {
        body.get_face(
            body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
                .unwrap()
                .face,
        )
        .unwrap()
        .surface
    };
    let (a, b) = (face(e.he_plus), face(e.he_minus));
    if a <= b { (a, b) } else { (b, a) }
}

fn band_torus(body: &Body<f64>, face: topo::FaceKey) -> (Point3<f64>, f64, f64) {
    let s = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    match s {
        &Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => (center, major_radius, minor_radius),
        other => panic!("the band face is a torus, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// The arm.
// ------------------------------------------------------------------

/// **The equator fillets, end to end, to its own closed form.**
///
/// The ball rests at distance `R − r` from BOTH centres, and the two
/// centres are `2c` apart on the axis, so by symmetry it sits level with
/// the rim and its radial coordinate — the spine radius the band's torus
/// is built on — is `√((R − r)² − c²)`. That number is derived here from
/// the fixture, not read back from the arm.
#[test]
fn the_sphere_sphere_equator_fillets_to_its_closed_form() {
    let r = 0.05;
    let source = lentil();
    let arcs = rim_arcs_at(&source, RIM_R, 0.0);
    assert_eq!(arcs.len(), 1, "the equator is ONE closed rim edge");
    let out = fillet_edges(&source, &arcs, r, band(), tol())
        .unwrap_or_else(|e| panic!("the sphere-sphere equator fillets, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one band face");
    let (center, major, minor) = band_torus(&out.body, out.band_faces[0]);
    let want = ((SPHERE_R - r).powi(2) - HALF_SEP.powi(2)).sqrt();
    assert!(
        (major - want).abs() < 1e-12,
        "the spine radius is √((R − r)² − c²) = {want}, got {major}"
    );
    assert!(
        (minor - r).abs() < 1e-15,
        "the tube radius is {r}, got {minor}"
    );
    // The pair is symmetric about the equator's own plane, so the spine
    // sits exactly on it.
    assert!(
        center.y.abs() < 1e-15,
        "the spine is level with the rim, got {}",
        center.y
    );
}

/// **The arm the link takes**, and the one thing that makes this arm
/// different from every other curved row: its coaxiality departure is
/// exactly zero by construction. Two spheres on distinct centres meet in
/// a circle whose axis IS the line through those centres, so there is no
/// configuration in which the kinds match and the hypothesis fails.
#[test]
fn the_equator_takes_the_sphere_sphere_arm_at_zero_departure() {
    let source = lentil();
    let arcs = rim_arcs_at(&source, RIM_R, 0.0);
    let req = BlendRequest {
        body: &source,
        edges: arcs.clone(),
        size: 0.05,
    };
    let verdict = run_battery(&req, band()).unwrap_or_else(|e| panic!("the battery passes: {e:?}"));
    let arms: Vec<BlendArm> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links().map(|l| l.arm))
        .collect();
    assert_eq!(arms, vec![BlendArm::SphereSphereTorus]);
    assert!(
        BlendArm::SphereSphereTorus.is_coaxial_torus(),
        "the arm mints a torus about a circular spine"
    );
    assert!(
        BlendArm::ALL.contains(&BlendArm::SphereSphereTorus),
        "the arm is in the table the coverage rows enumerate"
    );
}

/// **Both material configurations**, at the hand closed form.
///
/// The ball centre solves `|c − centre_i| = R ∓ r` for each sphere's own
/// stored sense bit; by the fixture's symmetry that pins the centre to
/// the plane of the rim only when the two folds agree, and otherwise
/// shifts it along the axis by the amount the two offsets differ. Both
/// are written out here, independently of the arm.
#[test]
fn the_sphere_sphere_arm_folds_both_sense_bits() {
    use sweep::fillet::blend::{Meridian, SupportTrace, sheet_center};
    let r = 0.05;
    let sheet = Meridian {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        rim: Point3::new(RIM_R, 0.0, 0.0),
    };
    let trace = |y: f64, side: f64| SupportTrace::Round {
        center: Point3::new(0.0, y, 0.0),
        radius: SPHERE_R,
        side,
    };
    for (sa, sb) in [(1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0)] {
        let center = sheet_center(
            sheet.rim,
            sheet.sheet_normal(),
            trace(-HALF_SEP, sa),
            trace(HALF_SEP, sb),
            r,
        );
        // The two offset radii, and the crossing they imply: the axial
        // station falls out of the two circles' own radical plane and
        // the radial coordinate out of Pythagoras. Nothing here is the
        // arm's algebra.
        let (oa, ob) = (SPHERE_R - r * sa, SPHERE_R - r * sb);
        let want_y =
            -HALF_SEP + (4.0 * HALF_SEP.powi(2) + oa.powi(2) - ob.powi(2)) / (4.0 * HALF_SEP);
        let want_x = (oa.powi(2) - (want_y + HALF_SEP).powi(2)).sqrt();
        assert!(
            (center.y - want_y).abs() < 1e-12 && (center.x - want_x).abs() < 1e-12,
            "senses ({sa}, {sb}): the ball centre is {center:?}, wanted ({want_x}, {want_y})"
        );
        // And the defining equations themselves, said the short way.
        for (y, side) in [(-HALF_SEP, sa), (HALF_SEP, sb)] {
            let d = (center - Point3::new(0.0, y, 0.0)).norm() - SPHERE_R;
            assert!(
                (d + r * side).abs() < 1e-12,
                "senses ({sa}, {sb}): the centre is {d} from a support, wanted {}",
                -r * side
            );
        }
    }
}

// ------------------------------------------------------------------
// The seam vertex.
// ------------------------------------------------------------------

/// The lantern's mouth rim (sphere meets cone at radius `0.8`), as the
/// two arcs a chart seam split it into, and one of the two vertices
/// where they meet.
fn mouth(body: &Body<f64>) -> (Vec<EdgeKey>, VertexKey) {
    let arcs = rim_arcs_at(body, 0.8, 0.6);
    assert_eq!(arcs.len(), 2, "the seam splits the mouth rim into two arcs");
    let e = body.get_edge(arcs[0]).unwrap();
    (arcs.clone(), body.get_half_edge(e.he_plus).unwrap().start)
}

/// **The vertex a chain stopping at a seam actually meets** — its
/// structure, asserted before the refusal that names it.
///
/// Four incident edges: the two rim arcs, which carry ONE support pair
/// between them (the rim arrives and leaves on the same two surfaces),
/// and two seam meridians, each with one surface on BOTH sides. That
/// second fact is the dihedral-zero one, and it is structural: a
/// co-surface edge cannot have a wedge, whatever a normal sampled there
/// would say. Each seam meridian says so in its own right too, refusing
/// `TangentialEdge` at margin exactly zero.
#[test]
fn the_seam_vertex_is_two_co_surface_seams_crossing_one_smooth_rim() {
    let body = lantern();
    let (arcs, vertex) = mouth(&body);
    let orbit = body
        .vertex_orbit(body.get_vertex(vertex).unwrap().emanating.unwrap())
        .unwrap();
    let mut edges: Vec<EdgeKey> = orbit
        .iter()
        .map(|h| body.get_half_edge(*h).unwrap().edge)
        .collect();
    edges.sort_unstable();
    edges.dedup();
    assert_eq!(edges.len(), 4, "the seam vertex is valence four");
    let (seams, rim): (Vec<EdgeKey>, Vec<EdgeKey>) = edges.iter().partition(|k| {
        let (a, b) = supports(&body, **k);
        a == b
    });
    assert_eq!(seams.len(), 2, "two co-surface seam meridians");
    assert_eq!(rim.len(), 2, "two rim arcs");
    assert_eq!(
        supports(&body, rim[0]),
        supports(&body, rim[1]),
        "the rim arrives and leaves on ONE support pair — the surface is smooth through \
         the vertex"
    );
    assert_eq!(rim, arcs, "and those two arcs are the mouth rim's own");
    // The dihedral along a co-surface seam is zero, and the kernel says
    // so on its own metered predicate.
    for seam in seams {
        match fillet_edges(&body, &[seam], 0.02, band(), tol()).map_err(|r| r.error) {
            Err(BlendError::TangentialEdge { margin, .. }) => assert!(
                margin == 0.0,
                "a co-surface seam's dihedral is exactly zero, got {margin}"
            ),
            other => panic!("a co-surface seam refuses as a tangency, got {other:?}"),
        }
    }
}

/// **The refusal, and what it now says.** A chain that stops at a seam
/// vertex is refused as a `SeamVertex` — with NO run-out policy named,
/// because none would help — and its recourse is the one that names the
/// request that describes what the caller wants.
#[test]
fn a_chain_stopping_at_a_seam_vertex_refuses_seam_vertex() {
    let body = lantern();
    let (arcs, _) = mouth(&body);
    match fillet_edges(&body, &arcs[..1], 0.02, band(), tol()).map_err(|r| r.error) {
        Err(
            e @ BlendError::UnsupportedCorner {
                corner: CornerConfig::SeamVertex,
                policy: None,
                ..
            },
        ) => {
            let text = e.to_string();
            assert!(
                text.contains(sweep::fillet::FILLET3_SEAM_VERTEX_RECOURSE),
                "the seam recourse is the one appended: {text}"
            );
            assert!(
                !text.contains(sweep::fillet::FILLET3_CORNER_RECOURSE),
                "the corner recourse names a door that would not help here: {text}"
            );
            assert!(
                !text.contains("run-out"),
                "no run-out policy is named at a vertex no run-out addresses: {text}"
            );
        }
        other => panic!("a chain stopping at a seam vertex refuses SeamVertex, got {other:?}"),
    }
    // The tag's own map: a seam vertex names no policy, every other tag
    // does.
    assert_eq!(CornerConfig::SeamVertex.policy(), None);
    assert!(CornerConfig::NEdgeVertex { valence: 4 }.policy().is_some());
}

/// **The recourse's own content, checked.** Asking for the rim WHOLE
/// gets past the seam entirely — no corner refusal of any kind — and
/// the closed-rim door CARVES it.
///
/// That is what makes the `SeamVertex` recourse true rather than a
/// pointer at a door that is not there. The band is ONE annulus over
/// both arcs: the walk carries THROUGH the seam vertices, and the row
/// above is the licence for it — the surface is smooth there, the two
/// extra edges are co-surface meridians of dihedral zero, and the
/// vertex is retired with the arcs it joined.
#[test]
fn requesting_the_rim_whole_gets_past_the_seam() {
    let body = lantern();
    let (arcs, vertex) = mouth(&body);
    let out = fillet_edges(&body, &arcs, 0.02, band(), tol())
        .unwrap_or_else(|e| panic!("the whole rim carves, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    assert_eq!(
        out.band_faces.len(),
        1,
        "the two arcs of ONE rim share ONE annulus band"
    );
    let naming = out
        .naming
        .as_ref()
        .expect("the rim phase records what it minted");
    let mut banded: Vec<EdgeKey> = naming
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
        .collect();
    banded.sort_unstable();
    let mut want = arcs.clone();
    want.sort_unstable();
    assert_eq!(banded, want, "both arcs are named by the one band");
    assert!(
        naming.dead.vertices.contains(&vertex),
        "the seam vertex a chain used to STOP at is retired with the rim"
    );
}

/// **A one-edge rim registers no seam vertex.** The differential that
/// keeps the new tag from firing where the charts did not split
/// anything: the lentil's own rims are self-closed, so they are closed
/// chains, and closed chains reach no corner classifier at all.
#[test]
fn a_one_edge_rim_never_reaches_the_seam_tag() {
    let source = lentil();
    for (r, y) in [(RIM_R, 0.0), (0.6, 0.2), (0.6, -0.2)] {
        let arcs = rim_arcs_at(&source, r, y);
        assert_eq!(arcs.len(), 1, "an annular revolve's rim is ONE edge");
        fillet_edges(&source, &arcs, 0.05, band(), tol())
            .unwrap_or_else(|e| panic!("the r={r} rim at y={y} fillets, got {e:?}"));
    }
}

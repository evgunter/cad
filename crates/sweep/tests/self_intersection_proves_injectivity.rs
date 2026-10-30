//! **Self-intersection proves the boundary injective** (ERROR-DESIGN's
//! *Self-intersection scope*): the engine examines every face pair, a
//! face against itself included, decides a touching cell pair by a local
//! certificate and a non-touching one by distance.
//!
//! Each row runs `topo::clearance`'s sweep on a built body at the strict
//! bound, one body against itself, and names the failure it goes red
//! under:
//!
//! - a 370° flat ribbon wall is `Violated` — red if a normal-cone rule
//!   (its normals are all one vector) replaced the monotone arm;
//! - the carvetail fold is `Violated`, its witness at `v ≈ ½`;
//! - a cone, a pyramid and an embedded triangle loft `Hold`, every pair
//!   examined and the touching ones certified — red if the global drop
//!   of vertex-sharing pairs returned;
//! - a declared wedge-0 kiss refuses typed, naming the second-order arm
//!   and the kiss;
//! - `clearance ≥ c` between scopes sharing an edge is `Violated` at 0,
//!   on the shared cell, and their minimum separation is `[0, 0]`;
//! - large curved bodies certify within the default budget, their
//!   cell-pair counts printed;
//! - kernel-built oblique cuts, whose arcs meet seam lines at corners,
//!   hold, some certificates read through a corner, and SHELL-4's vessel
//!   holds; a folded cut reads `Violated` off its corners;
//! - the cylinder–cylinder tee refuses typed at a vertex its junction
//!   shares with a cap and nothing else.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::{NurbsCurve3, Surface};
use geom_core::k_stats::decide;
use geom_core::{Band, Margin, Point2, Point3, Sign, Tol};
use profile::{Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start};
use std::collections::BTreeSet;
use sweep::test_support::revolved_about_y;
use sweep::{Revolution, extrude, loft_body, revolve, sweep_body};
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, CLEARANCE_MARGIN, Candidate, CertificateArm,
    ClearanceBound, DEFAULT_MAX_CELL_DEPTH, EngineRefusal, SharedCell, Side, Sweep, Which,
    min_separation,
};
use topo::{Body, FaceKey, LoopBoundary, VertexKey};

use crate::common::{normal_start_place, quad, to_offset};
use crate::revolve_common::{axis_y, donut_profile, validated};

/// The default whole-query budget the document door runs at
/// (`editor_core::clearance::DEFAULT_MAX_CELL_PAIRS`).
const BUDGET: usize = 65_536;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A verified witness: two faces, a point on each, their distance.
#[derive(Debug, Clone, Copy)]
struct Witness {
    a: FaceKey,
    b: FaceKey,
    pa: Point3<f64>,
    pb: Point3<f64>,
    d: f64,
}

fn surface(body: &Body<f64>, f: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(f).unwrap().surface)
        .unwrap()
        .clone()
}

/// **The witness check**: the closest pair of the two cells' on-face
/// stations, evaluated afresh on the stored surfaces, decided at the
/// bound's own site.
fn verify(
    body: &Body<f64>,
    bound: ClearanceBound,
) -> impl FnMut(Candidate<'_, '_, f64>) -> Result<Witness, String> + '_ {
    move |candidate| {
        let (fa, fb) = candidate.faces();
        let (sa, sb) = (surface(body, fa), surface(body, fb));
        let (la, lb) = candidate.on_face_stations();
        let mut best: Option<Witness> = None;
        for &(au, av) in &la {
            let pa = sa.eval(au, av);
            for &(bu, bv) in &lb {
                let pb = sb.eval(bu, bv);
                let d = (pa - pb).norm();
                if best.is_none_or(|w| d < w.d) {
                    best = Some(Witness {
                        a: fa,
                        b: fb,
                        pa,
                        pb,
                        d,
                    });
                }
            }
        }
        let w = best.ok_or("no station")?;
        let definite = match decide(bound.predicate(), Margin::of(w.d - bound.c()), band()) {
            Ok(Sign::Negative) => true,
            Ok(Sign::Zero) => !bound.zero_discharges(),
            _ => false,
        };
        if definite {
            Ok(w)
        } else {
            Err(format!("the closest station pair is {} apart", w.d))
        }
    }
}

/// **The injectivity question** over `faces` of `body` (every face when
/// `None`), at the default depth and `budget`.
fn injective(body: &Body<f64>, faces: Option<Vec<FaceKey>>, budget: usize) -> BoundReport<Witness> {
    let faces = faces.unwrap_or_else(|| body.faces().map(|(k, _)| k).collect());
    let side = Side { body, faces };
    let windows = side.windows(Which::First).expect("every face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget,
        },
    };
    let report = sweep.run(
        &windows,
        &windows,
        true,
        verify(body, ClearanceBound::StrictlyPositive),
    );
    assert!(
        report.receipt.holds(),
        "the receipt adds up: {:?}",
        report.receipt
    );
    report
}

/// A face pair count: every unordered pair, a face against itself
/// included.
fn pairs(n: usize) -> usize {
    n * (n + 1) / 2
}

/// **The face pairs of `body` that share a vertex**, read off its
/// topology: every one touches, and on a body that holds each is
/// certified at least once — the cells holding the shared point always
/// touch, and a touching pair is discharged only by a certificate.
fn touching_pairs(body: &Body<f64>) -> BTreeSet<(FaceKey, FaceKey)> {
    let vertices = |f: FaceKey| -> BTreeSet<VertexKey> {
        let face = body.get_face(f).unwrap();
        core::iter::once(face.outer)
            .chain(face.rings.iter().copied())
            .flat_map(|l| match body.get_loop(l).unwrap().boundary {
                LoopBoundary::Empty { vertex } => vec![vertex],
                LoopBoundary::Cycle { first } => body
                    .loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|he| body.get_half_edge(he).unwrap().start)
                    .collect(),
            })
            .collect()
    };
    let faces: Vec<(FaceKey, BTreeSet<VertexKey>)> =
        body.faces().map(|(k, _)| (k, vertices(k))).collect();
    let mut out = BTreeSet::new();
    for (i, (f, a)) in faces.iter().enumerate() {
        for (g, b) in faces.iter().skip(i + 1) {
            if !a.is_disjoint(b) {
                out.insert((*f.min(g), *f.max(g)));
            }
        }
    }
    out
}

/// **Holds, every pair examined, and the touching ones certified ACROSS
/// faces**: the receipt has a root for every face pair the tree admits,
/// each face against itself discharged by kind or examined, and at least
/// one cross-face certificate for EACH face pair of `body` that shares
/// a vertex ([`touching_pairs`]). The per-pair floor is what goes red
/// if the sweep stops examining any one pair that shares a cell: the
/// monotone self-pairs a free-form face certifies, and the other pairs'
/// certificates, are no part of it.
fn assert_holds(what: &str, report: &BoundReport<Witness>, body: &Body<f64>) {
    let r = report.receipt;
    let faces = body.faces().count();
    let touching = touching_pairs(body);
    println!(
        "[selfx] {what}: {:?} {r:?} {:?} touching pairs = {}",
        report.verdict,
        report.widths,
        touching.len()
    );
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "{what} is embedded: {r:?}"
    );
    assert!(
        r.candidates <= pairs(faces) && r.candidates >= faces + touching.len(),
        "{what}: every face against itself and every touching pair, and the pairs the tree \
         admits: {r:?}"
    );
    assert!(
        !touching.is_empty(),
        "{what}: the row is about touching faces"
    );
    let dropped: Vec<&(FaceKey, FaceKey)> = touching
        .iter()
        .filter(|p| report.across.get(p).is_none_or(|&n| n == 0))
        .collect();
    assert!(
        dropped.is_empty(),
        "{what}: every face pair sharing a vertex is certified across faces, and these \
         {} of {} are not: {dropped:?} (certified across: {:?})",
        dropped.len(),
        touching.len(),
        report.across
    );
}

// ------------------------------------------------------------ fixtures

/// A helix of `turns` revolutions, radius 1, `pitch` per revolution,
/// interpolated through 32 points a turn.
fn helix(turns: f64, pitch: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = TAU * turns * s;
            Point3::new(a.cos(), a.sin(), pitch * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The wall of a sweep whose centre point is highest.
fn top_wall(lofted: &sweep::Lofted<f64>) -> FaceKey {
    let centre = |f: FaceKey| {
        let s = surface(&lofted.body, f);
        let Surface::Nurbs(net) = &s else {
            panic!("a swept wall is NURBS")
        };
        let ((u0, u1), (v0, v1)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
        s.eval(0.5 * (u0 + u1), 0.5 * (v0 + v1))
    };
    lofted.side_faces[0]
        .iter()
        .flatten()
        .copied()
        .max_by(|&a, &b| centre(a).z.total_cmp(&centre(b).z))
        .unwrap()
}

// ---------------------------------------------------------- the rows

/// **A 370° flat ribbon reads `Violated`.** A thin rectangle swept 370°
/// round a planar circle: its top wall is one NURBS face, flat, every
/// normal the same vector, lying over itself across the last 10°. A
/// normal-cone rule would discharge that face against itself at the
/// root; the monotone arm cannot, since the wall turns through 370°, so
/// the face splits until its two passes over the overlap are cells that
/// do not touch, and the distance test finds them coincident.
#[test]
fn a_370_degree_flat_ribbon_reads_violated() {
    let path = helix(370.0 / 360.0, 0.0);
    let place = normal_start_place(&path);
    let profile = quad([(-0.1, -0.02), (0.1, -0.02), (0.1, 0.02), (-0.1, 0.02)]);
    let swept = sweep_body::<f64>(&profile, place, &path, 48, 3, tol()).expect("the ribbon sweeps");
    let wall = top_wall(&swept);
    let report = injective(&swept.body, Some(vec![wall]), BUDGET);
    println!("[selfx] ribbon: {:?} {:?}", report.verdict, report.receipt);
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the ribbon overlaps itself: {:?}", report.verdict)
    };
    assert_eq!((w.a, w.b), (wall, wall), "one wall against itself");
    let azimuth = |p: Point3<f64>| p.y.atan2(p.x).to_degrees();
    for p in [w.pa, w.pb] {
        assert!(
            (-1.0..=11.0).contains(&azimuth(p)),
            "the witness lies where the wall passes twice: {w:?}"
        );
    }
    assert!(
        report.receipt.splits > 0,
        "the wall against itself was not discharged at its root: {:?}",
        report.receipt
    );
}

/// **The carvetail fold reads `Violated`, its witness at `v ≈ ½`.** Two
/// pentagons lofted at `v_degree = 1`; joint 1 turns +170° in one
/// section and −170° in the other, so the seam between walls 0 and 1
/// passes through wedge 0 at `v = ½`, where wall 1 lies along wall 0
/// and crosses it. The two walls share that seam, so the pair is the
/// one the global drop never examined.
#[test]
fn the_carvetail_fold_reads_violated_at_its_middle_station() {
    let pent = |pts: [(f64, f64); 5]| {
        vec![ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let sections = [
        pent([
            (0.0, 0.0),
            (2.0, 0.0),
            (0.03, 0.347),
            (-2.0, 1.0),
            (-2.0, -1.0),
        ]),
        pent([
            (0.0, 0.0),
            (2.0, 0.0),
            (0.03, -0.347),
            (3.0, -3.0),
            (3.0, 3.0),
        ]),
    ];
    let places = [0.0, 1.0].map(|z| SketchPlane::xy_at(Point3::new(0.0, 0.0, z)));
    let lofted = loft_body::<f64>(&sections, &places, 1, tol()).expect("the fold lofts");
    // Walls 0 and 1: the two whose boundary holds the seam over (2, 0).
    let on_seam = |f: FaceKey| {
        let s = surface(&lofted.body, f);
        let Surface::Nurbs(net) = &s else {
            return false;
        };
        let ((u0, u1), (v0, _)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
        [u0, u1].iter().any(|&u| {
            let p = s.eval(u, v0);
            (p.x - 2.0).abs() < 1e-9 && p.y.abs() < 1e-9
        })
    };
    let walls: Vec<FaceKey> = lofted.side_faces[0]
        .iter()
        .flatten()
        .copied()
        .filter(|&f| on_seam(f))
        .collect();
    assert_eq!(walls.len(), 2, "two walls share the seam at (2, 0)");
    let report = injective(&lofted.body, Some(walls.clone()), BUDGET);
    println!("[selfx] fold: {:?} {:?}", report.verdict, report.receipt);
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the fold crosses itself: {:?}", report.verdict)
    };
    assert!(walls.contains(&w.a) && walls.contains(&w.b), "{w:?}");
    for p in [w.pa, w.pb] {
        assert!(
            (p.z - 0.5).abs() < 0.05,
            "the witness is at the fold's station: {w:?}"
        );
    }
}

/// **A cone holds**: the triangle `(0,0) (1,0) (0,1)` revolved a full
/// turn about `y` — two half-cones sharing their two meridians and the
/// apex, each sharing a half rim with the base disc.
#[test]
fn a_revolved_cone_holds() {
    let profile = validated(vec![ProfileLoop::polygon(
        [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)].map(|(x, y)| Point2::new(x, y)),
    )]);
    let body = revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body;
    let report = injective(&body, None, BUDGET);
    assert_holds("the cone", &report, &body);
}

/// **A pyramid holds**: a square base and an apex — four planar faces
/// meeting at the apex, two of each pair of opposite ones sharing only
/// that vertex.
#[test]
fn a_pyramid_holds() {
    use topo::test_support::meeting::{Pose, posed_pyramid};
    let at_rest = posed_pyramid(
        &[
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        [0.5, 0.5, 0.8],
        &Pose::rest(),
        tol(),
    );
    let body = at_rest.into_body();
    let report = injective(&body, None, BUDGET);
    assert_holds("the pyramid", &report, &body);
}

/// **An embedded triangle loft holds**: every wall shares a seam with
/// both others and a rim with each cap.
#[test]
fn an_embedded_triangle_loft_holds() {
    let tri = |s: f64, turn: f64| {
        let p = |a: f64| Point2::new(s * (a + turn).cos(), s * (a + turn).sin());
        vec![ProfileLoop::polygon(
            [0.0, TAU / 3.0, 2.0 * TAU / 3.0].map(p),
        )]
    };
    let sections = [tri(1.0, 0.0), tri(0.6, 0.3)];
    let places = [0.0, 1.0].map(|z| SketchPlane::xy_at(Point3::new(0.0, 0.0, z)));
    let lofted = loft_body::<f64>(&sections, &places, 1, tol()).expect("the triangles loft");
    let report = injective(&lofted.body, None, BUDGET);
    assert_holds("the triangle loft", &report, &lofted.body);
}

/// **A declared wedge-0 kiss refuses typed**: the lune between the
/// internally tangent circles `(0,1) r 1` and `(0,2) r 2`, `.cusp()` at
/// the kiss, extruded. The two walls meet along the kiss strut with
/// their outward normals opposed, which only the second-order arm
/// reads; it is not built, so the strut refuses, named.
#[test]
fn a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm() {
    let lune: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 4.0))
        .angle(-core::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(core::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lune])
        .validate(tol())
        .unwrap();
    let body = extrude(&profile, to_offset(1.0), tol()).unwrap().body;
    let report = injective(&body, None, BUDGET);
    println!("[selfx] kiss: {:?} {:?}", report.verdict, report.receipt);
    match report.verdict {
        BoundVerdict::Refused(EngineRefusal::Uncertified {
            shared: SharedCell::Edge(e),
            arm: CertificateArm::SecondOrder,
            ..
        }) => {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = *body
                .get_point(
                    body.get_vertex(body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .point,
                )
                .unwrap();
            assert!(
                p.x.abs() < 1e-9 && p.y.abs() < 1e-9,
                "the refusal names the kiss: {p:?}"
            );
        }
        other => panic!("the kiss refuses at the second-order arm: {other:?}"),
    }
}

/// **`clearance ≥ c` between scopes that share an edge is violated at
/// 0, on the shared cell**: two walls of an extruded square, each its
/// own scope. Their minimum separation is `[0, 0]`, naming the edge.
#[test]
fn clearance_between_scopes_sharing_an_edge_is_violated_at_zero() {
    let profile = Profile::new(
        SketchPlane::xy(),
        quad([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    )
    .validate(tol())
    .unwrap();
    let built = extrude(&profile, to_offset(1.0), tol()).unwrap();
    let walls = built.side_faces();
    let (fa, fb) = (walls[0][0], walls[0][1]);
    let side = |f| Side {
        body: &built.body,
        faces: vec![f],
    };
    let (sa, sb) = (side(fa), side(fb));
    let (wa, wb) = (
        sa.windows(Which::First).unwrap(),
        sb.windows(Which::Second).unwrap(),
    );
    let bound = ClearanceBound::AtLeast(0.1);
    let sweep = Sweep {
        bound,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: BUDGET,
        },
    };
    let mut met = None;
    let report = sweep.run(&wa, &wb, true, |candidate| {
        met = candidate.meeting();
        verify(&built.body, bound)(candidate)
    });
    assert!(report.receipt.holds(), "{:?}", report.receipt);
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!(
            "two walls sharing an edge are 0 apart: {:?}",
            report.verdict
        )
    };
    assert!(w.d < 1e-12, "the witness is at 0: {w:?}");
    assert!(
        matches!(met, Some(SharedCell::Edge(_))),
        "named on the shared edge: {met:?}"
    );
    assert_eq!(CLEARANCE_MARGIN, bound.predicate());
    let m = min_separation(&sa, &sb, true, band(), Budgets::MIN_SEPARATION).unwrap();
    assert_eq!((m.lo(), m.hi()), (0.0, Some(0.0)), "{}", m.render());
    let meets = m.meets().expect("the scopes meet");
    assert!(matches!(meets.shared, SharedCell::Edge(_)), "{meets:?}");
}

// ---------------------------------------------- large curved bodies

/// **A full torus certifies by kind**: one face, `R = 2 > r = 1/2`.
#[test]
fn a_full_torus_certifies_by_kind() {
    let body = revolve(
        &validated(vec![donut_profile()]),
        axis_y(),
        Revolution::Full,
        tol(),
    )
    .unwrap()
    .body;
    let report = injective(&body, None, BUDGET);
    assert_holds("the torus", &report, &body);
    assert_eq!(
        report.receipt.by_kind,
        body.faces().count(),
        "every torus face against itself discharged by kind, R > r decided: {:?}",
        report.receipt
    );
}

/// **A long helical sweep certifies within the default budget**: a
/// square tube two turns round a helix of pitch 0.4, its four walls
/// sharing helical seams.
#[test]
fn a_long_helical_sweep_certifies_within_the_default_budget() {
    let path = helix(2.0, 0.4);
    let place = normal_start_place(&path);
    let profile = quad([(-0.08, -0.08), (0.08, -0.08), (0.08, 0.08), (-0.08, 0.08)]);
    let swept = sweep_body::<f64>(&profile, place, &path, 65, 3, tol()).expect("the helix sweeps");
    let report = injective(&swept.body, None, BUDGET);
    assert_holds("the helical sweep", &report, &swept.body);
}

/// **A big thin-walled cylinder shell**: the tube `r ∈ [0.96, 1]`, two
/// high, revolved a full turn — two walls 0.04 apart that share
/// nothing, and two annular caps.
#[test]
fn a_big_thin_walled_cylinder_shell_certifies_within_the_default_budget() {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let body = revolved_about_y(
        vec![p(0.96, 0.0), p(1.0, 0.0), p(1.0, 2.0), p(0.96, 2.0)],
        Revolution::Full,
        tol(),
    );
    let report = injective(&body, None, BUDGET);
    assert_holds("the thin shell", &report, &body);
}

// ------------------------------------------- a face across its seam

/// **A cylinder face across its chart's seam, pierced by a planar face
/// it shares an edge with, never `Holds`.** The wall is the unit
/// cylinder over `u ∈ [β, α + 2π]`, `β = 4`, `α = 0.2`, so its chart
/// window crosses `u = 0`. A planar flap hangs off its `α` strut along
/// the chord toward the ruling at `u = 5` and past it: the flap crosses
/// the wall there, a coincidence of two points of M on no cell the two
/// faces share.
///
/// The wall and the flap touch on the strut, so the pair is a
/// departure question. A presentation that read the wall's lines in
/// the order they come, and wrapped each toward the rectangle the last
/// one left, presented the wall as the sliver `(0, α)` and certified the
/// pair at its root — `Holds` over a body that crosses itself. The wall
/// is presented on the one sheet of its own span a cell meets, and a
/// cell that meets two is split, so the crossing is reached.
#[test]
fn a_seam_crossing_cylinder_pierced_by_a_face_on_its_edge_never_holds() {
    use geom_brep::EdgeCurveSpec;
    use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
    use topo::test_support::{CylFrame, CylKey, cyl_wall_sheet_keyed, plane, set_sheet_sense};

    let (alpha, beta) = (0.2, 4.0);
    let frame = CylFrame::canonical(1.0);
    let mut body = Body::<f64>::new();
    let (wall, _) = cyl_wall_sheet_keyed(
        &mut body,
        frame,
        CylKey::Bare,
        (beta, alpha + TAU),
        (0.0, 1.0),
        tol(),
    );
    topo::mint_pcurves(&mut body, tol()).unwrap();
    set_sheet_sense(&mut body, wall, true, tol());

    // The `α` strut, and its half-edge on the wall's twin across it.
    let at = |u: f64, v: f64| frame.at::<f64>(u, v);
    let vertex_at = |body: &Body<f64>, p: Point3<f64>| {
        body.vertices()
            .find(|(_, x)| (*body.get_point(x.point).unwrap() - p).norm() < 1e-12)
            .unwrap()
            .0
    };
    let (s_lo, s_hi) = (
        vertex_at(&body, at(alpha, 0.0)),
        vertex_at(&body, at(alpha, 1.0)),
    );
    let on_wall = |body: &Body<f64>, he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
            == wall
    };
    let strut = body
        .edges()
        .find(|(_, e)| {
            let ends = [
                body.get_half_edge(e.he_plus).unwrap().start,
                body.get_half_edge(e.he_minus).unwrap().start,
            ];
            ends.contains(&s_lo) && ends.contains(&s_hi)
        })
        .unwrap()
        .1
        .clone();
    let h_s = [strut.he_plus, strut.he_minus]
        .into_iter()
        .find(|&he| !on_wall(&body, he))
        .unwrap();
    let h = body.get_half_edge(h_s).unwrap().clone();
    let s0 = h.start;
    let h_next = h.next;
    let s1 = body.get_half_edge(h_next).unwrap().start;
    let point = |body: &Body<f64>, v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let (q0, q1) = (point(&body, s0), point(&body, s1));
    // The flap's far side: the chord from the strut toward the ruling at
    // `u = 5`, carried 1.3 times as far, at each end's height.
    let past = |q: Point3<f64>| {
        let toward = at(5.0, q.z);
        q + (toward - q) * 1.3
    };
    let (a0, a1) = (past(q0), past(q1));

    let strut1 = body
        .mev_line(
            MevSite::Fan {
                he1: h_next,
                he2: h_next,
            },
            a1,
            tol(),
        )
        .unwrap();
    let strut0 = body
        .mev_line(MevSite::Fan { he1: h_s, he2: h_s }, a0, tol())
        .unwrap();
    let flap = body
        .mef(
            MefSite::Chords {
                he1: strut0.he_minus,
                he2: strut1.he_minus,
            },
            EdgeCurveSpec::line_between(a0, a1),
            FaceSurface::New {
                surface: plane(&[a0, q0, q1, a1], tol()),
                sense: true,
            },
            tol(),
        )
        .unwrap()
        .face;
    topo::mint_pcurves(&mut body, tol()).unwrap();

    let report = injective(&body, Some(vec![wall, flap]), BUDGET);
    println!(
        "[selfx] the seam-crossing wall and its flap: {:?} {:?}",
        report.verdict, report.receipt
    );
    assert!(
        !matches!(report.verdict, BoundVerdict::Holds),
        "the flap crosses the wall at u = 5: {:?} {:?}",
        report.verdict,
        report.receipt
    );
    if let BoundVerdict::Violated(w) = &report.verdict {
        assert!(w.d < 1e-9, "a coincidence: {w:?}");
    }
}

// ------------------------------------------- curved shared edges

/// The surface kind of `f`, by name.
fn kind(body: &Body<f64>, f: FaceKey) -> &'static str {
    match surface(body, f) {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Torus { .. } => "torus",
        _ => "other",
    }
}

/// **A filleted drum certifies across its fillet.** The extruded unit
/// disc, its top rim filleted at `0.3`: a torus band between the top
/// plane and the wall's two halves. The band meets the plane on a whole
/// circle tangentially — on the plane's chart no straight line, so the
/// plane is presented by the region inside the circle's chart curve,
/// and no carrier separates two faces that are tangent; the graph of the
/// circle does. It meets each half of the wall on an arc of one circle,
/// a coordinate line of both charts, which the circle's plane separates;
/// where the arcs meet, the band's side runs on past the shared arc
/// onto the next. Holds, every face pair sharing a vertex certified
/// across. Red if the curved presentation, the graph separator read on
/// a curved side, the conic's plane as a separator, or a zero side read
/// past the shared cell, is withdrawn.
#[test]
fn a_filleted_drum_certifies_across_its_fillet() {
    use sweep::blend::fillet_edges;
    use sweep::test_support::finished;
    let disc = profile::circle(Point2::new(0.0, 0.0), 1.0, tol()).unwrap();
    let profile = Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .unwrap();
    let drum = finished(
        "the drum",
        extrude(&profile, to_offset(2.0), tol()).unwrap().body,
        tol(),
    );
    let rim: Vec<topo::EdgeKey> = drum
        .edges()
        .filter(|(_, e)| {
            matches!(
                drum.get_curve_geom(e.curve).unwrap().certified().map(|g| g.carrier().clone()),
                Some(geom::Curve3::Circle { center, .. }) if (center.z - 2.0).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect();
    let filleted = fillet_edges(&drum, &rim, 0.3, tol())
        .map_err(|e| e.error)
        .expect("the rim fillets");
    let body: &Body<f64> = &filleted.body;
    let report = injective(body, None, BUDGET);
    assert_holds("the filleted drum", &report, body);
    let torus = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| kind(body, k) == "torus")
        .expect("a torus band");
    for (k, _) in body.faces() {
        if matches!(kind(body, k), "cylinder")
            || (kind(body, k) == "plane" && {
                let Surface::Plane { origin, normal, .. } = surface(body, k) else {
                    unreachable!()
                };
                (origin.z - 2.0).abs() < 1e-9 || normal.z.abs() < 1e-9
            })
        {
            let pair = (k.min(torus), k.max(torus));
            if touching_pairs(body).contains(&pair) {
                assert!(
                    report.across.get(&pair).is_some_and(|&n| n > 0),
                    "the band against its {} neighbour is certified across",
                    kind(body, k)
                );
            }
        }
    }
}

/// The certified carrier of edge `e`, where it is an ellipse.
fn is_ellipse(body: &Body<f64>, e: topo::EdgeKey) -> bool {
    matches!(
        body.get_curve_geom(body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .map(|g| g.carrier().clone()),
        Some(geom::Curve3::Ellipse { .. })
    )
}

/// `b` moved rigidly by `m`, at rest.
fn moved(b: &topo::AtRestBody<f64>, m: &geom_core::Affine3<f64>) -> topo::AtRestBody<f64> {
    use sweep::test_support::finished;
    let rigid = geom_core::test_support::rigid(m, tol());
    finished(
        "moved",
        topo::transform_rigid(b, &rigid, tol()).unwrap(),
        tol(),
    )
}

/// The box `(-50, 50)² × (-100, top)`, at rest: a half-space below
/// `z = top` within reach of every body here.
fn slab(top: f64) -> topo::AtRestBody<f64> {
    use sweep::test_support::{brick, finished};
    finished(
        "the slab",
        brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, top), tol()),
        tol(),
    )
}

/// The body a boolean built.
fn built(r: Result<topo::BooleanResult<f64>, topo::BooleanError>) -> topo::AtRestBody<f64> {
    match r {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the op builds: {other:?}"),
    }
}

/// The radius-1 rod `prism_at` builds round the `z` axis over `z`: a
/// disc of two half-circle arcs, extruded.
fn rod((z0, z1): (f64, f64)) -> topo::AtRestBody<f64> {
    use sweep::test_support::finished;
    let rod = sweep::test_support::prism_at(
        vec![(Point2::new(-1.0, 0.0), 1.0), (Point2::new(1.0, 0.0), 1.0)],
        z0,
        z1 - z0,
        tol(),
    );
    finished("the rod", rod, tol())
}

/// **The kernel-built oblique cut**: the extruded unit disc over
/// `z ∈ [0, 2]` less the half-space below the plane through `(0, 0, 1)`
/// tilted `π/7` about `y`. The extrusion splits its wall in halves, so
/// the cut is two open arcs of one ellipse, meeting the halves' seam
/// lines at two vertices.
fn oblique_cut() -> topo::AtRestBody<f64> {
    use core::f64::consts::PI;
    use geom_core::{Affine3, Vec3};
    use sweep::test_support::finished;
    let disc = profile::circle(Point2::new(0.0, 0.0), 1.0, tol()).unwrap();
    let profile = Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .unwrap();
    let drum = finished(
        "the drum",
        extrude(&profile, to_offset(2.0), tol()).unwrap().body,
        tol(),
    );
    let tilt = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 1.0), Vec3::unit_y(), PI / 7.0);
    built(topo::subtract(&drum, &moved(&slab(1.0), &tilt), tol()))
}

/// **A kernel-built oblique cut holds across its curved corners**
/// ([`oblique_cut`]). Where an arc of the cut meets a seam line, each
/// wall half has a corner whose sides are the arc and the line: a cell
/// holding it is presented through the vertex — by the ellipse's chart
/// curve continued past it against the cut plane, by the seam line
/// continued past it against the other half. Holds, every face pair
/// sharing a vertex certified across, and some of those certificates
/// read a corner. Red if the corner presentation is withdrawn (the cut
/// refuses `Uncertified { Departure }` at the corners), or if a wall
/// half whose loop holds an arc is left without its extent (its seam
/// lines are then placed nowhere and the caps refuse beside them).
#[test]
fn a_kernel_built_oblique_cut_holds_across_its_curved_corners() {
    let cut = oblique_cut();
    let arcs = cut.edges().filter(|&(k, _)| is_ellipse(&cut, k)).count();
    assert_eq!(arcs, 2, "two open arcs of the cut");
    let report = injective(&cut, None, BUDGET);
    println!(
        "[corner] oblique cut: cornered = {} {:?} {:?}",
        report.cornered, report.verdict, report.receipt
    );
    assert_holds("the oblique cut", &report, &cut);
    assert_eq!(
        report.cornered, 14,
        "the corners are certified by the corner presentation"
    );
}

/// [`rod`] over `z` less the half-space below the plane through the
/// origin tilted `π/5` about `y`: like [`oblique_cut`], two open arcs of
/// an ellipse meeting the rod's two seam lines.
fn rod_less_a_tilted_slab(z: (f64, f64)) -> topo::AtRestBody<f64> {
    use core::f64::consts::PI;
    use geom_core::{Affine3, Vec3};
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), PI / 5.0);
    built(topo::subtract(&rod(z), &moved(&slab(0.0), &tilt), tol()))
}

/// **The rod less a slab tilted `π/5` holds across its curved corners**
/// ([`rod_less_a_tilted_slab`] over `z ∈ [−3, 3]`). Red as the oblique
/// cut's row is.
#[test]
fn a_rod_less_a_tilted_slab_holds_across_its_curved_corners() {
    let cut = rod_less_a_tilted_slab((-3.0, 3.0));
    let report = injective(&cut, None, BUDGET);
    println!(
        "[corner] rod at pi/5: cornered = {} {:?} {:?}",
        report.cornered, report.verdict, report.receipt
    );
    assert_holds("the rod less the tilted slab", &report, &cut);
    assert_eq!(report.cornered, 14, "the corners are read through");
}

/// **A rod taller than a turn holds too** ([`rod_less_a_tilted_slab`]
/// over `z ∈ [−6, 6]`): each wall half's chart extent is a superset over
/// its arc, more than `2π` tall along the rod's axis. Red if a superset
/// that wide is refused on the axis that is not periodic, which leaves
/// the halves' seam lines placed nowhere.
#[test]
fn a_rod_taller_than_a_turn_less_a_tilted_slab_holds() {
    let cut = rod_less_a_tilted_slab((-6.0, 6.0));
    let report = injective(&cut, None, BUDGET);
    println!(
        "[corner] tall rod at pi/5: cornered = {} {:?} {:?}",
        report.cornered, report.verdict, report.receipt
    );
    assert_holds("the tall rod less the tilted slab", &report, &cut);
}

/// **SHELL-4's vessel holds**: `vessel(1.0, 2.0)`, a rectangle with a
/// side on the axis revolved a full turn — one wall and two discs,
/// whose rims are open half-circles. Each disc's two half-rims meet at
/// a vertex as one circle, a corner of no angle the plane's chart reads
/// as two arcs; the wall reads the rims as coordinate lines, so the
/// pairs certify off the wall's presentation by the disc's carrier. It
/// is no corner row: it held before the corner presentation, and reads
/// no corner (`cornered` 0).
#[test]
fn shell_4s_vessel_holds() {
    let vessel = crate::common::shell_operands::vessel(1.0, 2.0);
    let report = injective(&vessel, None, BUDGET);
    println!(
        "[corner] vessel: cornered = {} {:?} {:?}",
        report.cornered, report.verdict, report.receipt
    );
    assert_holds("the vessel", &report, &vessel);
}

/// **A cylinder–cylinder tee refuses typed where its junction meets a
/// cap at a vertex alone.** Two radius-1 rods on crossing axes, one
/// mitred by their bisector plane (`crossing_equal_rods`' pair at spins
/// `(0, 0)`, `B` from `x = 0.2`), united. Rod `B` starts at `x = 0.2`,
/// so the junction never reaches the node plane `x = 0`: it ends where
/// `B`'s cap circle meets the mitre plane, at `(0.2, ±0.98, 0.2)`, a
/// vertex `B`'s wall and `A`'s mitre share with no edge. Each face's
/// corner there has a curved side and neither side on the shared
/// cell, which no presentation reads, so the pair refuses
/// `Uncertified { Departure }` naming the vertex — never certified,
/// never measured by distance, and without splitting to the floor.
#[test]
fn a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone() {
    use core::f64::consts::{FRAC_PI_2, PI};
    use geom_core::{Affine3, Vec3};
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -PI / 4.0);
    let a = built(topo::subtract(
        &rod((-3.0, 3.0)),
        &moved(&slab(0.0), &tilt),
        tol(),
    ));
    let onto_x = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    let b = moved(&rod((0.2, 3.0)), &onto_x);
    let tee = built(topo::union(&a, &b, tol()));
    let tee: &Body<f64> = &tee;
    let report = injective(tee, None, BUDGET);
    println!(
        "[corner] tee: cornered = {} {:?} {:?}",
        report.cornered, report.verdict, report.receipt
    );
    match report.verdict {
        BoundVerdict::Refused(EngineRefusal::Uncertified {
            a,
            b,
            shared: SharedCell::Vertex(v),
            arm: CertificateArm::Departure,
        }) => {
            let mut kinds = [kind(tee, a), kind(tee, b)];
            kinds.sort_unstable();
            assert_eq!(kinds, ["cylinder", "plane"], "a wall and the mitre");
            let p = *tee.get_point(tee.get_vertex(v).unwrap().point).unwrap();
            assert!(
                (p.x - 0.2).abs() < 1e-9
                    && (p.z - 0.2).abs() < 1e-9
                    && (p.y.abs() - 0.96f64.sqrt()).abs() < 1e-9,
                "where B's cap circle meets the mitre plane: {p:?}"
            );
        }
        other => panic!("the tee refuses at the vertex: {other:?}"),
    }
    assert!(
        report.receipt.splits < 1024,
        "the vertex refuses where it is met, not at the floor: {:?}",
        report.receipt
    );
    assert_eq!(
        report.receipt.refused, 9,
        "nine root pairs refuse: six at a vertex alone, one across the mitre's line, two across \
         the mitre's ellipse ({:?})",
        report.receipt
    );
}

/// **A folded oblique cut reads `Violated` off its corners.** Two
/// [`oblique_cut`]s in one body, the second moved `0.3` along `x` and
/// grafted in as a solid of its own: the caps and walls pass through
/// each other. Faces of the two solids share nothing in M, so every
/// crossing pair is decided by distance, and the first is found before
/// any corner is read; the witness is clear of every vertex where an arc
/// meets a seam line. The row pins that the corner reading leaves a fold
/// off its corners to be found; it cannot go red for a lax corner guard.
/// A fold through a corner cell is a crossing of two faces that share
/// the corner, which no kernel operation builds — the booleans and sweeps
/// build embedded bodies, and a graft's solids share nothing — so the
/// corner guards' folds are `injective`'s
/// `a_fold_through_a_reflex_corner_is_not_certified` and
/// `a_fold_past_a_line_that_crosses_back_is_not_certified`, at the door
/// the sweep calls.
#[test]
fn a_folded_oblique_cut_reads_violated_off_its_corners() {
    use geom_core::{Affine3, Vec3};
    let cut = oblique_cut();
    let mut folded = cut.clone().into_body();
    let over = Affine3::translation(Vec3::new(0.3, 0.0, 0.0));
    let rigid = geom_core::test_support::rigid(&over, tol());
    let copy = topo::transform_rigid(&cut, &rigid, tol()).unwrap();
    topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
    let report = injective(&folded, None, BUDGET);
    println!(
        "[corner] folded cut: {:?} {:?}",
        report.verdict, report.receipt
    );
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the two cuts cross: {:?}", report.verdict)
    };
    assert!(w.d < 1e-9, "a coincidence: {w:?}");
    let corners: Vec<Point3<f64>> = folded
        .edges()
        .filter(|&(k, _)| is_ellipse(&folded, k))
        .flat_map(|(_, e)| [e.he_plus, e.he_minus])
        .map(|he| {
            let v = folded.get_half_edge(he).unwrap().start;
            *folded
                .get_point(folded.get_vertex(v).unwrap().point)
                .unwrap()
        })
        .collect();
    assert_eq!(corners.len(), 8, "four arcs, two corners each");
    for p in [w.pa, w.pb] {
        let off = corners
            .iter()
            .map(|c| (p - *c).norm())
            .fold(f64::INFINITY, f64::min);
        assert!(off > 0.05, "the witness is off every corner: {w:?}, {off}");
    }
}

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
//! - lofts whose walls reach every extreme in one half of an axis are
//!   admitted and decided: the arc spine past a full turn, the closed
//!   spine and the carvetail loft read `Violated`, and the arc spine
//!   short of a full turn runs past the pair budget;
//! - kernel-built oblique cuts, whose arcs meet seam lines at corners,
//!   hold, some certificates read through a corner, and SHELL-4's vessel
//!   holds, on one receipt at both scalars as an axis-aligned body; a
//!   folded cut reads `Violated` off its corners;
//! - the cylinder–cylinder tee refuses typed at a vertex its junction
//!   shares with a cap and nothing else;
//! - of the three bodies the shell's cavity gate asks, the corpus pot
//!   and the cone frustum's cavity hold at both scalars, and the klein
//!   elbow's cavity refuses typed at a spiric cap edge;
//! - a lens loft's cap refuses typed at the depth budget against a wall
//!   at the lens tip, while a twisted loft's seams and the half-turn
//!   helix's caps hold against their walls.
//!
//! Every row builds its body twice, by the same generic builders: at
//! `f64`, whose verdict and receipt the row pins, and at `Interval`, the
//! certifying scalar the shell gate and the document door ask at, which
//! must read the same verdict class ([`same_class`]) — `Holds`,
//! `Violated`, or the same refusal arm.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::{NurbsCurve3, Surface};
use geom_core::k_stats::decide;
use geom_core::{
    Band, Bounds, CertifiedBounds, Decide, Interval, Margin, Point2, Point3, Real, Sign, Tol,
};
use profile::{Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start, ValidatedProfile};
use std::collections::BTreeSet;
use sweep::test_support::revolved_about_y_at;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve, sweep_body};
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, CLEARANCE_MARGIN, Candidate, CellReceipt, CertificateArm,
    ClearanceBound, DEFAULT_MAX_CELL_DEPTH, EngineRefusal, SharedCell, Side, Sweep, Which,
    min_separation,
};
use topo::{Body, FaceKey, LoopBoundary, VertexKey};

use crate::common::{arc_spine_places, normal_start_place, quad};
use crate::revolve_common::donut_profile;

/// The default whole-query budget the document door runs at
/// (`editor_core::clearance::DEFAULT_MAX_CELL_PAIRS`).
const BUDGET: usize = 65_536;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The scalars a row builds its body at.
trait Scalar: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy {}
impl<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy> Scalar for T {}

/// The middle of `x`'s bracket: `x` itself at `f64`.
fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// `loops` on the `xy` sketch plane, lifted to `T` and validated.
fn validated<T: Scalar>(loops: &[ProfileLoop<f64>]) -> ValidatedProfile<T> {
    Profile::new(SketchPlane::xy(), lifted(loops))
        .validate(tol())
        .unwrap()
}

/// Sketch loops lifted to `T`.
fn lifted<T: Real>(loops: &[ProfileLoop<f64>]) -> Vec<ProfileLoop<T>> {
    loops.iter().map(|l| l.map_scalar(T::from_f64)).collect()
}

/// A sketch plane lifted to `T`.
fn plane_at<T: Scalar>(place: &SketchPlane<f64>) -> SketchPlane<T> {
    SketchPlane::from_frame(geom_core::test_support::rigid(
        &place.placement().map(T::from_f64),
        tol(),
    ))
}

/// The sketch `y` axis, at `T`.
fn axis_y<T: Real>() -> RevolveAxis<T> {
    let a = crate::revolve_common::axis_y();
    RevolveAxis {
        origin: a.origin.map(T::from_f64),
        dir: a.dir.map(T::from_f64),
    }
}

/// The extrusion to the signed offset `d` along the sketch normal, at `T`.
fn to_offset<T: Real>(d: f64) -> Extrusion<T> {
    match crate::common::to_offset(d) {
        Extrusion::Distance { depth, side } => Extrusion::Distance {
            depth: T::from_f64(depth),
            side,
        },
        other => panic!("an offset is a distance: {other:?}"),
    }
}

/// A verified witness: two faces, a point on each (its bracket's middle),
/// and the upper end of their distance's bracket.
#[derive(Debug, Clone, Copy)]
struct Witness {
    a: FaceKey,
    b: FaceKey,
    pa: Point3<f64>,
    pb: Point3<f64>,
    d: f64,
}

fn surface<T: Real>(body: &Body<T>, f: FaceKey) -> Surface<T> {
    body.get_surface(body.get_face(f).unwrap().surface)
        .unwrap()
        .clone()
}

/// **The witness check**: the closest pair of the two cells' on-face
/// stations, evaluated afresh on the stored surfaces, decided at the
/// bound's own site.
fn verify<T: Scalar>(
    body: &Body<T>,
    bound: ClearanceBound,
) -> impl FnMut(Candidate<'_, '_, T>) -> Result<Witness, String> + '_ {
    move |candidate| {
        let (fa, fb) = candidate.faces();
        let (sa, sb) = (surface(body, fa), surface(body, fb));
        let (la, lb) = candidate.on_face_stations();
        let at = T::from_f64;
        let mut best: Option<(Witness, T)> = None;
        for &(au, av) in &la {
            let pa = sa.eval(at(au), at(av));
            for &(bu, bv) in &lb {
                let pb = sb.eval(at(bu), at(bv));
                let d = (pa - pb).norm();
                if best.is_none_or(|(w, _)| d.hi() < w.d) {
                    let w = Witness {
                        a: fa,
                        b: fb,
                        pa: pa.map(mid),
                        pb: pb.map(mid),
                        d: d.hi(),
                    };
                    best = Some((w, d));
                }
            }
        }
        let (w, d) = best.ok_or("no station")?;
        let definite = match decide(bound.predicate(), Margin::of(d - at(bound.c())), band()) {
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
fn injective<T: Scalar>(
    body: &Body<T>,
    faces: Option<Vec<FaceKey>>,
    budget: usize,
) -> BoundReport<Witness> {
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
fn touching_pairs<T: Real>(body: &Body<T>) -> BTreeSet<(FaceKey, FaceKey)> {
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
fn assert_holds<T: Real>(what: &str, report: &BoundReport<Witness>, body: &Body<T>) {
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

/// A verdict's class: `Holds`, `Violated`, or the refusal's arm — an
/// uncertified pair's certificate arm and the kind of cell it shares.
fn class<W>(verdict: &BoundVerdict<W>) -> String {
    match verdict {
        BoundVerdict::Holds => "Holds".to_owned(),
        BoundVerdict::Violated(_) => "Violated".to_owned(),
        BoundVerdict::Refused(EngineRefusal::Uncertified { shared, arm, .. }) => {
            let cell = match shared {
                SharedCell::Vertex(_) => "vertex",
                SharedCell::Edge(_) => "edge",
                SharedCell::Face(_) => "face",
            };
            format!("Uncertified {arm:?} at a {cell}")
        }
        BoundVerdict::Refused(other) => {
            let named = format!("{other:?}");
            named
                .split(|c: char| !c.is_alphanumeric())
                .next()
                .unwrap_or_default()
                .to_owned()
        }
    }
}

/// **The row at `Interval` reads `f64`'s verdict class**, both receipts
/// printed.
fn same_class(what: &str, float: &BoundReport<Witness>, interval: &BoundReport<Witness>) {
    println!(
        "[rimscalar] {what}: f64 {} {:?} cornered = {} | Interval {} {:?} cornered = {}",
        class(&float.verdict),
        float.receipt,
        float.cornered,
        class(&interval.verdict),
        interval.receipt,
        interval.cornered
    );
    assert_eq!(
        class(&interval.verdict),
        class(&float.verdict),
        "{what}: the Interval body reads the f64 body's verdict class: {:?}",
        interval.verdict
    );
}

/// **The body holds at both scalars** ([`assert_holds`]), over every face,
/// in one verdict class; the `f64` report and the `Interval` one, for a
/// row's own pins.
fn holds_at_both(
    what: &str,
    float: Body<f64>,
    interval: Body<Interval>,
) -> [BoundReport<Witness>; 2] {
    let report = injective(&float, None, BUDGET);
    let twin = injective(&interval, None, BUDGET);
    same_class(what, &report, &twin);
    assert_holds(what, &report, &float);
    assert_holds(&format!("{what} at Interval"), &twin, &interval);
    [report, twin]
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
fn top_wall<T: Scalar>(lofted: &sweep::Lofted<T>) -> FaceKey {
    let centre = |f: FaceKey| {
        let s = surface(&lofted.body, f);
        let Surface::Nurbs(net) = &s else {
            panic!("a swept wall is NURBS")
        };
        let ((u0, u1), (v0, v1)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
        let half = T::from_f64(0.5);
        mid(s.eval(half * (u0 + u1), half * (v0 + v1)).z)
    };
    lofted.side_faces[0]
        .iter()
        .flatten()
        .copied()
        .max_by(|&a, &b| centre(a).total_cmp(&centre(b)))
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
    fn ribbon<T: Scalar>() -> (BoundReport<Witness>, FaceKey) {
        let path = helix(370.0 / 360.0, 0.0);
        let place = plane_at(&normal_start_place(&path));
        let profile = lifted(&quad([
            (-0.1, -0.02),
            (0.1, -0.02),
            (0.1, 0.02),
            (-0.1, 0.02),
        ]));
        let swept =
            sweep_body::<T>(&profile, place, &path, 48, 3, tol()).expect("the ribbon sweeps");
        let wall = top_wall(&swept);
        (injective(&swept.body, Some(vec![wall]), BUDGET), wall)
    }
    let (report, wall) = ribbon::<f64>();
    same_class("the ribbon", &report, &ribbon::<Interval>().0);
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

/// **The carvetail loft**: two pentagons at `z = 0` and `z = 1`,
/// lofted at `v_degree = 1`; joint 1 turns +170° in one section and
/// −170° in the other.
fn carvetail<T: Scalar>() -> sweep::Lofted<T> {
    let pent = |pts: [(f64, f64); 5]| {
        lifted(&[ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )])
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
    let places = [0.0, 1.0].map(|z| plane_at(&SketchPlane::xy_at(Point3::new(0.0, 0.0, z))));
    loft_body::<T>(&sections, &places, 1, tol()).expect("the carvetail lofts")
}

/// **The carvetail fold reads `Violated`, its witness at `v ≈ ½`.** In
/// the [`carvetail`] loft the seam between walls 0 and 1
/// passes through wedge 0 at `v = ½`, where wall 1 lies along wall 0
/// and crosses it. The two walls share that seam, so the pair is the
/// one the global drop never examined.
#[test]
fn the_carvetail_fold_reads_violated_at_its_middle_station() {
    fn fold<T: Scalar>() -> (sweep::Lofted<T>, Vec<FaceKey>) {
        let lofted = carvetail::<T>();
        // Walls 0 and 1: the two whose boundary holds the seam over (2, 0).
        let on_seam = |f: FaceKey| {
            let s = surface(&lofted.body, f);
            let Surface::Nurbs(net) = &s else {
                return false;
            };
            let ((u0, u1), (v0, _)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
            [u0, u1].iter().any(|&u| {
                let p = s.eval(u, v0).map(mid);
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
        (lofted, walls)
    }
    let (lofted, walls) = fold::<f64>();
    let report = injective(&lofted.body, Some(walls.clone()), BUDGET);
    let (twin, twin_walls) = fold::<Interval>();
    same_class(
        "the fold",
        &report,
        &injective(&twin.body, Some(twin_walls), BUDGET),
    );
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
    fn cone<T: Scalar>() -> Body<T> {
        let profile = validated(&[ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)].map(|(x, y)| Point2::new(x, y)),
        )]);
        revolve(&profile, axis_y(), Revolution::Full, tol())
            .unwrap()
            .body
    }
    holds_at_both("the cone", cone::<f64>(), cone::<Interval>());
}

/// **A pyramid holds**: a square base and an apex — four planar faces
/// meeting at the apex, two of each pair of opposite ones sharing only
/// that vertex.
#[test]
fn a_pyramid_holds() {
    fn pyramid<T: Scalar>() -> Body<T> {
        use topo::test_support::meeting::{Pose, posed_pyramid_at};
        posed_pyramid_at(
            &[
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            [0.5, 0.5, 0.8],
            &Pose::rest(),
            tol(),
        )
        .into_body()
    }
    holds_at_both("the pyramid", pyramid::<f64>(), pyramid::<Interval>());
}

/// **An embedded triangle loft holds**: every wall shares a seam with
/// both others and a rim with each cap.
#[test]
fn an_embedded_triangle_loft_holds() {
    fn loft<T: Scalar>() -> Body<T> {
        let tri = |s: f64, turn: f64| {
            let p = |a: f64| Point2::new(s * (a + turn).cos(), s * (a + turn).sin());
            lifted(&[ProfileLoop::polygon(
                [0.0, TAU / 3.0, 2.0 * TAU / 3.0].map(p),
            )])
        };
        let sections = [tri(1.0, 0.0), tri(0.6, 0.3)];
        let places = [0.0, 1.0].map(|z| plane_at(&SketchPlane::xy_at(Point3::new(0.0, 0.0, z))));
        loft_body::<T>(&sections, &places, 1, tol())
            .expect("the triangles loft")
            .body
    }
    holds_at_both("the triangle loft", loft::<f64>(), loft::<Interval>());
}

/// **A declared wedge-0 kiss refuses typed**: the lune between the
/// internally tangent circles `(0,1) r 1` and `(0,2) r 2`, `.cusp()` at
/// the kiss, extruded. The two walls meet along the kiss strut with
/// their outward normals opposed, which only the second-order arm
/// reads; it is not built, so the strut refuses, named.
#[test]
fn a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm() {
    fn kiss<T: Scalar>() -> Body<T> {
        extrude(&validated(&[lune()]), to_offset(1.0), tol())
            .unwrap()
            .body
    }
    fn lune() -> ProfileLoop<f64> {
        Open.at(Point2::new(0.0, 4.0))
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
            .into()
    }
    let body = kiss::<f64>();
    let report = injective(&body, None, BUDGET);
    same_class(
        "the kiss",
        &report,
        &injective(&kiss::<Interval>(), None, BUDGET),
    );
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
    fn walls<T: Scalar>() -> (Body<T>, FaceKey, FaceKey) {
        let profile = validated(&quad([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]));
        let built = extrude(&profile, to_offset(1.0), tol()).unwrap();
        let walls = built.side_faces();
        let (fa, fb) = (walls[0][0], walls[0][1]);
        (built.body, fa, fb)
    }
    fn apart<T: Scalar>(
        body: &Body<T>,
        fa: FaceKey,
        fb: FaceKey,
    ) -> (BoundReport<Witness>, Option<SharedCell>) {
        let side = |f| Side {
            body,
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
            verify(body, bound)(candidate)
        });
        assert!(report.receipt.holds(), "{:?}", report.receipt);
        (report, met)
    }
    let (body, fa, fb) = walls::<f64>();
    let (report, met) = apart(&body, fa, fb);
    let (twin, ta, tb) = walls::<Interval>();
    same_class(
        "the scopes sharing an edge",
        &report,
        &apart(&twin, ta, tb).0,
    );
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
    assert_eq!(CLEARANCE_MARGIN, ClearanceBound::AtLeast(0.1).predicate());
    let side = |f| Side {
        body: &body,
        faces: vec![f],
    };
    let m = min_separation(&side(fa), &side(fb), true, band(), Budgets::MIN_SEPARATION).unwrap();
    assert_eq!((m.lo(), m.hi()), (0.0, Some(0.0)), "{}", m.render());
    let meets = m.meets().expect("the scopes meet");
    assert!(matches!(meets.shared, SharedCell::Edge(_)), "{meets:?}");
}

// ---------------------------------------------- large curved bodies

/// **A full torus certifies by kind**: one face, `R = 2 > r = 1/2`.
#[test]
fn a_full_torus_certifies_by_kind() {
    fn torus<T: Scalar>() -> Body<T> {
        revolve(
            &validated(&[donut_profile()]),
            axis_y(),
            Revolution::Full,
            tol(),
        )
        .unwrap()
        .body
    }
    let body = torus::<f64>();
    for report in holds_at_both("the torus", body.clone(), torus::<Interval>()) {
        assert_eq!(
            report.receipt.by_kind,
            body.faces().count(),
            "every torus face against itself discharged by kind, R > r decided: {:?}",
            report.receipt
        );
    }
}

/// **A long helical sweep certifies within the default budget**: a
/// square tube two turns round a helix of pitch 0.4, its four walls
/// sharing helical seams. Each wall's enclosure extremes lie in one
/// half of an axis, so it is admitted by the other half narrowing.
#[test]
fn a_long_helical_sweep_certifies_within_the_default_budget() {
    fn helical<T: Scalar>() -> Body<T> {
        let path = helix(2.0, 0.4);
        let place = plane_at(&normal_start_place(&path));
        let profile = lifted(&quad([
            (-0.08, -0.08),
            (0.08, -0.08),
            (0.08, 0.08),
            (-0.08, 0.08),
        ]));
        sweep_body::<T>(&profile, place, &path, 65, 3, tol())
            .expect("the helix sweeps")
            .body
    }
    holds_at_both("the helical sweep", helical::<f64>(), helical::<Interval>());
}

// ------------------------------------- walls one half of an axis spans

/// A loft face's role: `top`, `bottom`, or `wall j` of the outer loop.
fn role<T: Real>(lofted: &sweep::Lofted<T>, f: FaceKey) -> String {
    if f == lofted.top {
        return "top".into();
    }
    if f == lofted.bottom {
        return "bottom".into();
    }
    lofted.side_faces[0]
        .iter()
        .position(|seg| seg.contains(&f))
        .map_or_else(|| format!("{f:?}"), |j| format!("wall {j}"))
}

/// **The body's every face has a window, and the sweep reads one
/// verdict class at `f64` and at `Interval`** within `budget` pairs:
/// the `f64` report, for the row's pins.
fn decided_at_both(
    what: &str,
    float: &sweep::Lofted<f64>,
    interval: &sweep::Lofted<Interval>,
    budget: usize,
) -> BoundReport<Witness> {
    let report = injective(&float.body, None, budget);
    let twin = injective(&interval.body, None, budget);
    same_class(what, &report, &twin);
    if let BoundVerdict::Violated(w) = &report.verdict {
        println!(
            "[selfx] {what}: {} x {} at {:?} | {:?}",
            role(float, w.a),
            role(float, w.b),
            w.pa,
            report.receipt
        );
    }
    report
}

/// A 0.1 square lofted along [`arc_spine_places`] at 17 stations,
/// `v`-degree 3.
fn arc_spine<T: Scalar>(curl: f64) -> sweep::Lofted<T> {
    let square = quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)]);
    let sections: Vec<_> = (0..17).map(|_| lifted(&square)).collect();
    let places: Vec<_> = arc_spine_places(curl, 17).iter().map(plane_at).collect();
    loft_body::<T>(&sections, &places, 3, tol()).expect("the arc spine lofts")
}

/// The station-angle of a point on the arc spine, folded into one turn.
fn arc_angle(p: Point3<f64>) -> f64 {
    p.x.atan2(1.0 - p.z).rem_euclid(TAU)
}

/// **An arc spine past a full turn reads `Violated`, a wall against
/// itself where the arc passes twice.** The walls reach every extreme
/// of `x` and `z` at the profile's `-0.05` edge, so each is admitted by
/// its upper `u` half narrowing; at 9 and 13 rad the sweep then finds a
/// wall over itself.
#[test]
fn an_arc_spine_past_a_full_turn_reads_violated() {
    for (curl, first_pass) in [(9.0, 1.0..=2.0), (13.0, 4.0..=5.0)] {
        let what = format!("the {curl} rad arc spine");
        let lofted = arc_spine::<f64>(curl);
        let report = decided_at_both(&what, &lofted, &arc_spine::<Interval>(curl), BUDGET);
        let BoundVerdict::Violated(w) = report.verdict else {
            panic!("{what} overlaps itself: {:?}", report.verdict)
        };
        assert_eq!(w.a, w.b, "{what}: one face against itself: {w:?}");
        assert!(
            role(&lofted, w.a).starts_with("wall"),
            "{what}: a wall: {w:?}"
        );
        let station = arc_angle(w.pa) / (curl / 16.0);
        assert!(
            first_pass.contains(&station) && arc_angle(w.pa) < curl - TAU,
            "{what}: the witness lies where the arc passes twice, between stations \
             {first_pass:?} on its first pass: station {station}, {w:?}"
        );
    }
}

/// **An arc spine short of a full turn holds within the default
/// budget.** Its walls are admitted as the 9 and 13 rad ones are, and
/// each carries interior knots a few ulps off a bisection point, which
/// the monotone arm reads through ([`walls_hold_against_themselves`]).
#[test]
fn an_arc_spine_short_of_a_full_turn_holds() {
    for curl in [3.5, 6.0] {
        let what = format!("the {curl} rad arc spine");
        let report = decided_at_both(
            &what,
            &arc_spine::<f64>(curl),
            &arc_spine::<Interval>(curl),
            BUDGET,
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "{what}: holds within the default budget: {:?}",
            report.verdict
        );
    }
}

/// **A closed spine reads `Violated`, top against bottom where its ends
/// meet**: a 0.5 square swept round a radius-3 circle at 9 stations. The
/// two caps lie on one plane at `(3, 0, 0)`.
#[test]
fn a_closed_spine_reads_violated_top_against_bottom() {
    fn closed<T: Scalar>() -> sweep::Lofted<T> {
        let pts: Vec<Point3<f64>> = (0..=32)
            .map(|k| {
                let a = TAU * f64::from(k % 32) / 32.0;
                Point3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
            })
            .collect();
        let path = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
        let place = plane_at(&normal_start_place(&path));
        let profile = lifted(&quad([
            (-0.25, -0.25),
            (0.25, -0.25),
            (0.25, 0.25),
            (-0.25, 0.25),
        ]));
        sweep_body::<T>(&profile, place, &path, 9, 3, tol()).expect("the closed spine sweeps")
    }
    let lofted = closed::<f64>();
    let report = decided_at_both("the closed spine", &lofted, &closed::<Interval>(), BUDGET);
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the closed spine meets itself: {:?}", report.verdict)
    };
    let mut caps = [role(&lofted, w.a), role(&lofted, w.b)];
    caps.sort();
    assert_eq!(caps, ["bottom", "top"], "the two caps: {w:?}");
    assert!(
        (w.pa.x - 3.0).abs() < 0.3 && w.pa.y.abs() < 1e-6 && w.pa.z.abs() <= 0.25 + 1e-6,
        "the witness is on the plane the caps share: {w:?}"
    );
}

/// **The carvetail loft reads `Violated` over the whole body**: the fold
/// of [`the_carvetail_fold_reads_violated_at_its_middle_station`], every
/// face asked. Wall 0 crosses wall 3.
#[test]
fn the_carvetail_loft_reads_violated_over_every_face() {
    let lofted = carvetail::<f64>();
    let report = decided_at_both(
        "the carvetail loft",
        &lofted,
        &carvetail::<Interval>(),
        BUDGET,
    );
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the carvetail loft crosses itself: {:?}", report.verdict)
    };
    let mut walls = [role(&lofted, w.a), role(&lofted, w.b)];
    walls.sort();
    assert_eq!(walls, ["wall 0", "wall 3"], "{w:?}");
}

/// Each interior `v` knot of `wall`'s surface that lies within four ulps
/// of the dyadic `m/2^d` (`d ≤ 6`) nearest it without being on it: the
/// bisection lands on `m/2^d`, and a cell ending there meets the next
/// span over those few ulps.
fn knots_ulps_off_a_split<T: Scalar>(body: &Body<T>, wall: FaceKey) -> Vec<f64> {
    let Surface::Nurbs(net) = surface(body, wall) else {
        panic!("a loft wall is a spline")
    };
    net.knot_values_v()
        .values()
        .iter()
        .map(|k| k.lo())
        .filter(|&k| {
            let split = (k * 64.0).round() / 64.0;
            k != split && k.to_bits().abs_diff(split.to_bits()) <= 4
        })
        .collect()
}

/// **Each wall against itself holds where its knots sit ulps off its
/// splits**: `body`'s walls, each alone, at the default budget; the
/// count of those examined with such a knot. A derivative enclosure
/// read off a cell's few-ulp meet with a span reads that meet's rounding
/// as slope, never shrinks along the line, and the monotone arm splits
/// the line until the budget refuses.
fn walls_hold_against_themselves<T: Scalar>(
    what: &str,
    body: &Body<T>,
    walls: &[FaceKey],
) -> usize {
    let mut examined = 0;
    for &wall in walls {
        let report = injective(body, Some(vec![wall]), BUDGET);
        println!(
            "[sliver] {what}: {wall:?} {} deepest {} {:?}",
            class(&report.verdict),
            report.widths.deepest,
            report.receipt
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "{what}: a wall against itself holds within the default budget, got {} {:?}",
            class(&report.verdict),
            report.receipt
        );
        if !knots_ulps_off_a_split(body, wall).is_empty() {
            examined += 1;
        }
    }
    examined
}

/// **An arc-spine loft's walls hold against themselves at the curls
/// whose chord-length knots round a few ulps off `k/16`**: 3.2, 3.4,
/// 3.5, 3.6 and 6.0 rad, at both scalars.
#[test]
fn arc_spine_walls_with_knots_ulps_off_their_splits_hold_against_themselves() {
    for curl in [3.2, 3.4, 3.5, 3.6, 6.0] {
        let float = arc_spine::<f64>(curl);
        let walls: Vec<FaceKey> = float
            .side_faces
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect();
        let what = format!("the {curl} rad arc spine");
        assert!(
            walls_hold_against_themselves(&what, &float.body, &walls) > 0,
            "{what}: the row examines a wall with a knot a few ulps off a split point"
        );
        let twin = arc_spine::<Interval>(curl);
        let walls: Vec<FaceKey> = twin
            .side_faces
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect();
        let what = format!("{what} at Interval");
        assert!(
            walls_hold_against_themselves(&what, &twin.body, &walls) > 0,
            "{what}: the row examines a wall with a knot a few ulps off a split point"
        );
    }
}

/// **The tour's S-duct walls hold against themselves**: the square swept
/// along the tour's `skinned` S path (two quarter arcs of radius 2, 17
/// points, cubic) at 13 stations. A regression row for a body the tour
/// renders, not a sliver row: its interior knots are `k/12` with `½`
/// exact, so none sits ulps off a split point.
#[test]
fn the_s_duct_walls_hold_against_themselves() {
    fn duct<T: Scalar>() -> sweep::Lofted<T> {
        let (r, h) = (2.0, 0.25);
        let quarter = core::f64::consts::FRAC_PI_2;
        let points: Vec<Point3<f64>> = (0..=8)
            .map(|k| {
                let th = quarter * f64::from(k) / 8.0;
                Point3::new(0.0, r * (1.0 - th.cos()), r * th.sin())
            })
            .chain((1..=8).map(|k| {
                let ph = quarter * f64::from(k) / 8.0;
                Point3::new(0.0, r + r * ph.sin(), 2.0 * r - r * ph.cos())
            }))
            .collect();
        let path = NurbsCurve3::interpolate(&points, 3).expect("the S path interpolates");
        let place = plane_at(&normal_start_place(&path));
        let profile = lifted(&quad([(-h, -h), (h, -h), (h, h), (-h, h)]));
        sweep_body::<T>(&profile, place, &path, 13, 3, tol()).expect("the S path sweeps")
    }
    let float = duct::<f64>();
    let walls: Vec<FaceKey> = float
        .side_faces
        .iter()
        .flatten()
        .flatten()
        .copied()
        .collect();
    walls_hold_against_themselves("the S duct", &float.body, &walls);
    let twin = duct::<Interval>();
    let walls: Vec<FaceKey> = twin
        .side_faces
        .iter()
        .flatten()
        .flatten()
        .copied()
        .collect();
    walls_hold_against_themselves("the S duct at Interval", &twin.body, &walls);
}

/// **A big thin-walled cylinder shell**: the tube `r ∈ [0.96, 1]`, two
/// high, revolved a full turn — two walls 0.04 apart that share
/// nothing, and two annular caps.
#[test]
fn a_big_thin_walled_cylinder_shell_certifies_within_the_default_budget() {
    fn shell<T: Scalar>() -> Body<T> {
        let p = |x, y| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
        revolved_about_y_at(
            vec![p(0.96, 0.0), p(1.0, 0.0), p(1.0, 2.0), p(0.96, 2.0)],
            Revolution::Full,
            tol(),
        )
    }
    holds_at_both("the thin shell", shell::<f64>(), shell::<Interval>());
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
    fn pierced<T: Scalar>() -> (Body<T>, FaceKey, FaceKey) {
        use geom_brep::EdgeCurveSpec;
        use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
        use topo::test_support::{CylFrame, CylKey, cyl_wall_sheet_keyed, plane, set_sheet_sense};

        let (alpha, beta) = (0.2, 4.0);
        let frame = CylFrame::canonical(1.0);
        let mut body = Body::<T>::new();
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
        let at = |u: f64, v: f64| frame.at::<T>(u, v);
        let vertex_at = |body: &Body<T>, p: Point3<T>| {
            body.vertices()
                .find(|(_, x)| (*body.get_point(x.point).unwrap() - p).norm().hi() < 1e-12)
                .unwrap()
                .0
        };
        let (s_lo, s_hi) = (
            vertex_at(&body, at(alpha, 0.0)),
            vertex_at(&body, at(alpha, 1.0)),
        );
        let on_wall = |body: &Body<T>, he| {
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
        let point = |body: &Body<T>, v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let (q0, q1) = (point(&body, s0), point(&body, s1));
        // The flap's far side: the chord from the strut toward the ruling at
        // `u = 5`, carried 1.3 times as far, at each end's height.
        let past = |q: Point3<T>| {
            let toward = at(5.0, mid(q.z));
            q + (toward - q) * T::from_f64(1.3)
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
        (body, wall, flap)
    }
    let (body, wall, flap) = pierced::<f64>();
    let report = injective(&body, Some(vec![wall, flap]), BUDGET);
    let (twin, tw, tf) = pierced::<Interval>();
    same_class(
        "the seam-crossing wall and its flap",
        &report,
        &injective(&twin, Some(vec![tw, tf]), BUDGET),
    );
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
fn kind<T: Real>(body: &Body<T>, f: FaceKey) -> &'static str {
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
    fn filleted<T: Scalar>() -> Body<T> {
        use sweep::blend::fillet_edges;
        use sweep::test_support::finished;
        let drum = finished(
            "the drum",
            extrude(&disc(), to_offset(2.0), tol()).unwrap().body,
            tol(),
        );
        let rim: Vec<topo::EdgeKey> = drum
            .edges()
            .filter(|(_, e)| {
                matches!(
                    drum.get_curve_geom(e.curve).unwrap().certified().map(|g| g.carrier().clone()),
                    Some(geom::Curve3::Circle { center, .. }) if (mid(center.z) - 2.0).abs() < 1e-9
                )
            })
            .map(|(k, _)| k)
            .collect();
        fillet_edges(&drum, &rim, T::from_f64(0.3), tol())
            .map_err(|e| e.error)
            .expect("the rim fillets")
            .body
    }
    let body: &Body<f64> = &filleted::<f64>();
    let report = injective(body, None, BUDGET);
    assert_holds("the filleted drum", &report, body);
    // The known divergence `a-fillet-band-rim-phase-outruns-the-touch-concession-at-interval`
    // (CLEAR): at `Interval` the band's rim phase is wider than the touch
    // concession, and the walk runs out of depth beside the rim.
    let twin = injective(&filleted::<Interval>(), None, BUDGET);
    println!(
        "[rimscalar] the filleted drum: f64 {} {:?} | Interval {} {:?}",
        class(&report.verdict),
        report.receipt,
        class(&twin.verdict),
        twin.receipt
    );
    assert!(
        matches!(
            twin.verdict,
            BoundVerdict::Refused(EngineRefusal::Budget(
                topo::clearance::CellBudget::Depth { .. }
            ))
        ),
        "the filleted drum's known divergence at Interval is a depth refusal: {:?}",
        twin.verdict
    );
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

/// The unit disc on the `xy` sketch plane, at `T`.
fn disc<T: Scalar>() -> ValidatedProfile<T> {
    let disc = profile::circle(Point2::new(T::zero(), T::zero()), T::one(), tol()).unwrap();
    Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .unwrap()
}

/// The certified carrier of edge `e`, where it is an ellipse.
fn is_ellipse<T: Real>(body: &Body<T>, e: topo::EdgeKey) -> bool {
    matches!(
        body.get_curve_geom(body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .map(|g| g.carrier().clone()),
        Some(geom::Curve3::Ellipse { .. })
    )
}

/// `b` moved rigidly by `m`, at rest.
fn moved<T: Scalar>(b: &topo::AtRestBody<T>, m: &geom_core::Affine3<f64>) -> topo::AtRestBody<T> {
    use sweep::test_support::finished;
    let rigid = geom_core::test_support::rigid(&m.map(T::from_f64), tol());
    finished(
        "moved",
        topo::transform_rigid(b, &rigid, tol()).unwrap(),
        tol(),
    )
}

/// The box `(-50, 50)² × (-100, top)`, at rest: a half-space below
/// `z = top` within reach of every body here.
fn slab<T: Scalar>(top: f64) -> topo::AtRestBody<T> {
    use sweep::test_support::{brick, finished};
    finished(
        "the slab",
        brick((-50.0, 50.0), (-50.0, 50.0), (-100.0, top), tol()),
        tol(),
    )
}

/// The body a boolean built.
fn built<T: Real>(r: Result<topo::BooleanResult<T>, topo::BooleanError>) -> topo::AtRestBody<T> {
    match r {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the op builds: {other:?}"),
    }
}

/// The radius-1 rod `prism_at` builds round the `z` axis over `z`: a
/// disc of two half-circle arcs, extruded.
fn rod<T: Scalar>((z0, z1): (f64, f64)) -> topo::AtRestBody<T> {
    use sweep::test_support::finished;
    let at = T::from_f64;
    let rod = sweep::test_support::prism_at(
        vec![
            (Point2::new(at(-1.0), T::zero()), T::one()),
            (Point2::new(T::one(), T::zero()), T::one()),
        ],
        at(z0),
        at(z1 - z0),
        tol(),
    );
    finished("the rod", rod, tol())
}

/// **The kernel-built oblique cut**: the extruded unit disc over
/// `z ∈ [0, 2]` less the half-space below the plane through `(0, 0, 1)`
/// tilted `π/7` about `y`. The extrusion splits its wall in halves, so
/// the cut is two open arcs of one ellipse, meeting the halves' seam
/// lines at two vertices.
fn oblique_cut<T: Scalar>() -> topo::AtRestBody<T> {
    use core::f64::consts::PI;
    use geom_core::{Affine3, Vec3};
    use sweep::test_support::finished;
    let drum = finished(
        "the drum",
        extrude(&disc(), to_offset(2.0), tol()).unwrap().body,
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
    let cut = oblique_cut::<f64>();
    let arcs = cut.edges().filter(|&(k, _)| is_ellipse(&cut, k)).count();
    assert_eq!(arcs, 2, "two open arcs of the cut");
    for report in holds_at_both(
        "the oblique cut",
        cut.into_body(),
        oblique_cut::<Interval>().into_body(),
    ) {
        assert_eq!(
            report.cornered, 14,
            "the corners are certified by the corner presentation"
        );
    }
}

/// [`rod`] over `z` less the half-space below the plane through the
/// origin tilted `π/5` about `y`: like [`oblique_cut`], two open arcs of
/// an ellipse meeting the rod's two seam lines.
fn rod_less_a_tilted_slab<T: Scalar>(z: (f64, f64)) -> topo::AtRestBody<T> {
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
    let z = (-3.0, 3.0);
    for report in holds_at_both(
        "the rod less the tilted slab",
        rod_less_a_tilted_slab(z).into_body(),
        rod_less_a_tilted_slab(z).into_body(),
    ) {
        assert_eq!(report.cornered, 14, "the corners are read through");
    }
}

/// **A rod taller than a turn holds too** ([`rod_less_a_tilted_slab`]
/// over `z ∈ [−6, 6]`): each wall half's chart extent is a superset over
/// its arc, more than `2π` tall along the rod's axis. Red if a superset
/// that wide is refused on the axis that is not periodic, which leaves
/// the halves' seam lines placed nowhere.
#[test]
fn a_rod_taller_than_a_turn_less_a_tilted_slab_holds() {
    let z = (-6.0, 6.0);
    holds_at_both(
        "the tall rod less the tilted slab",
        rod_less_a_tilted_slab(z).into_body(),
        rod_less_a_tilted_slab(z).into_body(),
    );
}

/// **SHELL-4's vessel holds**: `vessel(1.0, 2.0)`, a rectangle with a
/// side on the axis revolved a full turn — one wall and two discs,
/// whose rims are open half-circles. Each disc's two half-rims meet at
/// a vertex as one circle, a corner of no angle the plane's chart reads
/// as two arcs. Holds at both scalars, nothing refused, on one receipt:
/// the wall reads the rims as coordinate lines, so the pairs certify off
/// the wall's presentation by the disc's carrier, reading no corner. At
/// `Interval` the rims' axial terms on the wall's chart are enclosures of
/// zero, which the pcurve derivation decides zero at
/// `pcurve_cylinder_chart_axial`. Red if it does not: the wall then reads
/// each rim as a curve through its corners (`cornered` 8). One receipt at
/// both scalars is this axis-aligned vessel's: on a cylinder whose axis
/// is not a coordinate axis, `f64` rounds a rim's terms to nonzero values
/// and reads a curve where `Interval` reads the line.
#[test]
fn shell_4s_vessel_holds() {
    use crate::common::shell_operands::vessel_at;
    let [float, interval] = holds_at_both("the vessel", vessel_at(1.0, 2.0), vessel_at(1.0, 2.0));
    assert_eq!(
        interval.receipt, float.receipt,
        "the vessel at Interval reads the f64 receipt"
    );
    for (report, at) in [(&float, "f64"), (&interval, "Interval")] {
        assert_eq!(
            report.receipt.refused, 0,
            "the vessel at {at} refuses nothing: {:?}",
            report.receipt
        );
        assert_eq!(
            report.cornered, 0,
            "the vessel at {at} reads no corner: {:?}",
            report.receipt
        );
    }
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
    fn tee_at<T: Scalar>() -> Result<topo::BooleanResult<T>, topo::BooleanError> {
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
        topo::union(&a, &b, tol())
    }
    // The known divergence `a-rod-union-refuses-at-interval-on-a-poison-ruled-region-span`
    // (CLEAVE): at `Interval` the union does not build, so there is no
    // body to ask. Which decision refuses first moves with ε; that the
    // union refuses, naming its cause rather than a kernel defect, does
    // not.
    match tee_at::<Interval>() {
        Err(e) if !topo::test_support::offer_key(&e).1 => {
            println!("[corner] tee at Interval refuses {e:?}");
        }
        other => panic!(
            "the tee's known divergence at Interval is a typed refusal of the union: {:?}",
            other.map(|_| ())
        ),
    }
    let built = built(tee_at::<f64>());
    let tee: &Body<f64> = &built;
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
    fn folded_at<T: Scalar>() -> Body<T> {
        use geom_core::{Affine3, Vec3};
        let cut = oblique_cut::<T>();
        let mut folded = cut.clone().into_body();
        let over = Affine3::translation(Vec3::new(0.3, 0.0, 0.0)).map(T::from_f64);
        let rigid = geom_core::test_support::rigid(&over, tol());
        let copy = topo::transform_rigid(&cut, &rigid, tol()).unwrap();
        topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
        folded
    }
    let folded = folded_at::<f64>();
    let report = injective(&folded, None, BUDGET);
    same_class(
        "the folded cut",
        &report,
        &injective(&folded_at::<Interval>(), None, BUDGET),
    );
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

// ------------------------------------------ the cavity gate's three bodies

/// **The corpus pot** (`editor-core`'s `corpus/vessel.rs`, station for
/// station): a base disc, a cylinder foot of radius `4/64` up to
/// `y = 1/64`, a spherical belly of radius `5/64` about `(0, 4/64)` up to
/// the mouth at `(3/64, 8/64)`, and the mouth disc, revolved a full turn.
fn corpus_pot<T: Scalar>() -> Body<T> {
    let p = |x: f64, y: f64| Point2::new(x, y).map(T::from_f64);
    let (foot, mouth) = (p(4.0 / 64.0, 1.0 / 64.0), p(3.0 / 64.0, 8.0 / 64.0));
    let belly = profile::bulge_from_center(foot, mouth, p(0.0, 4.0 / 64.0), profile::ArcSweep::Ccw);
    let lp = profile::test_support::bulge_loop(vec![
        (p(0.0, 0.0), T::zero()),
        (p(4.0 / 64.0, 0.0), T::zero()),
        (foot, belly),
        (mouth, T::zero()),
        (p(0.0, 8.0 / 64.0), T::zero()),
    ]);
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
}

/// **The corpus pot holds at both scalars, reading no corner.** At
/// `Interval` the circle where the foot meets the belly has, on the
/// cylinder's chart, an axial coordinate `0 + a·cos t` with `a` an
/// enclosure of zero, the rounding of the rim's reference direction
/// against the axis; at `f64` it is `0.0`. The pcurve derivation decides
/// it zero at `pcurve_cylinder_chart_axial`, so the presentation reads
/// the line `v = 0`. Read as a curve, the
/// cylinder half refuses `Departure` at the vertex where its seam meets
/// the sphere's other half, which is how the cavity gate over-refused
/// this pot's operand (`shell-cavity-gate-over-refuses-three-valid-bodies`).
/// Red if the rim is not decided: the `Interval` twin then refuses where
/// `f64` holds.
#[test]
fn the_corpus_pot_holds_at_both_scalars() {
    let [float, interval] = holds_at_both("the corpus pot", corpus_pot(), corpus_pot());
    assert_eq!(
        (float.cornered, interval.cornered),
        (0, 0),
        "no certificate is read through a corner"
    );
}

/// The faces of `body` shelled to `t` on its cavity's shell, with the
/// shelled body: the clone the cavity gate asks.
fn cavity_of<T: Scalar>(body: Body<T>, t: f64) -> (Body<T>, Vec<FaceKey>) {
    let hollow = topo::shell(
        &sweep::test_support::finished("the operand", body, tol()),
        T::from_f64(t),
        tol(),
    )
    .expect("the operand shells")
    .body;
    let roles = topo::classify_shells(
        &sweep::test_support::finished("the hollow", hollow.clone(), tol()),
        tol(),
    )
    .expect("the hollow classifies");
    let [void] = roles
        .iter()
        .filter(|c| c.role == topo::ShellRole::Void)
        .map(|c| c.shell)
        .collect::<Vec<_>>()[..]
    else {
        panic!("one cavity")
    };
    let faces = hollow
        .faces()
        .filter(|(_, f)| f.shell == void)
        .map(|(k, _)| k)
        .collect();
    (hollow, faces)
}

/// **The klein elbow** (`torax_interval`'s): a disc of radius `0.275`
/// revolved a quarter turn about an axis `1.2` off its centre.
fn klein_elbow<T: Scalar>() -> Body<T> {
    let r = 0.275;
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(-r, 0.0).map(T::from_f64), T::one()),
        (Point2::new(r, 0.0).map(T::from_f64), T::one()),
    ]);
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(1.2, 0.0).map(T::from_f64),
        dir: geom_core::Vec2::new(0.0, -1.0).map(T::from_f64),
    };
    let quarter = Revolution::Partial(T::from_f64(-core::f64::consts::FRAC_PI_2));
    revolve(&profile, axis, quarter, tol()).unwrap().body
}

/// **The klein elbow's cavity holds across its spiric caps at both
/// scalars**, shelled `0.05`. The cavity's caps are the operand's moved
/// `0.05` along their normals, off the torus's axis, so each meets the
/// cavity's torus in a spiric section: no conic of the cap's plane and
/// no harmonic row of the torus's chart. The torus is presented beside
/// it by the edge's wall image, a graph over the minor angle, and the
/// cap's own plane separates; where a spiric arc meets the parallel
/// splitting the torus, each torus half is read through the corner, by
/// the arc continued past it against a cap and by the parallel against
/// the other half. Holds, every face pair sharing a vertex certified
/// across, each spiric edge's cap and torus among them, and some
/// certificates read a corner. Red if the wall presentation is
/// withdrawn (the pair refuses `Uncertified { Departure }` at the root),
/// or either torus corner reading is.
#[test]
fn the_klein_elbows_cavity_holds_across_its_spiric_caps() {
    holds_at_both("the klein elbow", klein_elbow(), klein_elbow());
    fn cavity_at<T: Scalar>() -> BoundReport<Witness> {
        let (hollow, faces) = cavity_of(klein_elbow::<T>(), 0.05);
        let spiric: Vec<(FaceKey, FaceKey)> = spiric_pairs(&hollow)
            .into_iter()
            .filter(|(a, b)| faces.contains(a) && faces.contains(b))
            .collect();
        assert_eq!(spiric.len(), 4, "two spiric arcs on each cap");
        for &(a, b) in &spiric {
            let mut kinds = [kind(&hollow, a), kind(&hollow, b)];
            kinds.sort_unstable();
            assert_eq!(kinds, ["plane", "torus"], "a cap and a torus half");
        }
        let report = injective(&hollow, Some(faces), BUDGET);
        println!(
            "[spiric] the elbow's cavity: {:?} {:?} cornered = {} across = {:?}",
            report.verdict, report.receipt, report.cornered, report.across
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "the elbow's cavity holds: {:?}",
            report.verdict
        );
        for p in &spiric {
            assert!(
                report.across.get(p).is_some_and(|&n| n > 0),
                "a cap and its torus half certified across their spiric arc: {p:?}"
            );
        }
        assert!(report.cornered > 0, "a corner of the torus is read through");
        report
    }
    same_class(
        "the elbow's cavity",
        &cavity_at::<f64>(),
        &cavity_at::<Interval>(),
    );
}

/// **The ring torus `(2, 0.5)` about `y`, less the half-space `x < 0.3`**:
/// a plane parallel to the axis, `0.3` off it, cuts the tube in two
/// spiric ovals, each a planar cap of the arc of torus left. The
/// profile's two vertices stand `0.3` rad round the tube from its
/// equator, so the parallels splitting the torus lie off the window's
/// bisections (`spiric-extent-overshoots-the-sheet-slack-at-interval`
/// has the body whose parallels lie on them).
fn torus_cut_off_its_axis<T: Scalar>() -> topo::AtRestBody<T> {
    use core::f64::consts::FRAC_PI_2;
    use geom_core::{Affine3, Vec3};
    use sweep::test_support::finished;
    let (c, s) = (0.5 * 0.3f64.cos(), 0.5 * 0.3f64.sin());
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(2.0 + c, s).map(T::from_f64), T::one()),
        (Point2::new(2.0 - c, -s).map(T::from_f64), T::one()),
    ]);
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let ring = revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body;
    let ring = finished("the torus", ring, tol());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    built(topo::subtract(&ring, &moved(&slab(0.3), &turn), tol()))
}

/// **A torus cut off its axis holds across its spiric caps at both
/// scalars** ([`torus_cut_off_its_axis`]): each cap meets the torus on
/// two spiric arcs, which the torus halves present by their wall images,
/// the cap's own plane separating, and read through the corners where
/// the arcs meet the parallels. Holds, every face pair sharing a vertex
/// certified across, some of them through a corner. Red if the wall
/// presentation, or either torus corner reading, is withdrawn.
#[test]
fn a_torus_cut_off_its_axis_holds_across_its_spiric_caps() {
    let cut = torus_cut_off_its_axis::<f64>();
    let spirics = cut
        .edges()
        .filter(|(_, e)| {
            cut.get_curve_geom(e.curve)
                .and_then(|c| c.certified())
                .is_some_and(|c| c.carrier().kind() == geom::CurveKind::Spiric)
        })
        .count();
    assert_eq!(spirics, 4, "two spiric arcs on each cap");
    for report in holds_at_both(
        "the torus cut off its axis",
        cut.into_body(),
        torus_cut_off_its_axis::<Interval>().into_body(),
    ) {
        assert!(report.cornered > 0, "a corner of the torus is read through");
    }
}

/// The spiric edges of `body`: each one's carrier sampled at 512
/// points of its span.
fn spiric_points(body: &Body<f64>) -> Vec<Point3<f64>> {
    body.edges()
        .filter_map(|(_, e)| body.get_curve_geom(e.curve).and_then(|c| c.certified()))
        .filter(|c| c.carrier().kind() == geom::CurveKind::Spiric)
        .flat_map(|c| {
            let (t0, t1) = c.params();
            (0..=512).map(move |k| c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 512.0))
        })
        .collect()
}

/// **A folded torus cut reads `Violated` off its spiric edges.** Two
/// [`torus_cut_off_its_axis`]es in one body, the second moved `0.2`
/// along `x` and grafted in as a solid of its own: its caps stand inside
/// the first's tube, so each cap face crosses the first's torus faces.
/// Faces of the two solids share nothing in M, so the crossing is found
/// by distance, and the witness is clear of every spiric edge. As for the
/// folded oblique cut, a fold across a shared spiric is a crossing of two
/// faces that share the edge, which no kernel operation builds; the
/// arm's own fold is `injective`'s
/// `a_torus_folded_back_across_its_caps_plane_is_not_certified`.
#[test]
fn a_folded_torus_cut_reads_violated_off_its_spiric_edges() {
    fn folded_at<T: Scalar>() -> Body<T> {
        use geom_core::{Affine3, Vec3};
        let cut = torus_cut_off_its_axis::<T>();
        let mut folded = cut.clone().into_body();
        let over = Affine3::translation(Vec3::new(0.2, 0.25, 0.0)).map(T::from_f64);
        let rigid = geom_core::test_support::rigid(&over, tol());
        let copy = topo::transform_rigid(&cut, &rigid, tol()).unwrap();
        topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
        folded
    }
    let folded = folded_at::<f64>();
    let report = injective(&folded, None, BUDGET);
    same_class(
        "the folded torus cut",
        &report,
        &injective(&folded_at::<Interval>(), None, BUDGET),
    );
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the two cuts cross: {:?}", report.verdict)
    };
    assert!(w.d < 1e-9, "a coincidence: {w:?}");
    let edges = spiric_points(&folded);
    assert!(!edges.is_empty(), "the spiric edges are sampled");
    for p in [w.pa, w.pb] {
        let off = edges
            .iter()
            .map(|q| (*q - p).norm())
            .fold(f64::INFINITY, f64::min);
        assert!(
            off >= 0.01,
            "the witness {p:?} is off every spiric edge: {off}"
        );
    }
}

/// **The one-face torus `(2, 0.5)` about `y`, less the half-space
/// `x < 0.3`**: the profile one circle from profile angle `0.3`, so the
/// torus closes on itself across a meridian and a parallel through that
/// point. The cut splits it into two faces across the meridian, each
/// holding one whole spiric oval, closed at a vertex on its parallel
/// seam, beside one cap.
fn one_face_torus_cut<T: Scalar>() -> topo::AtRestBody<T> {
    use core::f64::consts::FRAC_PI_2;
    use geom_core::{Affine3, Vec3};
    use sweep::test_support::finished;
    let phase = 0.3f64;
    let lp: ProfileLoop<T> = RawLoop::new([(
        Point2::new(2.0 + 0.5 * phase.cos(), 0.5 * phase.sin()).map(T::from_f64),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(2.0, 0.0).map(T::from_f64),
            radius: T::from_f64(0.5),
            sweep: T::from_f64(TAU),
        }),
    )]);
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let ring = revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body;
    let ring = finished("the one-face torus", ring, tol());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    built(topo::subtract(&ring, &moved(&slab(0.3), &turn), tol()))
}

/// **A one-face torus cut certifies its caps by the wall alone**
/// ([`one_face_torus_cut`]): each cap shares a whole oval with its torus
/// face, with no corner on it, so each cap–torus pair is certified across
/// by the plain wall presentation, none through a corner. The body
/// refuses `Departure` across the meridian the two torus faces share,
/// pinned typed at both scalars
/// (`one-face-torus-cut-refuses-across-its-meridian`). Red if the plain
/// wall presentation is withdrawn.
#[test]
fn a_one_face_torus_cut_certifies_its_caps_by_the_wall_alone() {
    fn at<T: Scalar>() -> BoundReport<Witness> {
        let body = one_face_torus_cut::<T>().into_body();
        let cut = spiric_pairs(&body);
        let report = injective(&body, None, BUDGET);
        let BoundVerdict::Refused(EngineRefusal::Uncertified {
            a,
            b,
            shared: SharedCell::Edge(e),
            arm: CertificateArm::Departure,
        }) = report.verdict
        else {
            panic!("the cut refuses at its meridian: {:?}", report.verdict)
        };
        assert_eq!(
            [kind(&body, a), kind(&body, b)],
            ["torus", "torus"],
            "the two torus faces"
        );
        let carrier = body
            .get_curve_geom(body.get_edge(e).unwrap().curve)
            .and_then(|c| c.certified())
            .map(|c| c.carrier().kind());
        assert_eq!(
            carrier,
            Some(geom::CurveKind::Circle),
            "across the meridian"
        );
        assert_eq!(cut.len(), 2, "two caps, each on one whole oval");
        for p in &cut {
            assert!(
                report.across.get(p).is_some_and(|&n| n > 0),
                "a cap and its torus face certified across their oval: {p:?}"
            );
        }
        assert_eq!(report.cornered, 0, "no corner is read");
        report
    }
    same_class("the one-face torus cut", &at::<f64>(), &at::<Interval>());
}

/// The face pairs of `body` sharing a spiric edge, in key order.
fn spiric_pairs<T: Real>(body: &Body<T>) -> Vec<(FaceKey, FaceKey)> {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(|c| c.certified())
                .is_some_and(|c| c.carrier().kind() == geom::CurveKind::Spiric)
        })
        .map(|(_, e)| {
            let face = |he| {
                let l = body.get_half_edge(he).unwrap().parent_loop;
                body.get_loop(l).unwrap().face
            };
            let (a, b) = (face(e.he_plus), face(e.he_minus));
            (a.min(b), a.max(b))
        })
        .collect()
}

/// **A frustum** of base radius `4/64`, top radius `r1` and height
/// `8/64`, revolved a full turn.
fn frustum<T: Scalar>(r1: f64) -> Body<T> {
    let (r0, h) = (4.0 / 64.0, 8.0 / 64.0);
    let lp = profile::test_support::bulge_loop(
        [(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y).map(T::from_f64), T::zero()))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
}

/// **A cone frustum holds at both scalars, and so does its cavity**
/// (`sf2b_interval_probe`'s frustum, top radius `2/64`, shelled `1/128`;
/// and top radius `3/64`). A frustum's faces miss their apex, so their
/// windows end at their rims (`clearance::window_of`): the cone's two
/// halves meet in space only at the apex, which no cell then holds.
/// Each holds well inside the budget.
#[test]
fn a_cone_frustum_and_its_cavity_hold_at_both_scalars() {
    fn within(what: &str, report: &BoundReport<Witness>) {
        assert!(
            report.receipt.splits < 1024,
            "{what} holds without splitting toward the apex: {:?}",
            report.receipt
        );
    }
    for (what, r1) in [
        ("the 2/64 frustum", 2.0 / 64.0),
        ("the 3/64 frustum", 3.0 / 64.0),
    ] {
        for report in &holds_at_both(what, frustum(r1), frustum(r1)) {
            within(what, report);
        }
    }
    fn cavity_at<T: Scalar>(what: &str) -> BoundReport<Witness> {
        let (hollow, faces) = cavity_of(frustum::<T>(2.0 / 64.0), 1.0 / 128.0);
        let report = injective(&hollow, Some(faces), BUDGET);
        println!("[selfx] {what}: {:?} {:?}", report.verdict, report.receipt);
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "{what} is embedded: {:?} {:?}",
            report.verdict,
            report.receipt
        );
        within(what, &report);
        report
    }
    same_class(
        "the frustum's cavity",
        &cavity_at::<f64>("the frustum's cavity"),
        &cavity_at::<Interval>("the frustum's cavity at Interval"),
    );
}

// ------------------------------------- cap corners and twisted seams

/// **Two faces of `body` asked alone**, each in its own window, at the
/// default depth and the pair budget `budget`. No candidate is
/// witnessed: the verdict is the certificates'.
fn pair_report<T: Scalar>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    budget: usize,
) -> BoundReport<()> {
    let window = |f| {
        Side {
            body,
            faces: vec![f],
        }
        .windows(Which::First)
        .expect("every face has a window")
    };
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget,
        },
    };
    sweep.run(&window(a), &window(b), true, |_| {
        Err::<(), _>("no witness asked".to_owned())
    })
}

/// Whether `f` of `body` is a plane.
fn is_plane<T: Real>(body: &Body<T>, f: FaceKey) -> bool {
    matches!(surface(body, f), Surface::Plane { .. })
}

/// The face pairs of `body` sharing a vertex, of a plane and a spline
/// (`caps`) or of two splines.
fn adjacent<T: Real>(body: &Body<T>, caps: bool) -> Vec<(FaceKey, FaceKey)> {
    touching_pairs(body)
        .into_iter()
        .filter(|&(a, b)| (is_plane(body, a) != is_plane(body, b)) == caps)
        .filter(|&(a, b)| caps || !(is_plane(body, a) || is_plane(body, b)))
        .collect()
}

/// Two unit lens sections, two arcs on one chord, stacked one apart three
/// times: `lily_leaf_a`'s section, lofted straight.
fn lens_loft<T: Scalar>() -> Body<T> {
    let lens = vec![profile::test_support::bulge_loop(vec![
        (Point2::new(0.5, 0.0), -0.3),
        (Point2::new(-0.5, 0.0), -0.3),
    ])];
    let sections = sweep::test_support::lift_sections::<T>(&[lens.clone(), lens.clone(), lens]);
    let places = sweep::test_support::lift_places::<T>(
        &sweep::test_support::stacked_at(&[0.0, 1.0, 2.0]),
        tol(),
    );
    loft_body::<T>(&sections, &places, 2, tol())
        .expect("the lens loft lofts")
        .body
}

/// **A lens loft's rational walls hold against themselves, at both
/// scalars and moved off the origin**: each wall alone at the default
/// budget, at the origin and moved by `(10, 10, 10)`, certifying a
/// cell-pair count within 1% of its unmoved twin's. The lens arcs are
/// rational, so the monotone arm reads each wall through a rational
/// net's derivative over a cell; read in the absolute frame, the moved
/// walls' derivative carries the distance times the weight's slope.
/// The `Interval` twin of the tour's `lily_leaf_a` wall rows, which
/// build at `f64` only.
#[test]
fn a_lens_lofts_walls_hold_against_themselves_moved_off_the_origin_at_both_scalars() {
    fn row<T: Scalar>(what: &str) {
        let home = lens_loft::<T>();
        let by = geom_core::Vec3::new(10.0, 10.0, 10.0).map(T::from_f64);
        let motion = geom_core::RigidMotion::translation(by).expect("a translation");
        let away = topo::transform_rigid(&home, &motion, tol()).expect("the loft moves");
        let walls: Vec<FaceKey> = home
            .faces()
            .map(|(k, _)| k)
            .filter(|&f| matches!(surface(&home, f), Surface::Nurbs(_)))
            .collect();
        assert_eq!(walls.len(), 2, "{what}: two lens walls");
        for wall in walls {
            let (a, b) = (
                injective(&home, Some(vec![wall]), BUDGET),
                injective(&away, Some(vec![wall]), BUDGET),
            );
            println!(
                "[jet frame] {what}: {wall:?} {} {:?}; moved {} {:?}",
                class(&a.verdict),
                a.receipt,
                class(&b.verdict),
                b.receipt
            );
            for (at, r) in [("at the origin", &a), ("moved", &b)] {
                assert!(
                    matches!(r.verdict, BoundVerdict::Holds),
                    "{what}: {wall:?} {at} holds against itself, got {} {:?}",
                    class(&r.verdict),
                    r.receipt
                );
            }
            let (x, y) = (a.receipt.certified, b.receipt.certified);
            #[allow(clippy::cast_precision_loss)]
            let close = (x as f64 - y as f64).abs() <= 0.01 * (x.max(y) as f64);
            assert!(close, "{what}: {wall:?} certifies {y} moved against {x}");
        }
    }
    row::<f64>("the lens loft");
    row::<Interval>("the lens loft at Interval");
}

/// **A lens loft's caps hold against its walls at the lens tip, at both
/// scalars.** Two unit lens sections, two arcs on one chord, one above
/// the other: each cap is a plane bounded by two circles meeting at the
/// two tips. A cap cell holding a tip holds both arcs, and no
/// presentation reads a corner of two curved edges on a plane; the
/// wall's departure from the cap's plane certifies it, the wall's side
/// on the shared edge read off its own restricted iso net, which lies
/// in the cap's plane. This is the tour's `lily_leaf_a` cap corner
/// (`demos/tour/tests/lily.rs`), built here at both scalars.
#[test]
fn a_lens_loft_cap_holds_against_its_walls_at_its_tip_at_both_scalars() {
    fn receipts<T: Scalar>(what: &str, body: &Body<T>) -> Vec<(usize, u32)> {
        adjacent(body, true)
            .into_iter()
            .map(|(a, b)| {
                let report = pair_report(body, a, b, 8192);
                assert!(
                    matches!(report.verdict, BoundVerdict::Holds),
                    "{what}: {a:?} against {b:?} holds: {:?} {:?}",
                    report.verdict,
                    report.receipt
                );
                (report.receipt.splits, report.widths.deepest)
            })
            .collect()
    }
    let (float, interval) = (lens_loft::<f64>(), lens_loft::<Interval>());
    let pairs = adjacent(&float, true);
    assert_eq!(pairs.len(), 4, "two caps against two walls: {pairs:?}");
    assert_eq!(
        adjacent(&interval, true),
        pairs,
        "one topology at both scalars"
    );
    let pinned = [(0, 0), (0, 0), (0, 0), (0, 0)];
    assert_eq!(
        receipts("the lens loft", &float),
        pinned,
        "each cap-wall pair's (splits, deepest cell)"
    );
    assert_eq!(
        receipts("the lens loft at Interval", &interval),
        pinned,
        "each cap-wall pair's (splits, deepest cell) at Interval"
    );
}

/// **A twisted loft holds along its seams, at both scalars.** The
/// prism's sections, the top square authored from its second corner, so
/// every strut runs a quarter turn round (the tour's `twisted_loft`,
/// `demos/tour/src/skinned.rs`). Across a seam the departure arm's
/// graph separator bisects the two faces' headings about the seam; two
/// cells offset along it, at this seam's dihedral (about 51°), read
/// opposite sides of it. Every seam holds, well under the depth budget.
#[test]
fn a_twisted_loft_holds_along_its_seams_at_both_scalars() {
    fn twisted<T: Scalar>() -> Body<T> {
        use sweep::test_support::{
            PRISM_SQUARE, PRISM_TRAPEZOID, lift_places, lift_sections, stacked_at,
        };
        let mut top = PRISM_SQUARE;
        top.rotate_left(1);
        let sections = lift_sections::<T>(&[quad(PRISM_SQUARE), quad(PRISM_TRAPEZOID), quad(top)]);
        let places = lift_places::<T>(&stacked_at(&[0.0, 1.0, 2.0]), tol());
        loft_body::<T>(&sections, &places, 2, tol())
            .expect("the twisted sections loft")
            .body
    }
    /// Each seam's receipt and depth, every one holding, nothing
    /// refused, well under the depth budget.
    fn holds<T: Scalar>(
        what: &str,
        body: &Body<T>,
        seams: &[(FaceKey, FaceKey)],
    ) -> Vec<(CellReceipt, u32)> {
        seams
            .iter()
            .map(|&(a, b)| {
                let report = pair_report(body, a, b, 8192);
                let (r, deepest) = (report.receipt, report.widths.deepest);
                println!(
                    "[heading] {what}: {a:?} against {b:?}: {:?} {r:?} deepest {deepest}",
                    report.verdict
                );
                assert!(
                    matches!(report.verdict, BoundVerdict::Holds)
                        && r.refused == 0
                        && r.certified > 0
                        && deepest < 30,
                    "{what}: {a:?} against {b:?} holds, certified, nothing refused, well \
                     under the depth budget {DEFAULT_MAX_CELL_DEPTH}: {:?} {r:?} deepest {deepest}",
                    report.verdict
                );
                (r, deepest)
            })
            .collect()
    }
    let (float, interval) = (twisted::<f64>(), twisted::<Interval>());
    let seams = adjacent(&float, false);
    assert_eq!(seams.len(), 4, "four walls meet at four seams: {seams:?}");
    assert_eq!(
        adjacent(&interval, false),
        seams,
        "one topology at both scalars"
    );
    assert_eq!(
        holds("the twisted loft at Interval", &interval, &seams),
        holds("the twisted loft", &float, &seams),
        "every seam reads one receipt at both scalars"
    );
}

/// **The half-turn helix's caps hold against its walls, at both
/// scalars.** `m8_14_long_turn_sweep`'s half-turn helix at 33 stations,
/// a 0.16 square swept along it: CARVE's measurement found two of its
/// top cap's four wall pairs refusing Departure at depth
/// (`injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams`), on a branch from before the carried frame
/// and the clip sides' run-on past a corner. Every cap–wall pair now
/// holds, at a depth far under the budget.
#[test]
fn the_half_turn_helixs_caps_hold_against_its_walls_at_both_scalars() {
    fn helical<T: Scalar>() -> Body<T> {
        let path = helix(0.5, 0.4);
        let place = plane_at(&normal_start_place(&path));
        let profile = lifted(&quad([
            (-0.08, -0.08),
            (0.08, -0.08),
            (0.08, 0.08),
            (-0.08, 0.08),
        ]));
        sweep_body::<T>(&profile, place, &path, 33, 3, tol())
            .expect("the helix sweeps")
            .body
    }
    fn holds<T: Scalar>(what: &str, body: &Body<T>) -> usize {
        let pairs = adjacent(body, true);
        for &(a, b) in &pairs {
            let report = pair_report(body, a, b, 8192);
            assert!(
                matches!(report.verdict, BoundVerdict::Holds) && report.widths.deepest < 20,
                "{what}: {a:?} against {b:?} holds well under the depth budget: {:?} {:?} \
                 deepest {}",
                report.verdict,
                report.receipt,
                report.widths.deepest
            );
        }
        pairs.len()
    }
    assert_eq!(
        holds("the half-turn helix", &helical::<f64>()),
        8,
        "two caps against four walls"
    );
    assert_eq!(
        holds("the half-turn helix at Interval", &helical::<Interval>()),
        8,
        "two caps against four walls"
    );
}

// ----------------------------------- two walls apart along a direction

/// **A thin curved slab**: the section `(-0.4, 0.4) × (-g/2, g/2)`
/// lofted at five stations along a unit-radius arc turning `curl` in
/// the `xz` plane, the whole turned `phi` about the world `y` axis.
/// Its two wide walls are near-parallel sheets `g` apart with a radius
/// of about 1, sharing no vertex; they are returned with the body.
fn slab_wall_pair<T: Scalar>(g: f64, curl: f64, phi: f64) -> (sweep::Lofted<T>, [FaceKey; 2]) {
    use geom_core::{Affine3, Mat3, Vec3};
    let section = quad([
        (-0.4, -g / 2.0),
        (0.4, -g / 2.0),
        (0.4, g / 2.0),
        (-0.4, g / 2.0),
    ]);
    let turn = Mat3::rotation_about(Vec3::new(0.0, 1.0, 0.0), phi);
    let places: Vec<SketchPlane<T>> = (0..5)
        .map(|i| {
            let theta = curl * f64::from(i) / 4.0;
            let (s, c) = theta.sin_cos();
            let linear = Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
                Vec3::new(c, 0.0, s),
            );
            let at = Affine3::from_parts(turn * linear, turn * Vec3::new(s, 0.0, 1.0 - c));
            plane_at(&crate::common::plane_of(&at))
        })
        .collect();
    let sections: Vec<_> = (0..5).map(|_| lifted(&section)).collect();
    let lofted = loft_body::<T>(&sections, &places, 3, tol()).expect("the slab lofts");
    let walls = [lofted.side_faces[0][0][0], lofted.side_faces[0][2][0]];
    (lofted, walls)
}

/// The bound question between `walls[0]` and `walls[1]` of `body`, at
/// the default depth and pair budget.
fn wall_against_wall<T: Scalar>(
    body: &Body<T>,
    walls: [FaceKey; 2],
    bound: ClearanceBound,
) -> BoundReport<Witness> {
    let window = |f, which| {
        Side {
            body,
            faces: vec![f],
        }
        .windows(which)
        .expect("a wall has a window")
    };
    let sweep = Sweep {
        bound,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: BUDGET,
        },
    };
    let report = sweep.run(
        &window(walls[0], Which::First),
        &window(walls[1], Which::Second),
        true,
        verify(body, bound),
    );
    assert!(
        report.receipt.holds(),
        "the receipt adds up: {:?}",
        report.receipt
    );
    report
}

/// **Two near-parallel walls cost about the same pairs however they are
/// turned against the axes.** The slab's walls `0.005` apart, turning
/// 0.5 rad, hold under the strict question at both scalars, upright
/// (`phi = 0`, the walls facing within 0.5 rad of `x`) and turned 45°
/// about `y`. Each examines at most 512 cell pairs, and the turned pair
/// at most 1.5 times the upright one's. Red where the distance arm
/// separates axis-aligned boxes only: the boxes of walls tilted against
/// the axes stay apart only for cells about as small as the gap.
#[test]
fn near_parallel_walls_cost_the_same_pairs_turned_45_degrees() {
    let examined =
        |r: &BoundReport<Witness>| r.receipt.discharged + r.receipt.refused + r.receipt.violated;
    let mut counts = Vec::new();
    for phi in [0.0, core::f64::consts::FRAC_PI_4] {
        let (float, walls) = slab_wall_pair::<f64>(0.005, 0.5, phi);
        let (twin, twin_walls) = slab_wall_pair::<Interval>(0.005, 0.5, phi);
        let report = wall_against_wall(&float.body, walls, ClearanceBound::StrictlyPositive);
        let interval = wall_against_wall(&twin.body, twin_walls, ClearanceBound::StrictlyPositive);
        println!(
            "[dirsupport] slab turned {phi}: f64 {:?} deepest {} | Interval {:?} deepest {}",
            report.receipt, report.widths.deepest, interval.receipt, interval.widths.deepest
        );
        for (what, r) in [("f64", &report), ("Interval", &interval)] {
            assert!(
                matches!(r.verdict, BoundVerdict::Holds),
                "the slab's walls turned {phi} hold at {what}: {:?} {:?}",
                r.verdict,
                r.receipt
            );
        }
        for (what, r) in [("f64", &report), ("Interval", &interval)] {
            assert!(
                examined(r) <= 512,
                "the slab's walls turned {phi} examine at most 512 pairs at {what}: {:?}",
                r.receipt
            );
        }
        counts.push(examined(&report));
    }
    let [upright, turned] = counts[..] else {
        unreachable!("two turns")
    };
    assert!(
        2 * turned <= 3 * upright,
        "the turned walls examine at most 1.5× the upright walls' pairs: {turned} against {upright}"
    );
}

/// **A wall pair closer than the bound is never separated.** The slab's
/// walls turned 45° are `0.05` apart: `AtLeast(0.08)` reads `Violated`
/// with a verified witness closer than `0.08`, at both scalars, and
/// `AtLeast(0.03)` holds. Red if the support gap is compared without
/// the bound.
#[test]
fn a_wall_pair_closer_than_the_bound_is_violated_not_separated() {
    let phi = core::f64::consts::FRAC_PI_4;
    let (float, walls) = slab_wall_pair::<f64>(0.05, 0.3, phi);
    let (twin, twin_walls) = slab_wall_pair::<Interval>(0.05, 0.3, phi);
    for (what, report) in [
        (
            "f64",
            wall_against_wall(&float.body, walls, ClearanceBound::AtLeast(0.08)),
        ),
        (
            "Interval",
            wall_against_wall(&twin.body, twin_walls, ClearanceBound::AtLeast(0.08)),
        ),
    ] {
        let BoundVerdict::Violated(w) = report.verdict else {
            panic!(
                "{what}: walls 0.05 apart violate AtLeast(0.08): {:?} {:?}",
                report.verdict, report.receipt
            )
        };
        assert!(
            w.d < 0.08,
            "{what}: the witness is closer than the bound: {w:?}"
        );
    }
    let report = wall_against_wall(&float.body, walls, ClearanceBound::AtLeast(0.03));
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "walls 0.05 apart hold AtLeast(0.03): {:?} {:?}",
        report.verdict,
        report.receipt
    );
}

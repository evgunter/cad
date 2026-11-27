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
//! - a windowed tube holds across its caps' circles, where each cap's
//!   two semicircles meet on the walls' seams;
//! - a three-arc disc's filleted rim holds across the joints of its
//!   band's foot parallel, as built, turned and far from the origin, and
//!   two such discs through each other at a joint read `Violated`;
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
//! - a lens-walled body's cap refuses typed at the depth budget against a wall
//!   at the lens tip, while a twisted loft's seams and the half-turn
//!   helix's caps hold against their walls;
//! - a two-arc lune's hollow holds, its cavity's sphere read beside its
//!   caps' circles in the caps' poses, turned six ways, moved at random,
//!   and hollowed after a turn; balls crossing or touching its posed
//!   sphere read `Violated` and balls clear of it hold; with its seam off
//!   the equator it refuses across the seam.
//!
//! Every row builds its body twice, by the same generic builders: at
//! `f64`, whose verdict and receipt the row pins, and at `Interval`, the
//! certifying scalar the shell gate and the document door ask at, which
//! must read the same verdict class ([`same_class`]) — `Holds`,
//! `Violated`, or the same refusal arm. The exception is a body the
//! `Interval` build does not reach: the lune's hollow, whose shell
//! refuses at `Interval`, so its rows read `f64` alone, and
//! [`a_two_arc_lunes_hollow_is_refused_by_its_shell_at_interval`] pins
//! the refusal. The body that reaches a sphere's pose at both scalars is
//! the lens of two spheres
//! ([`a_lens_of_two_spheres_holds_at_both_scalars_read_in_its_poses`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;
use topo::LeafBody as _;

use geom::{NurbsCurve3, Surface};
use geom_core::k_stats::decide;
use geom_core::{
    Band, Bounds, CertifiedBounds, Decide, Interval, Margin, Point2, Point3, Real, Sign, Tol,
};
use profile::{Open, Profile, ProfileLoop, RawLoop, SketchPlane, Start, ValidatedProfile};
use std::collections::BTreeSet;
use sweep::test_support::revolved_about_y_at;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve, sweep_body};
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, CLEARANCE_MARGIN, Candidate, CellBudget, CellReceipt,
    CertificateArm, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, EngineRefusal, SharedCell, Side, Sweep,
    Which, min_separation,
};
use topo::{Body, FaceKey, LoopBoundary, VertexKey};

use crate::common::{arc_spine_places, bulge, normal_start_place, quad};
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
trait Scalar:
    Decide
    + Bounds
    + CertifiedBounds
    + topo::CertifiedPolicy
    + topo::AtRestPolicy<Finished = topo::AtRestBody<Self>>
{
}
impl<
    T: Decide
        + Bounds
        + CertifiedBounds
        + topo::CertifiedPolicy
        + topo::AtRestPolicy<Finished = topo::AtRestBody<T>>,
> Scalar for T
{
}

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
    body.face_surface(f).unwrap().clone()
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
        let definite = match decide(
            bound.predicate(),
            Margin::of(d - at(bound.lower().map_or(0.0, |lo| lo.lo()))),
            band(),
        ) {
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
        body.face(f)
            .unwrap()
            .loops()
            .map(|(lk, _, _)| lk)
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
        BoundVerdict::Within(_) => "Within".to_owned(),
        BoundVerdict::Exceeded => "Exceeded".to_owned(),
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

/// **A body of one face holds**, its self-pair discharged by kind: what
/// [`assert_holds`] reads of a body with no touching pair to examine (a
/// full revolve's ball, one sphere face). Red if it does not hold, if a
/// pair refuses, or if a face's self-pair is not discharged by kind.
fn holds_alone<T: Real>(what: &str, report: &BoundReport<Witness>, body: &Body<T>) {
    let r = report.receipt;
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "{what} is embedded: {r:?}"
    );
    assert_eq!(body.faces().count(), 1, "{what} is one face");
    assert_eq!(
        (r.candidates, r.refused, r.by_kind),
        (1, 0, 1),
        "{what}: its one face against itself, discharged by kind: {r:?}"
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
fn top_wall<T: Scalar>(lofted: &sweep::Lofted<T, Body<T>>) -> FaceKey {
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
        .flatten()
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
            sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, 48, 3, tol())
                .expect("the ribbon sweeps");
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
fn carvetail<T: Scalar>() -> sweep::Lofted<T, Body<T>> {
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
    sweep::test_support::loft_body_construction::<T>(&sections, &places, 1, tol())
        .expect("the carvetail lofts")
}

/// **The carvetail fold reads `Violated`, its witness at `v ≈ ½`.** In
/// the [`carvetail`] loft the seam between walls 0 and 1
/// passes through wedge 0 at `v = ½`, where wall 1 lies along wall 0
/// and crosses it. The two walls share that seam, so the pair is the
/// one the global drop never examined.
#[test]
fn the_carvetail_fold_reads_violated_at_its_middle_station() {
    fn fold<T: Scalar>() -> (sweep::Lofted<T, Body<T>>, Vec<FaceKey>) {
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
            .flatten()
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

/// **A revolved ball holds**: a half disc revolved a full turn about
/// its diameter is one sphere face, its one meridian walked out and
/// back, and it holds at both scalars. Cut into two lunes sharing their
/// two meridians and both poles, each lune is read in its own half turn
/// and the pair splits there, so the body holds only after it splits.
/// Red if the lunes hold without a split: one was then read as holding
/// nothing in its own window, on an extent off its sheet.
#[test]
fn a_revolved_ball_holds_only_after_it_splits() {
    fn ball<T: Scalar>() -> Body<T> {
        let p = |y: f64, b: f64| (Point2::new(0.0, y).map(T::from_f64), T::from_f64(b));
        revolved_about_y_at(vec![p(-1.0, 1.0), p(1.0, 0.0)], Revolution::Full, tol())
    }
    let (float, twin) = (
        injective(&ball::<f64>(), None, BUDGET),
        injective(&ball::<Interval>(), None, BUDGET),
    );
    same_class("the one-face ball", &float, &twin);
    assert!(
        matches!(float.verdict, BoundVerdict::Holds),
        "the one-face ball is embedded: {:?}",
        float.receipt
    );
    let mut lunes = ball::<f64>();
    crate::seam_region_common::two_lunes(&mut lunes, tol());
    let report = injective(&lunes, None, BUDGET);
    assert_holds("the two lunes", &report, &lunes);
    assert!(report.receipt.splits > 0, "{:?}", report.receipt);
}

/// **A cone holds**: the triangle `(0,0) (1,0) (0,1)` revolved a full
/// turn about `y` — one cone face, its meridian walked out from the apex
/// and back, sharing its closed rim with the base disc.
#[test]
fn a_revolved_cone_holds() {
    fn cone<T: Scalar>() -> Body<T> {
        let profile = validated::<T>(&[ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)].map(|(x, y)| Point2::new(x, y)),
        )]);
        revolve(&profile, axis_y(), Revolution::Full, tol())
            .unwrap()
            .body
            .into_body()
    }
    holds_at_both("the cone", cone::<f64>(), cone::<Interval>());
}

/// **The box `2 × 1` at depth `h = 1 ± 1e-2` holds at `Sym<Interval>`**:
/// a family, each plane the engine builds through a shared edge built
/// from that edge's own stored direction at the lane scalar.
#[test]
fn a_box_family_holds_at_sym_interval() {
    use geom_core::sym::with_session_rules;
    use geom_core::{ParamSymbol, Sym, SymBudget, SymRules};
    use sweep::ExtrudeSide;
    type S = Sym<Interval>;
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let ((), _) = with_session_rules(budget, SymRules::shipped(), || {
        let h: S = Sym::param(
            ParamSymbol::new(test_utils::symbol_id("h")),
            Interval::from_bounds(1.0 - 1e-2, 1.0 + 1e-2),
        );
        let profile = validated::<S>(&quad([(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]));
        let body = Body::clone(
            &extrude(
                &profile,
                Extrusion::Distance {
                    depth: h,
                    side: ExtrudeSide::Along,
                },
                tol(),
            )
            .expect("the family extrudes")
            .body,
        );
        let report = injective(&body, None, BUDGET);
        assert_holds("the box at h = 1 ± 1e-2", &report, &body);
    });
}

/// **A pyramid holds**: a square base and an apex — four planar faces
/// meeting at the apex, two of each pair of opposite ones sharing only
/// that vertex.
#[test]
fn a_pyramid_holds() {
    fn pyramid<T: Scalar>() -> Body<T> {
        use topo::test_support::meeting::{Pose, posed_pyramid_at};
        posed_pyramid_at::<T>(
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
        sweep::test_support::loft_body_construction::<T>(&sections, &places, 1, tol())
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
        extrude(&validated::<T>(&[lune()]), to_offset(1.0), tol())
            .unwrap()
            .body
            .into_body()
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
    assert_eq!(
        report.receipt.refused, 1,
        "the ask ends at the kiss: {:?}",
        report.receipt
    );
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
        let profile = validated::<T>(&quad([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]));
        let built = extrude(&profile, to_offset(1.0), tol()).unwrap();
        let walls = built.side_faces();
        let (fa, fb) = (walls[0][0], walls[0][1]);
        (built.body.into_body(), fa, fb)
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
        let bound = ClearanceBound::at_least(0.1);
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
    assert_eq!(CLEARANCE_MARGIN, ClearanceBound::at_least(0.1).predicate());
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
            &validated::<T>(&[donut_profile()]),
            axis_y(),
            Revolution::Full,
            tol(),
        )
        .unwrap()
        .body
        .into_body()
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

/// The football of the spindle sheet `(R, 1)`, its centre `by` up the
/// axis ([`crate::revolve_common::football`]), revolved whole: two sheet
/// faces, each a π-band between meridians ending at both apexes.
fn football<T: Scalar>(big: f64, by: f64) -> Body<T> {
    let lp = crate::revolve_common::football(big, 1.0, by);
    revolve(&validated::<T>(&[lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

/// A horn's needle tip ([`crate::revolve_common::needle_tip`]), its arc
/// an eighth of the circle from the cusp, so it meets its disc at 45°,
/// not tangent.
fn needle<T: Scalar>() -> Body<T> {
    let lp = crate::revolve_common::needle_tip(core::f64::consts::FRAC_PI_4);
    revolve(&validated::<T>(&[lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

/// The spindle-sheet faces of `body`.
fn sheet_faces<T: Real>(body: &Body<T>) -> usize {
    body.faces()
        .filter(|&(k, _)| matches!(surface(body, k), Surface::Spindle { .. }))
        .count()
}

/// **Bodies on a spindle sheet hold at both scalars, each sheet face
/// against itself by kind**: the lemon and the apple, each one sheet
/// face from apex to apex, read at each apex as their chart's collapsed
/// side; and a horn's needle, its one sheet face reaching the horn's
/// cusp from one side only, so that no two of its points meet but at
/// the cusp, the chart's collapsed side (`topo::injective`'s premise
/// for discharging a horn's self-pair by kind). Red if a sheet face has
/// no window, if its self-pair is not discharged by kind, if a cell at
/// an apex is read as anything but the collapsed side, or if any of the
/// three does not hold.
#[test]
fn bodies_on_a_spindle_sheet_hold_by_kind_at_both_scalars() {
    for (what, float, interval) in [
        ("the lemon", football(-0.5, 0.0), football(-0.5, 0.0)),
        ("the apple", football(0.4, 0.0), football(0.4, 0.0)),
        ("the needle", needle::<f64>(), needle::<Interval>()),
    ] {
        let (float, interval): (Body<f64>, Body<Interval>) = (float, interval);
        let sheets = sheet_faces(&float);
        assert!(sheets >= 1, "{what} is built on a spindle sheet");
        // The lemon and the apple are one face each, with no touching
        // pair for [`holds_at_both`] to read: each scalar holds and
        // reads the class the other does.
        let (report, twin) = (
            injective(&float, None, BUDGET),
            injective(&interval, None, BUDGET),
        );
        same_class(what, &report, &twin);
        for (scalar, report) in [("f64", report), ("Interval", twin)] {
            assert!(
                matches!(report.verdict, BoundVerdict::Holds),
                "{what} is embedded at {scalar}: {:?}",
                report.receipt
            );
            assert!(
                report.receipt.by_kind >= sheets,
                "{what} at {scalar}: each of its {sheets} sheet faces against itself \
                 discharged by kind: {:?}",
                report.receipt
            );
        }
    }
}

/// **The point door places a sheet face's interior**, at both scalars:
/// on the lemon and the apple, each one sheet face from apex to apex
/// (a full revolve's one band), a station short of its wrap edge and of
/// the apexes is `In` on that face, at every azimuth round the turn.
/// Red if a sheet's interior is unplaced or placed `Out` anywhere round
/// the turn.
#[test]
fn the_point_door_places_a_sheet_faces_interior_at_both_scalars() {
    fn placed<T: Scalar>(what: &str, big: f64) {
        let body = football::<T>(big, 0.0);
        let sheets: Vec<FaceKey> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&k| matches!(surface(&body, k), Surface::Spindle { .. }))
            .collect();
        let [sheet] = sheets[..] else {
            panic!("{what}: one sheet face, not {}", sheets.len());
        };
        let apex = geom::sheet_apex(big, 1.0).v;
        for u in [0.4, 1.5, 2.7, 3.6, 4.7, 5.9] {
            for v in [-0.95, -0.5, 0.0, 0.5, 0.95].map(|f| f * apex) {
                let got = topo::cell_region::station_region(&body, sheet, (u, v), band());
                assert!(
                    matches!(got, Some(topo::FaceContainment::In)),
                    "{what}: ({u}, {v}) is in the sheet face: {got:?}"
                );
            }
        }
    }
    for (what, big) in [("the lemon", -0.5), ("the apple", 0.4)] {
        placed::<f64>(&format!("{what} at f64"), big);
        placed::<Interval>(&format!("{what} at Interval"), big);
    }
}

/// `body`'s verdict door within a small budget, at `T`: a crossing
/// read `Crosses` where `crosses`, and never `Embedded` in any case.
/// Where the walk offers no pair whose stations coincide it refuses on
/// its budget: it probes a straddling pair only at a candidate's root
/// and at the depth floor
/// (`work/embedwalk/the-verdict-door-probes-a-crossing-only-at-a-candidates-root-and-the-depth-floor.md`).
fn crosses_at_both(what: &str, crosses: bool, float: &Body<f64>, interval: &Body<Interval>) {
    let budgets = Budgets {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: 1024,
    };
    for (scalar, report) in [
        ("f64", embedding_within(float, budgets)),
        ("Interval", embedding_within(interval, budgets)),
    ] {
        let verdict = &report.verdict;
        assert!(
            !matches!(verdict, topo::clearance::Embedding::Embedded),
            "{what} cross at {scalar}: {report:?}"
        );
        assert!(
            !crosses || matches!(verdict, topo::clearance::Embedding::Crosses(_)),
            "{what} read their crossing at {scalar}: {report:?}"
        );
    }
}

/// A football of `R = first` and a copy of `R = copy` built `by` up its
/// axis, grafted in as a solid of its own: nothing in M is shared.
fn footballs_through<T: Scalar>(first: f64, copy: f64, by: f64) -> Body<T> {
    let mut folded = football::<T>(first, 0.0);
    topo::graft_disjoint(&mut folded, &football::<T>(copy, by)).expect("the copy grafts");
    folded
}

/// **Two footballs through one another never read embedded**, at both
/// scalars and through the verdict door: the football and a copy built
/// `by` up its axis, so their sheets meet on a circle. Moved `0.3` the
/// circle is near the sheets' widest; moved `1.2` the lemons' circle is
/// at `v = ±0.64`, about `0.61 v*`, on the first's upper half and the
/// copy's lower. The lemons moved `1.2` read the crossing; the others
/// refuse on the budget. Red if any reads `Embedded`, or the lemons
/// moved `1.2` read anything but `Crosses`.
#[test]
fn footballs_through_one_another_never_read_embedded() {
    for (big, by, crosses) in [(-0.5, 0.3, false), (-0.5, 1.2, true), (0.4, 1.2, false)] {
        crosses_at_both(
            &format!("the footballs of R = {big} moved {by}"),
            crosses,
            &footballs_through(big, big, by),
            &footballs_through(big, big, by),
        );
    }
}

/// **Footballs crossing a hair from an apex never read embedded**, at
/// both scalars: a football and a copy moved up its axis by just short
/// of the sum of their apex heights, so their sheets meet on a small
/// circle beside the first's upper apex and the copy's lower one, at
/// `v` within a few thousandths of `±v*`. Each reads `Crosses`. Red if
/// a sheet's window falls short of its apexes by more than that.
#[test]
fn footballs_crossing_beside_an_apex_never_read_embedded() {
    let h = |big: f64| ((1.0 - big) * (1.0 + big)).sqrt();
    for (first, copy, short) in [(-0.5, -0.5, 1e-3), (-0.5, -0.9, 1e-3), (-0.9, -0.5, 1e-5)] {
        let by = (h(first) + h(copy)) * (1.0 - short);
        crosses_at_both(
            &format!("footballs of R = {first} and {copy} moved {by}"),
            true,
            &footballs_through(first, copy, by),
            &footballs_through(first, copy, by),
        );
    }
}

/// **A windowed tube holds across its caps' circles at both scalars**
/// (the tour's `tube_along_arc` wedge: `R = 2`, `r = 1/2`, the spine
/// from `1/4` to `7/4` rad). Each cap is a disc on two semicircles,
/// which meet where the two half-tube walls' seams land. A cap cell at
/// such a vertex has no presentation, so the pair is decided by the
/// cap's carrier against the wall alone, whose side on the shared
/// meridian is read off the meridian's closed form: the centred form's
/// error there shrinks only with the cell's length squared, and the
/// pair ran to the depth floor.
#[test]
fn a_windowed_tubes_caps_hold_against_its_walls_at_both_scalars() {
    fn tube<T: Scalar>() -> Body<T> {
        let at = T::from_f64;
        sweep::tube_along_arc(
            sweep::test_support::tube_frame(
                Point3::new(at(0.0), at(0.0), at(0.0)),
                geom_core::Vec3::new(at(0.0), at(1.0), at(0.0)),
                geom_core::Vec3::new(at(1.0), at(0.0), at(0.0)),
                tol(),
            ),
            at(2.0),
            sweep::TubeWindow::Arc {
                t0: at(0.25),
                t1: at(1.75),
            },
            at(0.5),
            tol(),
        )
        .expect("the wedge builds")
        .body
        .into_body()
    }
    holds_at_both("the windowed tube", tube(), tube());
}

/// **A stepped ring's floor holds against its end caps**: an L section
/// revolved three quarters of a turn about `y`. The step's floor is an
/// annulus, and its line edge on each end cap meets the floor's two rim
/// arcs. At the step's wall the cap is a reflex corner of two lines,
/// which no presentation reads, so the floor is presented through its
/// line–arc corner and the cap's plane separates it.
#[test]
fn a_stepped_rings_floor_holds_against_its_end_caps() {
    fn ring<T: Scalar>() -> Body<T> {
        let profile = validated::<T>(&[ProfileLoop::polygon(
            [
                (1.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.5, 1.0),
                (1.5, 0.5),
                (1.0, 0.5),
            ]
            .map(|(x, y)| Point2::new(x, y)),
        )]);
        let turn = Revolution::Partial(T::from_f64(1.5 * core::f64::consts::PI));
        revolve(&profile, axis_y(), turn, tol())
            .unwrap()
            .body
            .into_body()
    }
    holds_at_both("the stepped ring", ring(), ring());
}

/// The verdict door over every face of `body`, at the default budgets.
fn embedding<T: Scalar + geom_core::Certifies>(body: &Body<T>) -> topo::clearance::EmbeddingReport {
    embedding_within(
        body,
        Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: BUDGET,
        },
    )
}

/// The verdict door over every face of `body`, within `budgets`.
fn embedding_within<T: Scalar + geom_core::Certifies>(
    body: &Body<T>,
    budgets: Budgets,
) -> topo::clearance::EmbeddingReport {
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    topo::clearance::embedded(&side, band(), budgets)
}

/// [`a_stepped_rings_floor_holds_against_its_end_caps`]' ring, at rest.
fn stepped_ring<T: Scalar>() -> topo::AtRestBody<T> {
    let profile = validated::<T>(&[ProfileLoop::polygon(
        [
            (1.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.5, 1.0),
            (1.5, 0.5),
            (1.0, 0.5),
        ]
        .map(|(x, y)| Point2::new(x, y)),
    )]);
    let turn = Revolution::Partial(T::from_f64(1.5 * core::f64::consts::PI));
    sweep::test_support::taken::<T>(
        revolve(&profile, axis_y(), turn, tol()).unwrap().body,
        tol(),
    )
    .expect("the ring is a finished body")
}

/// **The stepped ring is embedded with its thin cells read, where it
/// stands, turned and placed far off.** Its annuli meet the end caps on
/// radial lines, and a cell across such a line from its annulus reaches
/// it by rounding: the annulus's part of the cell is a rectangle with no
/// width, or one of rounding's, presented on the line. The cap's plane
/// separates it, read at the reach of the two faces. The same body turned
/// rigidly about a generic axis, and placed a hundred units from the
/// origin, reads the same, at both scalars. Red if a rectangle's own
/// width is what its departure is levered at, in any pose.
#[test]
fn a_stepped_ring_is_embedded_with_its_thin_cells_read_in_every_pose() {
    use geom_core::{Affine3, Vec3};
    let axis = Vec3::new(1.0, 2.0, 3.0) * (1.0 / 14.0_f64.sqrt());
    let poses = [
        ("as built", Affine3::translation(Vec3::new(0.0, 0.0, 0.0))),
        (
            "turned about a generic axis",
            Affine3::rotation_about_axis(Point3::origin(), axis, 0.7),
        ),
        (
            "a hundred units off",
            Affine3::translation(Vec3::new(60.0, -80.0, 0.0)),
        ),
    ];
    for (pose, map) in poses {
        let float = moved(&stepped_ring::<f64>(), &map).into_body();
        let twin = moved(&stepped_ring::<Interval>(), &map).into_body();
        for (scalar, report) in [("f64", embedding(&float)), ("Interval", embedding(&twin))] {
            println!("[remeter] the stepped ring {pose} at {scalar}: {report:?}");
            assert!(
                matches!(report.verdict, topo::clearance::Embedding::Embedded),
                "the stepped ring {pose} at {scalar} is embedded: {report:?}"
            );
        }
    }
}

/// **Two stepped rings through one another cross.** The ring and a copy
/// moved `0.3` along `z`, grafted in as a solid of its own: the copy's
/// faces pass through the first ring's beside the radial lines whose
/// cells the row above reads, and its end caps lie in the first ring's.
/// The two solids share nothing in M, so nothing a departure reads can
/// part them. Red if the verdict is anything but a crossing, at either
/// scalar.
#[test]
fn two_stepped_rings_through_one_another_cross() {
    fn folded<T: Scalar>() -> Body<T> {
        use geom_core::{Affine3, Vec3};
        let ring = stepped_ring::<T>();
        let mut folded = ring.clone().into_body();
        let over = Affine3::translation(Vec3::new(0.0, 0.0, 0.3)).map(T::from_f64);
        let rigid = geom_core::test_support::rigid(&over, tol());
        let copy = topo::transform_rigid(&ring, &rigid).unwrap();
        topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
        folded
    }
    for (scalar, report) in [
        ("f64", embedding(&folded::<f64>())),
        ("Interval", embedding(&folded::<Interval>())),
    ] {
        println!("[remeter] two stepped rings at {scalar}: {report:?}");
        assert!(
            matches!(report.verdict, topo::clearance::Embedding::Crosses(_)),
            "the two rings cross at {scalar}: {report:?}"
        );
    }
}

/// **A quarter ball holds with its caps read at each pole**: a half
/// disc revolved a quarter turn about its diameter. Each cap is a lens,
/// the diameter and the semicircle meeting at both poles, so a cell
/// holding the whole cap is not presented, and one holding a single pole
/// is read through its corner there.
#[test]
fn a_quarter_balls_lens_caps_hold_read_at_each_pole() {
    fn lune<T: Scalar>() -> Body<T> {
        let p = |y: f64, b: f64| (Point2::new(0.0, y).map(T::from_f64), T::from_f64(b));
        revolved_about_y_at(
            vec![p(-0.3, 0.0), p(0.3, -1.0)],
            Revolution::Partial(T::from_f64(core::f64::consts::FRAC_PI_2)),
            tol(),
        )
    }
    holds_at_both("the quarter ball", lune(), lune());
}

/// **The two-arc lune** of `torax_axial`, at `T`: a half disc of radius
/// 0.3 revolved a quarter turn about its diameter, its arc split at the
/// latitude `phi` by a seam cut by hand (the revolve builds the run as one
/// wall), and, where `thick` is given, hollowed by it.
fn two_arc_lune<T: Scalar>(phi: f64, thick: Option<f64>) -> Body<T> {
    use crate::common::latitude_seam::latitude_on_partial_wall_at;
    let r = 0.3_f64;
    let quarter = -(core::f64::consts::PI / 8.0).tan();
    let at = |a: f64| Point2::new(r * a.cos(), r * a.sin());
    let arcs = if phi == 0.0 {
        vec![
            (Point2::new(0.0, -r), 0.0),
            (Point2::new(0.0, r), quarter),
            (at(0.0), quarter),
        ]
    } else {
        vec![(Point2::new(0.0, -r), 0.0), (Point2::new(0.0, r), -1.0)]
    };
    let profile = validated::<T>(&[profile::test_support::bulge_loop(arcs)]);
    let mut body = revolve(
        &profile,
        axis_y(),
        Revolution::Partial(T::from_f64(core::f64::consts::FRAC_PI_2)),
        tol(),
    )
    .expect("the lune revolves")
    .body
    .into_body();
    let sphere = body
        .faces()
        .find(|(fk, _)| matches!(body.face_surface(*fk), Ok(Surface::Sphere { .. })))
        .expect("the sphere wall")
        .1
        .surface;
    latitude_on_partial_wall_at(&mut body, sphere, at(phi), core::f64::consts::FRAC_PI_2);
    match thick {
        None => body,
        Some(t) => Body::clone(
            &topo::shell(
                &sweep::test_support::finished("the lune", body, tol()),
                T::from_f64(t),
                tol(),
            )
            .expect("the lune hollows")
            .body,
        ),
    }
}

/// **A two-arc lune's hollow holds, its cavity's sphere read in its
/// caps' poses**: a half disc whose arc is split at the equator,
/// revolved a quarter turn about its diameter, its equator seam cut by
/// hand (`torax_axial`'s two-arc lune), then hollowed by 0.05. The
/// cavity's caps stand 0.05 off the axis, in the planes `x = 0.05` and
/// `z = −0.05`, so each meets the cavity's sphere in a circle off the
/// sphere's axis, which its stored chart has no coordinate line for.
/// Each sphere face is read beside a cap's circle in the chart about
/// that cap's normal, where the circle is a parallel and the equator a
/// meridian. Red if no pose reads them: the cavity's sphere halves then
/// refuse `Uncertified` across the equator, and each cap against each
/// sphere half beside its circle.
#[test]
fn a_two_arc_lunes_hollow_holds_its_cavity_read_in_its_caps_poses() {
    use crate::common::latitude_seam::latitude_on_partial_wall;
    use profile::test_support::bulge_loop;
    let r = 0.3_f64;
    let quarter_arc = -(core::f64::consts::PI / 8.0).tan();
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(0.0, -r), 0.0),
            (Point2::new(0.0, r), quarter_arc),
            (Point2::new(r, 0.0), quarter_arc),
        ])],
    )
    .validate(tol())
    .expect("the two-arc lune's cross-section validates");
    let mut body = revolve(
        &profile,
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
    .expect("the two-arc lune revolves")
    .body
    .into_body();
    let sphere = body
        .faces()
        .find(|(fk, _)| matches!(body.face_surface(*fk), Ok(Surface::Sphere { .. })))
        .expect("the sphere wall")
        .1
        .surface;
    latitude_on_partial_wall(
        &mut body,
        sphere,
        Point2::new(r, 0.0),
        core::f64::consts::FRAC_PI_2,
    );
    let hollow = topo::shell(
        &sweep::test_support::finished("the lune", body, tol()),
        0.05,
        tol(),
    )
    .expect("the lune hollows")
    .body;
    let spheres = hollow
        .faces()
        .filter(|(k, _)| matches!(hollow.face_surface(*k), Ok(Surface::Sphere { .. })))
        .count();
    assert_eq!(spheres, 4, "two sphere halves outside, two in the cavity");
    let report = injective(&hollow, None, BUDGET);
    assert_holds("the two-arc lune's hollow", &report, &hollow);
}

/// **The two-arc lune holds at both scalars, its seam at the equator or
/// off it**: [`two_arc_lune`] solid, its arc split at the latitudes 0,
/// 0.4 and −0.3. Off the equator the seam is a small circle, a meridian
/// of neither cap's pose. Red if a lune refuses at either scalar, or
/// the two read different classes.
#[test]
fn a_two_arc_lune_holds_at_both_scalars() {
    for phi in [0.0, 0.4, -0.3] {
        holds_at_both(
            &format!("the lune split at {phi}"),
            two_arc_lune(phi, None),
            two_arc_lune(phi, None),
        );
    }
}

/// **The two-arc lune's hollow is built at `f64` only: at `Interval`
/// its shell refuses**, at its own certificate, `CavityUncertified` at
/// `Departure`. That is why the hollow's rows read `f64` alone, against
/// the module's rule (`work/embed/a-lunes-cavity-refuses-its-shell-at-interval.md`).
/// Red the day the hollow is built at `Interval`: its rows then read
/// both scalars.
#[test]
fn a_two_arc_lunes_hollow_is_refused_by_its_shell_at_interval() {
    let solid = two_arc_lune::<Interval>(0.0, None);
    let got = topo::shell(
        &sweep::test_support::finished("the lune", solid, tol()),
        Interval::from_f64(0.05),
        tol(),
    );
    assert!(
        matches!(
            got,
            Err(topo::ShellError::CavityUncertified {
                refusal: EngineRefusal::Uncertified {
                    arm: CertificateArm::Departure,
                    ..
                }
            })
        ),
        "the Interval shell refuses at Departure: {:?}",
        got.err()
    );
}

/// **The lune's hollow with its seam off the equator refuses across the
/// seam**: [`two_arc_lune`] split at the latitudes 0.4 and −0.3, then
/// hollowed by 0.05. The seam is a small circle, a parallel of the
/// stored chart and a meridian of neither cap's pose; where it meets a
/// cavity cap's circle, a parallel of that cap's pose alone, no chart
/// reads the corner, and the cavity's two sphere halves refuse
/// `Departure` across the seam. Measured: about 2,700 splits and 9 cells
/// refused, 3 s, at either latitude. `f64` alone, as
/// [`a_two_arc_lunes_hollow_is_refused_by_its_shell_at_interval`] says.
/// Red if the corner is read (the row then holds and is to be re-pinned)
/// or the refusal moves to another arm or off the seam.
#[test]
fn a_lunes_hollow_with_its_seam_off_the_equator_refuses_across_the_seam() {
    for phi in [0.4, -0.3] {
        let hollow = two_arc_lune::<f64>(phi, Some(0.05));
        let seams: Vec<topo::EdgeKey> = hollow
            .edges()
            .filter(|&(ek, _)| {
                topo::edge_faces(&hollow, ek)
                    .iter()
                    .all(|&f| matches!(hollow.face_surface(f), Ok(Surface::Sphere { .. })))
            })
            .map(|(ek, _)| ek)
            .collect();
        let report = injective(&hollow, None, BUDGET);
        assert!(
            matches!(
                report.verdict,
                BoundVerdict::Refused(EngineRefusal::Uncertified {
                    arm: CertificateArm::Departure,
                    shared: SharedCell::Edge(e),
                    ..
                }) if seams.contains(&e)
            ),
            "split at {phi}: Departure across a seam of two sphere halves: {:?}",
            report.verdict
        );
    }
}

/// The two-arc lune's hollow with a ball of radius 0.02 grafted into its
/// wall, the ball's sphere `depth` past the cavity's sphere (radius 0.25)
/// along `dir`: a crossing where `depth` is positive, a gap where it is
/// negative, a tangency at an interior point of the cavity's sphere face
/// at zero.
fn lune_with_a_ball(dir: [f64; 3], depth: f64) -> Body<f64> {
    let mut body = two_arc_lune::<f64>(0.0, Some(0.05));
    let n = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
    let rho = 0.02;
    let k = (0.25 + rho - depth) / n;
    let p = |y: f64, bulge: f64| (Point2::new(0.0, y), bulge);
    let ball = topo::transform_rigid(
        &revolved_about_y_at::<f64>(vec![p(-rho, 1.0), p(rho, 0.0)], Revolution::Full, tol()),
        &geom_core::RigidMotion::translation(geom_core::Vec3::new(
            dir[0] * k,
            dir[1] * k,
            dir[2] * k,
        ))
        .expect("a translation"),
    )
    .expect("the ball moves");
    topo::graft_disjoint_all_keyed(&mut body, &ball).expect("the ball grafts");
    body
}

/// A direction onto the cavity's sphere at `(x, y)`, below the equator
/// plane `z = 0`.
fn onto_the_cavity(x: f64, y: f64) -> [f64; 3] {
    [x, y, -(0.0625_f64 - x * x - y * y).sqrt()]
}

/// **A ball crossing the lune's posed cavity sphere is never read
/// embedded**: [`lune_with_a_ball`] just past the cavity cap `x = 0.05`,
/// off the equator, where the cavity's sphere is read in the cap's pose,
/// crossing by 5e-2, 1e-3 and 1e-6; and straddling the cap's circle
/// itself, by 1e-2 and 1e-4; and touching it, tangent at an interior
/// point of the sphere face. The ask ends at its first finding, so a
/// placement is told the crossing or a refusal met before it. Measured
/// at ε = 1e-9, 1e-6 and 1e-12: the straddle by 1e-2 reads `Violated`
/// at each, and so does the crossing by 5e-2 except at 1e-6; the
/// shallower crossings, the straddle by 1e-4 and the touch run out of
/// depth on the ball's face against the cavity's first. Red if a pose
/// reads a crossing or a touch embedded, or if no placement is told the
/// crossing, which would say no pose reads one.
#[test]
fn a_ball_crossing_or_touching_the_lunes_posed_cavity_sphere_reads_violated() {
    let near = onto_the_cavity(0.06, 0.1);
    let on_circle = onto_the_cavity(0.05, 0.1);
    let mut told = 0;
    for (dir, depth) in [
        (near, 5e-2),
        (near, 1e-3),
        (near, 1e-6),
        (on_circle, 1e-2),
        (on_circle, 1e-4),
        (near, 0.0),
    ] {
        let body = lune_with_a_ball(dir, depth);
        let report = injective(&body, None, BUDGET);
        println!("{dir:?} by {depth:e}: {:?}", report.verdict);
        assert!(
            matches!(
                report.verdict,
                BoundVerdict::Violated(_) | BoundVerdict::Refused(_)
            ),
            "{dir:?} by {depth:e}: {:?}",
            report.verdict
        );
        told += usize::from(matches!(report.verdict, BoundVerdict::Violated(_)));
    }
    assert!(told > 0, "some placement is told the crossing");
}

/// **A ball clear of the lune's posed cavity sphere holds**:
/// [`lune_with_a_ball`] just past the cavity cap `x = 0.05`, short of the
/// cavity's sphere by 5e-3 and 1e-4. Red if a pose's extent or touch
/// drops a gap.
#[test]
fn a_ball_clear_of_the_lunes_posed_cavity_sphere_holds() {
    for gap in [5e-3, 1e-4] {
        let body = lune_with_a_ball(onto_the_cavity(0.06, 0.1), -gap);
        let report = injective(&body, None, BUDGET);
        assert_holds(&format!("a gap of {gap:e}"), &report, &body);
    }
}

/// **A ball less two tilted half-spaces holds, its larger cap read in a
/// pose**: the unit ball less the slabs above `z = 0.3` turned about `x`
/// by 0.5 and by −0.9, whose circles meet on the sphere. The boolean
/// charts the sphere face about one circle's axis, so the other is off
/// it, and the face — the sphere less both small caps — holds a pole of
/// that circle's pose: the circle lifts to an arc of a parallel, the
/// loop wraps the azimuth, and the pose grants the face no extent
/// ([`topo::test_support::posed_extents_holding`]); its cells are read on
/// the rectangle's own sheet. Red if a pose grants the face an extent,
/// or the ball refuses.
#[test]
fn a_ball_less_two_tilted_half_spaces_holds_its_larger_cap_read_in_a_pose() {
    let p = |y: f64, bulge: f64| (Point2::new(0.0, y), bulge);
    let fin = |b| sweep::test_support::finished("the cut", b, tol());
    let mut body =
        revolved_about_y_at::<f64>(vec![p(-1.0, 1.0), p(1.0, 0.0)], Revolution::Full, tol());
    for angle in [0.5, -0.9] {
        let slab = lune_moved(
            &sweep::test_support::brick((-3.0, 3.0), (-3.0, 3.0), (0.3, 3.0), tol()),
            [1.0, 0.0, 0.0],
            angle,
            [0.0; 3],
        );
        let cut = topo::boolean::subtract(&fin(body), &fin(slab), tol()).expect("the ball is cut");
        body = Body::clone(&cut.body().expect("a body").body);
    }
    let cap = body
        .faces()
        .find(|(k, _)| matches!(body.face_surface(*k), Ok(Surface::Sphere { .. })))
        .expect("the sphere face")
        .0;
    let poses = topo::test_support::posed_extents_holding(&body, cap, [1.0, 0.0, 0.0], band())
        .expect("the face lifts");
    assert!(
        !poses.is_empty() && poses.iter().all(Option::is_none),
        "a pose about an off-axis circle, granting no extent: {poses:?}"
    );
    assert_holds("the larger cap", &injective(&body, None, BUDGET), &body);
}

/// **A lune's cap pushed through another's reads `Violated`**: two
/// two-arc lune hollows in one body, the second moved 0.1 along `x`, so
/// its cap `z = 0` runs through the first's cavity and wall. Within one
/// lune neither cap can be pushed through the other: a revolve past a
/// turn is refused (`FullRangeAngle`) and the shell builds no cavity
/// whose caps cross. Red if the crossing reads embedded.
#[test]
fn a_lunes_cap_pushed_through_anothers_reads_violated() {
    let hollow = two_arc_lune::<f64>(0.0, Some(0.05));
    let mut body = Body::clone(&hollow);
    let other = lune_moved(&hollow, [0.0, 1.0, 0.0], 0.0, [0.1, 0.0, 0.0]);
    topo::graft_disjoint_all_keyed(&mut body, &other).expect("the lunes graft");
    let report = injective(&body, None, BUDGET);
    assert!(
        matches!(report.verdict, BoundVerdict::Violated(_)),
        "the caps cross: {:?}",
        report.verdict
    );
}

/// **A sphere face's poses are built once per ask**: the walk over the
/// two-arc lune's hollow reads thousands of cells of its cavity's sphere
/// halves in their caps' poses, and each face builds each pose's lifted
/// boundary, its extent included, once (`Windows::posed_builds`): no more
/// than the two cap circles' poses per face, whatever the cells. Red if
/// a pose is rebuilt per cell, as it was when a cell's store held it.
#[test]
fn a_sphere_faces_poses_are_built_once_per_ask() {
    let body = two_arc_lune::<f64>(0.0, Some(0.05));
    let side = Side {
        body: &body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    let windows = side.windows(Which::First).expect("every face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: BUDGET,
        },
    };
    let report = sweep.run(
        &windows,
        &windows,
        true,
        verify(&body, ClearanceBound::StrictlyPositive),
    );
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "{:?}",
        report.verdict
    );
    let builds = windows.posed_builds();
    assert!(
        builds.iter().all(|&(_, n)| n <= 2),
        "each face builds each pose once: {builds:?} over {} cells discharged",
        report.receipt.discharged
    );
    assert!(
        builds.iter().filter(|&&(_, n)| n > 0).count() >= 2,
        "the cavity's sphere halves are posed: {builds:?}"
    );
}

/// Two unit balls `dx` apart along `x`, intersected: a lens of two
/// spheres meeting in a circle about the `x` axis, off both spheres'
/// stored axis `y`.
fn two_ball_lens<T: Scalar>(dx: f64) -> Body<T> {
    let p = |y: f64, bulge: f64| (Point2::new(0.0, y).map(T::from_f64), T::from_f64(bulge));
    let ball =
        || revolved_about_y_at::<T>(vec![p(-1.0, 1.0), p(1.0, 0.0)], Revolution::Full, tol());
    let moved = topo::transform_rigid(
        &ball(),
        &geom_core::RigidMotion::translation(geom_core::Vec3::new(
            T::from_f64(dx),
            T::zero(),
            T::zero(),
        ))
        .expect("a translation"),
    )
    .expect("the ball moves");
    let fin = |b| sweep::test_support::finished("the ball", b, tol());
    let lens = topo::boolean::intersect(&fin(ball()), &fin(moved), tol()).expect("the balls meet");
    Body::clone(&lens.body().expect("a body").body)
}

/// **A lens of two spheres holds at both scalars, read in its circle's
/// poses**: [`two_ball_lens`] at `dx` 0.7, 1 and 1.3. Each sphere face is
/// bounded by the circle about `x`, which neither sphere's stored chart
/// reads as a line, so its cells beside the circle are read in the
/// circle's pose. At `Interval` the row shows the walk reaches a pose:
/// its windows build one (`Windows::posed_builds` above zero), which
/// says a cell asked for it, not which cells a pose answered. The one
/// body of these rows that reaches a pose at the `Interval` scalar. Red
/// if either scalar refuses, they differ in class, or the `Interval`
/// walk builds no pose.
#[test]
fn a_lens_of_two_spheres_holds_at_both_scalars_read_in_its_poses() {
    for dx in [0.7, 1.0, 1.3] {
        holds_at_both(
            &format!("the lens {dx} apart"),
            two_ball_lens(dx),
            two_ball_lens(dx),
        );
        let body = two_ball_lens::<Interval>(dx);
        let side = Side {
            body: &body,
            faces: body.faces().map(|(k, _)| k).collect(),
        };
        let windows = side.windows(Which::First).expect("every face has a window");
        let sweep = Sweep {
            bound: ClearanceBound::StrictlyPositive,
            band: band(),
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: BUDGET,
            },
        };
        let _ = sweep.run(
            &windows,
            &windows,
            true,
            verify(&body, ClearanceBound::StrictlyPositive),
        );
        let built: usize = windows.posed_builds().iter().map(|&(_, n)| n).sum();
        assert!(
            built > 0,
            "the lens {dx} apart: the Interval walk builds a pose"
        );
    }
}

/// The six turns the two-arc lune's hollow was refused under before a
/// pose granted its extent: about `y` by 0.1, 0.7 and 2.0, about `x` by
/// 0.3, about `(1, 1, 1)` by 1.0 and about `z` by 1.2.
const LUNE_TURNS: [([f64; 3], f64); 6] = [
    ([0.0, 1.0, 0.0], 0.1),
    ([0.0, 1.0, 0.0], 0.7),
    ([0.0, 1.0, 0.0], 2.0),
    ([1.0, 0.0, 0.0], 0.3),
    ([1.0, 1.0, 1.0], 1.0),
    ([0.0, 0.0, 1.0], 1.2),
];

/// `body` turned about `axis` by `angle`, then moved by `by`.
fn lune_moved(body: &Body<f64>, axis: [f64; 3], angle: f64, by: [f64; 3]) -> Body<f64> {
    let a = geom_core::UnitVec3::new(
        geom_core::Vec3::new(axis[0], axis[1], axis[2]),
        "the lune's turn",
        band(),
    )
    .expect("a turn axis");
    let turned = topo::transform_rigid(
        body,
        &geom_core::RigidMotion::rotation(a, angle).expect("a turn"),
    )
    .expect("the lune turns");
    topo::transform_rigid(
        &turned,
        &geom_core::RigidMotion::translation(geom_core::Vec3::new(by[0], by[1], by[2]))
            .expect("a translation"),
    )
    .expect("the lune moves")
}

/// **Every face point a sphere face's pose grants an extent for lies in
/// it**: points of the face's sphere along a Fibonacci spiral of 2,000
/// directions, those [`topo::curved_face_containment`] places inside the
/// face, each read in every pose that grants the face an extent
/// ([`topo::test_support::posed_extents_holding`]). Panics on a point an
/// extent leaves out; returns how many (point, pose) readings it made, so
/// a row that granted nothing is seen to.
fn posed_extents_hold_its_points(what: &str, body: &Body<f64>) -> usize {
    let mut read = 0;
    for (fk, _) in body.faces() {
        let Ok(Surface::Sphere { frame, radius }) = body.face_surface(fk) else {
            continue;
        };
        let center = frame.origin();
        let n = 2000;
        for i in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let (z, t) = (
                1.0 - (2.0 * i as f64 + 1.0) / n as f64,
                i as f64 * core::f64::consts::PI * (3.0 - 5.0_f64.sqrt()),
            );
            let w = (1.0 - z * z).sqrt();
            let q = [
                center.x + radius * w * t.cos(),
                center.y + radius * w * t.sin(),
                center.z + radius * z,
            ];
            let inside = topo::curved_face_containment(
                body,
                fk,
                geom_core::Point3::new(q[0], q[1], q[2]),
                band(),
            );
            if !matches!(inside, Ok(Some(topo::FaceContainment::In))) {
                continue;
            }
            let holds = topo::test_support::posed_extents_holding(body, fk, q, band())
                .unwrap_or_else(|| panic!("{what}: face {fk:?} lifts"));
            for (j, h) in holds.into_iter().enumerate() {
                match h {
                    Some(true) => read += 1,
                    Some(false) => panic!(
                        "{what}: face {fk:?}'s pose {j} grants an extent leaving out its point {q:?}"
                    ),
                    None => {}
                }
            }
        }
    }
    read
}

/// **The two-arc lune's hollow holds turned six ways, and its poses'
/// extents hold its faces**: the hollow of
/// [`a_two_arc_lunes_hollow_holds_its_cavity_read_in_its_caps_poses`]
/// turned by each of [`LUNE_TURNS`], each of which it refused under
/// before a pose granted its extent (`Uncertified` at the root, a cell
/// beside a cap's circle read `Local` at every depth). The extent is what
/// reads the cells beside the circle from outside the face `Missed`. Red
/// if a pose's extent is not granted, leaves out a face point, or a turn
/// refuses.
#[test]
fn a_two_arc_lunes_hollow_holds_turned_six_ways() {
    let hollow = two_arc_lune::<f64>(0.0, Some(0.05));
    assert!(
        posed_extents_hold_its_points("the hollow", &hollow) > 0,
        "the hollow's poses grant an extent"
    );
    for (axis, angle) in LUNE_TURNS {
        let what = format!("the hollow turned about {axis:?} by {angle}");
        let body = lune_moved(&hollow, axis, angle, [0.0; 3]);
        assert!(
            posed_extents_hold_its_points(&what, &body) > 0,
            "{what}: an extent is granted"
        );
        assert_holds(&what, &injective(&body, None, BUDGET), &body);
    }
}

/// **The two-arc lune hollowed after it is turned holds**: the solid
/// lune turned about `(1, 1, 1)` by 1.0, then hollowed, so the cavity's
/// caps and circles are built in the turned pose rather than carried
/// into it; the certificate's analogue of re-posing the shell's input.
/// Red if the cavity built off every axis plane refuses where the
/// carried one holds.
#[test]
fn a_two_arc_lune_turned_then_hollowed_holds() {
    let solid = lune_moved(
        &two_arc_lune::<f64>(0.0, None),
        [1.0, 1.0, 1.0],
        1.0,
        [0.0; 3],
    );
    let hollow = Body::clone(
        &topo::shell(
            &sweep::test_support::finished("the turned lune", solid, tol()),
            0.05,
            tol(),
        )
        .expect("the turned lune hollows")
        .body,
    );
    assert!(posed_extents_hold_its_points("the re-posed hollow", &hollow) > 0);
    assert_holds(
        "the re-posed hollow",
        &injective(&hollow, None, BUDGET),
        &hollow,
    );
}

/// **The two-arc lune's hollow holds under a seeded sweep of rigid
/// motions**: 24 draws of a turn about a uniform axis by a uniform angle
/// and a move within 2 of the origin, each read for its poses' extents
/// and certified. Pinned; `CAD_FUZZ_SEED` explores. Red on any draw that
/// refuses or whose extent leaves out a face point.
#[test]
fn a_two_arc_lunes_hollow_holds_moved_at_random() {
    let hollow = two_arc_lune::<f64>(0.0, Some(0.05));
    let mut rng = test_utils::fuzz::pinned_sweep("the lune's rigid motions", 0x6c75_6e65);
    for draw in 0..test_utils::fuzz::scaled(24) {
        let axis = loop {
            let a = [0; 3].map(|_| rng.range(-1.0, 1.0));
            let n = a.iter().map(|x| x * x).sum::<f64>();
            if (0.01..=1.0).contains(&n) {
                break a;
            }
        };
        let angle = rng.range(0.0, TAU);
        let by = [0; 3].map(|_| rng.range(-2.0, 2.0));
        let what = format!(
            "draw {draw}: turned about {axis:?} by {angle}, moved by {by:?} ({})",
            test_utils::fuzz::replay()
        );
        let body = lune_moved(&hollow, axis, angle, by);
        posed_extents_hold_its_points(&what, &body);
        assert_holds(&what, &injective(&body, None, BUDGET), &body);
    }
}

/// Balls of radius `r` about `c`, united in turn.
fn balls_united(balls: &[(f64, [f64; 3])]) -> Body<f64> {
    let ball = |r: f64, c: [f64; 3]| {
        let p = |y: f64, bulge: f64| (Point2::new(0.0, y), bulge);
        let b = revolved_about_y_at::<f64>(vec![p(-r, 1.0), p(r, 0.0)], Revolution::Full, tol());
        topo::transform_rigid(
            &b,
            &geom_core::RigidMotion::translation(geom_core::Vec3::new(c[0], c[1], c[2]))
                .expect("a translation"),
        )
        .expect("the ball moves")
    };
    let fin = |b| sweep::test_support::finished("the ball", b, tol());
    balls[1..]
        .iter()
        .fold(ball(balls[0].0, balls[0].1), |cur, &(r, c)| {
            Body::clone(
                &topo::boolean::union(&fin(cur), &fin(ball(r, c)), tol())
                    .expect("the balls unite")
                    .body()
                    .expect("a body")
                    .body,
            )
        })
}

/// The unit ball meeting every half-space `p·n ≥ d`, `n` the `z` axis
/// turned about `x` by `a` and then about `y` by `spin`, for each
/// `(d, a, spin)`.
fn ball_in_half_spaces(slabs: &[(f64, f64, f64)]) -> Body<f64> {
    let p = |y: f64, bulge: f64| (Point2::new(0.0, y), bulge);
    let fin = |b| sweep::test_support::finished("the cut", b, tol());
    let ball = revolved_about_y_at::<f64>(vec![p(-1.0, 1.0), p(1.0, 0.0)], Revolution::Full, tol());
    slabs.iter().fold(ball, |cur, &(d, a, spin)| {
        let slab = lune_moved(
            &lune_moved(
                &sweep::test_support::brick((-3.0, 3.0), (-3.0, 3.0), (d, 3.0), tol()),
                [1.0, 0.0, 0.0],
                a,
                [0.0; 3],
            ),
            [0.0, 1.0, 0.0],
            spin,
            [0.0; 3],
        );
        Body::clone(
            &topo::boolean::intersect(&fin(cur), &fin(slab), tol())
                .expect("the ball is cut")
                .body()
                .expect("a body")
                .body,
        )
    })
}

/// **Sphere bodies off each other's axes hold within a small walk**:
/// five unions of balls whose waists are circles off both spheres' axes,
/// and a spherical triangle and a lens cut from the unit ball by tilted
/// half-spaces. Each holds, within a ceiling of its measured split count
/// (measured + 25% + 8). On main, where each full revolve's ball is two
/// π-bands, each union and the triangle refuse `Departure` at once: a
/// ball's cell holds a line of its stored chart with no place at any
/// depth (the two bands' shared meridian). Main's counts, at
/// `704ed655`'s merge base, were 0, 253, 0, 259 and 275 for the unions,
/// 536 for the triangle and 115 for the lens. A full revolve's ball is
/// one band, with no meridian two of its faces share, and the walk
/// proves each body. Red if any refuses, or costs more than its ceiling.
#[test]
fn off_axis_sphere_bodies_hold_within_a_small_walk() {
    type Balls = Vec<(f64, [f64; 3])>;
    let unions: [(&str, Balls, usize); 5] = [
        (
            "head x 0.1",
            vec![(1.0, [0.0; 3]), (0.8, [0.1, 1.4, 0.0])],
            129,
        ),
        (
            "head (0.2, 0.15)",
            vec![(1.0, [0.0; 3]), (0.8, [0.2, 1.4, 0.15])],
            223,
        ),
        (
            "small head x 0.3",
            vec![(1.0, [0.0; 3]), (0.4, [0.3, 1.1, 0.0])],
            174,
        ),
        (
            "three balls",
            vec![
                (1.0, [0.0; 3]),
                (0.8, [0.3, 1.4, 0.0]),
                (0.5, [0.5, 2.3, 0.1]),
            ],
            335,
        ),
        (
            "side by side",
            vec![(1.0, [0.0; 3]), (0.9, [1.5, 0.2, 0.1])],
            223,
        ),
    ];
    let ceiling = |measured: usize| measured + measured / 4 + 8;
    let holds = |what: &str, r: &BoundReport<Witness>, measured: usize| {
        assert!(
            matches!(r.verdict, BoundVerdict::Holds),
            "{what}: holds: {:?}",
            r.verdict
        );
        assert!(
            r.receipt.splits <= ceiling(measured),
            "{what}: {} splits, measured {measured}: {:?}",
            r.receipt.splits,
            r.receipt
        );
    };
    for (what, balls, measured) in unions {
        holds(
            what,
            &injective(&balls_united(&balls), None, BUDGET),
            measured,
        );
    }
    holds(
        "the triangle",
        &injective(
            &ball_in_half_spaces(&[(0.2, 0.9, 0.0), (0.2, 0.9, 2.1), (0.2, 0.9, -2.1)]),
            None,
            BUDGET,
        ),
        260,
    );
    holds(
        "the lens",
        &injective(
            &ball_in_half_spaces(&[(0.3, 0.9, 0.0), (0.3, -0.9, 0.0)]),
            None,
            BUDGET,
        ),
        107,
    );
}

/// **A filleted cube corner's ball holds against the faces it touches
/// at its pole**: the unit cube with the three edges at `(1, 1, 1)`
/// filleted. The corner's sphere is charted with a pole at the vertex
/// it shares with one face alone, tangent to it there, so neither
/// carrier separates the pair; a plane through the vertex does, the
/// sphere read across the pole as a collapsed side of its chart.
#[test]
fn a_filleted_corners_ball_holds_against_a_face_at_its_pole() {
    fn corner<T: Scalar>() -> Body<T> {
        let cube = sweep::test_support::cube::<T>(1.0, tol());
        let at_corner: Vec<topo::EdgeKey> = cube
            .edges()
            .map(|(k, _)| k)
            .filter(|&e| {
                let (s, t) = cube.edge_vertices(e).unwrap();
                [s, t].into_iter().any(|v| {
                    let p = *cube.get_point(cube.get_vertex(v).unwrap().point).unwrap();
                    [p.x, p.y, p.z]
                        .into_iter()
                        .all(|c| (mid(c) - 1.0).abs() < 1e-9)
                })
            })
            .collect();
        assert_eq!(at_corner.len(), 3, "three edges meet at the corner");
        sweep::blend::fillet_edges(
            &sweep::test_support::at_rest(&cube, tol()),
            &at_corner,
            T::from_f64(0.25),
            tol(),
        )
        .map_err(|e| e.error)
        .expect("the corner fillets")
        .body
        .into_body()
    }
    holds_at_both("the filleted corner", corner(), corner());
    let body: &Body<f64> = &corner::<f64>();
    let ball = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| kind(body, k) == "sphere")
        .expect("the corner's ball");
    let alone: Vec<FaceKey> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            kind(body, k) == "plane" && touching_pairs(body).contains(&(k.min(ball), k.max(ball)))
        })
        .collect();
    assert_eq!(
        alone.len(),
        3,
        "the ball touches the three faces at its corners"
    );
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
        sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, 65, 3, tol())
            .expect("the helix sweeps")
            .body
    }
    holds_at_both("the helical sweep", helical::<f64>(), helical::<Interval>());
}

// ------------------------------------- walls one half of an axis spans

/// A loft face's role: `top`, `bottom`, or `wall j` of the outer loop.
fn role<T: topo::AtRestPolicy<Finished = topo::AtRestBody<T>> + Real>(
    lofted: &sweep::Lofted<T, Body<T>>,
    f: FaceKey,
) -> String {
    if f == lofted.top {
        return "top".into();
    }
    if f == lofted.bottom {
        return "bottom".into();
    }
    lofted.side_faces[0]
        .iter()
        .position(|seg| seg.iter().flatten().any(|w| *w == Some(f)))
        .map_or_else(|| format!("{f:?}"), |j| format!("wall {j}"))
}

/// **The body's every face has a window, and the sweep reads one
/// verdict class at `f64` and at `Interval`** within `budget` pairs:
/// the `f64` report, for the row's pins.
fn decided_at_both(
    what: &str,
    float: &sweep::Lofted<f64, Body<f64>>,
    interval: &sweep::Lofted<Interval, Body<Interval>>,
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

/// A 0.1 square lofted at `f64` along [`arc_spine_places`] at 17
/// stations, `v`-degree 3, its carriers held at `T` as points
/// ([`sweep::test_support::f64_body_held_as_points_construction`]).
fn arc_spine<T: Scalar>(curl: f64) -> sweep::Lofted<T, Body<T>> {
    let square = quad([(-0.05, -0.05), (0.05, -0.05), (0.05, 0.05), (-0.05, 0.05)]);
    let sections: Vec<_> = (0..17).map(|_| square.clone()).collect();
    sweep::test_support::f64_body_held_as_points_construction::<T>(
        &sections,
        &arc_spine_places(curl, 17),
        3,
        tol(),
    )
    .expect("the arc spine body builds")
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
    fn closed<T: Scalar>() -> sweep::Lofted<T, Body<T>> {
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
        sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, 9, 3, tol())
            .expect("the closed spine sweeps")
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
    fn duct<T: Scalar>() -> sweep::Lofted<T, Body<T>> {
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
        sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, 13, 3, tol())
            .expect("the S path sweeps")
    }
    let float = duct::<f64>();
    let walls: Vec<FaceKey> = float
        .side_faces
        .iter()
        .flatten()
        .flatten()
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

/// **Two nested tori a thin gap apart hold at their roots.** The hollow
/// torus `R = 2`, tube radius `0.45`, wall `0.025`, as a shelled torus's
/// cavity is: four faces, each of the inner torus's two sharing nothing
/// with each of the outer's. Each of those four pairs is read apart
/// off the outer torus's whole carrier, so what is split is the two
/// pairs of halves that touch. Red where a pair that shares nothing is
/// separated by the cells' boxes and supports alone: the four spend the
/// default budget between them, and the body refuses for pairs.
#[test]
fn two_nested_tori_a_thin_gap_apart_hold_at_their_roots() {
    fn hollow<T: Scalar>() -> Body<T> {
        let at = T::from_f64;
        sweep::tube_along_arc_hollow(
            sweep::test_support::tube_frame(
                Point3::new(at(0.0), at(0.0), at(0.0)),
                geom_core::Vec3::new(at(0.0), at(1.0), at(0.0)),
                geom_core::Vec3::new(at(1.0), at(0.0), at(0.0)),
                tol(),
            ),
            at(2.0),
            sweep::TubeWindow::Full,
            at(0.45),
            at(0.025),
            tol(),
        )
        .expect("the hollow torus builds")
        .body
        .into_body()
    }
    let float = hollow::<f64>();
    assert_eq!(float.faces().count(), 4, "two tori, each in two halves");
    for report in holds_at_both("the nested tori", float, hollow::<Interval>()) {
        assert!(
            report.receipt.splits < BUDGET / 16,
            "the nested tori split under a sixteenth of the default budget: {:?}",
            report.receipt
        );
    }
}

/// **A conical wall and a toroidal wall `0.05` thick**, each revolved a
/// full turn: the meridian band between two lines `30°` off the axis
/// and `0.05` apart, capped by two annuli, as the klein bulb's flare;
/// and the band between two arcs about a centre off the axis, radii
/// `0.8 ∓ 0.025`, closed by two radial lines, as its rim. Each wall's
/// two faces share nothing. Both hold at both scalars, each splitting
/// under a quarter of the default budget. The conical wall's faces
/// have no depth that parts them, so the supports along a direction
/// decide them, in about 3,100 splits: red where the support test reads
/// an analytic cell by its box, since two slanted patches' boxes
/// overlap until the cells are about as small as the gap, which costs
/// about 39,000. The toroidal wall's faces are apart in their core's
/// depth and discharge at their root; it splits about 200 times, on
/// the pairs that touch, and under 512: red where that root is split
/// (6,647 splits).
#[test]
fn a_conical_and_a_toroidal_wall_hold_within_the_default_budget() {
    fn conical<T: Scalar>() -> Body<T> {
        let (s, c) = (30f64.to_radians().sin(), 30f64.to_radians().cos());
        let (slant, across) = (1.6, 0.05 / c);
        let p = |x: f64, y: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
        revolved_about_y_at(
            vec![
                p(0.4, 2.0),
                p(0.4 + slant * s, 2.0 - slant * c),
                p(0.4 + across + slant * s, 2.0 - slant * c),
                p(0.4 + across, 2.0),
            ],
            Revolution::Full,
            tol(),
        )
    }
    fn toroidal<T: Scalar>() -> Body<T> {
        let centre = Point2::new(1.6, 1.0);
        let at = |r: f64, a: f64| {
            let a = a.to_radians();
            Point2::new(centre.x + r * a.cos(), centre.y + r * a.sin())
        };
        let (outer, inner) = (0.825, 0.775);
        let (o0, o1, i1, i0) = (
            at(outer, 120.0),
            at(outer, 240.0),
            at(inner, 240.0),
            at(inner, 120.0),
        );
        let p = |q: Point2<f64>, b: f64| (q.map(T::from_f64), T::from_f64(b));
        revolved_about_y_at(
            vec![
                p(o0, bulge(o0, o1, centre)),
                p(o1, 0.0),
                p(i1, bulge(i1, i0, centre)),
                p(i0, 0.0),
            ],
            Revolution::Full,
            tol(),
        )
    }
    for (what, ceiling, reports) in [
        (
            "the conical wall",
            BUDGET / 4,
            holds_at_both("the conical wall", conical::<f64>(), conical::<Interval>()),
        ),
        (
            "the toroidal wall",
            512,
            holds_at_both(
                "the toroidal wall",
                toroidal::<f64>(),
                toroidal::<Interval>(),
            ),
        ),
    ] {
        for report in &reports {
            assert!(
                report.receipt.splits < ceiling,
                "{what} splits under {ceiling}: {:?}",
                report.receipt
            );
        }
    }
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
            .find(|&(k, _)| {
                let ends = <[_; 2]>::from(body.edge_vertices(k).unwrap());
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
        Surface::Sphere { .. } => "sphere",
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

        let drum = sweep::test_support::taken::<T>(
            extrude(&disc::<T>(), to_offset(2.0), tol()).unwrap().body,
            tol(),
        )
        .expect("the drum is a finished body");
        let rim: Vec<topo::EdgeKey> = drum
            .edges()
            .filter(|(_, e)| {
                matches!(
                    drum.get_curve_geom(e.curve).unwrap().certified().map(|g| g.carrier().clone()),
                    Some(geom::Curve3::Circle(the_circle @ geom::Circle { .. })) if (mid(the_circle.frame().origin().z) - 2.0).abs() < 1e-9
                )
            })
            .map(|(k, _)| k)
            .collect();
        fillet_edges(&drum, &rim, T::from_f64(0.3), tol())
            .map_err(|e| e.error)
            .expect("the rim fillets")
            .body
            .into_body()
    }
    let body: &Body<f64> = &filleted::<f64>();
    let report = injective(body, None, BUDGET);
    assert_holds("the filleted drum", &report, body);
    // At `Interval` too: the band's window is its own chart rectangle,
    // so no cell past the band's end reaches the circle it shares with the
    // top plane.
    let interval_body = filleted::<Interval>();
    let twin = injective(&interval_body, None, BUDGET);
    assert_holds("the filleted drum at Interval", &twin, &interval_body);
    let torus = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| kind(body, k) == "torus")
        .expect("a torus band");
    for (k, _) in body.faces() {
        if matches!(kind(body, k), "cylinder")
            || (kind(body, k) == "plane" && {
                let Surface::Plane { frame, .. } = surface(body, k) else {
                    unreachable!()
                };
                let (origin, normal) = (frame.origin(), frame.axis());
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

/// **The three-arc disc, its top rim filleted**: the disc of radius
/// `0.5` and height `1` whose circle is authored as three equal arcs,
/// its top rim filleted at `0.1`. The band is one torus face whose foot
/// parallel is three arcs, joined at the wall's three seams, at azimuths
/// `0`, `2π/3` and `4π/3`. `Err` where the fillet refuses it, which at
/// `Interval` and ε = 1e-12 it does (its pcurve envelope escalates).
fn filleted_arc_disc<T: Scalar>() -> Result<topo::AtRestBody<T>, String> {
    use sweep::blend::fillet_edges;
    use sweep::test_support::{arc_polygon, finished, prism_at};
    let at = T::from_f64;
    let verts = arc_polygon(3, 0.5, Point2::new(0.0, 0.0))
        .into_iter()
        .map(|(p, bulge)| (p.map(at), at(bulge)))
        .collect();
    let disc = finished(
        "the disc",
        prism_at(verts, T::zero(), T::one(), tol()),
        tol(),
    );
    let rim: Vec<topo::EdgeKey> = disc
        .edges()
        .filter(|(_, e)| {
            matches!(
                disc.get_curve_geom(e.curve).unwrap().certified().map(|g| g.carrier().clone()),
                Some(geom::Curve3::Circle(the_circle @ geom::Circle { .. })) if (mid(the_circle.frame().origin().z) - 1.0).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(rim.len(), 3, "the rim is three arcs");
    fillet_edges(&disc, &rim, at(0.1), tol())
        .map(|blended| blended.body)
        .map_err(|e| format!("{:?}", e.error))
}

/// A pose of [`filleted_arc_disc`].
#[derive(Clone, Copy)]
enum Pose {
    AsBuilt,
    /// Moved rigidly.
    Moved(geom_core::Affine3<f64>),
    /// Mirrored across the plane through a point with a normal.
    Mirrored([f64; 3], [f64; 3]),
}

/// [`filleted_arc_disc`] in `pose`; `Err` where it, or its pose, is not
/// built.
fn posed_arc_disc<T: Scalar>(pose: Pose) -> Result<Body<T>, String> {
    let home = filleted_arc_disc::<T>()?;
    let at = T::from_f64;
    match pose {
        Pose::AsBuilt => Ok(home.into_body()),
        Pose::Moved(m) => {
            let rigid = geom_core::test_support::rigid(&m.map(at), tol());
            T::finish(topo::transform_rigid(&home, &rigid).unwrap(), tol())
                .map(topo::AtRestBody::into_body)
                .map_err(|e| format!("{e:?}"))
        }
        Pose::Mirrored(p, n) => {
            let across = topo::Reflection::across(
                Point3::new(at(p[0]), at(p[1]), at(p[2])),
                geom_core::Vec3::new(at(n[0]), at(n[1]), at(n[2])),
                tol(),
            )
            .map_err(|e| format!("{e:?}"))?;
            // The mirror image's rows completed, as its finish completes them.
            let image =
                topo::mirror_body(&home.into_body(), &across).map_err(|e| format!("{e:?}"))?;
            topo::test_support::completed(image, tol()).map_err(|e| format!("{e:?}"))
        }
    }
}

/// A rigid turn of one radian about a generic axis through a point off
/// the origin, a placement a hundred units from the origin, and a
/// mirror across a generic plane.
fn generic_poses() -> [(&'static str, Pose); 4] {
    use geom_core::{Affine3, Vec3};
    [
        ("as built", Pose::AsBuilt),
        (
            "turned about a generic axis",
            Pose::Moved(Affine3::rotation_about_axis(
                Point3::new(0.3, -0.2, 0.1),
                Vec3::new(0.48, -0.6, 0.64),
                1.0,
            )),
        ),
        (
            "a hundred units from the origin",
            Pose::Moved(Affine3::translation(Vec3::new(60.0, -80.0, 0.0))),
        ),
        (
            "mirrored across a generic plane",
            Pose::Mirrored([0.3, -0.2, 0.1], [0.48, -0.6, 0.64]),
        ),
    ]
}

/// **Whether an `Interval` twin's refusal is the one this file skips
/// on**: at ε = 1e-12, a pcurve envelope that escalates where the body
/// is built or finished
/// (`work/onehome/the-plain-sphere-witness-escalates-its-pcurve-envelope-at-1e-12-since-the-rigid-motion`).
/// Any other refusal, or that one at another ε, is a failure.
fn envelope_at_the_tightest_eps(refusal: &str) -> bool {
    band().zero() < 1e-11 && refusal.contains("pcurve_envelope")
}

/// **A rim band whose foot parallel is split at the wall's seams holds
/// across the joints.** On the band's chart the parallel is three edges
/// of one line, each ending at the vertex the next starts at. Their
/// stored ends at a joint are two readings of one point (equal at one
/// joint of the body as built, an ulp apart at another), and in a frame
/// that is not the chart's their rows hold the parallel's own value a
/// rounding apart (mirrored across a generic plane: `0` and `4.0e-16`).
/// A cell beside the line across a joint is covered by the two edges
/// together: below the parallel it holds none of the band, above it the
/// parallel is its side. Holds at `f64` as built, turned about a generic
/// axis, a hundred units from the origin and mirrored across a generic
/// plane. Red if a side is read against one edge of its line alone (the
/// wall against the band then runs out of depth at a joint), if two
/// edges are joined only where their stored ends agree, or if two rows
/// are one line only where their values agree (the mirrored pose).
///
/// **At `Interval` the body refuses off the joints, and is pinned
/// there**: two of the wall's cylinders against each other across the
/// edge they share, `Uncertified` on the departure arm (untraced). The
/// top plane against the band holds there: the band's window is its own
/// chart rectangle, so no cell past its end reaches their tangent circle
/// (`work/embed/the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam.md`).
/// It is asked of the body as built; the other poses' twins are built
/// and not asked. At ε = 1e-12
/// the twin's pcurve envelope escalates
/// ([`envelope_at_the_tightest_eps`]) and that half is skipped, said.
#[test]
fn a_three_arc_discs_filleted_rim_holds_across_its_joints() {
    for (what, pose) in generic_poses() {
        let what = format!("the three-arc disc's filleted rim, {what}");
        let body = posed_arc_disc::<f64>(pose).expect("the f64 body builds");
        let report = injective(&body, None, BUDGET);
        assert_holds(&what, &report, &body);
        println!("[joint] {what}: {:?}", report.receipt);
        match posed_arc_disc::<Interval>(pose) {
            Ok(twin) if matches!(pose, Pose::AsBuilt) => {
                let report = injective(&twin, None, BUDGET);
                let BoundVerdict::Refused(EngineRefusal::Uncertified {
                    a,
                    b,
                    shared: SharedCell::Edge(_),
                    arm: CertificateArm::Departure,
                }) = report.verdict
                else {
                    panic!(
                        "{what} at Interval refuses at its tangent seam: {:?} {:?}",
                        report.verdict, report.receipt
                    )
                };
                let kinds = [kind(&twin, a), kind(&twin, b)];
                assert_eq!(
                    kinds,
                    ["cylinder", "cylinder"],
                    "two of the wall's cylinders"
                );
            }
            Ok(_) => {}
            Err(refusal) if envelope_at_the_tightest_eps(&refusal) => {
                println!("SKIPPED: {what} at Interval, its twin unbuilt: {refusal}");
            }
            Err(refusal) => panic!("{what}: the Interval twin builds: {refusal}"),
        }
    }
}

/// **Two filleted discs through each other at a joint read violated.**
/// The second, grafted in as a solid of its own, is moved `0.5` along
/// `x` and `0.95` up, so its bottom plane cuts the first's band at
/// `z = 0.95`, through the azimuth `0` where the band's foot parallel
/// has a joint. Faces of two solids share nothing, so the crossing is
/// decided by distance; the row pins that the joint reading leaves a
/// crossing beside a joint to be found. A crossing of two faces that
/// share the joint is no kernel operation's to build; the joint's own
/// guards are `injective`'s unit rows. In one class at `Interval` where
/// the twin builds ([`filleted_arc_disc`]).
#[test]
fn two_filleted_discs_through_each_other_at_a_joint_read_violated() {
    fn folded_at<T: Scalar>() -> Result<Body<T>, String> {
        use geom_core::{Affine3, Vec3};
        let disc = filleted_arc_disc::<T>()?;
        let mut folded = disc.clone().into_body();
        let over = Affine3::translation(Vec3::new(0.5, 0.0, 0.95)).map(T::from_f64);
        let rigid = geom_core::test_support::rigid(&over, tol());
        let copy = topo::transform_rigid(&disc, &rigid).unwrap();
        topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
        Ok(folded)
    }
    let folded = folded_at::<f64>().expect("the f64 body builds");
    let report = injective(&folded, None, BUDGET);
    match folded_at::<Interval>() {
        Ok(twin) => same_class("the folded discs", &report, &injective(&twin, None, BUDGET)),
        Err(refusal) if envelope_at_the_tightest_eps(&refusal) => println!(
            "SKIPPED: the folded discs are asked at f64 alone, their Interval twin unbuilt: {refusal}"
        ),
        Err(refusal) => panic!("the folded discs' Interval twin builds: {refusal}"),
    }
    let BoundVerdict::Violated(w) = report.verdict else {
        panic!("the two discs cross: {:?}", report.verdict)
    };
    assert!(w.d < 1e-9, "a coincidence: {w:?}");
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
        Some(geom::Curve3::Ellipse(..))
    )
}

/// `b` moved rigidly by `m`, at rest.
fn moved<T: Scalar>(b: &topo::AtRestBody<T>, m: &geom_core::Affine3<f64>) -> topo::AtRestBody<T> {
    use sweep::test_support::finished;
    // The motion stated at `f64` and lifted as the type lifts a stated
    // frame (its polar enclosure at `Interval`).
    let rigid = geom_core::test_support::rigid(m, tol()).map_scalar(T::from_f64);
    finished("moved", topo::transform_rigid(b, &rigid).unwrap(), tol())
}

/// The box `(-50, 50)² × (-100, top)`, at rest: a half-space below
/// `z = top` within reach of every body here.
fn slab<T: Scalar>(top: f64) -> topo::AtRestBody<T> {
    use sweep::test_support::{brick, finished};
    finished(
        "the slab",
        brick::<T>((-50.0, 50.0), (-50.0, 50.0), (-100.0, top), tol()),
        tol(),
    )
}

/// The body a boolean built.
fn built<T: topo::AtRestPolicy<Finished = topo::AtRestBody<T>> + Real>(
    r: Result<topo::BooleanResult<T>, topo::BooleanError>,
) -> topo::AtRestBody<T> {
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

    let drum = sweep::test_support::taken::<T>(
        extrude(&disc::<T>(), to_offset(2.0), tol()).unwrap().body,
        tol(),
    )
    .expect("the drum is a finished body");
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
/// `pcurve_cylinder_chart_axial`, as it decides an `f64` rounding off
/// zero on a cylinder whose axis is not a coordinate axis. Red if it
/// does not: the wall then reads each rim as a curve through its corners
/// (`cornered` 8).
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

/// **A drum's cavity whose cap is one plane in three faces holds**: the
/// drum's cap ring is cut at half its radius
/// (`common::latitude_seam::collinear_cap_drum`), and the axial door's
/// cavity keeps it, a disc bounded by two half-circles of one circle,
/// stored on opposite axes, between two half-annuli. No carrier
/// separates the coplanar faces, and the circle's own plane is theirs;
/// each half-annulus against the disc is certified by the graph
/// separator, the disc read on through each vertex along its other
/// half-circle. Red, `Uncertified` on the disc across a half-circle, if
/// that corner is not read.
#[test]
fn a_drum_cavitys_cap_disc_on_two_half_circles_holds() {
    use crate::common::latitude_seam::{collinear_cap_drum, door_cavity};
    let cavity = door_cavity(&collinear_cap_drum(), 0.05);
    let report = injective(&cavity, None, BUDGET);
    assert_holds("the drum's cavity", &report, &cavity);
    assert_eq!(report.receipt.refused, 0, "{:?}", report.receipt);
}

/// **A two-arc sphere's cavity holds across its poles**: a sphere in
/// four faces, two lunes over each side of a latitude seam
/// (`common::latitude_seam::two_arc_sphere`), and the axial door's
/// cavity of it. Each pair of lunes shares both halves of a great
/// circle and a pole vertex, and each meridian's pcurve ends an ulp
/// short of the north pole's latitude. A lune holds its pole vertex, so
/// its extent reaches the latitude, and a cell at the pole is read
/// through the pole's collapsed side. Red, out of pairs on the two
/// northern lunes, where the cell is clipped to the stored extent. Built
/// at `f64` alone, as the drum's cavity above is: the fixture and the
/// door's cavity are `f64` bodies.
#[test]
fn a_two_arc_spheres_cavity_holds_across_its_poles() {
    use crate::common::latitude_seam::{door_cavity, two_arc_sphere};
    let cavity = door_cavity(&two_arc_sphere(), 0.05);
    let report = injective(&cavity, None, BUDGET);
    assert_holds("the two-arc sphere's cavity", &report, &cavity);
    assert_eq!(report.receipt.refused, 0, "{:?}", report.receipt);
    assert!(report.receipt.splits < 1_000, "{:?}", report.receipt);
}

/// **The drum the cavity above is cut from holds**: a full revolve, its
/// wall one band and its caps whole discs, so it has no half-annulus
/// cells beside a half-circle for the walk to run out of depth on, the
/// case `work/embed/a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth.md`
/// is about (its witness is the radius-0.7 drum below, whose half-circles
/// are drawn by hand). Red if the drum refuses.
#[test]
fn the_collinear_cap_drum_itself_runs_out_of_depth_off_its_half_circle() {
    use crate::common::latitude_seam::collinear_cap_drum;
    let drum = collinear_cap_drum();
    let report = injective(&drum, None, BUDGET);
    assert_holds("the collinear-cap drum", &report, &drum);
}

/// **A drum whose splits fall a rounding off its half-circle runs out of
/// depth**: the same drum at radius 0.7, whose half-annulus cells beside
/// the disc's half-circle are read by no presentation
/// (`work/embed/a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth.md`).
/// Red when it holds, and that row's reading has landed.
#[test]
fn a_drum_of_radius_0_7_runs_out_of_depth_off_its_half_circle() {
    use crate::common::latitude_seam::ring_on_cap;
    use sweep::test_support::{corners, revolved_about_y};
    let (r, h) = (0.7, 2.0);
    let mut drum = revolved_about_y(
        corners(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)]),
        Revolution::Full,
        Tol::witness(),
    );
    ring_on_cap(&mut drum, h, r, r / 2.0);
    let report = injective(&drum, None, BUDGET);
    assert!(
        matches!(
            report.verdict,
            BoundVerdict::Refused(EngineRefusal::Budget { .. })
        ),
        "the drum runs out of depth: {:?} {:?}",
        report.verdict,
        report.receipt
    );
    assert!(report.receipt.refused > 0, "{:?}", report.receipt);
}

/// **A cylinder–cylinder tee builds at both scalars, and its injectivity
/// holds where two arcs of its ellipse meet.** At `Interval`
/// the union builds at every ε, as the `f64` body: as many faces, edges
/// and vertices, and a volume enclosure that brackets the `f64` volume.
/// Two radius-1 rods on crossing axes, one mitred by
/// their bisector plane (`crossing_equal_rods`' pair at spins `(0, 0)`,
/// `B` from `x = 0.2`), united. Rod `B` starts at `x = 0.2`, so the
/// junction never reaches the node plane `x = 0`: it ends where `B`'s
/// cap circle meets the mitre plane, at `(0.2, ±0.98, 0.2)`, and `A`'s
/// seam line meets it at `(1, 0, 1)`. A plane's or a wall's corner of a
/// straight edge and the ellipse at a vertex its pair shares alone is
/// read as its quadrant, so `B`'s wall against the mitre at
/// `(0.2, ±0.98, 0.2)` and the walls' diagonal halves at `(1, 0, 1)`
/// certify, their quadrants strictly apart across a plane through the
/// vertex. `A`'s wall halves against the mitre across its ellipse
/// certify too: each face's corner of two arcs of that one ellipse is
/// read on through the vertex. So are `A`'s wall halves against `B`'s cap
/// at `(0.2, ±0.98, 0.2)`, a vertex they share alone, where `A`'s wall's
/// corner is the ellipse's two arcs, a straight angle no quadrant holds,
/// and no plane through the vertex parts the wall from the cap: they are
/// parted by the graph separator over the ellipse through the vertex, the
/// cap read as its quadrant. Every touching pair holds, asked alone. The
/// report names no arm per pair, so which arm parts them is not asserted
/// here: the graph separator's listing of the vertex is the one change
/// that lets them certify, and without it this row is red.
#[test]
fn a_cylinder_cylinder_tee_builds_and_its_injectivity_holds_where_two_arcs_of_its_ellipse_meet() {
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
    // At `Interval` the union builds, the ruled region's span reading the
    // rim's parameter within half a turn of its middle (`Period::near`).
    match tee_at::<Interval>() {
        Ok(topo::BooleanResult::Body(iv)) => {
            // The same body as the `f64` lane's: as many faces, edges and
            // vertices, and a volume enclosure, padded by the pinned
            // resolution, that brackets the `f64` body's pinned volume,
            // the enclosure itself no wider than 1e-9 of it.
            use geom_core::Bounds;
            fn counts<T: Real>(b: &Body<T>) -> (usize, usize, usize) {
                (b.faces().count(), b.edges().count(), b.vertices().count())
            }
            let f = built(tee_at::<f64>());
            assert_eq!(
                counts(&iv.body),
                counts(&f),
                "the tee: Interval f/e/v against f64"
            );
            let want = topo::test_support::pinned_volume("the tee", &f, tol()).volume;
            let m = topo::mass_properties(
                &iv.body,
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .expect("interval props");
            let (lo, hi) = (
                m.enclosure.volume.lo() - m.enclosure.volume_pad,
                m.enclosure.volume.hi() + m.enclosure.volume_pad,
            );
            assert!(
                lo <= want
                    && want <= hi
                    && m.enclosure.volume.hi() - m.enclosure.volume.lo()
                        <= 1e-9 * want.abs().max(1.0),
                "the tee: Interval volume [{lo}, {hi}] against f64 {want}, its enclosure less the pad no wider than 1e-9 of it"
            );
        }
        other => panic!(
            "the tee's union builds at Interval: {:?}",
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
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "the tee holds: {:?}",
        report.verdict
    );
    assert_eq!(report.receipt.refused, 0, "{:?}", report.receipt);
    assert!(
        report.receipt.splits < 640,
        "the tee splits a few hundred cells, 465 when pinned: {:?}",
        report.receipt
    );
    // Every touching pair holds asked alone: A's wall halves against B's
    // cap at a vertex alone, the wall halves against the mitre across its
    // ellipse, and every other pair at a vertex alone or across an edge.
    for (a, b) in touching_pairs(tee) {
        let alone = injective(tee, Some(vec![a, b]), BUDGET);
        assert!(
            matches!(alone.verdict, BoundVerdict::Holds),
            "{a:?} ({}) against {b:?} ({}) holds alone: {:?}",
            kind(tee, a),
            kind(tee, b),
            alone.verdict
        );
    }
    assert!(
        report
            .across
            .iter()
            .any(|(&(a, b), &n)| kind(tee, a) == "plane" && kind(tee, b) == "plane" && n > 0),
        "the mitre against B's cap is certified across their line: {:?}",
        report.across
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
    // Pinned red at 1e-12: the f64 body spends its budget where the
    // Interval body reads the fold
    // (`work/germ/main-red-the-folded-oblique-cut-spends-its-f64-budget-at-1e-12-since-the-posed-belly.md`).
    test_utils::pinned_red::row(
        tol().eps(),
        &[1e-12],
        "the folded oblique cut",
        &[
            "the folded cut: the Interval body reads the f64 body's verdict class",
            "left: \"Violated\"",
            "right: \"Budget\"",
        ],
        a_folded_oblique_cut_reads_violated_off_its_corners_row,
    );
}

/// The row [`a_folded_oblique_cut_reads_violated_off_its_corners`] runs, under its pin.
fn a_folded_oblique_cut_reads_violated_off_its_corners_row() {
    fn folded_at<T: Scalar>() -> Body<T> {
        use geom_core::{Affine3, Vec3};
        let cut = oblique_cut::<T>();
        let mut folded = cut.clone().into_body();
        let over = Affine3::translation(Vec3::new(0.3, 0.0, 0.0)).map(T::from_f64);
        let rigid = geom_core::test_support::rigid(&over, tol());
        let copy = topo::transform_rigid(&cut, &rigid).unwrap();
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
        .into_body()
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
fn cavity_of<T: Scalar + topo::CertifiedPolicy>(body: Body<T>, t: f64) -> (Body<T>, Vec<FaceKey>) {
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
    (hollow.into_body(), faces)
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
    revolve(&profile, axis, quarter, tol())
        .unwrap()
        .body
        .into_body()
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
    fn cavity_at<T: Scalar + topo::CertifiedPolicy>() -> BoundReport<Witness> {
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
    let ring = sweep::test_support::taken::<T>(ring, tol()).expect("the torus is a finished body");
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
        let copy = topo::transform_rigid(&cut, &rigid).unwrap();
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
    let ring = sweep::test_support::taken::<T>(ring, tol())
        .expect("the one-face torus is a finished body");
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    built(topo::subtract(&ring, &moved(&slab(0.3), &turn), tol()))
}

/// **A one-face torus cut certifies its caps by the wall alone**
/// ([`one_face_torus_cut`]): each cap shares a whole oval with its torus
/// face, with no corner on it, so each cap–torus pair is certified across
/// by the plain wall presentation, none through a corner. The body
/// refuses `Departure` across the meridian the two torus faces share,
/// pinned typed at both scalars
/// (`one-face-torus-cut-refuses-across-its-meridian`), and its ask ends
/// there, so each cap is asked against its torus face alone. Red if the
/// plain wall presentation is withdrawn.
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
        assert_eq!(
            (report.receipt.refused, report.receipt.abandoned > 0),
            (1, true),
            "the ask ends at the meridian: {:?}",
            report.receipt
        );
        assert_eq!(cut.len(), 2, "two caps, each on one whole oval");
        for p in &cut {
            let alone = pair_report(&body, p.0, p.1, BUDGET);
            assert_eq!(alone.verdict, BoundVerdict::Holds, "{p:?} asked alone");
            assert_eq!(alone.cornered, 0, "{p:?}: no corner is read");
            assert!(
                alone.across.get(p).is_some_and(|&n| n > 0),
                "a cap and its torus face certified across their oval: {p:?}"
            );
        }
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
        .into_body()
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
    fn cavity_at<T: Scalar + topo::CertifiedPolicy>(what: &str) -> BoundReport<Witness> {
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

/// A body with rational lens walls: two unit lens sections, two arcs on
/// one chord, stacked one apart three times (`lily_leaf_a`'s section),
/// lofted straight at `f64` and its carriers held at `T` as points
/// ([`sweep::test_support::f64_body_held_as_points_construction`]).
fn lens_walled<T: Scalar>() -> Body<T> {
    let lens = vec![profile::test_support::bulge_loop(vec![
        (Point2::new(0.5, 0.0), -0.3),
        (Point2::new(-0.5, 0.0), -0.3),
    ])];
    sweep::test_support::f64_body_held_as_points_construction::<T>(
        &[lens.clone(), lens.clone(), lens],
        &sweep::test_support::stacked_at(&[0.0, 1.0, 2.0]),
        2,
        tol(),
    )
    .expect("the lens-walled body builds")
    .body
}

/// **Rational lens walls hold against themselves, at both scalars and
/// moved off the origin** ([`lens_walled`]: at `Interval` the `f64`
/// body's carriers held as points, not the loft built there, which
/// refuses at the carrier's door until it holds an enclosure weight): each wall alone at the default
/// budget, at the origin and moved by `(10, 10, 10)`, certifying a
/// cell-pair count within 1% of its unmoved twin's. The lens arcs are
/// rational, so the monotone arm reads each wall through a rational
/// net's derivative over a cell; read in the absolute frame, the moved
/// walls' derivative carries the distance times the weight's slope.
/// The `Interval` twin of the tour's `lily_leaf_a` wall rows, which
/// build at `f64` only.
#[test]
fn lens_walls_hold_against_themselves_moved_off_the_origin_at_both_scalars() {
    fn row<T: Scalar>(what: &str) {
        let home = lens_walled::<T>();
        let by = geom_core::Vec3::new(10.0, 10.0, 10.0).map(T::from_f64);
        let motion = geom_core::RigidMotion::translation(by).expect("a translation");
        let away = topo::transform_rigid(&home, &motion).expect("the loft moves");
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
    row::<f64>("the lens-walled body");
    row::<Interval>("the lens-walled body at Interval");
}

/// **A lens-walled body's caps hold against its walls at the lens tip,
/// at both scalars** ([`lens_walled`]). Two unit lens sections, two arcs on one chord, one above
/// the other: each cap is a plane bounded by two circles meeting at the
/// two tips. A cap cell holding a tip holds both arcs, and no
/// presentation reads a corner of two curved edges on a plane; the
/// wall's departure from the cap's plane certifies it, the wall's side
/// on the shared edge read off its own restricted iso net, which lies
/// in the cap's plane. This is the tour's `lily_leaf_a` cap corner
/// (`demos/tour/tests/lily.rs`), built here at both scalars.
#[test]
fn a_lens_walled_cap_holds_against_its_walls_at_its_tip_at_both_scalars() {
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
    let (float, interval) = (lens_walled::<f64>(), lens_walled::<Interval>());
    let pairs = adjacent(&float, true);
    assert_eq!(pairs.len(), 4, "two caps against two walls: {pairs:?}");
    assert_eq!(
        adjacent(&interval, true),
        pairs,
        "one topology at both scalars"
    );
    let pinned = [(0, 0), (0, 0), (0, 0), (0, 0)];
    assert_eq!(
        receipts("the lens-walled body", &float),
        pinned,
        "each cap-wall pair's (splits, deepest cell)"
    );
    assert_eq!(
        receipts("the lens-walled body at Interval", &interval),
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
        sweep::test_support::loft_body_construction::<T>(&sections, &places, 2, tol())
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
        sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, 33, 3, tol())
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
/// lofted at `f64` at five stations along a unit-radius arc turning
/// `curl` in the `xz` plane, the whole turned `phi` about the world `y`
/// axis, its carriers held at `T` as points
/// ([`sweep::test_support::f64_body_held_as_points_construction`]: the
/// walls' interior v knot is a chord-length parameter, which the loft
/// built at `Interval` holds as an enclosure no carrier takes yet).
/// Its two wide walls are near-parallel sheets `g` apart with a radius
/// of about 1, sharing no vertex; they are returned with the body.
fn slab_wall_pair<T: Scalar>(
    g: f64,
    curl: f64,
    phi: f64,
) -> (sweep::Lofted<T, Body<T>>, [FaceKey; 2]) {
    use geom_core::{Affine3, Mat3, Vec3};
    let section = quad([
        (-0.4, -g / 2.0),
        (0.4, -g / 2.0),
        (0.4, g / 2.0),
        (-0.4, g / 2.0),
    ]);
    let turn = Mat3::rotation_about(Vec3::new(0.0, 1.0, 0.0), phi);
    let places: Vec<SketchPlane<f64>> = (0..5)
        .map(|i| {
            let theta = curl * f64::from(i) / 4.0;
            let (s, c) = theta.sin_cos();
            let linear = Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-s, 0.0, c),
                Vec3::new(c, 0.0, s),
            );
            let at = Affine3::from_parts(turn * linear, turn * Vec3::new(s, 0.0, 1.0 - c));
            crate::common::plane_of(&at)
        })
        .collect();
    let sections: Vec<_> = (0..5).map(|_| section.clone()).collect();
    let lofted = sweep::test_support::f64_body_held_as_points_construction::<T>(
        &sections,
        &places,
        3,
        tol(),
    )
    .expect("the slab builds");
    let walls = [
        lofted.side_faces[0][0][0][0].unwrap(),
        lofted.side_faces[0][2][0][0].unwrap(),
    ];
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
/// at most 1.5 times the upright one's, at each scalar. Red where the distance arm
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
        counts.push([examined(&report), examined(&interval)]);
    }
    let [upright, turned] = counts[..] else {
        unreachable!("two turns")
    };
    for (k, what) in ["f64", "Interval"].iter().enumerate() {
        let (upright, turned) = (upright[k], turned[k]);
        assert!(
            2 * turned <= 3 * upright,
            "the turned walls examine at most 1.5× the upright walls' pairs at {what}: {turned} \
             against {upright}"
        );
    }
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
            wall_against_wall(&float.body, walls, ClearanceBound::at_least(0.08)),
        ),
        (
            "Interval",
            wall_against_wall(&twin.body, twin_walls, ClearanceBound::at_least(0.08)),
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
    let report = wall_against_wall(&float.body, walls, ClearanceBound::at_least(0.03));
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "walls 0.05 apart hold AtLeast(0.03): {:?} {:?}",
        report.verdict,
        report.receipt
    );
}

/// The bound question between `walls[0]` and `walls[1]` of `body`, with
/// a verifier that confirms no witness: no candidate ends the walk, so
/// every pair the support does not separate is refined.
fn wall_against_wall_unwitnessed<T: Scalar>(
    body: &Body<T>,
    walls: [FaceKey; 2],
    bound: ClearanceBound,
    budget: usize,
) -> BoundReport<()> {
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
            max_cell_pairs: budget,
        },
    };
    let report = sweep.run(
        &window(walls[0], Which::First),
        &window(walls[1], Which::Second),
        true,
        |_| Err::<(), _>("no witness confirmed".to_owned()),
    );
    assert!(
        report.receipt.holds(),
        "the receipt adds up: {:?}",
        report.receipt
    );
    report
}

/// **The support at depth never separates a pair closer than the
/// bound.** The slab's walls turned 45° are `0.05` apart. With no
/// witness confirmed, the exhibit arm ends nothing, so the walk refines
/// every pair the support does not separate. `AtLeast(0.045)` holds
/// below the root, and `AtLeast(0.08)`, at a 2048-pair budget, does not
/// hold, refined below the root. Red if the support gap is compared
/// without the bound, or if it reads a pair farther apart than it is.
#[test]
fn the_support_at_depth_never_separates_a_pair_closer_than_the_bound() {
    let phi = core::f64::consts::FRAC_PI_4;
    let (float, walls) = slab_wall_pair::<f64>(0.05, 0.3, phi);
    let held =
        wall_against_wall_unwitnessed(&float.body, walls, ClearanceBound::at_least(0.045), BUDGET);
    println!(
        "[dirsupport] held at depth: {:?} {:?} deepest {}",
        held.verdict, held.receipt, held.widths.deepest
    );
    assert!(
        matches!(held.verdict, BoundVerdict::Holds) && held.receipt.splits > 0,
        "walls 0.05 apart hold AtLeast(0.045) below the root: {:?} {:?}",
        held.verdict,
        held.receipt
    );
    let broken =
        wall_against_wall_unwitnessed(&float.body, walls, ClearanceBound::at_least(0.08), 2048);
    println!(
        "[dirsupport] broken at depth: {:?} {:?} deepest {}",
        broken.verdict, broken.receipt, broken.widths.deepest
    );
    assert!(
        !matches!(broken.verdict, BoundVerdict::Holds) && broken.receipt.splits > 0,
        "walls 0.05 apart do not hold AtLeast(0.08), read at depth: {:?} {:?}",
        broken.verdict,
        broken.receipt
    );
}

/// **A gap over a metre is read in metres.** The slab's walls `1.6`
/// apart, upright and turned 45°, read `Violated` under `AtLeast(1.62)`
/// with a verified witness closer than `1.62`, at both scalars. The
/// line between two cells' centres is about `1.6` long here, so a
/// support gap read along it and not divided by its length would read
/// the walls `1.6` times as far apart as they are, and hold.
#[test]
fn a_gap_over_a_metre_is_read_in_metres() {
    for phi in [0.0, core::f64::consts::FRAC_PI_4] {
        let (float, walls) = slab_wall_pair::<f64>(1.6, 0.3, phi);
        let (twin, twin_walls) = slab_wall_pair::<Interval>(1.6, 0.3, phi);
        for (what, report) in [
            (
                "f64",
                wall_against_wall(&float.body, walls, ClearanceBound::at_least(1.62)),
            ),
            (
                "Interval",
                wall_against_wall(&twin.body, twin_walls, ClearanceBound::at_least(1.62)),
            ),
        ] {
            let BoundVerdict::Violated(w) = report.verdict else {
                panic!(
                    "{what}: walls 1.6 apart turned {phi} violate AtLeast(1.62): {:?} {:?}",
                    report.verdict, report.receipt
                )
            };
            assert!(
                w.d < 1.62,
                "{what} turned {phi}: the witness is closer than the bound: {w:?}"
            );
        }
    }
}

/// **The walk reads its charts and edges through their memos.** A
/// half-turn helical sweep's spline walls and spline edges, read into
/// windows and asked a few dozen pairs: every spline
/// chart and spline edge of the windows keeps a memo, and the walk has
/// filled the charts' and the edges'. Red if a window's chart or a
/// lifted edge is read without its memo, so that each pair restricts
/// its cells afresh.
#[test]
fn the_walk_reads_its_charts_and_edges_through_their_memos() {
    let path = helix(0.5, 0.4);
    let place = normal_start_place(&path);
    let body = sweep_body::<f64>(
        &lifted(&quad([
            (-0.08, -0.08),
            (0.08, -0.08),
            (0.08, 0.08),
            (-0.08, 0.08),
        ])),
        plane_at(&place),
        &path,
        9,
        3,
        tol(),
    )
    .expect("the half-turn helix sweeps")
    .body;
    let side = Side {
        body: &body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    let windows = side.windows(Which::First).expect("every face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 64,
        },
    };
    let _ = sweep.run(
        &windows,
        &windows,
        true,
        verify(&body, ClearanceBound::StrictlyPositive),
    );
    let (charts, edges) = windows.spline_memos();
    assert!(
        !charts.is_empty() && !edges.is_empty(),
        "the sweep has spline walls ({}) and spline edges ({})",
        charts.len(),
        edges.len()
    );
    assert!(
        charts.iter().chain(&edges).all(Option::is_some),
        "every spline chart and edge keeps a memo: charts {charts:?}, edges {edges:?}"
    );
    let filled = |m: &[Option<usize>]| m.iter().flatten().sum::<usize>();
    assert!(
        filled(&charts) > 0 && filled(&edges) > 0,
        "the walk read through the memos: charts {charts:?}, edges {edges:?}"
    );
}

// ------------------------------------------------ a sphere's pole

/// **Two coaxial balls' union holds, its poles read**: the snowman of
/// `snowman.rs`, radii 1 and 0.8 on centres 1.4 apart. Each ball is one
/// sphere face, its one meridian walked out from a pole and back, and
/// the union keeps one pole of each. A cell holding a pole is near the
/// meridian and the pole, and the pole is the whole latitude line
/// `v = ±π/2` of the chart, so no split moves a cell off it. Read as a
/// collapsed side, the polar cells certify against the meridian's
/// plane; read as a loose vertex, every polar strip split until the walk
/// ran out of its 65,536 pairs, at depth 32.
///
/// At a pole the face's corner is on its one meridian, and the corner is
/// zero under the plane only where the plane lists that meridian, which
/// the plane built from it does at both scalars: at `Interval` too, where
/// the small ball's centre is a bracket (its profile arc's centre
/// rounds), the union holds. Red if the corner is read as zero off the
/// list, or if either scalar refuses.
#[test]
fn a_two_ball_union_holds_with_its_poles_read() {
    fn snowman<T: Scalar>() -> Body<T> {
        let ball = |r: f64, y: f64| {
            let p = |y: f64, bulge: f64| {
                (
                    Point2::new(T::from_f64(0.0), T::from_f64(y)),
                    T::from_f64(bulge),
                )
            };
            sweep::test_support::finished(
                "the ball",
                revolved_about_y_at::<T>(
                    vec![p(y - r, 1.0), p(y + r, 0.0)],
                    Revolution::Full,
                    tol(),
                ),
                tol(),
            )
        };
        let union =
            topo::boolean::union(&ball(1.0, 0.0), &ball(0.8, 1.4), tol()).expect("the balls unite");
        Body::clone(&union.body().expect("a body").body)
    }
    let float = snowman::<f64>();
    let report = injective(&float, None, BUDGET);
    assert_holds("the snowman", &report, &float);
    assert!(
        report.receipt.discharged + report.receipt.refused < BUDGET / 16,
        "the poles cost a few hundred pairs, not the budget: {:?}",
        report.receipt
    );
    let twin = snowman::<Interval>();
    let at_interval = injective(&twin, None, BUDGET);
    assert_holds("the snowman at Interval", &at_interval, &twin);
    println!(
        "[snowman] f64 {:?} | Interval {:?}",
        report.receipt, at_interval.receipt
    );
}

// ------------------------------------- a boundary C¹ through a vertex

/// The class of the verdict door's answer over every face of `body`
/// (`topo::clearance::embedded`), at the default depth and budget:
/// `Embedded`, `Crosses`, or the refusal with the arm and the kinds of
/// the two faces it names.
fn door_class<T: Scalar + geom_core::Certifies>(body: &Body<T>) -> String {
    door_over(body, body.faces().map(|(k, _)| k).collect())
}

/// [`door_class`] over `faces` of `body` alone.
fn door_over<T: Scalar + geom_core::Certifies>(body: &Body<T>, faces: Vec<FaceKey>) -> String {
    use topo::clearance::{Embedding, embedded};
    let report = embedded(
        &Side { body, faces },
        band(),
        Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: BUDGET,
        },
    );
    match report.verdict {
        Embedding::Embedded => "Embedded".into(),
        Embedding::Crosses(_) => "Crosses".into(),
        Embedding::Refused(EngineRefusal::Uncertified {
            a, b, shared, arm, ..
        }) => {
            let cell = match shared {
                SharedCell::Edge(_) => "an edge",
                SharedCell::Vertex(_) => "a vertex",
                SharedCell::Face(_) => "a face",
            };
            let mut kinds = [kind(body, a), kind(body, b)];
            kinds.sort_unstable();
            format!(
                "Uncertified {arm:?} {} x {} across {cell}",
                kinds[0], kinds[1]
            )
        }
        Embedding::Refused(other) => format!("Refused {other:?}"),
    }
}

/// A rigid turn about a generic axis through a generic point, and a
/// placement a hundred units from the origin: the two poses every body
/// of this section is also asked in.
fn poses() -> [(&'static str, geom_core::Affine3<f64>); 2] {
    use geom_core::{Affine3, Vec3};
    let axis = Vec3::new(0.3, -0.5, 0.81);
    let axis = axis * (1.0 / axis.norm());
    let turn = Affine3::rotation_about_axis(Point3::new(0.2, -0.1, 0.3), axis, 0.7);
    let far = Affine3::translation(Vec3::new(100.0, -60.0, 80.0));
    [
        ("turned about a generic axis", turn),
        ("a hundred units out", far),
    ]
}

/// `edges` of `body` filleted at `r`, at rest; `None`, said aloud with
/// the refusal, where the certified scalar's fillet refuses at the
/// tightest ε ([`crate::band_closed_mixed_chain_shapes::tightest_interval`]:
/// a mixed rim's section does not certify there, which is the band's
/// row and not this one's).
fn rounded<T: Scalar>(
    what: &str,
    body: &Body<T>,
    edges: &[topo::EdgeKey],
    r: f64,
) -> Option<topo::AtRestBody<T>> {
    use crate::band_closed_mixed_chain_shapes::tightest_interval;
    match sweep::blend::fillet_edges(
        &sweep::test_support::at_rest(body, tol()),
        edges,
        T::from_f64(r),
        tol(),
    ) {
        Ok(out) => Some(out.body),
        Err(e) if tightest_interval::<T>() => {
            println!(
                "[tangent] {what}: SKIPPED, the fillet refuses at Interval at this ε: {:?}",
                e.error
            );
            None
        }
        Err(e) => panic!("{what}: fillets, got {:?}", e.error),
    }
}

/// `body` asked at the verdict door as built and in each of [`poses`]:
/// every answer is `want`.
fn door_reads<T: Scalar + geom_core::Certifies>(
    what: &str,
    body: Option<topo::AtRestBody<T>>,
    want: &str,
) {
    let Some(body) = body else {
        return;
    };
    assert_eq!(door_class::<T>(&body), want, "{what}, as built");
    for (pose, m) in poses() {
        assert_eq!(door_class::<T>(&moved(&body, &m)), want, "{what}, {pose}");
    }
}

/// The inset corner (`band_mixed_corner::inset`), its three edges
/// filleted at `0.1`: the plate's top is bounded by the corner patch's
/// arc and, tangent to it at each end, a strip's straight foot, the face
/// outside the arc's circle.
fn inset_filleted<T: Scalar>() -> Option<topo::AtRestBody<T>> {
    let (body, edges) = crate::band_mixed_corner::inset::<T>();
    rounded("the inset corner", &body, &edges, 0.1)
}

/// A `2 × 1.2 × 0.5` plate of corner radius `0.3`, every edge filleted
/// at `0.1`: each cap is bounded by four arcs and the four lines tangent
/// to them, the face inside each arc's circle.
fn rounded_plate<T: Scalar>() -> Option<topo::AtRestBody<T>> {
    use crate::band_closed_mixed_chain_shapes::{every_edge, prism, rect};
    let body = prism::<T>(&rect(1.0, 0.6, [0.3; 4]), 0.0, 0.5);
    rounded("the rounded plate", &body, &every_edge(&body), 0.1)
}

/// [`rounded_plate`] with its outline drawn turned 30° about the
/// origin: the same body in another pose, built there and not moved.
fn rounded_plate_drawn_turned<T: Scalar>() -> Option<topo::AtRestBody<T>> {
    use crate::band_closed_mixed_chain_shapes::{every_edge, prism, rect, rotated};
    let outline = rotated(&rect(1.0, 0.6, [0.3; 4]), core::f64::consts::FRAC_PI_6);
    let body = prism::<T>(&outline, 0.0, 0.5);
    rounded("the plate drawn turned", &body, &every_edge(&body), 0.1)
}

/// **A support face whose boundary is C¹ through a vertex is embedded
/// against the band faces that meet it there**: a fillet band's corner
/// patch is a torus whose foot on the support plane is an arc, tangent
/// at each end to a strip's straight foot, and tangent to the plane
/// along it, so neither carrier parts the pair and the plane's cell at
/// the joint is read by no edge alone. The arc and the line are read as
/// one run through the vertex (`topo::injective`'s `present_corner`),
/// from either edge, under a graph separator run on along both.
///
/// Two bodies: the face outside the arc's circle (the inset corner) and
/// inside it (the rounded plate). Each as built, turned about a generic
/// axis and placed a hundred units out: a rigid move keeps the stored
/// images, so those poses vary the carriers and not how the body was
/// drawn. Red if the run is withdrawn (each then refuses `Uncertified
/// Departure plane x torus across an edge`) or is read from the arc's
/// side only (plane x cylinder).
///
/// **The plate drawn turned 30°**, built off the axes, is embedded too:
/// a strip's section arc is a rounding off a parallel there, and the
/// mint decides it a parallel (`cylinder_rim_axial`). Red if it does
/// not: the strip's corner is then read by its ruling run on past the
/// vertex, `Uncertified Departure cylinder x plane across an edge`.
#[test]
fn a_support_face_c1_through_a_vertex_is_embedded_at_its_tangent_joints() {
    door_reads::<f64>("the inset corner", inset_filleted(), "Embedded");
    door_reads::<f64>("the rounded plate", rounded_plate(), "Embedded");
    door_reads::<f64>(
        "the plate drawn turned",
        rounded_plate_drawn_turned(),
        "Embedded",
    );
}

/// The edges of `f`'s loops.
fn edges_of<T: Real>(body: &Body<T>, f: FaceKey) -> BTreeSet<topo::EdgeKey> {
    body.face(f)
        .unwrap()
        .loops()
        .map(|(lk, _, _)| lk)
        .flat_map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Empty { .. } => Vec::new(),
            LoopBoundary::Cycle { first } => body
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| body.get_half_edge(he).unwrap().edge)
                .collect(),
        })
        .collect()
}

/// **The same bodies at `Interval`**, the scalar the shell gate and the
/// document door ask at, as built: every support plane asked alone
/// against a corner patch it shares an arc with is `Embedded`, the
/// inset corner's patches across their arcs and the rounded plate's,
/// tangent along theirs. Each patch's window is its own chart
/// rectangle, so no cell past the patch's end reaches the plane at the
/// tangent arc. Red if a patch's window is the whole tube again: the
/// rounded plate's pairs then refuse `Uncertified` on the departure arm
/// at the arc
/// (`work/embed/the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam.md`).
///
/// The whole bodies do not hold, on pairs this reading does not touch
/// (`work/embed/three-fillet-bodies-embedded-at-f64-refuse-at-interval-on-pairs-untraced.md`):
/// pinned as refused, and printed.
#[test]
fn a_support_face_c1_through_a_vertex_at_interval() {
    use topo::clearance::{Embedding, embedded};
    for (what, body) in [
        ("the inset corner", inset_filleted::<Interval>()),
        ("the rounded plate", rounded_plate::<Interval>()),
    ] {
        let Some(body) = body else {
            continue;
        };
        let mut asked = 0;
        for (a, b) in touching_pairs(&body) {
            let mut kinds = [kind(&body, a), kind(&body, b)];
            kinds.sort_unstable();
            if kinds != ["plane", "torus"] || edges_of(&body, a).is_disjoint(&edges_of(&body, b)) {
                continue;
            }
            asked += 1;
            let verdict = embedded(
                &Side {
                    body: &*body,
                    faces: vec![a, b],
                },
                band(),
                Budgets {
                    max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                    max_cell_pairs: BUDGET,
                },
            )
            .verdict;
            assert!(
                matches!(verdict, Embedding::Embedded),
                "{what} at Interval: a plane against a patch across their arc: {verdict:?}"
            );
        }
        assert!(
            asked > 0,
            "{what} at Interval: no plane shares an arc with a patch"
        );
        let whole = door_class::<Interval>(&body);
        println!("[tangent] {what} at Interval: {asked} pairs; whole: {whole}");
        assert!(
            whole.starts_with("Uncertified") || whole.starts_with("Refused"),
            "{what} at Interval, whole: refused on another pair, not {whole}"
        );
    }
}

/// **One plate in every pose.** A rounded plate of four different
/// corner radii (so its mirror image is not itself), every edge
/// filleted, at `f64`: as built, turned about a generic axis, a hundred
/// units out, 990 units out on each axis (the session box's wall is at
/// 1000; at ε = 1e-12 that move does not finish, at `shared_row_touch`,
/// and only that refusal is passed over), and
/// mirrored across `x = 0` and across a generic plane, all `Embedded`.
/// The generic mirror re-mints the rows on a turned frame, where a
/// strip's section arc is a rounding off a parallel that the mint
/// decides a parallel (`cylinder_rim_axial`); red if it does not
/// (`Uncertified Departure cylinder x plane across an edge`). The
/// mirror's result is asked as a body and not put through the at-rest
/// finish, which refuses its re-minted rows on a generic plane.
#[test]
fn one_uneven_plate_is_embedded_in_every_rigid_pose_and_its_axis_mirror() {
    use crate::band_closed_mixed_chain_shapes::{every_edge, prism, rect};
    use geom_core::{Affine3, Vec3};
    let body = prism::<f64>(&rect(1.0, 0.6, [0.3, 0.26, 0.34, 0.28]), 0.0, 0.5);
    let body = rounded("the uneven plate", &body, &every_edge(&body), 0.1).expect("f64 fillets");
    assert_eq!(door_class::<f64>(&body), "Embedded", "as built");
    for (pose, m) in poses() {
        assert_eq!(door_class::<f64>(&moved(&body, &m)), "Embedded", "{pose}");
    }
    // 990 units out. At ε = 1e-12 the moved plate is not a finished body:
    // four edges' `shared_row_touch` reads 1.4e-12, inside the band
    // (`work/band/a-filleted-plate-moved-990-units-out-does-not-finish-at-f64-at-the-tightest-eps.md`),
    // the finish's and not this row's. Only that refusal, at that ε, is
    // said aloud and passed over: every finding an edge's certification
    // naming `shared_row_touch` and nothing else. Any other refusal, or
    // that one at another ε, fails the row.
    let wall = Affine3::translation(Vec3::new(990.0, -990.0, 990.0));
    let rigid = geom_core::test_support::rigid(&wall, tol());
    let out = topo::transform_rigid(&body, &rigid).expect("the plate moves");
    match <f64 as topo::AtRestPolicy>::finish(out, tol()) {
        Ok(out) => assert_eq!(
            door_class::<f64>(&out),
            "Embedded",
            "990 units out on each axis"
        ),
        Err(e)
            if tol().eps().to_bits() == 1e-12_f64.to_bits()
                && !e.as_slice().is_empty()
                && e.as_slice().iter().all(|f| {
                    let read = format!("{f:?}");
                    let named: Vec<&str> = read
                        .split("predicate: Some(\"")
                        .skip(1)
                        .filter_map(|r| r.split('"').next())
                        .collect();
                    matches!(f, topo::ValidationError::EdgeCertification { .. })
                        && !named.is_empty()
                        && named.iter().all(|p| *p == "shared_row_touch")
                }) =>
        {
            println!(
                "[tangent] SKIPPED at ε 1e-12: the uneven plate 990 units out does not finish: {e:?}"
            );
        }
        Err(e) => panic!("the uneven plate 990 units out does not finish: {e:?}"),
    }
    let mirrored = |point, n| {
        let reflection = topo::Reflection::across(point, n, tol()).expect("a plane");
        let mirrored = topo::test_support::completed(
            topo::mirror_body(&body, &reflection).expect("mirrors"),
            tol(),
        )
        .expect("the mirror image's rows complete");
        door_class::<f64>(&mirrored)
    };
    assert_eq!(
        mirrored(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        "Embedded",
        "mirrored across x = 0"
    );
    let generic = mirrored(Point3::new(0.2, -0.1, 0.3), Vec3::new(0.3, -0.5, 0.81));
    assert_eq!(generic, "Embedded", "mirrored across a generic plane");
}

/// **A copy pushed through a tangent joint crosses**: the inset corner
/// filleted, and a copy of it moved `0.02` along `(1, 1, 0)`, grafted in
/// as a solid of its own. The copy's support plane is the body's own, so
/// its corner patch passes through the body's plate top beside the
/// joint. Faces of two solids share nothing in M, so the crossing is
/// decided by distance: the row pins that reading the joint as one run
/// leaves a crossing beside it to be found. A fold through the joint's
/// own cell is `injective`'s `a_fold_through_a_tangent_joint_is_not_certified`.
#[test]
fn a_copy_pushed_through_a_tangent_joint_crosses() {
    fn folded<T: Scalar>() -> Option<Body<T>> {
        use geom_core::{Affine3, Vec3};
        let body = inset_filleted::<T>()?;
        let over = Affine3::translation(Vec3::new(0.02, 0.02, 0.0));
        let copy = moved(&body, &over);
        let mut folded = body.into_body();
        topo::graft_disjoint(&mut folded, &copy).expect("the copy grafts");
        Some(folded)
    }
    assert_eq!(
        door_class::<f64>(&folded().expect("f64 fillets")),
        "Crosses",
        "the two copies cross"
    );
    if let Some(twin) = folded::<Interval>() {
        assert_eq!(door_class::<Interval>(&twin), "Crosses", "at Interval");
    }
}

/// The frames an apex row is read in: the plain one, a rigid turn about
/// a generic axis, and the plain one moved a hundred units off the
/// origin.
fn apex_frames() -> [(&'static str, SketchPlane<f64>); 3] {
    use geom_core::{Affine3, Mat3, Vec3};
    let a = Vec3::new(1.0, 2.0, 3.0) * (1.0 / 14f64.sqrt());
    let (c, s) = (0.7f64.cos(), 0.7f64.sin());
    let col = |e: Vec3<f64>| e * c + a.cross(e) * s + a * (a.dot(e) * (1.0 - c));
    let turn = Mat3::from_cols(
        col(Vec3::new(1.0, 0.0, 0.0)),
        col(Vec3::new(0.0, 1.0, 0.0)),
        col(Vec3::new(0.0, 0.0, 1.0)),
    );
    let frame = |m: Mat3<f64>, d: Vec3<f64>| {
        SketchPlane::from_frame(geom_core::test_support::rigid(
            &Affine3::from_parts(m, d),
            tol(),
        ))
    };
    [
        ("plain", SketchPlane::xy()),
        ("turned", frame(turn, Vec3::new(0.0, 0.0, 0.0))),
        (
            "moved",
            frame(Mat3::identity(), Vec3::new(64.0, -57.0, 52.0)),
        ),
    ]
}

/// Whether a revolve refused at the certification limit a wedge reaches
/// at ε = 1e-12 with coordinates of a hundred and more: the analytic
/// rung 3's kernel limit on its cone or its limb 2 undecided there (the
/// `−θ` wedge a hundred times the size, at `Interval`), the mapped source's check
/// escalating, or a pcurve's envelope term escalating
/// (`work/select/an-interval-revolve-of-a-cone-wedge-a-thousand-units-out-refuses-its-cones-rung-3-at-1e-12.md`).
/// Any other refusal is not.
fn at_the_build_limit(e: &sweep::RevolveError) -> bool {
    use geom_brep::{
        AnalyticRung3Refusal, CertCheck, CertifyError, PcurveCertifyError, PcurveCheck,
    };
    match e {
        sweep::RevolveError::Op {
            source: topo::EulerOpError::DescriptionUncertified { error, .. },
        } => matches!(
            error,
            CertifyError::AnalyticRung3(
                AnalyticRung3Refusal::KernelLimit { .. } | AnalyticRung3Refusal::Stalled { .. }
            ) | CertifyError::Escalated {
                check: CertCheck::MappedSource,
                ..
            }
        ),
        // The leaf's completion refuses a row whose envelope term
        // escalates, beside the verdict's findings.
        sweep::RevolveError::NotFinished { findings } => findings.as_slice().iter().any(|f| {
            matches!(
                f,
                topo::ValidationError::Pcurve {
                    finding: topo::PcurveMintError::Certify {
                        error: PcurveCertifyError::Escalated {
                            check: PcurveCheck::EnvelopeTerm(_),
                            ..
                        },
                        ..
                    },
                }
            )
        }),
        _ => false,
    }
}

/// **A cone wedge past a half turn is embedded at its apex**: a
/// triangle with a leg on the axis, revolved by `θ` about `y`. Each
/// wedge plane holds the axis, so its carrier holds a second ruling of
/// the cone through the apex, half a turn from the one it shares, and
/// past a half turn that ruling is on the cone face: no plane through
/// the shared ruling parts the cone's cells about it from the wedge
/// plane's at the apex. A plane through the apex does, each face read
/// against the apex alone. `Embedded` at both scalars: the unit
/// triangle in each of [`apex_frames`] and a thousand units off the
/// origin, and a triangle a hundred times the size, a needle and a flat
/// one in the plain frame; and the same poses revolved by `−θ`.
/// Red while the departure arm reads no plane but those through the
/// shared ruling. Off the plain frame the `−θ` poses are red, too, where
/// the shared ruling's image stops a rounding of its stored end short of
/// the apex and is not run on to the apex's collapsed side
/// (`ends_on_collapse`): the cone is then not presented at the apex and
/// no plane is tried. The `−5.5` wedge in the turned frame is red, out
/// of depth between its start plane and its cone, where its faces'
/// windows are read off their boundaries' world boxes.
#[test]
fn a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex() {
    use geom_core::{Affine3, Mat3, Vec3};
    use topo::clearance::Embedding;
    fn wedge<T: Scalar>(
        place: &SketchPlane<f64>,
        legs: (f64, f64),
        theta: f64,
    ) -> Result<Body<T>, sweep::RevolveError> {
        let (r, h) = legs;
        let profile = Profile::new(
            plane_at::<T>(place),
            lifted(&[ProfileLoop::polygon(
                [(0.0, 0.0), (r, 0.0), (0.0, h)].map(|(x, y)| Point2::new(x, y)),
            )]),
        )
        .validate(tol())
        .unwrap();
        revolve(
            &profile,
            axis_y(),
            Revolution::Partial(T::from_f64(theta)),
            tol(),
        )
        .map(|b| b.body.into_body())
    }
    let far = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(Mat3::identity(), Vec3::new(640.0, -570.0, 520.0)),
        tol(),
    ));
    let mut poses: Vec<(&str, SketchPlane<f64>, (f64, f64))> = apex_frames()
        .into_iter()
        .map(|(name, place)| (name, place, (1.0, 1.0)))
        .collect();
    poses.extend([
        ("moved a thousand", far, (1.0, 1.0)),
        ("a hundred times", SketchPlane::xy(), (100.0, 100.0)),
        ("a needle", SketchPlane::xy(), (0.05, 1.0)),
        ("flat", SketchPlane::xy(), (1.0, 0.05)),
    ]);
    for theta in [1.5 * core::f64::consts::PI + 0.01, 5.5] {
        for turn in [theta, -theta] {
            for (name, place, legs) in &poses {
                let what = format!("the {turn} wedge, {name}");
                // At ε = 1e-12 the two poses at coordinates of a hundred and
                // more refuse their certification at the build:
                // work/select/an-interval-revolve-of-a-cone-wedge-a-thousand-units-out-refuses-its-cones-rung-3-at-1e-12.md.
                let built = |r: Result<topo::clearance::EmbeddingReport, sweep::RevolveError>,
                             scalar: &str| match r {
                    Ok(report) => Some(report),
                    Err(e)
                        if matches!(*name, "moved a thousand" | "a hundred times")
                            && tol().eps() < 1e-11
                            && at_the_build_limit(&e) =>
                    {
                        println!(
                            "SKIPPED (ε = {:e}): {what} at {scalar} does not build: {e}",
                            tol().eps()
                        );
                        None
                    }
                    Err(e) => panic!("{what} builds at {scalar}: {e}"),
                };
                let float = built(
                    wedge::<f64>(place, *legs, turn)
                        .map(|b| door(&b, DEFAULT_MAX_CELL_DEPTH, BUDGET)),
                    "f64",
                );
                let twin = built(
                    wedge::<Interval>(place, *legs, turn)
                        .map(|b| door(&b, DEFAULT_MAX_CELL_DEPTH, BUDGET)),
                    "Interval",
                );
                println!(
                    "[apex wedge] {what}: f64 {:?} | Interval {:?}",
                    float.as_ref().map(|t| (&t.verdict, t.receipt)),
                    twin.as_ref().map(|t| (&t.verdict, t.receipt))
                );
                for (scalar, report) in [("f64", float.as_ref()), ("Interval", twin.as_ref())]
                    .into_iter()
                    .filter_map(|(scalar, r)| r.map(|r| (scalar, r)))
                {
                    assert_eq!(
                        report.verdict,
                        Embedding::Embedded,
                        "{what} at {scalar} is embedded"
                    );
                }
            }
        }
    }
}

// ------------------------------- the ask ends at its first finding

/// **A plate whose void touches its top**: the box `(0, 3) × (0, 2) ×
/// (0, 1)` less a pyramid hanging from `(1.5, 1, 1)`, a point of the
/// top's interior. The void's apex and the top are one point of space
/// and two of the boundary, so the body is not embedded; its faces are
/// all planes.
fn void_touching_plate<T: Scalar>() -> topo::AtRestBody<T> {
    void_touching_plate_at([0.0; 3], false, 0)
}

/// [`void_touching_plate`] built `o` off the origin, mirrored in `x`
/// where `mirror` (built so, not moved), its void's base listed from
/// its vertex `from`.
///
/// `topo`'s `held::tests::wall_at` copies a wall of this void (its
/// base's last two corners and its apex); a move here is owed there.
fn void_touching_plate_at<T: Scalar>(
    o: [f64; 3],
    mirror: bool,
    from: usize,
) -> topo::AtRestBody<T> {
    use sweep::test_support::{brick, finished};
    use topo::test_support::meeting::{Pose, posed_pyramid_at};
    let plate = finished(
        "the plate",
        brick::<T>(
            (o[0], 3.0 + o[0]),
            (o[1], 2.0 + o[1]),
            (o[2], 1.0 + o[2]),
            tol(),
        ),
        tol(),
    );
    let x = |x: f64| if mirror { 3.0 - x } else { x };
    let mut base: Vec<[f64; 3]> = [[1.2, 0.8, 0.5], [1.9, 0.9, 0.5], [1.4, 1.3, 0.5]]
        .iter()
        .map(|p| [x(p[0]) + o[0], p[1] + o[1], p[2] + o[2]])
        .collect();
    if mirror {
        base.reverse();
    }
    base.rotate_left(from);
    let void = posed_pyramid_at::<T>(
        &base,
        [x(1.5) + o[0], 1.0 + o[1], 1.0 + o[2]],
        &Pose::rest(),
        tol(),
    );
    built(topo::subtract(&plate, &void, tol()))
}

/// The verdict door over every face of `body`.
fn door<T: Scalar + geom_core::Certifies>(
    body: &Body<T>,
    max_cell_depth: u32,
    max_cell_pairs: usize,
) -> topo::clearance::EmbeddingReport {
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    let report = topo::clearance::embedded(
        &side,
        band(),
        Budgets {
            max_cell_depth,
            max_cell_pairs,
        },
    );
    assert!(
        report.receipt.holds(),
        "the receipt adds up: {:?}",
        report.receipt
    );
    report
}

/// **Whether a body passes is the shape's; which finding a body that
/// does not pass is told is the walk's.** The plate whose void touches
/// its top, placed many ways: set off the origin, mirrored, its void's
/// base listed from each vertex, its faces asked in their order and
/// reversed, and turned about four axes. No placement reads `Embedded`.
/// Some are told the crossing and some the refusal met before it, at
/// each scalar: the level a pair is refused at moves with the pose, and
/// the candidates' order with the faces'. A refusal is a finding, so the
/// ask ends at it: every refusal counts one cell pair refused and leaves
/// pairs abandoned. Red where a placement holds, where one class is all
/// there is, which would be a claim the engine does not make, and where
/// a refusal is kept and the walk reads on.
#[test]
fn a_body_that_does_not_pass_is_told_one_finding_or_another_and_never_that_it_holds() {
    use geom_core::{Affine3, Vec3};
    use topo::clearance::Embedding;
    fn class<T: Scalar + geom_core::Certifies>(body: &Body<T>, reversed: bool) -> &'static str {
        let mut faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        if reversed {
            faces.reverse();
        }
        let report = topo::clearance::embedded(
            &Side { body, faces },
            band(),
            Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: BUDGET,
            },
        );
        match report.verdict {
            Embedding::Embedded => "embedded",
            Embedding::Crosses(_) => "crosses",
            Embedding::Refused(_) => {
                let r = report.receipt;
                assert!(
                    r.refused == 1 && r.abandoned > 0,
                    "a refusal ends the ask: {r:?}"
                );
                "refused"
            }
        }
    }
    fn at<T: Scalar + geom_core::Certifies>() -> BTreeSet<&'static str> {
        let mut seen = BTreeSet::new();
        for o in [
            [0.0; 3],
            [100.0, 100.0, 100.0],
            [900.0, -900.0, 900.0],
            [0.1, 0.0, 0.0],
        ] {
            // At `Interval` below ε = 1e-9 the body 900 off does not
            // build: `newell_plane` escalates in the fixture
            // (`work/oneround/a-planar-body-two-hundred-off-the-origin-does-not-finish-at-interval-at-1e-12.md`).
            let unbuilt_at_interval = o == [900.0, -900.0, 900.0];
            if T::NAME == Interval::NAME && tol().eps() < 1e-9 && unbuilt_at_interval {
                println!(
                    "SKIPPED at Interval, eps {}: the placement {o:?} off the origin does not build",
                    tol().eps()
                );
                continue;
            }
            for mirror in [false, true] {
                for from in 0..3 {
                    let body = void_touching_plate_at::<T>(o, mirror, from).into_body();
                    for reversed in [false, true] {
                        let c = class(&body, reversed);
                        assert_ne!(
                            c, "embedded",
                            "off {o:?}, mirrored {mirror}, base from {from}, reversed {reversed}"
                        );
                        seen.insert(c);
                    }
                }
            }
        }
        let rest = void_touching_plate::<T>();
        for axis in [
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.3, -0.5, 0.8),
            Vec3::new(1.0, 1.0, 1.0),
        ] {
            for k in 1..12 {
                let angle = f64::from(k) * core::f64::consts::PI / 12.0;
                let turn = Affine3::rotation_about_axis(Point3::origin(), axis.normalize(), angle);
                let c = class(&moved(&rest, &turn).into_body(), false);
                assert_ne!(c, "embedded", "turned {k}π/12 about {axis:?}");
                seen.insert(c);
            }
        }
        seen
    }
    let both = BTreeSet::from(["crosses", "refused"]);
    assert_eq!(at::<f64>(), both, "f64");
    assert_eq!(at::<Interval>(), both, "Interval");
}

/// **The plate built 100 off the origin is told a finding at
/// `Interval`, at every ε**: it is not embedded. Its void's walls are Newell planes whose frames are enclosures about
/// 1.8e-12 wide there, so a station of a wall's own chart reads its
/// residual undecided at ε = 1e-12, centred on zero. Red, in a debug
/// build (the region door's held check runs there only), where that check
/// asks a constructed point's enclosure to read `Zero`.
#[test]
fn the_plate_built_far_off_the_origin_is_told_a_finding_at_interval() {
    use topo::clearance::Embedding;
    let body = void_touching_plate_at::<Interval>([100.0; 3], false, 0).into_body();
    let report = door(&body, DEFAULT_MAX_CELL_DEPTH, BUDGET);
    assert_ne!(
        report.verdict,
        Embedding::Embedded,
        "the plate 100 off is told a finding"
    );
}

/// **A refusal for a budget names the two faces it was made on, and
/// ends the ask**, on the plate above at three dials, each read alike
/// at both scalars:
///
/// - no depth: the first touching pair that would split refuses
///   `Uncertified` across the edge its two faces share;
/// - five cell pairs: the first level does not fit, and the refusal
///   names its first pair, the first face's, with the whole level
///   abandoned and no pair counted refused, since none was examined;
/// - two hundred cell pairs: a split is refused at admission.
///
/// The first and third refuse one cell pair and abandon the rest. Red
/// where a site keeps its refusal and reads on: the receipt then
/// refuses more, or abandons nothing.
#[test]
fn a_budget_refusal_names_its_two_faces_and_ends_the_ask() {
    use topo::clearance::{Embedding, EmbeddingReport};
    fn at<T: Scalar + geom_core::Certifies>() -> [EmbeddingReport; 3] {
        let plate = void_touching_plate::<T>().into_body();
        let first = plate.faces().next().unwrap().0;
        let by_kind = plate.faces().count();

        let no_depth = door(&plate, 0, BUDGET);
        let Embedding::Refused(EngineRefusal::Uncertified {
            a,
            b,
            shared: SharedCell::Edge(_),
            ..
        }) = no_depth.verdict
        else {
            panic!("no depth refuses a touching pair: {:?}", no_depth.verdict)
        };
        assert!(
            touching_pairs(&plate).contains(&(a.min(b), a.max(b))),
            "{a:?} and {b:?} touch"
        );

        let no_level = door(&plate, DEFAULT_MAX_CELL_DEPTH, 5);
        let Embedding::Refused(EngineRefusal::Budget {
            stop: CellBudget::Pairs { max_cell_pairs: 5 },
            a,
            b,
        }) = no_level.verdict
        else {
            panic!("the first level does not fit: {:?}", no_level.verdict)
        };
        assert!(
            a == first && a != b,
            "the first pair of the level: {a:?} {b:?}"
        );

        for (what, r, refused) in [("no depth", &no_depth, 1), ("no level", &no_level, 0)] {
            let r = r.receipt;
            assert_eq!(
                (r.discharged, r.by_kind, r.refused, r.splits),
                (by_kind, by_kind, refused, 0),
                "{what}: every face by its kind, then the refusal: {r:?}"
            );
            assert_eq!(
                r.abandoned,
                r.candidates - by_kind - refused,
                "{what}: the rest of the level abandoned: {r:?}"
            );
        }

        let admission = door(&plate, DEFAULT_MAX_CELL_DEPTH, 200);
        let Embedding::Refused(EngineRefusal::Budget {
            stop: CellBudget::Pairs {
                max_cell_pairs: 200,
            },
            a,
            b,
        }) = admission.verdict
        else {
            panic!("a split is refused at admission: {:?}", admission.verdict)
        };
        assert_ne!(a, b, "two faces");
        let r = admission.receipt;
        assert!(r.splits > 0 && r.refused == 1 && r.abandoned > 0, "{r:?}");
        [no_depth, no_level, admission]
    }
    assert_eq!(at::<Interval>(), at::<f64>(), "one report at both scalars");
}

/// **A metric bound still reads a violation past a refusal.** The
/// plate's top against the four faces of its void, asked `≥ 0.2`, beside
/// two bricks 30ε across grafted onto the plate's body exactly `0.2`
/// apart, one on each side of the ask: the bricks' facing cells come to
/// lie wholly inside the band about the bound, a sliver no refinement
/// decides, on the first levels, before the pair at the void's apex is
/// confirmed closer than the bound. The verdict is the violation, and
/// the walk ends at it, which is why pairs are left abandoned. Red,
/// refused as a sliver, where the metric question ends at its first
/// refusal as the embedding question does.
#[test]
fn a_metric_bound_reads_a_violation_past_a_refusal() {
    fn at<T: Scalar>() -> String {
        use sweep::test_support::{brick, finished};
        let mut plate = void_touching_plate::<T>().into_body();
        // The box's six faces come first, its top the first of them;
        // the void's four follow.
        let faces: Vec<FaceKey> = plate.faces().map(|(k, _)| k).collect();
        let (top, void) = (faces[0], faces[6..].to_vec());
        let s = 30.0 * tol().eps();
        let mut graft = |x0: f64| -> Vec<FaceKey> {
            let before: BTreeSet<FaceKey> = plate.faces().map(|(k, _)| k).collect();
            let tiny = finished(
                "a brick 30ε across",
                brick::<T>((x0, x0 + s), (0.5, 0.5 + s), (0.5, 0.5 + s), tol()),
                tol(),
            );
            topo::instance::graft_disjoint(&mut plate, &tiny.into_body())
                .expect("the brick is off the plate");
            plate
                .faces()
                .map(|(k, _)| k)
                .filter(|k| !before.contains(k))
                .collect()
        };
        let near = graft(5.0);
        let far = graft(5.0 + s + 0.2);
        let window = |faces: Vec<FaceKey>, which| {
            Side {
                body: &plate,
                faces,
            }
            .windows(which)
            .expect("every face has a window")
        };
        let bound = ClearanceBound::at_least(0.2);
        let sweep = Sweep {
            bound,
            band: band(),
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: BUDGET,
            },
        };
        let report = sweep.run(
            &window([vec![top], near].concat(), Which::First),
            &window([void, far].concat(), Which::Second),
            true,
            verify(&plate, bound),
        );
        assert!(report.receipt.holds(), "{:?}", report.receipt);
        assert!(
            report.receipt.refused > 0 && report.receipt.abandoned > 0,
            "a refusal is met, and the walk ends after the violation: {:?}",
            report.receipt
        );
        let BoundVerdict::Violated(w) = &report.verdict else {
            panic!("the violation outranks the refusal: {:?}", report.verdict)
        };
        assert!(w.d < 0.2, "closer than the bound: {w:?}");
        class(&report.verdict)
    }
    assert_eq!(at::<Interval>(), at::<f64>());
}

/// **Two bricks of one body a distance inside the band apart refuse as
/// a sliver, on the two facing faces, and the ask ends there.** Bricks
/// 30ε across, grafted 3ε apart: the facing cells' separation comes to
/// lie wholly inside the band, where no refinement decides it. Red
/// where the sliver is kept and the walk reads on: more is refused, or
/// nothing abandoned.
#[test]
fn two_bricks_a_band_apart_refuse_as_a_sliver_and_end_the_ask() {
    use sweep::test_support::{brick, finished};
    use topo::clearance::{Embedding, SELF_INTERSECTION_GAP};
    let e = tol().eps();
    let s = 30.0 * e;
    let tiny = |x0: f64, y0: f64| {
        finished(
            "a brick 30ε across",
            brick::<f64>((x0, x0 + s), (y0, y0 + s), (0.5, 0.5 + s), tol()),
            tol(),
        )
        .into_body()
    };
    let mut body = tiny(0.5, 0.5);
    let first: BTreeSet<FaceKey> = body.faces().map(|(k, _)| k).collect();
    topo::instance::graft_disjoint(&mut body, &tiny(0.5 + s + 3.0 * e, 0.5 + 0.3 * s))
        .expect("the bricks are disjoint");
    let report = door(&body, DEFAULT_MAX_CELL_DEPTH, BUDGET);
    let Embedding::Refused(EngineRefusal::Sliver { predicate, a, b }) = report.verdict else {
        panic!("the gap refuses as a sliver: {:?}", report.verdict)
    };
    assert_eq!(predicate, SELF_INTERSECTION_GAP);
    assert_ne!(
        first.contains(&a),
        first.contains(&b),
        "one face of each brick: {a:?} {b:?}"
    );
    let r = report.receipt;
    assert!(r.refused == 1 && r.abandoned > 0, "{r:?}");
}

// ------------------------------------------------- a reflex corner

/// **A rod notched across its rim**: the rod of radius `r` round the
/// `z` axis over `z ∈ [0, r]` less the brick
/// `[−0.3r, 0.3r] × [0.4r, 2r] × [0.5r, 2r]`, or its mirror image in
/// `y = 0` (built so, not moved). The rod's wall keeps two reflex corners
/// at the notch's floor, where a ruling (shared with a notch wall
/// `x = ±0.3r`) meets an arc (shared with the floor `z = 0.5r`), and the
/// wall has 270° about each.
fn notched_rod_of<T: Scalar>(r: f64, mirrored: bool) -> topo::AtRestBody<T> {
    use sweep::test_support::{brick, finished, prism_at};
    let at = T::from_f64;
    let rod = finished(
        "the rod",
        prism_at(
            vec![
                (Point2::new(at(-r), T::zero()), T::one()),
                (Point2::new(at(r), T::zero()), T::one()),
            ],
            T::zero(),
            at(r),
            tol(),
        ),
        tol(),
    );
    let y = if mirrored {
        (-2.0 * r, -0.4 * r)
    } else {
        (0.4 * r, 2.0 * r)
    };
    let notch = finished(
        "the notch",
        brick::<T>((-0.3 * r, 0.3 * r), y, (0.5 * r, 2.0 * r), tol()),
        tol(),
    );
    built(topo::subtract(&rod, &notch, tol()))
}

/// The unit notched rod.
fn notched_rod<T: Scalar>() -> topo::AtRestBody<T> {
    notched_rod_of(1.0, false)
}

/// The verdict door's answer over every face of `body`.
fn embedding_of<T: Scalar + geom_core::Certifies>(body: &Body<T>) -> topo::clearance::Embedding {
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    let budgets = Budgets {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: BUDGET,
    };
    topo::clearance::embedded(&side, band(), budgets).verdict
}

/// `body` moved by each of `poses` in turn and finished, or `None` where
/// a move leaves a body the finish refuses only for one of two named
/// reads of a thousand-metre placement at the tightest band, which it
/// then says, loudly: the wall's volume at `props_cylinder_circle_off_axis`
/// (`work/nearpoint/a-cylinder-wall-near-the-session-boxs-corner-does-not-finish-at-the-tightest-eps.md`),
/// and, turned there too, its edges' and pcurves' incidence at
/// `pcurve_line_chart_incident` and `pcurve_cylinder_chart_incident`
/// (`work/onehome/a-body-moved-by-a-boxed-turn-refuses-its-edges-and-pcurves-at-first-order-in-the-box.md`).
/// Any other refusal fails the row.
fn posed<T: Scalar>(
    mut body: topo::AtRestBody<T>,
    poses: &[geom_core::Affine3<f64>],
    what: &str,
) -> Option<topo::AtRestBody<T>> {
    for pose in poses {
        let rigid = geom_core::test_support::rigid(&pose.map(T::from_f64), tol());
        let moved = topo::transform_rigid(&body, &rigid).expect("the rod moves");
        body = match <T as topo::AtRestPolicy>::finish(moved, tol()) {
            Ok(b) => b,
            Err(e)
                if e.as_slice().iter().all(|f| {
                    let kind = matches!(
                        f,
                        topo::ValidationError::VolumeUncomputable { .. }
                            | topo::ValidationError::EdgeCertification { .. }
                            | topo::ValidationError::Pcurve { .. }
                    );
                    let read = format!("{f:?}");
                    let named: Vec<&str> = read
                        .split("predicate: Some(\"")
                        .skip(1)
                        .filter_map(|r| r.split('"').next())
                        .collect();
                    kind && !named.is_empty()
                        && named.iter().all(|p| {
                            [
                                "props_cylinder_circle_off_axis",
                                "pcurve_line_chart_incident",
                                "pcurve_cylinder_chart_incident",
                            ]
                            .contains(p)
                        })
                }) =>
            {
                println!(
                    "SKIPPED at ε {:e}: the notched rod {what} does not finish: {e:?}",
                    tol().eps()
                );
                return None;
            }
            Err(e) => panic!("the notched rod {what} does not finish: {e:?}"),
        };
    }
    Some(body)
}

/// **A notched rod holds across its reflex corners in every pose and
/// size**, at both scalars and through the verdict door: radius 1 as
/// built, turned about a generic axis, a hundred units up and across its
/// axis, a quarter turn about `x`, at the corner `(998, −998, 998)` of the
/// session box and turned there; its mirror image as built and turned
/// 2.3 rad about another axis; and radii 0.02 and 30. Each wall cell at a
/// corner is read by the wall's own carrier against the notch's presented
/// face, and that carrier's rate along a ruling is zero at every point:
/// red if it is read from the box's enclosure of the radial unit, whose
/// axial part grows with the cell's width over the radius and with its
/// tilt (the pair refuses `Uncertified { Departure }` at a ruling at every
/// depth), or if the radial term is not a unit (at a radius below 1 the
/// rate is then too narrow, above 1 too wide).
#[test]
fn a_notched_rod_holds_across_its_reflex_corners_in_every_pose() {
    use geom_core::{Affine3, Vec3};
    use topo::clearance::Embedding;

    let turn = || {
        Affine3::rotation_about_axis(
            Point3::new(0.3, -0.2, 0.1),
            Vec3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0),
            0.7,
        )
    };
    let corner = Vec3::new(998.0, -998.0, 998.0);
    let cases: Vec<(&str, f64, bool, Vec<Affine3<f64>>)> = vec![
        ("as built", 1.0, false, vec![]),
        ("turned", 1.0, false, vec![turn()]),
        (
            "a hundred units up its axis",
            1.0,
            false,
            vec![Affine3::translation(Vec3::new(0.0, 0.0, 100.0))],
        ),
        (
            "a hundred units across its axis",
            1.0,
            false,
            vec![Affine3::translation(Vec3::new(100.0, -100.0, 0.0))],
        ),
        (
            "a quarter turn about x",
            1.0,
            false,
            vec![Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                core::f64::consts::FRAC_PI_2,
            )],
        ),
        (
            "at the session box's corner",
            1.0,
            false,
            vec![Affine3::translation(corner)],
        ),
        (
            "turned, at the session box's corner",
            1.0,
            false,
            vec![
                turn(),
                Affine3::translation(-corner + Vec3::new(1.0, -1.0, 1.0)),
            ],
        ),
        ("mirrored", 1.0, true, vec![]),
        (
            "mirrored and turned",
            1.0,
            true,
            vec![Affine3::rotation_about_axis(
                Point3::new(-0.4, 0.7, 0.2),
                Vec3::new(6.0 / 7.0, -2.0 / 7.0, 3.0 / 7.0),
                2.3,
            )],
        ),
        ("of radius 0.02", 0.02, false, vec![]),
        ("of radius 30", 30.0, false, vec![]),
    ];
    for (what, r, mirrored, poses) in cases {
        let (Some(body), Some(twin)) = (
            posed(notched_rod_of::<f64>(r, mirrored), &poses, what),
            posed(notched_rod_of::<Interval>(r, mirrored), &poses, what),
        ) else {
            continue;
        };
        let (body, twin) = (body.into_body(), twin.into_body());
        assert_eq!(
            embedding_of(&body),
            Embedding::Embedded,
            "{what}: the verdict door"
        );
        assert_eq!(
            embedding_of(&twin),
            Embedding::Embedded,
            "{what}: the verdict door at Interval"
        );
        holds_at_both(&format!("the notched rod {what}"), body, twin);
    }
}

/// **A ball through the notched rod's wall beside a reflex corner reads
/// `Violated`**: a ball of radius 0.05 grafted into the notched rod, its
/// centre on the wall's carrier just below the corner at `x = 0.3`, so it
/// crosses the wall where the ruling's line runs on past the corner
/// through the wall's interior. The ball shares no cell with the wall, so
/// this pins the distance arm's reading of a crossing near the corner,
/// not the carrier arm's (its negative is
/// `injective::rate_rows::a_plane_cell_crossing_a_cylinders_carrier_along_a_ruling_is_not_signed`):
/// red if a crossing beside a reflex corner reads embedded.
#[test]
fn a_ball_through_the_wall_beside_a_reflex_corner_reads_violated() {
    let mut body = notched_rod::<f64>().into_body();
    let rho = 0.05;
    let p = |y: f64, bulge: f64| (Point2::new(0.0, y), bulge);
    let (x, z) = (0.3_f64, 0.45);
    let ball = topo::transform_rigid(
        &revolved_about_y_at::<f64>(vec![p(-rho, 1.0), p(rho, 0.0)], Revolution::Full, tol()),
        &geom_core::RigidMotion::translation(geom_core::Vec3::new(x, (1.0 - x * x).sqrt(), z))
            .expect("a translation"),
    )
    .expect("the ball moves");
    topo::graft_disjoint_all_keyed(&mut body, &ball).expect("the ball grafts");
    let report = injective(&body, None, BUDGET);
    assert!(
        matches!(report.verdict, BoundVerdict::Violated(_)),
        "{:?}",
        report.verdict
    );
}

/// **A unit cone whose apex is off the origin holds**: the triangle
/// `(0, 2.7) (0, 1.7) (1, 2.7)` revolved a full turn about `y`, its apex
/// at `y = 1.7`. Each half-cone's stored extent starts at `v =
/// 2.2e-16`, a rounding above the apex. A half-cone holds the apex
/// vertex, so its extent reaches `v = 0` and the shared apex cell is
/// read through the apex's collapsed side. Red, out of pairs on the two
/// half-cones, where the cell is clipped to the stored extent; the same
/// triangle at `y = 0.3` stores `0`.
#[test]
fn a_unit_cone_with_its_apex_off_the_origin_holds() {
    for report in holds_at_both(
        "the cone with its apex at y = 1.7",
        cone_tip(1.7, 1.0, 1.0, false),
        cone_tip(1.7, 1.0, 1.0, false),
    ) {
        assert_eq!(report.receipt.refused, 0, "{:?}", report.receipt);
        assert!(report.receipt.splits < 400, "{:?}", report.receipt);
    }
}

/// A cone tip revolved a full turn about `y`: the triangle with its
/// apex on the axis at `y0 + h` (`up`) or at `y0`, radius `r`.
fn cone_tip<T: Scalar>(y0: f64, r: f64, h: f64, up: bool) -> Body<T> {
    try_cone_tip(y0, r, h, up).unwrap()
}

/// [`cone_tip`], or the builder's refusal.
fn try_cone_tip<T: Scalar>(y0: f64, r: f64, h: f64, up: bool) -> Result<Body<T>, String> {
    let pts = if up {
        [(0.0, y0), (r, y0), (0.0, y0 + h)]
    } else {
        [(0.0, y0 + h), (0.0, y0), (r, y0 + h)]
    };
    let profile = validated::<T>(&[ProfileLoop::polygon(pts.map(|(x, y)| Point2::new(x, y)))]);
    revolve(&profile, axis_y(), Revolution::Full, tol())
        .map(|b| b.body.into_body())
        .map_err(|e| format!("{e:?}"))
}

/// A rigid turn about a generic axis and a translation, as a sketch
/// plane: three frames, and the plain one at `0`.
fn turned_plane(k: usize) -> SketchPlane<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    if k == 0 {
        return SketchPlane::xy();
    }
    let (ax, th, off) = [
        ((1.0, 2.0, 3.0), 0.7, (0.3, -1.1, 2.9)),
        ((-0.4, 0.1, 0.9), 2.3, (17.0, 4.2, -8.8)),
        ((0.0, 1.0, 1.0), 1.1, (0.0, 0.0, 0.0)),
    ][k - 1];
    let a = Vec3::new(ax.0, ax.1, ax.2) * (1.0 / (ax.0 * ax.0 + ax.1 * ax.1 + ax.2 * ax.2).sqrt());
    let (c, s) = (f64::cos(th), f64::sin(th));
    let col = |e: Vec3<f64>| e * c + a.cross(e) * s + a * (a.dot(e) * (1.0 - c));
    let m = Mat3::from_cols(
        col(Vec3::new(1.0, 0.0, 0.0)),
        col(Vec3::new(0.0, 1.0, 0.0)),
        col(Vec3::new(0.0, 0.0, 1.0)),
    );
    SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(m, Vec3::new(off.0, off.1, off.2)),
        tol(),
    ))
}

/// A profile of straight and bulged segments on `plane`, revolved a
/// full turn about its `y`, at `f64`.
fn revolved_on(plane: SketchPlane<f64>, verts: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    let profile = Profile::new(plane, vec![profile::test_support::bulge_loop(verts)])
        .validate(tol())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

/// **Cones a hundred units off the origin and fifty long hold**, where
/// a meridian's image ends an ulp of its own far end off the apex:
/// - the triangle `r = 20`, `h = 50` with its apex down at `y = 101.3`,
///   at both scalars where its `Interval` twin builds (at ε = 1e-12 the
///   revolve refuses it, and the row says so). Its half-cones store `v`
///   from `7.1e-15`, an ulp of 53.85;
/// - the same shape with its apex up at `y = 50`, its axial door's
///   cavity at `t = 1` (`f64`: the door's fixture is);
/// - the triangle `r = 0.37`, `h = 2.3` with its apex up at `y = 103.6`,
///   its cavity at `t = 0.0185`, whose half-cones store `v` to
///   `-1.4e-14`.
///
/// Each stored end is at the apex by the band, so the extent reaches
/// `v = 0`, and each meridian ends at the apex's vertex, so it runs on
/// to the apex's side. Red, `Uncertified` on the departure arm, where
/// the extent moves only across `chart_slack`, or the meridian must
/// cover its side to within the cover's own `few_ulps`.
#[test]
fn cones_far_from_the_origin_hold_across_their_apexes() {
    use crate::common::latitude_seam::door_cavity;
    let long = "the long cone with its apex at y = 101.3";
    let reports = match try_cone_tip::<Interval>(101.3, 20.0, 50.0, false) {
        Ok(twin) => holds_at_both(long, cone_tip(101.3, 20.0, 50.0, false), twin).to_vec(),
        // At ε = 1e-12, and only there, the `Interval` revolve refuses its
        // own pcurve at the envelope's `FidelityV` term
        // (`work/pcert/an-interval-revolve-of-a-long-cone-a-hundred-units-out-escalates-its-pcurve-envelope-at-1e-12.md`).
        // Any other refusal, or that one at another ε, fails the row.
        Err(refusal) => {
            assert!(
                tol().eps() < 1e-11
                    && refusal.contains("EnvelopeTerm(FidelityV)")
                    && refusal.contains("pcurve_envelope"),
                "{long}: the Interval twin is unbuilt at ε = {:e}: {refusal}",
                tol().eps()
            );
            println!("SKIPPED: {long} is asked at f64 alone, its Interval twin unbuilt: {refusal}");
            let body = cone_tip::<f64>(101.3, 20.0, 50.0, false);
            let report = injective(&body, None, BUDGET);
            assert_holds(long, &report, &body);
            vec![report]
        }
    };
    for report in reports {
        assert_eq!(report.receipt.refused, 0, "{:?}", report.receipt);
        assert!(report.receipt.splits < 500, "{:?}", report.receipt);
    }
    for (what, y0, r, h, t, ceiling) in [
        ("the long cone's cavity", 0.0, 20.0, 50.0, 1.0, 500),
        (
            "the cone's cavity at y = 101.3",
            101.3,
            0.37,
            2.3,
            0.0185,
            700,
        ),
    ] {
        let cavity = door_cavity(&cone_tip(y0, r, h, true), t);
        let report = injective(&cavity, None, BUDGET);
        assert_holds(what, &report, &cavity);
        assert_eq!(report.receipt.refused, 0, "{what}: {:?}", report.receipt);
        assert!(
            report.receipt.splits < ceiling,
            "{what}: {:?}",
            report.receipt
        );
    }
}

/// **A ball a hundred units off the origin holds at both scalars, and
/// balls and cones in turned frames hold**: the unit ball about
/// `y = 101.3`, and the unit ball on each of the three turned sketch
/// planes, each one sphere face (a full revolve's wall is one band)
/// whose self-pair is discharged by kind, its poles its wrap edge's tips
/// ([`holds_alone`]); the
/// unit cone tip on each of three sketch planes turned about a generic
/// axis and moved ([`turned_plane`]); and the slender cone (`r = 0.37`,
/// `h = 2.3`, apex at `y = 103.6`) on all three, at `f64`. A sphere's
/// poles and a cone's apex are read off the face's own vertices wherever
/// the body is.
///
/// The unit cone holds in each frame within 400 splits, as in the plain
/// frame's 174: its cone face's window ends at the apex. Red where a
/// window reaches past the apex into the other nappe, as one read off
/// the boundary's world box does in a turned frame (thousands of splits
/// on the first plane, a `Departure` refusal on the third). A cone here
/// that refuses otherwise is
/// `work/embed/two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame.md`'s
/// evidence.
#[test]
fn singular_points_hold_far_from_the_origin_and_in_turned_frames() {
    fn ball<T: Scalar>(y0: f64) -> Body<T> {
        let p = |y: f64, b: f64| (Point2::new(0.0, y).map(T::from_f64), T::from_f64(b));
        revolved_about_y_at(
            vec![p(y0 - 1.0, 1.0), p(y0 + 1.0, 0.0)],
            Revolution::Full,
            tol(),
        )
    }
    let (float, interval): (Body<f64>, Body<Interval>) = (ball(101.3), ball(101.3));
    let (report, twin) = (
        injective(&float, None, BUDGET),
        injective(&interval, None, BUDGET),
    );
    same_class("the ball about y = 101.3", &report, &twin);
    holds_alone("the ball about y = 101.3", &report, &float);
    holds_alone("the ball about y = 101.3 at Interval", &twin, &interval);
    let at = |x: f64, y: f64| Point2::new(x, y);
    for k in 1..4 {
        let (a, b, c) = (at(0.0, -1.0), at(0.0, 1.0), at(0.0, 0.0));
        let ball = revolved_on(turned_plane(k), vec![(a, bulge(a, b, c)), (b, 0.0)]);
        let report = injective(&ball, None, BUDGET);
        holds_alone(&format!("the ball in frame {k}"), &report, &ball);
    }
    let unit = || {
        vec![
            (at(0.0, 0.0), 0.0),
            (at(1.0, 0.0), 0.0),
            (at(0.0, 1.0), 0.0),
        ]
    };
    let slender = || {
        vec![
            (at(0.0, 101.3), 0.0),
            (at(0.37, 101.3), 0.0),
            (at(0.0, 103.6), 0.0),
        ]
    };
    for (what, k, verts, ceiling) in [
        ("the unit cone", 1, unit(), 400),
        ("the unit cone", 2, unit(), 400),
        ("the unit cone", 3, unit(), 400),
        ("the slender cone", 1, slender(), usize::MAX),
        ("the slender cone", 2, slender(), usize::MAX),
        ("the slender cone", 3, slender(), usize::MAX),
    ] {
        let cone = revolved_on(turned_plane(k), verts);
        let report = injective(&cone, None, BUDGET);
        assert_holds(&format!("{what} in frame {k}"), &report, &cone);
        assert_eq!(
            report.receipt.refused, 0,
            "{what} in frame {k}: {:?}",
            report.receipt
        );
        assert!(
            report.receipt.splits < ceiling,
            "{what} in frame {k} within {ceiling} splits: {:?}",
            report.receipt
        );
    }
}

/// The affine chart coordinates of `surface` at `p` that a face's window
/// bounds from its boundary: a plane's `u` and `v`, a cylinder's axial
/// `v`, a cone's slant `v`, each `None` where the coordinate is periodic
/// or the carrier another kind.
fn affine_coordinates(surface: &Surface<f64>, p: Point3<f64>) -> [Option<f64>; 2] {
    match surface {
        Surface::Plane { frame } => {
            let (o, u) = (frame.origin(), frame.u_ref());
            let w = frame.axis().cross(u);
            [
                Some((p - o).dot(u) / u.norm_squared()),
                Some((p - o).dot(w) / w.norm_squared()),
            ]
        }
        Surface::Cylinder { frame, .. } => {
            let a = frame.axis();
            [None, Some((p - frame.origin()).dot(a) / a.norm_squared())]
        }
        Surface::Cone {
            frame, half_angle, ..
        } => [
            None,
            Some((p - frame.origin()).dot(frame.axis()) / half_angle.cos()),
        ],
        _ => [None, None],
    }
}

/// The windows of every face of `body`, as `(face, u, v)`.
fn window_rects(what: &str, body: &Body<f64>) -> Vec<topo::clearance::WindowRect> {
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    Side { body, faces }
        .windows(Which::First)
        .unwrap_or_else(|e| panic!("{what}: every face has a window: {e:?}"))
        .rects()
}

/// The range of each of [`affine_coordinates`] over `face`'s boundary,
/// each half-edge's certified carrier sampled at 401 parameters over its
/// span; `None` for a coordinate the face does not bound.
fn boundary_ranges(body: &Body<f64>, face: FaceKey) -> [Option<(f64, f64)>; 2] {
    let surface = body.face(face).unwrap().surface().unwrap();
    let mut range: [Option<(f64, f64)>; 2] = [None, None];
    for (lk, _, _) in body.face(face).unwrap().loops() {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            let curve = body.get_curve_geom(edge.curve).unwrap();
            let curve = curve.certified().expect("a certified edge");
            let (t0, t1) = curve.params();
            for i in 0..=400 {
                let t = t0 + (t1 - t0) * f64::from(i) / 400.0;
                let p = curve.carrier().eval(t);
                for (r, c) in range.iter_mut().zip(affine_coordinates(surface, p)) {
                    if let Some(c) = c {
                        *r = Some(r.map_or((c, c), |(lo, hi)| (lo.min(c), hi.max(c))));
                    }
                }
            }
        }
    }
    range
}

/// **Every face's window holds its boundary, in every frame**: for each
/// plane, cylinder and cone face, the window's range of each affine chart
/// coordinate ([`affine_coordinates`]) holds that coordinate over the
/// face's boundary, its edges sampled at 401 parameters each, to `1e-12`.
/// Bodies: the unit cone, a frustum, a ring (a cylinder pair and two
/// annuli on circle edges) and a cone ring, each revolved on the plain and
/// the three turned sketch planes ([`turned_plane`]); the oblique cut and
/// the rod less a tilted slab, turned; the torus cut off its axis, plain
/// and turned; the one-face torus cut, turned. Red where a window is
/// narrower than its boundary on any one coordinate, which leaves part
/// of a face outside the cells the walk reads.
#[test]
fn every_face_window_holds_its_boundary_in_every_frame() {
    use geom_core::{Affine3, Vec3};
    let at = |x: f64, y: f64| Point2::new(x, y);
    let mut bodies: Vec<(String, Body<f64>)> = Vec::new();
    for k in 0..4 {
        for (what, pts) in [
            ("the unit cone", &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)][..]),
            (
                "the frustum",
                &[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)][..],
            ),
            (
                "the ring",
                &[(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)][..],
            ),
            (
                "the cone ring",
                &[(0.5, 0.0), (1.0, 0.0), (0.7, 1.0), (0.5, 1.0)][..],
            ),
        ] {
            let verts = pts.iter().map(|&(x, y)| (at(x, y), 0.0)).collect();
            bodies.push((
                format!("{what} in frame {k}"),
                revolved_on(turned_plane(k), verts),
            ));
        }
    }
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.3, -0.2, 0.1),
        Vec3::new(0.48, -0.6, 0.64),
        1.0,
    );
    for (what, body) in [
        (
            "the oblique cut, turned",
            moved(&oblique_cut::<f64>(), &turn),
        ),
        (
            "the rod less a tilted slab, turned",
            moved(&rod_less_a_tilted_slab::<f64>((-3.0, 3.0)), &turn),
        ),
        (
            "the torus cut off its axis",
            torus_cut_off_its_axis::<f64>(),
        ),
        (
            "the torus cut off its axis, turned",
            moved(&torus_cut_off_its_axis::<f64>(), &turn),
        ),
        (
            "the one-face torus cut, turned",
            moved(&one_face_torus_cut::<f64>(), &turn),
        ),
    ] {
        bodies.push((what.to_owned(), body.into_body()));
    }
    let mut read = 0;
    for (what, body) in &bodies {
        for (face, u, v) in window_rects(what, body) {
            for (name, win, range) in [("u", u, 0), ("v", v, 1)]
                .map(|(name, win, i)| (name, win, boundary_ranges(body, face)[i]))
            {
                let Some((lo, hi)) = range else { continue };
                read += 1;
                assert!(
                    win.0 <= lo + 1e-12 && hi <= win.1 + 1e-12,
                    "{what}, face {face:?}, {name}: the window {win:?} holds the boundary's \
                     [{lo}, {hi}]"
                );
            }
        }
    }
    assert!(read > 100, "the coordinates read: {read}");
}

/// **A turned cone's window runs from its apex to its rim**: the unit
/// cone revolved on the plain and each turned sketch plane
/// ([`turned_plane`]), apex up and apex down. A full revolve's wall is
/// one band, one cone face whose boundary runs out to the apex along its
/// wrap edge and back, so its slant window is `(−√2, 0)` with the apex
/// up and `(0, √2)` with it down, each end to `1e-12`. Red where the window reaches past the apex into the other
/// nappe or past the rim, and where it falls short of either.
#[test]
fn a_turned_cones_window_runs_from_its_apex_to_its_rim() {
    let at = |x: f64, y: f64| Point2::new(x, y);
    let rim = core::f64::consts::SQRT_2;
    for k in 0..4 {
        for (what, verts, want) in [
            (
                "apex up",
                vec![
                    (at(0.0, 0.0), 0.0),
                    (at(1.0, 0.0), 0.0),
                    (at(0.0, 1.0), 0.0),
                ],
                (-rim, 0.0),
            ),
            (
                "apex down",
                vec![
                    (at(0.0, 2.7), 0.0),
                    (at(0.0, 1.7), 0.0),
                    (at(1.0, 2.7), 0.0),
                ],
                (0.0, rim),
            ),
        ] {
            let cone = revolved_on(turned_plane(k), verts);
            let mut walls = 0;
            for (face, _, v) in window_rects(what, &cone) {
                if !matches!(
                    cone.face(face).unwrap().surface().unwrap(),
                    Surface::Cone { .. }
                ) {
                    continue;
                }
                walls += 1;
                assert!(
                    (v.0 - want.0).abs() < 1e-12 && (v.1 - want.1).abs() < 1e-12,
                    "the cone {what} in frame {k}, face {face:?}: its slant window {v:?} is {want:?}"
                );
            }
            assert_eq!(walls, 1, "the cone {what} in frame {k} has one cone face");
        }
    }
}

/// **Review sweep (PR 7115): cone tips revolved off the origin, and their
/// door cavities**, f64. The triangle with its apex on the axis at the
/// top (`up`: `(0, y0) (r, y0) (0, y0 + h)`) or the bottom (`(0, y0 + h)
/// (0, y0) (r, y0 + h)`), revolved a full turn about `y`; each body, and
/// its `door_cavity` at `t = 0.05·min(r, h)`, asked `injective`. It prints
/// one line per body and asserts nothing.
///
/// At `1bf4de792` (PR 7115's head), 120 asked, every door took its cut.
/// Twelve refuse `Budget(Pairs)` (discharged / refused / splits); the
/// other 108 hold with `refused == 0`:
/// - `y0 = 0`, `(20, 50)`, apex up, cavity: 41,452 / 24,084 / 65,530
/// - `y0 = 0`, `(0.013, 0.9)`, apex down, body: 42,167 / 23,369 / 65,530
/// - `y0 = 0.3`, `(0.013, 0.9)`, apex down, body: 42,167 / 23,369 / 65,530
/// - `y0 = 1.7`, `(1, 1)`, apex down, body: 41,321 / 24,215 / 65,530
/// - `y0 = 1.7`, `(0.37, 2.3)`, apex down, cavity: 41,472 / 24,064 / 65,530
/// - `y0 = 1.7`, `(0.013, 0.9)`, apex down, body: 42,167 / 23,369 / 65,530
/// - `y0 = -2.9`, `(1, 1)`, apex down, cavity: 41,436 / 24,100 / 65,530
/// - `y0 = -2.9`, `(0.013, 0.9)`, apex down, body: 42,167 / 23,369 / 65,530
/// - `y0 = 13.1`, `(0.013, 0.9)`, apex down, body: 42,167 / 23,369 / 65,530
/// - `y0 = 101.3`, `(0.37, 2.3)`, apex up, cavity: 41,623 / 23,913 / 65,530
/// - `y0 = 101.3`, `(20, 50)`, apex down, body: 41,612 / 23,924 / 65,530
/// - `y0 = 101.3`, `(0.013, 0.9)`, apex down, body: 42,033 / 23,503 / 65,530
///
/// On PR 7115's final head all 120 hold at `refused == 0`: the apex is
/// read in `face_extent` by the band, and a meridian that ends at the
/// apex's vertex runs on to the apex's side. Three of the twelve are
/// pinned by `cones_far_from_the_origin_hold_across_their_apexes`.
///
/// Run with `--run-ignored only --no-capture`, about 3 min at the dev
/// profile.
#[test]
#[ignore = "review sweep: prints, asserts nothing"]
fn review_sweep_cone_tips_off_the_origin() {
    use crate::common::latitude_seam::door_cavity;
    let mut n = 0;
    for &y0 in &[0.0, 0.3, 1.7, -2.9, 13.1, 101.3] {
        for &(r, h) in &[
            (1.0, 1.0),
            (0.37, 2.3),
            (3.1, 0.21),
            (20.0, 50.0),
            (0.013, 0.9),
        ] {
            for up in [true, false] {
                let pts = if up {
                    [(0.0, y0), (r, y0), (0.0, y0 + h)]
                } else {
                    [(0.0, y0 + h), (0.0, y0), (r, y0 + h)]
                };
                let profile =
                    validated::<f64>(&[ProfileLoop::polygon(pts.map(|(x, y)| Point2::new(x, y)))]);
                let Ok(built) = revolve(&profile, axis_y(), Revolution::Full, tol()) else {
                    println!("SWEEP y0={y0} r={r} h={h} up={up}: revolve refused");
                    continue;
                };
                let body = built.body.into_body();
                let rep = injective(&body, None, BUDGET);
                println!(
                    "SWEEP y0={y0} r={r} h={h} up={up} body: {:?} {:?}",
                    rep.verdict, rep.receipt
                );
                n += 1;
                let t = 0.05 * r.min(h);
                match std::panic::catch_unwind(|| door_cavity(&body, t)) {
                    Ok(cavity) => {
                        let rep = injective(&cavity, None, BUDGET);
                        println!(
                            "SWEEP y0={y0} r={r} h={h} up={up} cavity: {:?} {:?}",
                            rep.verdict, rep.receipt
                        );
                        n += 1;
                    }
                    Err(_) => println!("SWEEP y0={y0} r={r} h={h} up={up} cavity: door panicked"),
                }
            }
        }
    }
    println!("SWEEP asked {n}");
}

/// **Review sweep, delta of PR 7115**: shapes the first sweep did not
/// vary. It prints one line per case and asserts nothing. Of the cases
/// that build, 46 hold and 22 refuse on the PR's final head: six
/// turned-frame cone cases, on
/// `work/embed/two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame.md`,
/// which gives each main's set, as it has moved with main; and the
/// sixteen turned ice-cream cones, on their sphere caps, since main's
/// carrier frames
/// (`work/embed/a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval.md`).
#[test]
#[ignore = "review sweep: prints, asserts nothing"]
fn review_sweep_delta_shapes() {
    use crate::common::latitude_seam::door_cavity;
    let ask = |what: &str, body: Body<f64>, t: f64| {
        let rep = injective(&body, None, BUDGET);
        println!("DELTA {what} body: {:?} {:?}", rep.verdict, rep.receipt);
        match std::panic::catch_unwind(|| door_cavity(&body, t)) {
            Ok(c) => {
                let rep = injective(&c, None, BUDGET);
                println!("DELTA {what} cavity: {:?} {:?}", rep.verdict, rep.receipt);
            }
            Err(_) => println!("DELTA {what} cavity: door panicked"),
        }
    };
    let build = |plane: SketchPlane<f64>, verts: Vec<(Point2<f64>, f64)>| {
        let lp = profile::test_support::bulge_loop(verts);
        let profile = Profile::new(plane, vec![lp])
            .validate(tol())
            .map_err(|e| format!("{e:?}"))?;
        revolve(&profile, axis_y(), Revolution::Full, tol())
            .map(|b| b.body.into_body())
            .map_err(|e| format!("{e:?}"))
    };
    let poly = |pts: &[(f64, f64)]| {
        pts.iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect::<Vec<_>>()
    };
    let turned = turned_plane;
    for k in 0..4 {
        for &(y0, r, h, up) in &[
            (0.0, 1.0, 1.0, true),
            (1.7, 1.0, 1.0, false),
            (0.3, 0.013, 0.9, false),
            (0.0, 20.0, 50.0, true),
            (101.3, 0.37, 2.3, true),
        ] {
            let pts = if up {
                vec![(0.0, y0), (r, y0), (0.0, y0 + h)]
            } else {
                vec![(0.0, y0 + h), (0.0, y0), (r, y0 + h)]
            };
            match build(turned(k), poly(&pts)) {
                Ok(b) => ask(
                    &format!("turned{k} cone y0={y0} r={r} h={h} up={up}"),
                    b,
                    0.05 * r.min(h),
                ),
                Err(e) => {
                    println!("DELTA turned{k} cone y0={y0} r={r} h={h} up={up}: build refused {e}")
                }
            }
        }
    }
    // Truncated tips: the small end `d` off the axis.
    for &y0 in &[0.0, 1.7] {
        for &d in &[1e-3, 1e-6, 1e-8, 1e-9, 3e-10, 1e-10, 1e-11] {
            let pts = [(0.0, y0), (1.0, y0), (d, y0 + 1.0 - d), (0.0, y0 + 1.0 - d)];
            match build(SketchPlane::xy(), poly(&pts)) {
                Ok(b) => ask(&format!("frustum y0={y0} d={d:e}"), b, 0.05),
                Err(e) => println!(
                    "DELTA frustum y0={y0} d={d:e}: build refused {}",
                    &e[..e.len().min(160)]
                ),
            }
        }
    }
    // A double cone, both nappes through one apex.
    match build(
        SketchPlane::xy(),
        poly(&[(0.0, -1.0), (1.0, -1.0), (0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    ) {
        Ok(b) => ask("double cone", b, 0.05),
        Err(e) => println!(
            "DELTA double cone: build refused {}",
            &e[..e.len().min(160)]
        ),
    }
    // An ice-cream cone: apex at the bottom, a quarter circle cap whose
    // pole is a vertex, in plain and turned frames, at off-origin heights.
    for k in 0..3 {
        for &(y0, r, h) in &[
            (0.0, 1.0, 1.0),
            (1.7, 1.0, 1.0),
            (-2.9, 0.37, 2.3),
            (13.1, 3.1, 0.21),
        ] {
            let (a, b, c) = (
                Point2::new(r, y0 + h),
                Point2::new(0.0, y0 + h + r),
                Point2::new(0.0, y0 + h),
            );
            let verts = vec![(Point2::new(0.0, y0), 0.0), (a, bulge(a, b, c)), (b, 0.0)];
            match build(turned(k), verts) {
                Ok(body) => ask(
                    &format!("icecream turned{k} y0={y0} r={r} h={h}"),
                    body,
                    0.05 * r.min(h),
                ),
                Err(e) => println!(
                    "DELTA icecream turned{k} y0={y0}: build refused {}",
                    &e[..e.len().min(160)]
                ),
            }
        }
    }
    // A hemisphere whose arc ends `d` off the axis, closed by a tiny cap.
    for &d in &[1e-6, 1e-8, 1e-9, 5e-10, 1e-10, 1e-11, 1e-12] {
        let top = (1.0f64 - d * d).sqrt();
        let (a, b, c) = (
            Point2::new(1.0, 0.0),
            Point2::new(d, top),
            Point2::new(0.0, 0.0),
        );
        let verts = vec![
            (Point2::new(0.0, 0.0), 0.0),
            (a, bulge(a, b, c)),
            (b, 0.0),
            (Point2::new(0.0, top), 0.0),
        ];
        match build(SketchPlane::xy(), verts) {
            Ok(body) => ask(&format!("near-pole dome d={d:e}"), body, 0.05),
            Err(e) => println!(
                "DELTA near-pole dome d={d:e}: build refused {}",
                &e[..e.len().min(200)]
            ),
        }
    }
}

/// Review probe: the refusers of the delta sweeps, one line each.
#[test]
#[ignore = "review probe"]
fn review_probe_delta_refusers() {
    use crate::common::latitude_seam::door_cavity;
    let turned = turned_plane;
    for &(k, y0, r, h, up, cav) in &[
        (0, 0.0, 20.0, 50.0, true, true),
        (0, 101.3, 0.37, 2.3, true, true),
        (0, 101.3, 20.0, 50.0, false, false),
        (1, 0.3, 0.013, 0.9, false, true),
        (1, 0.0, 20.0, 50.0, true, false),
        (2, 0.3, 0.013, 0.9, false, true),
        (2, 101.3, 0.37, 2.3, true, false),
        (2, 101.3, 0.37, 2.3, true, true),
        (3, 0.0, 20.0, 50.0, true, true),
    ] {
        if std::env::var("ONLY").is_ok_and(|o| o != format!("{k}/{y0}/{r}/{cav}")) {
            continue;
        }
        let pts = if up {
            [(0.0, y0), (r, y0), (0.0, y0 + h)]
        } else {
            [(0.0, y0 + h), (0.0, y0), (r, y0 + h)]
        };
        let lp = ProfileLoop::polygon(pts.map(|(x, y)| Point2::new(x, y)));
        let profile = Profile::new(turned(k), vec![lp]).validate(tol()).unwrap();
        let mut body = revolve(&profile, axis_y(), Revolution::Full, tol())
            .unwrap()
            .body
            .into_body();
        if cav {
            body = door_cavity(&body, 0.05 * r.min(h));
        }
        let rep = injective(&body, None, BUDGET);
        let rc = rep.receipt;
        println!(
            "REF k={k} y0={y0} r={r} h={h} up={up} cav={cav}: {:?} {}/{}/{}",
            rep.verdict, rc.discharged, rc.refused, rc.splits
        );
    }
}

/// Review probe (second delta of PR 7115): cone tips far from the
/// origin, along the axis, across it and in a turned frame moved far,
/// each as a body and as its axial-door cavity. Prints one line per
/// case and asserts nothing; run at each ε through `CAD_TOLERANCE_EPS`.
#[test]
#[ignore = "review probe: prints, asserts nothing"]
fn review_probe_far_witnesses() {
    use crate::common::latitude_seam::door_cavity;
    use geom_core::{Affine3, Mat3, Vec2, Vec3};
    let far_turned = |off: (f64, f64, f64)| -> SketchPlane<f64> {
        let a = Vec3::new(1.0, 2.0, 3.0) * (1.0 / 14.0_f64.sqrt());
        let (c, s) = (0.7_f64.cos(), 0.7_f64.sin());
        let col = |e: Vec3<f64>| e * c + a.cross(e) * s + a * (a.dot(e) * (1.0 - c));
        let m = Mat3::from_cols(
            col(Vec3::new(1.0, 0.0, 0.0)),
            col(Vec3::new(0.0, 1.0, 0.0)),
            col(Vec3::new(0.0, 0.0, 1.0)),
        );
        SketchPlane::from_frame(geom_core::test_support::rigid(
            &Affine3::from_parts(m, Vec3::new(off.0, off.1, off.2)),
            tol(),
        ))
    };
    let shapes = [
        (1.0, 1.0, true),
        (1.0, 1.0, false),
        (20.0, 50.0, true),
        (0.37, 2.3, true),
    ];
    for far in [100.0, 1e4] {
        for (place, plane, x0, y0) in [
            ("along", SketchPlane::xy(), 0.0, far),
            ("across", SketchPlane::xy(), far, 0.0),
            ("turned", far_turned((far, -0.7 * far, 0.3 * far)), 0.0, 0.0),
        ] {
            for &(r, h, up) in &shapes {
                let pts = if up {
                    [(x0, y0), (x0 + r, y0), (x0, y0 + h)]
                } else {
                    [(x0, y0 + h), (x0, y0), (x0 + r, y0 + h)]
                };
                let what = format!("{place} {far:e} r={r} h={h} up={up}");
                let profile = Profile::new(
                    plane,
                    vec![ProfileLoop::polygon(pts.map(|(x, y)| Point2::new(x, y)))],
                )
                .validate(tol());
                let Ok(profile) = profile else {
                    println!("FAR {what}: profile refused");
                    continue;
                };
                let axis = RevolveAxis {
                    origin: Point2::new(x0, 0.0),
                    dir: Vec2::new(0.0, 1.0),
                };
                let body = match revolve(&profile, axis, Revolution::Full, tol()) {
                    Ok(b) => b.body.into_body(),
                    Err(e) => {
                        println!("FAR {what}: revolve refused {e:?}");
                        continue;
                    }
                };
                let rep = injective(&body, None, BUDGET);
                let rc = rep.receipt;
                println!(
                    "FAR {what} body: {:?} {}/{}/{}",
                    rep.verdict, rc.discharged, rc.refused, rc.splits
                );
                match std::panic::catch_unwind(|| door_cavity(&body, 0.05 * r.min(h))) {
                    Ok(c) => {
                        let rep = injective(&c, None, BUDGET);
                        let rc = rep.receipt;
                        println!(
                            "FAR {what} cavity: {:?} {}/{}/{}",
                            rep.verdict, rc.discharged, rc.refused, rc.splits
                        );
                    }
                    Err(_) => println!("FAR {what} cavity: door panicked"),
                }
            }
        }
    }
}

/// Review probe: one ice-cream cone (a cone under a sphere cap with its
/// pole a vertex) on the first two turned planes, and a plain ball on
/// them, one line each.
#[test]
#[ignore = "review probe"]
fn review_probe_turned_icecream() {
    for k in 1..3 {
        let (a, b, c) = (
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 2.0),
            Point2::new(0.0, 1.0),
        );
        let cone = revolved_on(
            turned_plane(k),
            vec![(Point2::new(0.0, 0.0), 0.0), (a, bulge(a, b, c)), (b, 0.0)],
        );
        let rep = injective(&cone, None, BUDGET);
        let rc = rep.receipt;
        println!(
            "ICE k={k}: {:?} {}/{}/{}",
            rep.verdict, rc.discharged, rc.refused, rc.splits
        );
        let (a, b, c) = (
            Point2::new(0.0, -1.0),
            Point2::new(0.0, 1.0),
            Point2::new(0.0, 0.0),
        );
        let ball = revolved_on(turned_plane(k), vec![(a, bulge(a, b, c)), (b, 0.0)]);
        let rep = injective(&ball, None, BUDGET);
        let rc = rep.receipt;
        println!(
            "BALL k={k}: {:?} {}/{}/{}",
            rep.verdict, rc.discharged, rc.refused, rc.splits
        );
    }
}

// ------------------------------------------ two wedges at a vertex alone

/// **A triangular prism standing on its corner**: its cap the triangle
/// the origin and the edges `e1`, `e2` span, extruded `1` along
/// `e1 × e2`, at `T`.
fn corner_prism<T: Scalar>(e1: [f64; 3], e2: [f64; 3]) -> topo::AtRestBody<T> {
    use geom_core::{RigidMotion, Vec3};
    let (a, b) = (
        Vec3::new(e1[0], e1[1], e1[2]),
        Vec3::new(e2[0], e2[1], e2[2]),
    );
    let n = a.cross(b);
    let n = n * (1.0 / n.dot(n).sqrt());
    let u = a * (1.0 / a.dot(a).sqrt());
    let v = n.cross(u);
    let plane = SketchPlane::from_frame(RigidMotion::squared(Point3::origin(), n, u));
    let at = |x: f64, y: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    sweep::test_support::finished(
        "the corner prism",
        sweep::test_support::prism_on(
            plane_at::<T>(&plane),
            vec![at(0.0, 0.0), at(a.dot(u), 0.0), at(b.dot(u), b.dot(v))],
            T::one(),
            tol(),
        ),
        tol(),
    )
}

/// The block `(0, 4)² × (−1, 0)`, its top face's corner at the origin.
fn floor_block<T: Scalar>() -> topo::AtRestBody<T> {
    use sweep::test_support::{brick, finished};
    finished(
        "the block",
        brick::<T>((0.0, 4.0), (0.0, 4.0), (-1.0, 0.0), tol()),
        tol(),
    )
}

/// **The deep keel** (`a_keel_tree_in_the_plane.rs`'s): the block
/// `0..4 × 0..4 × 0..2` less a V-groove of half-width `0.3` from outside
/// the face `x = 0` to `x = 2`, then a wider one, `0.5`, on to `x = 3`,
/// both their floors on the line `y = 1`, `z = 1`; then moved by `m`.
/// Where the floors meet at `(2, 1, 1)` each narrow flank shares that
/// vertex alone with each wide one, every face there a convex quadrant.
fn deep_keel<T: Scalar>(m: &geom_core::Affine3<f64>) -> Body<T> {
    use crate::common::keel_tree::{deep, grooved};
    let keel = sweep::test_support::finished("the keel", grooved(deep::<T>(tol()), tol()), tol());
    moved(&keel, m).into_body()
}

/// **Two plane faces at a vertex they share alone hold where a plane
/// parts their wedges** ([`deep_keel`]): at the keel's joint each narrow
/// flank and each wide one are convex quadrants there, and the plane
/// across the presentations' middles leaves them split to the depth
/// floor, `Departure` at the vertex; the plane leaning furthest from
/// all four edges parts them. Embedded at `f64` and at `Interval`, at
/// the origin, turned `0.7` about `(1, 2, 3)` through `(0.3, −0.2, 0.1)`,
/// and moved `(100, −100, 100)`. Red where the vertex-alone separator is
/// taken from the rectangles' middles alone.
#[test]
fn a_deep_keels_joint_is_embedded_in_every_pose() {
    use geom_core::{Affine3, Vec3};
    let poses = [
        ("at the origin", Affine3::identity()),
        (
            "turned",
            Affine3::rotation_about_axis(
                Point3::new(0.3, -0.2, 0.1),
                Vec3::new(1.0, 2.0, 3.0),
                0.7,
            ),
        ),
        (
            "a hundred out",
            Affine3::translation(Vec3::new(100.0, -100.0, 100.0)),
        ),
    ];
    for (what, m) in poses {
        for (scalar, verdict) in [
            (
                "f64",
                door(&deep_keel::<f64>(&m), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
            ),
            (
                "Interval",
                door(&deep_keel::<Interval>(&m), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
            ),
        ] {
            assert!(
                matches!(verdict, topo::clearance::Embedding::Embedded),
                "the deep keel, {what}, at {scalar}: {verdict:?}"
            );
        }
    }
}

/// **A roof folded down across the block's corner crosses it**: the
/// [`corner_prism`] on `(1, 1, 1)` and `(1, 1, −1)`, whose cap holds the
/// ray `(1, 1, 0)` of the block's top face, grafted beside
/// [`floor_block`] as a solid of its own, so the cap runs through the
/// top face along the diagonal from the corner. `Crosses` at `f64` and
/// at `Interval`, never `Embedded`. The two solids share nothing in M,
/// so the pair is decided by distance and the vertex-alone plane is not
/// asked: the kernel builds no body whose faces cross while sharing a
/// vertex, and that reading's crossing wedges are `injective`'s
/// `two_plane_wedges_at_a_vertex_alone_certify_only_where_a_plane_parts_them`.
/// So no change to the vertex-alone separator can redden this row: it
/// goes red if two coincident vertices of two solids were read as one
/// shared cell, or the crossing were certified apart by distance.
#[test]
fn a_roof_folded_across_a_blocks_corner_crosses_it() {
    fn folded<T: Scalar>() -> Body<T> {
        let mut body = floor_block::<T>().into_body();
        let roof = corner_prism::<T>([1.0, 1.0, 1.0], [1.0, 1.0, -1.0]);
        topo::graft_disjoint(&mut body, &roof).expect("the roof grafts");
        body
    }
    for (scalar, verdict) in [
        (
            "f64",
            door(&folded::<f64>(), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
        ),
        (
            "Interval",
            door(&folded::<Interval>(), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
        ),
    ] {
        assert!(
            matches!(verdict, topo::clearance::Embedding::Crosses(_)),
            "the folded roof at {scalar}: {verdict:?}"
        );
    }
}

// ------------------------------------------------------ thin plane faces

/// `topo::clearance::embedded`'s verdict on `body`, every face, at the
/// default depth and budget.
fn embedded_verdict<T: Scalar + geom_core::Certifies>(
    body: &Body<T>,
) -> topo::clearance::Embedding {
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
    };
    let budgets = Budgets {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: BUDGET,
    };
    topo::clearance::embedded(&side, band(), budgets).verdict
}

/// An embedding verdict's class: `Embedded`, `Crosses`, or the
/// refusal's arm and the kind of cell it shares, or its kind.
fn embedding_class(verdict: &topo::clearance::Embedding) -> String {
    use topo::clearance::Embedding;
    match verdict {
        Embedding::Embedded => "Embedded".to_owned(),
        Embedding::Crosses(_) => "Crosses".to_owned(),
        Embedding::Refused(refusal) => class::<()>(&BoundVerdict::Refused(refusal.clone())),
    }
}

/// A pose of a body row: the plain sketch plane, a rigid turn about a
/// generic axis, and another turn a hundred units from the origin.
fn thin_pose(k: usize) -> SketchPlane<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    let Some((ax, th, off)) = [
        None,
        Some(((1.0, 2.0, 3.0), 0.7, (0.0, 0.0, 0.0))),
        Some(((-0.4, 0.1, 0.9), 2.3, (100.0, 0.0, 0.0))),
    ][k] else {
        return SketchPlane::xy();
    };
    let a = Vec3::new(ax.0, ax.1, ax.2) * (1.0 / (ax.0 * ax.0 + ax.1 * ax.1 + ax.2 * ax.2).sqrt());
    let (c, s) = (f64::cos(th), f64::sin(th));
    let col = |e: Vec3<f64>| e * c + a.cross(e) * s + a * (a.dot(e) * (1.0 - c));
    let m = Mat3::from_cols(
        col(Vec3::new(1.0, 0.0, 0.0)),
        col(Vec3::new(0.0, 1.0, 0.0)),
        col(Vec3::new(0.0, 0.0, 1.0)),
    );
    SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(m, Vec3::new(off.0, off.1, off.2)),
        tol(),
    ))
}

/// The prism over the polygon `pts` on `place`, extruded `d` along its
/// normal.
fn prism<T: Scalar>(place: &SketchPlane<f64>, pts: &[(f64, f64)], d: f64) -> Body<T> {
    let profile = Profile::new(
        plane_at::<T>(place),
        lifted(&[ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]),
    )
    .validate(tol())
    .expect("the polygon validates");
    Body::clone(
        &extrude(&profile, to_offset(d), tol())
            .expect("the prism extrudes")
            .body,
    )
}

/// The box `[0, w] × [0, h]` on `place`, extruded `d` along its normal.
fn block<T: Scalar>(place: &SketchPlane<f64>, (w, h, d): (f64, f64, f64)) -> Body<T> {
    prism(place, &rectangle(w, h), d)
}

/// The rectangle `w × h` from the origin.
fn rectangle(w: f64, h: f64) -> Vec<(f64, f64)> {
    vec![(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
}

/// The poses of a thin body row: plain; turned about a generic axis;
/// turned a hundred units out; extruded the other way; mirrored in its
/// sketch's `x`; turned by 0.3 and by 0.785 in its sketch plane, so its
/// thin faces' sides lie off their charts' axes; and at half the session
/// box's extent and two units inside its wall. A face turned in its own
/// chart is pinned by `injective`'s
/// `a_strip_end_turned_in_its_chart_presents_its_short_side`; the turns
/// here read it through whole bodies.
const POSES: [&str; 9] = [
    "plain",
    "turned",
    "a hundred units out",
    "extruded the other way",
    "mirrored",
    "turned 0.3 in its plane",
    "turned 0.785 in its plane",
    "at half the session box",
    "at the session box's wall",
];

/// The profile `pts` extruded `d` in pose `k` of [`POSES`]: its sketch
/// plane, its points and its depth.
fn thin_posed(k: usize, pts: &[(f64, f64)], d: f64) -> (SketchPlane<f64>, Vec<(f64, f64)>, f64) {
    use geom_core::{Affine3, Vec3};
    let turn = |th: f64| {
        let (c, s) = (th.cos(), th.sin());
        pts.iter()
            .map(|&(x, y)| (c * x - s * y, s * x + c * y))
            .collect()
    };
    let at = |off: f64| {
        SketchPlane::from_frame(geom_core::test_support::rigid(
            &Affine3::translation(Vec3::new(off, off, off)),
            tol(),
        ))
    };
    let extent = geom_core::tolerance::SESSION_BOX_EXTENT;
    match k {
        0..=2 => (thin_pose(k), pts.to_vec(), d),
        3 => (SketchPlane::xy(), pts.to_vec(), -d),
        4 => (
            SketchPlane::xy(),
            pts.iter().rev().map(|&(x, y)| (-x, y)).collect(),
            d,
        ),
        5 => (SketchPlane::xy(), turn(0.3), d),
        6 => (SketchPlane::xy(), turn(0.785), d),
        7 => (at(0.5 * extent), pts.to_vec(), d),
        _ => (at(extent - 2.0), pts.to_vec(), d),
    }
}

/// Each prism `(what, profile)`, extruded a unit, reads `Embedded` at
/// `f64` and at `Interval` in every pose of [`POSES`].
fn thin_prisms_embedded(prisms: &[(&str, Vec<(f64, f64)>)]) {
    for (what, pts) in prisms {
        for (k, pose) in POSES.iter().enumerate() {
            let (place, pts, d) = thin_posed(k, pts, 1.0);
            let float = embedded_verdict(&prism::<f64>(&place, &pts, d));
            let interval = embedded_verdict(&prism::<Interval>(&place, &pts, d));
            println!(
                "[thin] {what}, {pose}: f64 {float:?} | Interval {}",
                embedding_class(&interval)
            );
            assert_eq!(
                embedding_class(&float),
                "Embedded",
                "{what}, {pose}: embedded at f64: {float:?}"
            );
            assert_eq!(
                embedding_class(&interval),
                "Embedded",
                "{what}, {pose}: embedded at Interval: {interval:?}"
            );
        }
    }
}

/// The plain sketch plane moved by `(x, y, 0)`.
fn shifted(x: f64, y: f64) -> SketchPlane<f64> {
    use geom_core::{Affine3, Vec3};
    SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::translation(Vec3::new(x, y, 0.0)),
        tol(),
    ))
}

/// **A thin slab is embedded**, a unit square `t` thick at `2Kε` and at
/// `20Kε`: its four thin faces meet in pairs at edges `t` long, where
/// the metric split cuts a cell's length a few dozen times before its
/// width, and each pair is read at the thin faces' ends. In every pose of
/// [`POSES`]. Red on a planar arm that reads a cell holding a thin face's
/// whole end as `Local`: the slab refuses `Departure` at an edge there.
#[test]
fn a_thin_slab_is_embedded_across_its_ends_in_every_pose() {
    let k_eps = band().escalate();
    thin_prisms_embedded(&[
        ("a slab 2Kε thick", rectangle(1.0, 2.0 * k_eps)),
        ("a slab 20Kε thick", rectangle(1.0, 20.0 * k_eps)),
    ]);
}

/// **A needle is embedded**, a unit length `t` square at `20Kε` and at
/// `1000Kε`. Its long faces are thin rectangles, and the first split of
/// each leaves two halves that each hold an end, so every cell the
/// certificate asks is read at a thin face's end: a slab's reading. In
/// every pose of [`POSES`]. Red on a planar arm that reads a cell
/// holding an end as `Local`: the needle then spends the pair budget.
#[test]
fn a_needle_is_embedded_along_its_sides_in_every_pose() {
    let k_eps = band().escalate();
    thin_prisms_embedded(&[
        (
            "a needle 20Kε square",
            rectangle(20.0 * k_eps, 20.0 * k_eps),
        ),
        (
            "a needle 1000Kε square",
            rectangle(1e3 * k_eps, 1e3 * k_eps),
        ),
    ]);
}

/// **Two thin slabs through each other cross**: a slab `20Kε` thick
/// along `y` and one along `x`, grafted as two solids, pass through each
/// other in a column `20Kε` square, at both scalars. Their faces share
/// nothing, so the crossing is found by distance and no presentation is
/// on its path: the row pins that thin faces' cells reach the distance
/// test, not any reading of a strip.
#[test]
fn two_thin_slabs_through_each_other_cross() {
    fn crossed<T: Scalar>(t: f64) -> Body<T> {
        let mut along = block::<T>(&SketchPlane::xy(), (1.0, t, 1.0));
        let across = block::<T>(&shifted(0.5 - 0.5 * t, -0.5), (t, 1.0, 1.0));
        topo::graft_disjoint(&mut along, &across).expect("the slab grafts");
        along
    }
    let t = 20.0 * band().escalate();
    let float = embedded_verdict(&crossed::<f64>(t));
    let interval = embedded_verdict(&crossed::<Interval>(t));
    println!("[thin] crossed slabs: f64 {float:?} | Interval {interval:?}");
    for (scalar, verdict) in [("f64", &float), ("Interval", &interval)] {
        assert_eq!(
            embedding_class(verdict),
            "Crosses",
            "{scalar}: the slabs cross: {verdict:?}"
        );
    }
}

// ------------------------------- a cone's section arcs, as its split states them

/// A frustum's base radius, top radius and height, and a cut plane
/// through `(0, q, 0)` whose normal is `y` tilted by `phi` toward the
/// horizontal direction at azimuth `psi`, as `(q, phi, psi)`; the side
/// the normal points to where the flag is set.
type SplitCone = ((f64, f64, f64), (f64, f64, f64), bool);

/// Cut through the wall alone, on dyadic sizes: the section's two arcs
/// meet on the rulings at `±x`.
const WALL_CUT: SplitCone = ((1.0, 0.5, 1.0), (0.5, 0.25, 0.0), true);
/// The same on sizes that are no dyadic numbers, the tilt off the
/// rulings.
const WALL_CUT_OFF_GRID: SplitCone = ((0.7, 0.31, 0.9), (0.43, 0.3, 0.7), true);
/// Cut out through the top cap, the side below: the two arcs each end
/// on the top rim and meet at one vertex on a ruling.
const CAP_CUT: SplitCone = ((0.7, 0.31, 0.9), (0.86, 0.3, 0.7), false);

/// How a split cone is placed before it is asked.
#[derive(Clone, Copy, Debug)]
enum SplitPose {
    AsBuilt,
    /// Turned 0.7 about a generic axis off the origin.
    Turned,
    /// Moved a hundred units from the origin.
    Out,
    /// Mirrored across a generic plane: its rows are minted again.
    Mirrored,
}

const SPLIT_POSES: [SplitPose; 4] = [
    SplitPose::AsBuilt,
    SplitPose::Turned,
    SplitPose::Out,
    SplitPose::Mirrored,
];

/// **A frustum split by a tilted plane, one side of it, placed**: the
/// frustum revolved a full turn about `y` (its wall two half-cones
/// meeting along the rulings at `±x`) and split.
fn split_cone<T: Scalar>(cut: SplitCone, pose: SplitPose) -> Body<T> {
    match try_split_cone(cut, pose) {
        Ok(body) => body,
        Err(SplitConeRefusal::Map(e)) => {
            panic!("the split cone is placed: the mirror refused {e:?}")
        }
        Err(SplitConeRefusal::Rows(e)) => {
            panic!("the split cone is placed: its mirror image's rows refused {e:?}")
        }
    }
}

/// Why a split cone was not placed: the mirror's data map refused, or
/// the mirror image's rows did not complete.
#[derive(Debug)]
enum SplitConeRefusal {
    Map(topo::TransformError),
    Rows(Vec<topo::ValidationError>),
}

/// [`split_cone`], with the mirror door's refusal, or its image's
/// completion refusal, handed back for a row that reads a build limit.
fn try_split_cone<T: Scalar>(
    ((r0, r1, h), (q, phi, psi), above): SplitCone,
    pose: SplitPose,
) -> Result<Body<T>, SplitConeRefusal> {
    use geom_core::{Affine3, Vec3};
    use sweep::test_support::finished;
    use topo::splitting::{SplitPart, SplitPlane, split};
    let p = |x: f64, y: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    let frustum = finished(
        "the frustum",
        revolved_about_y_at::<T>(
            vec![p(0.0, 0.0), p(r0, 0.0), p(r1, h), p(0.0, h)],
            Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let normal = Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
    let plane = SplitPlane {
        origin: Point3::new(0.0, q, 0.0).map(T::from_f64),
        normal: geom_core::UnitVec3::new(normal.map(T::from_f64), topo::POSE_UNIT_NORM, band())
            .expect("a cut normal has a length"),
    };
    let result = split(&frustum, &plane, tol()).expect("the frustum splits");
    let body = match if above { result.above } else { result.below } {
        SplitPart::Body(b) => b,
        SplitPart::Empty => panic!("the side carries material"),
    };
    Ok(match pose {
        SplitPose::AsBuilt => body.into_body(),
        SplitPose::Turned => moved(
            &body,
            &Affine3::rotation_about_axis(
                Point3::new(0.3, -0.2, 0.1),
                Vec3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0),
                0.7,
            ),
        )
        .into_body(),
        SplitPose::Out => {
            moved(&body, &Affine3::translation(Vec3::new(60.0, -70.0, 40.0))).into_body()
        }
        SplitPose::Mirrored => {
            let across = topo::Reflection::across(
                Point3::new(0.2, 0.1, -0.3).map(T::from_f64),
                Vec3::new(3.0 / 7.0, -6.0 / 7.0, 2.0 / 7.0).map(T::from_f64),
                tol(),
            )
            .expect("a mirror plane");
            // The mirror image's rows completed, as its finish completes
            // them: a row the image cannot certify refuses here.
            let image = topo::mirror_body(&body, &across).map_err(SplitConeRefusal::Map)?;
            topo::test_support::completed(image, tol()).map_err(SplitConeRefusal::Rows)?
        }
    })
}

/// The stored ellipses of `body`'s edges.
fn section_arcs(body: &Body<f64>) -> Vec<geom::Ellipse<f64>> {
    body.edges()
        .filter_map(
            |(_, e)| match body.get_curve_geom(e.curve).unwrap().certified()?.carrier() {
                geom::Curve3::Ellipse(e) => Some(*e),
                _ => None,
            },
        )
        .collect()
}

/// **A cone's split states its section as one ellipse through its wall,
/// and as two arcs a rounded quarter turn apart through its cap.** A
/// full revolve's wall is one band, so a cut through the wall alone
/// leaves its section one closed ellipse, one edge: there is no second
/// arc to restate. The cap cut here leaves the wall's arc cut by the
/// wall's wrap edge into two, seamed at the principal vertex nearest
/// each one's middle, which lie in adjacent quadrants: stated a quarter
/// turn apart with the semi-axes exchanged, through a turn that rounds,
/// so no restatement of one is the other
/// ([`geom::Ellipse::restatements`]). Counted over 1,152 splits
/// (`a_splits_section_is_one_ellipse_through_the_wall_and_one_or_two_arcs_through_a_cap`):
/// every cut through the wall alone states one ellipse, and through a cap
/// two arcs state a quarter or a half by where their middles fall. The
/// row pins what these three splits state, read through the same
/// restatements the certificate reads (`injective`'s
/// `one_stated_conic`), so it goes red when the split's statements move
/// and not when that reading does. Asked of the `f64` body alone: a body
/// held as enclosures stores no statement.
#[test]
fn a_cones_split_states_one_ellipse_through_its_wall_and_two_arcs_a_rounded_quarter_apart_through_its_cap()
 {
    let restated = |a: &geom::Ellipse<f64>, b: &geom::Ellipse<f64>| {
        a.restatements()
            .map(|r| geom::Curve3::Ellipse(r).same_content(&geom::Curve3::Ellipse(*b)))
    };
    for pose in SPLIT_POSES {
        for (what, cut) in [
            ("on the grid", WALL_CUT),
            ("off the grid", WALL_CUT_OFF_GRID),
        ] {
            let arcs = section_arcs(&split_cone::<f64>(cut, pose));
            assert_eq!(
                arcs.len(),
                1,
                "{what} {pose:?}: one section ellipse: {arcs:?}"
            );
        }
        let arcs = section_arcs(&split_cone::<f64>(CAP_CUT, pose));
        let [a, b] = arcs.as_slice() else {
            panic!("the cap cut {pose:?}: two section arcs: {arcs:?}")
        };
        assert_eq!(
            restated(a, b),
            [false; 4],
            "the cap cut {pose:?}: no restatement: {a:?} {b:?}"
        );
        assert_eq!(
            a.semi_axes(),
            (b.semi_v(), b.semi_u()),
            "the cap cut {pose:?}: the semi-axes exchanged"
        );
    }
}

/// **A split cone refuses on a pair that holds its wall where its cut
/// meets its cap, and its wall cuts hold, at both scalars, in every
/// placement.** A full revolve's wall is one band, so a cut through the
/// wall alone leaves one cone face whose section is one closed ellipse:
/// no ruling parts two halves and no two arcs meet, and both wall cuts
/// read `Embedded` at `f64` and at `Interval`. The cap cut leaves the
/// wall's arc cut in two by its wrap edge, stated a rounded quarter turn
/// apart
/// (`a_cones_split_states_one_ellipse_through_its_wall_and_two_arcs_a_rounded_quarter_apart_through_its_cap`),
/// and the verdict door refuses `Departure` on the wall against the cut
/// cap across one of them, at both scalars. The row that reads that
/// corner re-pins the cap cut.
#[test]
fn a_split_cone_refuses_on_a_pair_holding_its_wall() {
    use topo::clearance::Embedding;
    fn refuses<T: Scalar + geom_core::Certifies>(what: &str, body: &Body<T>) {
        let verdict = door(body, DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict;
        let Embedding::Refused(EngineRefusal::Uncertified { a, b, arm, .. }) = verdict else {
            panic!("{what}: refuses uncertified: {verdict:?}")
        };
        assert_eq!(arm, CertificateArm::Departure, "{what}");
        assert!(
            [a, b]
                .iter()
                .any(|&f| matches!(surface(body, f), Surface::Cone { .. })),
            "{what}: the refused pair holds a cone: {a:?} {b:?}"
        );
    }
    // A wall cut holds; the cap cut refuses.
    fn reads<T: Scalar + geom_core::Certifies>(wall: bool, what: &str, body: &Body<T>) {
        if wall {
            let verdict = door(body, DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict;
            assert!(
                matches!(verdict, Embedding::Embedded),
                "{what}: embedded: {verdict:?}"
            );
        } else {
            refuses(what, body);
        }
    }
    // The split cone placed, or `None` at the one build limit: below
    // 1e-11 the mirror door refuses a wall cut's `Interval` twin at its
    // pcurve envelope, an indeterminate margin at the band. Any other
    // refusal, or that one anywhere else, fails the row.
    fn placed<T: Scalar>(
        what: &str,
        cut: SplitCone,
        wall: bool,
        pose: SplitPose,
        twin: bool,
    ) -> Option<Body<T>> {
        use geom_brep::{PcurveCertifyError, PcurveCheck};
        use topo::PcurveMintError;
        let envelope = |f: &topo::ValidationError| {
            matches!(
                f,
                topo::ValidationError::Pcurve {
                    finding: PcurveMintError::Certify {
                        error: PcurveCertifyError::Escalated {
                            check: PcurveCheck::Envelope,
                            ..
                        },
                        ..
                    },
                }
            )
        };
        match try_split_cone::<T>(cut, pose) {
            Ok(body) => Some(body),
            Err(SplitConeRefusal::Rows(refusal))
                if !refusal.is_empty()
                    && refusal.iter().all(envelope)
                    && twin
                    && wall
                    && matches!(pose, SplitPose::Mirrored)
                    && tol().eps() < 1e-11 =>
            {
                println!(
                    "SKIPPED (ε = {:e}): {what} {pose:?} at Interval is not placed: {refusal:?}",
                    tol().eps()
                );
                None
            }
            Err(refusal) => panic!("{what} {pose:?} is placed: {refusal:?}"),
        }
    }
    for pose in SPLIT_POSES {
        for (what, cut, wall) in [
            ("the wall cut on the grid", WALL_CUT, true),
            ("the wall cut off the grid", WALL_CUT_OFF_GRID, true),
            ("the cap cut", CAP_CUT, false),
        ] {
            if let Some(float) = placed::<f64>(what, cut, wall, pose, false) {
                reads(wall, &format!("{what} {pose:?} at f64"), &float);
            }
            if let Some(twin) = placed::<Interval>(what, cut, wall, pose, true) {
                reads(wall, &format!("{what} {pose:?} at Interval"), &twin);
            }
        }
    }
}

/// **A ball through a split cylinder's wall and cut cap, at its section
/// on a ruling, crosses, at both scalars.** A rod of radius 0.7 split by
/// a tilted plane states its section as one ellipse (asserted): a full
/// revolve's wall is one band, so no ruling parts the
/// section into two arcs and no corner of two arcs is there. A ball of
/// radius 0.02 centred on the section's point on the ruling at `+x`
/// passes through the wall and the cut cap there. It reads `Crosses` at
/// the default ε; at another the walk may run out of its budget on the
/// crossing pair first, and then refuses on it, never embedded. What it
/// pins is that a crossing there is found, a grafted ball sharing no
/// cell with the rod.
#[test]
fn a_ball_through_a_split_cylinders_wall_and_cut_cap_at_its_section_crosses() {
    use topo::clearance::Embedding;
    const ROD: SplitCone = ((0.7, 0.7, 1.9), (0.93, 0.3, 0.7), true);
    fn with_ball<T: Scalar>() -> Body<T> {
        let mut body = split_cone::<T>(ROD, SplitPose::AsBuilt);
        // The section's point on the ruling at `+x`.
        let ((r, _, _), (q, phi, psi), _) = ROD;
        let at = geom_core::Vec3::new(r, q - phi.tan() * psi.cos() * r, 0.0);
        let rho = 0.02;
        let p = |y: f64, bulge: f64| (Point2::new(T::zero(), T::from_f64(y)), T::from_f64(bulge));
        let ball = topo::transform_rigid(
            &revolved_about_y_at::<T>(vec![p(-rho, 1.0), p(rho, 0.0)], Revolution::Full, tol()),
            &geom_core::RigidMotion::translation(at.map(T::from_f64)).expect("a translation"),
        )
        .expect("the ball moves");
        topo::graft_disjoint_all_keyed(&mut body, &ball).expect("the ball grafts");
        body
    }
    let arcs = section_arcs(&split_cone::<f64>(ROD, SplitPose::AsBuilt));
    assert_eq!(arcs.len(), 1, "one section ellipse: {arcs:?}");
    for (scalar, verdict) in [
        (
            "f64",
            door(&with_ball::<f64>(), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
        ),
        (
            "Interval",
            door(&with_ball::<Interval>(), DEFAULT_MAX_CELL_DEPTH, BUDGET).verdict,
        ),
    ] {
        assert!(
            matches!(
                verdict,
                Embedding::Crosses(_) | Embedding::Refused(EngineRefusal::Budget { .. })
            ),
            "the ball at the corner at {scalar}: {verdict:?}"
        );
        if tol().eps() == geom_core::tolerance::DEFAULT_EPS {
            assert!(
                matches!(verdict, Embedding::Crosses(_)),
                "the ball at the corner at {scalar}: {verdict:?}"
            );
        }
    }
}

/// **A split states its section as one ellipse through the wall, and
/// through a cap as one or two arcs, counted over 1,152 splits.** A
/// frustum and a rod, each a full revolve whose wall is one band, cut at
/// six heights, three tilts and sixteen azimuths, either side kept.
/// Every cut through the wall alone states one closed ellipse. A cut out
/// through a cap states one arc, or two where the wall's wrap edge cuts
/// it, and two arcs are stated a half turn apart
/// ([`geom::Ellipse::restatements`]) or a quarter turn with the
/// semi-axes exchanged, by where their middles fall; none states two
/// arcs otherwise, and no split refuses. The counts are pinned as
/// measured, so the row goes red when the split's seaming moves,
/// whichever way.
#[test]
fn a_splits_section_is_one_ellipse_through_the_wall_and_one_or_two_arcs_through_a_cap() {
    let (mut one, mut wall, mut cap, mut rest) = ([0usize; 2], [0usize; 2], [0usize; 3], 0usize);
    for shape in [(0.7, 0.31, 0.9), (0.7, 0.7, 1.9)] {
        for qi in 0..6 {
            let q = 0.2 + 0.13 * f64::from(qi);
            for phi in [0.12f64, 0.3, 0.55] {
                for k in 0..16 {
                    let psi = 0.05 + TAU * f64::from(k) / 16.0;
                    for above in [true, false] {
                        let cut: SplitCone = (shape, (q, phi, psi), above);
                        // A split that refuses, or one that states another
                        // count of section arcs (a side with none, a cut
                        // through both caps), is the rest.
                        let arcs = std::panic::catch_unwind(|| {
                            section_arcs(&split_cone::<f64>(cut, SplitPose::AsBuilt))
                        });
                        let (r0, r1, h) = shape;
                        let n = (phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
                        // The plane meets the wall alone where the section's
                        // height on the wall lies inside the wall's at every
                        // azimuth.
                        let wall_alone = (0..720).all(|i| {
                            let th = TAU * f64::from(i) / 720.0;
                            let (c, slope) = (n.0 * th.cos() + n.2 * th.sin(), (r1 - r0) / h);
                            let y = (n.1 * q - c * r0) / (n.1 + c * slope);
                            y > 0.0 && y < h
                        });
                        let (a, b) = match arcs.as_deref() {
                            Ok([_]) => {
                                one[usize::from(!wall_alone)] += 1;
                                continue;
                            }
                            Ok([a, b]) => (a, b),
                            _ => {
                                rest += 1;
                                continue;
                            }
                        };
                        let restated = a.restatements().map(|r| {
                            geom::Curve3::Ellipse(r).same_content(&geom::Curve3::Ellipse(*b))
                        });
                        let exchanged = a.semi_axes() == (b.semi_v(), b.semi_u());
                        let half = restated == [false, true, false, false];
                        let quarter = restated == [false; 4] && exchanged;
                        match (wall_alone, half, quarter) {
                            (true, true, _) => wall[0] += 1,
                            (true, false, _) => wall[1] += 1,
                            (false, true, _) => cap[0] += 1,
                            (false, false, true) => cap[1] += 1,
                            (false, false, false) => cap[2] += 1,
                        }
                    }
                }
            }
        }
    }
    assert_eq!(
        (one, wall, cap, rest),
        ([864, 177], [0, 0], [53, 58, 0], 0),
        "one ellipse [through the wall alone, through a cap]; two arcs through the wall alone \
         [half, other]; two arcs through a cap [half, quarter, other]; the rest"
    );
}

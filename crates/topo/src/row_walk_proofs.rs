//! **A door that decides pcurve rows by a loop walk proves the walk is
//! the loop's before it writes.**
//!
//! A row is keyed on a half-edge and stated in the chart of the face its
//! loop is on, so a door that moves a loop, re-charts a face, or re-mints
//! a face's rows decides which rows to drop or write by walking the
//! loop's `next` cycle. A torn `next` can divert that walk through
//! another loop's members and back, so the door drops or re-mints a row
//! of a loop it does not own; or close it past a member, which keeps a
//! row stated in the chart its loop left. Neither is a tier-1 fault the
//! door writes, so only the pcurve pass would see it.
//!
//! The proof has one home. [`crate::pcurves::loop_rows`] and
//! [`Body::site_cycle_from`] walk through [`Body::loop_cycle_of`], which
//! stops at a member that does not claim the loop; a door that moves or
//! re-charts the whole loop proves its walk misses no member
//! ([`Body::whole_cycle`]), in its plan, and drops the rows of that list.
//! A walk that fails the proof is a torn body, which only a kernel bug
//! reaches: the plan panics naming the loop (D2 row 4), before it writes.
//!
//! The fixture is a [`CylKey::Bare`] wall sheet split along a ruling: two
//! curved faces on one cylinder carrying four rows each, and the seed
//! face on its `mvfs` placeholder, a chart that mints nothing. Each torn
//! row tears the wall's loop and names the split face's loop as the one
//! whose rows the door must leave alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};

use crate::Body;
use crate::MekrSite;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite, MevSite};
use crate::fixtures::deep_snapshot;
use crate::review_d18::{
    BuildFixture, FIXTURES, PanicCapture, ROW_FOUR, Tear, assert_torn_op_panics, plant,
};
use crate::test_support_fixtures::{CylFrame, CylKey, cyl_wall_sheet_keyed};

fn tol() -> Tol {
    Tol::witness()
}

/// The minted split sheet ([module docs](self)).
pub(crate) struct Sheet {
    pub body: Body<f64>,
    /// The seed face, on the placeholder chart: no rows.
    pub seed: FaceKey,
    /// The wall's half on the `he1` side of the split: four rows.
    pub wall: FaceKey,
    /// The face the split minted: four rows.
    pub split: FaceKey,
}

pub(crate) fn sheet() -> Sheet {
    let tol = tol();
    let mut body = Body::<f64>::new();
    let (wall, _) = cyl_wall_sheet_keyed(
        &mut body,
        CylFrame::canonical(1.0),
        CylKey::Bare,
        None,
        (0.2, 1.4),
        (0.0, 1.0),
        tol,
    );
    let seed = body.faces().map(|(k, _)| k).find(|&f| f != wall).unwrap();
    let wall_loop = body.get_face(wall).unwrap().outer;
    let height = |body: &Body<f64>, he| body.half_edge_start_point(he).unwrap().z;
    // Both rims at the ruling `u = 0.8`: the bottom rim runs forward
    // over `[0.2, 1.4]`, the top one reversed over `[0, 1.2]`.
    let rims: Vec<_> = body
        .edges()
        .filter(|(_, e)| (height(&body, e.he_plus) - height(&body, e.he_minus)).abs() < 1e-9)
        .map(|(k, e)| (k, height(&body, e.he_plus) < 0.5))
        .collect();
    let mids: Vec<_> = rims
        .into_iter()
        .map(|(edge, bottom)| {
            let t = if bottom { 0.8 } else { 0.6 };
            body.split_edge(edge, t, tol).unwrap().vertex
        })
        .collect();
    let leaving = |body: &Body<f64>, v| {
        body.half_edges()
            .find(|(_, d)| d.parent_loop == wall_loop && d.start == v)
            .unwrap()
            .0
    };
    let (he1, he2) = (leaving(&body, mids[0]), leaving(&body, mids[1]));
    let split = body
        .mef_chord(MefSite::Chords { he1, he2 }, tol)
        .unwrap()
        .face;
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    Sheet {
        body,
        seed,
        wall,
        split,
    }
}

fn outer(body: &Body<f64>, face: FaceKey) -> LoopKey {
    body.get_face(face).unwrap().outer
}

fn first_of(body: &Body<f64>, r#loop: LoopKey) -> HalfEdgeKey {
    match body.get_loop(r#loop).unwrap().boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => panic!("the sheet's loops are cycles"),
    }
}

/// Routes `r#loop`'s walk from its `first` through `via`'s `first` and
/// back: `next(a) := f`, `next(f) := next(a)`. The walk closes, keeps
/// every member of `r#loop`, and lists `f`.
fn divert(body: &mut Body<f64>, r#loop: LoopKey, via: LoopKey) -> HalfEdgeKey {
    let (a, f) = (first_of(body, r#loop), first_of(body, via));
    let after = body.get_half_edge(a).unwrap().next;
    body.get_half_edge_mut(a).unwrap().next = f;
    body.get_half_edge_mut(f).unwrap().next = after;
    assert!(body.loop_cycle(a).unwrap().contains(&f), "the walk strays");
    f
}

/// Closes `r#loop`'s walk past the member after its `first`, which still
/// claims the loop; returns that member.
fn skip(body: &mut Body<f64>, r#loop: LoopKey) -> HalfEdgeKey {
    let a = first_of(body, r#loop);
    let skipped = body.get_half_edge(a).unwrap().next;
    let after = body.get_half_edge(skipped).unwrap().next;
    body.get_half_edge_mut(a).unwrap().next = after;
    assert!(
        !body.loop_cycle(a).unwrap().contains(&skipped),
        "the walk skips"
    );
    skipped
}

/// The stored row of every half-edge `r#loop` holds, as the deep
/// snapshot prints them.
fn rows_of_loop(body: &Body<f64>, r#loop: LoopKey) -> Vec<(HalfEdgeKey, String)> {
    body.half_edges()
        .filter(|(_, data)| data.parent_loop == r#loop)
        .map(|(he, _)| (he, format!("{:?}", body.pcurve(he))))
        .collect()
}

/// What a door did on a torn body: returned, or panicked with this
/// report.
#[derive(Debug, PartialEq)]
enum Outcome {
    Returned(Result<(), EulerOpError>),
    Panicked(String),
}

/// Why `outcome` of `door` on a torn body breaks the plan's contract,
/// or `None`: a panic must state a row-4 premise ([`ROW_FOUR`]) — an
/// index out of bounds or an unwrap on `None` is a panic nobody
/// announced — and a typed refusal must be a real one, never
/// [`EulerOpError::Argument`], since every key the door was passed was
/// read out of the body.
fn unannounced(door: &str, outcome: &Outcome) -> Option<String> {
    match outcome {
        Outcome::Panicked(report) if !report.contains(ROW_FOUR) => {
            Some(format!("{door} panicked with no row-4 premise: {report}"))
        }
        Outcome::Returned(Err(e @ EulerOpError::Argument(_))) => Some(format!(
            "{door} refused a key it read out of the body as the caller's: {e}"
        )),
        _ => None,
    }
}

/// Runs `op` inside a surgery scope (a debug build's tier-1 sweep would
/// otherwise answer a torn input first) and asserts it leaves the rows
/// of `other`, a loop it does not own, as they were. An `Ok` that changed
/// them fails naming the half-edges whose rows it dropped or re-minted;
/// a refusal or a panic leaves the whole body deep-unchanged, and is
/// announced ([`unannounced`]). Returns the outcome.
fn assert_other_rows_kept(
    body: &mut Body<f64>,
    other: LoopKey,
    op: impl FnOnce(&mut Body<f64>) -> Result<(), EulerOpError>,
) -> Outcome {
    let before = deep_snapshot(body);
    let rows_before = rows_of_loop(body, other);
    let capture = PanicCapture::install();
    let got = match capture.run(|| op(&mut body.begin_surgery())) {
        Ok(returned) => Outcome::Returned(returned),
        Err(report) => Outcome::Panicked(report),
    };
    drop(capture);
    if let Some(why) = unannounced("the door", &got) {
        panic!("{why}");
    }
    if got != Outcome::Returned(Ok(())) {
        assert_eq!(deep_snapshot(body), before, "body changed ({got:?})");
    }
    let rows_after = rows_of_loop(body, other);
    let changed: Vec<HalfEdgeKey> = rows_before
        .iter()
        .filter(|row| !rows_after.contains(row))
        .map(|(he, _)| *he)
        .collect();
    assert!(
        changed.is_empty(),
        "the door returned {got:?} and dropped or re-minted the rows of {changed:?}, \
         of a loop it does not own"
    );
    got
}

/// For a door that moves or re-charts the walked loop whole: its plan
/// panics naming that loop, the body deep-unchanged.
fn assert_panics_on_the_walk(
    label: &str,
    body: &mut Body<f64>,
    walked: LoopKey,
    op: impl FnOnce(&mut Body<f64>) -> Result<(), EulerOpError>,
) {
    assert_torn_op_panics(label, body, &[&format!("loop {walked:?}"), ROW_FOUR], op);
}

#[test]
fn kfmrh_drops_no_row_of_a_loop_its_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    divert(&mut body, wall, split);
    let (seed, w) = (s.seed, s.wall);
    assert_panics_on_the_walk("kfmrh", &mut body.clone(), wall, |b| {
        b.kfmrh(seed, w).map(|_| ())
    });
    assert_panics_on_the_walk("kfmrh_minting", &mut body, wall, |b| {
        b.kfmrh_minting(seed, w, tol()).map(|_| ())
    });
}

#[test]
fn ring_move_drops_no_row_of_a_loop_its_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    body.kfmrh(s.seed, s.wall).unwrap();
    divert(&mut body, wall, split);
    let to = s.split;
    assert_panics_on_the_walk("ring_move", &mut body.clone(), wall, |b| {
        b.ring_move(wall, to)
    });
    assert_panics_on_the_walk("ring_move_minting", &mut body, wall, |b| {
        b.ring_move_minting(wall, to, tol())
    });
}

#[test]
fn mfkrh_drops_no_row_of_a_loop_its_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    body.kfmrh(s.seed, s.wall).unwrap();
    divert(&mut body, wall, split);
    let cylinder = body
        .get_surface(body.get_face(s.split).unwrap().surface)
        .unwrap()
        .clone();
    assert_panics_on_the_walk("mfkrh", &mut body, wall, |b| {
        // Lifts RechartUnvouched: the ring's meridian lines name the cylinder's own key, and a promotion onto a fresh key of it is the chart change whose drop is the row.
        b.lifting_rechart_refusals_for_tests(|b| {
            b.mfkrh(
                wall,
                FaceSurface::New {
                    surface: cylinder,
                    sense: true,
                },
            )
            .map(|_| ())
        })
    });
}

#[test]
fn set_face_surface_drops_no_row_of_a_loop_its_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    divert(&mut body, wall, split);
    let face = s.wall;
    let cylinder = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone();
    assert_panics_on_the_walk("set_face_surface", &mut body, wall, |b| {
        // Lifts RechartUnvouched: the wall's meridian lines name the cylinder's own key, and a re-chart onto a fresh key of it is the chart change whose drop is the row.
        b.lifting_rechart_refusals_for_tests(|b| {
            b.set_face_surface(
                face,
                FaceSurface::New {
                    surface: cylinder,
                    sense: true,
                },
            )
            .map(|_| ())
        })
    });
}

/// The claimed walk read directly, where the doors' own gates answer
/// first: the walk a site-row plan takes from a member
/// ([`Body::site_cycle_from`]) panics on a diverted loop, from every
/// member and from the stray, naming the loop and where it started; and
/// on the untorn sheet it is the loop's whole cycle.
#[test]
fn a_walk_from_a_member_panics_on_a_loop_it_strays_out_of() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let untorn = s.body.loop_cycle(first_of(&s.body, wall)).unwrap();
    for &he in &untorn {
        assert_eq!(
            s.body.site_cycle_from(he, wall).len(),
            untorn.len(),
            "the untorn walk from {he:?} is the loop's"
        );
    }
    let mut body = s.body;
    let stray = divert(&mut body, wall, split);
    let capture = PanicCapture::install();
    for &he in untorn.iter().chain([&stray]) {
        let premise = format!("the cycle walk of loop {wall:?} from {he:?} does not close");
        match capture.run(|| body.site_cycle_from(he, wall)) {
            Err(report) => assert!(
                report.contains(&premise) && report.contains(ROW_FOUR),
                "the walk from {he:?}: {report}"
            ),
            Ok(walk) => panic!("the walk from {he:?} returned {walk:?}"),
        }
    }
    let premise = format!("loop {wall:?} does not resolve, or its cycle walk does not close");
    match capture.run(|| body.site_cycle(wall)) {
        Err(report) => assert!(
            report.contains(&premise) && report.contains(ROW_FOUR),
            "the walk from the loop's first: {report}"
        ),
        Ok(walk) => panic!("the walk from the loop's first returned {walk:?}"),
    }
}

/// The two site-row plans a make operator runs, `mev`'s fan and `mef`'s
/// chord, at every site of the diverted wall. A door that re-minted the
/// wall from the walk wrote the split face's member in the wall's chart;
/// each stops instead, the body deep-unchanged, and some site's plan
/// reaches the wall's walk and panics naming it.
#[test]
fn the_make_operators_re_mint_no_row_of_a_loop_their_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    let stray = divert(&mut body, wall, split);
    let walk = body.loop_cycle(first_of(&body, wall)).unwrap();
    let members: Vec<HalfEdgeKey> = walk.iter().copied().filter(|&he| he != stray).collect();
    let names_the_walk = |got: &Outcome| {
        matches!(got, Outcome::Panicked(report)
            if report.contains(&format!("loop {wall:?}")))
    };
    let mut plan_panicked = 0;
    for &he in &members {
        let p = body.half_edge_start_point(he).unwrap();
        let tip = Point3::new(
            p.x * 0.95,
            p.y * 0.95,
            p.z + if p.z < 0.5 { 0.1 } else { -0.1 },
        );
        let got = assert_other_rows_kept(&mut body.clone(), split, |b| {
            b.mev_line(MevSite::Fan { he1: he, he2: he }, tip, tol())
                .map(|_| ())
        });
        assert_ne!(
            got,
            Outcome::Returned(Ok(())),
            "mev_line fan at {he:?} returned Ok"
        );
        plan_panicked += usize::from(names_the_walk(&got));
        for &he2 in &members {
            if he2 != he {
                let got = assert_other_rows_kept(&mut body.clone(), split, |b| {
                    b.mef_chord(MefSite::Chords { he1: he, he2 }, tol())
                        .map(|_| ())
                });
                assert_ne!(
                    got,
                    Outcome::Returned(Ok(())),
                    "mef_chord at {he:?}, {he2:?} returned Ok"
                );
                plan_panicked += usize::from(names_the_walk(&got));
            }
        }
    }
    assert!(
        plan_panicked > 0,
        "some site reached its rows plan, which panicked on the wall's walk"
    );
}

/// The pcurve pass over a named face clears and re-derives that face's
/// rows by the same walk ([`crate::pcurves::mint_pcurves_of`]): every
/// other row of the body is left exactly as found, a diverted loop's
/// stray included, and the pass refuses the torn loop as corrupt rather
/// than as a geometry fault of the stray it walked.
#[test]
fn the_pass_over_one_face_leaves_the_rows_of_a_loop_its_walk_strays_into() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    divert(&mut body, wall, split);
    let rows_before = rows_of_loop(&body, split);
    let got = crate::pcurves::mint_pcurves_of(&mut body, &[s.wall], tol());
    assert_eq!(
        rows_of_loop(&body, split),
        rows_before,
        "the pass over the wall returned {got:?} and changed the split face's rows"
    );
    assert_eq!(got, Err(crate::PcurveMintError::Corrupt));
}

/// `kef` at every half-edge of the diverted sheet stops — a real
/// refusal or a row-4 panic — the body deep-unchanged. Its site-row plan
/// reads the wall's rows to decide whether it re-mints the face; a loop
/// that does not walk as its own has no rows to read, so the plan panics
/// rather than read it as nothing to re-mint and splice the tear
/// further.
#[test]
fn kef_stops_at_every_half_edge_of_a_diverted_sheet() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    divert(&mut body, wall, split);
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(he, _)| he).collect();
    assert!(
        halves.len() >= 14,
        "the sheet's half-edges: {}",
        halves.len()
    );
    for he in halves {
        let got = assert_other_rows_kept(&mut body.clone(), split, |b| b.kef(he).map(|_| ()));
        assert_ne!(
            got,
            Outcome::Returned(Ok(())),
            "kef({he:?}) on the diverted sheet returned Ok"
        );
    }
}

/// **The limit of the claimed walk** ([`Body::loop_cycle_of`]): a
/// diversion through the split face's `first` paired with that member's
/// `parent_loop` re-pointed at the wall passes the claim and the `Whole`
/// proof, so `kfmrh` moves the stray with the wall and drops its row.
/// Only the stray's `prev` still names the split face's loop. This row
/// pins the limit as it stands; a door that reads `prev` reds it.
#[test]
fn a_diversion_paired_with_a_parent_loop_tear_passes_the_proof() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    let stray = divert(&mut body, wall, split);
    body.get_half_edge_mut(stray).unwrap().parent_loop = wall;
    assert!(body.pcurve(stray).is_some(), "the stray carries a row");
    assert!(
        body.whole_cycle(wall).contains(&stray),
        "the walk, claim and Whole proof take the stray as a member"
    );
    let mut scope = body.begin_surgery();
    let got = scope.kfmrh(s.seed, s.wall).map(|_| ());
    drop(scope);
    assert_eq!(got, Ok(()), "kfmrh moves the wall with the stray");
    assert!(
        body.pcurve(stray).is_none(),
        "the stray's row is dropped with the wall's"
    );
}

/// A walk closed past a member hands the door every row but that
/// member's: a door that moves or re-charts the loop whole would leave
/// it stated in the chart the loop left.
#[test]
fn the_loop_doors_panic_on_a_walk_that_skips_a_member() {
    let s = sheet();
    let wall = outer(&s.body, s.wall);
    let mut torn = s.body.clone();
    skip(&mut torn, wall);
    let (seed, w, to) = (s.seed, s.wall, s.split);
    assert_panics_on_the_walk("kfmrh", &mut torn.clone(), wall, |b| {
        b.kfmrh(seed, w).map(|_| ())
    });
    let cylinder = torn
        .get_surface(torn.get_face(w).unwrap().surface)
        .unwrap()
        .clone();
    assert_panics_on_the_walk("set_face_surface", &mut torn, wall, |b| {
        // Lifts RechartUnvouched: the wall's meridian lines name the cylinder's own key, and a re-chart onto a fresh key of it is the chart change whose drop is the row.
        b.lifting_rechart_refusals_for_tests(|b| {
            b.set_face_surface(
                w,
                FaceSurface::New {
                    surface: cylinder,
                    sense: true,
                },
            )
            .map(|_| ())
        })
    });
    let mut ringed = s.body;
    ringed.kfmrh(seed, w).unwrap();
    skip(&mut ringed, wall);
    assert_panics_on_the_walk("ring_move_minting", &mut ringed.clone(), wall, |b| {
        b.ring_move_minting(wall, to, tol())
    });
    assert_panics_on_the_walk("mfkrh", &mut ringed, wall, |b| {
        b.mfkrh(wall, FaceSurface::Inherit).map(|_| ())
    });
}

/// A door's whole-loop proof panics on a torn ring before anything moves,
/// even where the chart does not change and nothing is dropped: the
/// proof is the plan's, not the drop's.
#[test]
fn the_whole_proof_runs_on_the_same_chart_too() {
    let s = sheet();
    let (wall, split) = (outer(&s.body, s.wall), outer(&s.body, s.split));
    let mut body = s.body;
    body.kfmrh(s.seed, s.wall).unwrap();
    divert(&mut body, wall, split);
    assert_panics_on_the_walk("mfkrh", &mut body, wall, |b| {
        b.mfkrh(wall, FaceSurface::Inherit).map(|_| ())
    });
}

/// No false alarm: on the untorn sheet, and on it with the wall demoted
/// to a ring of the seed face, every door the torn rows drive runs at
/// every site without a walk panic, each reaching `Ok` somewhere.
#[test]
fn the_untorn_sheet_panics_on_no_walk() {
    let s = sheet();
    let mut ringed = s.body.clone();
    ringed.kfmrh(s.seed, s.wall).unwrap();
    let mut ok: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for body in [s.body, ringed] {
        for call in row_calls(&body) {
            let mut trial = body.clone();
            match (call.run)(&mut trial) {
                Ok(()) => *ok.entry(call.door).or_default() += 1,
                Err(e @ EulerOpError::Argument(_)) => {
                    panic!("{} refused a key read out of the body: {e}", call.door)
                }
                Err(_) => {}
            }
        }
    }
    for door in [
        "kfmrh",
        "kfmrh_minting",
        "ring_move",
        "ring_move_minting",
        "mfkrh",
        "mev_line",
        "mef_chord",
        "kef",
        "mekr_chord",
        "set_face_surface",
    ] {
        assert!(
            ok.get(door).is_some_and(|&n| n > 0),
            "{door} ran to Ok somewhere: {ok:?}"
        );
    }
}

/// A door call on a body, as [`RowCall`] holds it.
type DoorCall = dyn Fn(&mut Body<f64>) -> Result<(), EulerOpError>;

/// One door call the torn sweep makes: the faces whose rows it may
/// write, and the call.
struct RowCall {
    door: &'static str,
    faces: Vec<FaceKey>,
    run: Box<DoorCall>,
}

/// Every call of the doors that decide rows by a loop walk, at every
/// site `body` offers: `kfmrh` (both doors) at every ordered face pair
/// whose second face has no rings, `ring_move` (both) of every ring to
/// every face, `mfkrh` of every ring inheriting the chart, `mev_line`'s
/// fan at every half-edge, `mef_chord` at every same-loop pair, `kef` at
/// every half-edge, `mekr_chord` joining every ring's first member to
/// every member of its face's outer loop, and `set_face_surface` onto a
/// fresh key of every face's own surface (lifting the re-chart refusals,
/// so the drop is reached). The faces a call may write rows on are the
/// faces of the loops it names.
fn row_calls(body: &Body<f64>) -> Vec<RowCall> {
    let face_of = |he: HalfEdgeKey| {
        body.get_half_edge(he)
            .and_then(|d| body.get_loop(d.parent_loop))
            .map(|l| l.face)
    };
    let faces: Vec<(FaceKey, crate::entity::Face)> =
        body.faces().map(|(k, f)| (k, f.clone())).collect();
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let mut calls: Vec<RowCall> = Vec::new();
    for (f1, _) in &faces {
        for (f2, f2_data) in &faces {
            if f1 == f2 || !f2_data.rings.is_empty() {
                continue;
            }
            let (f1, f2) = (*f1, *f2);
            calls.push(RowCall {
                door: "kfmrh",
                faces: vec![f1, f2],
                run: Box::new(move |b| b.kfmrh(f1, f2).map(|_| ())),
            });
            calls.push(RowCall {
                door: "kfmrh_minting",
                faces: vec![f1, f2],
                run: Box::new(move |b| b.kfmrh_minting(f1, f2, tol()).map(|_| ())),
            });
        }
    }
    for (from, from_data) in &faces {
        for &ring in &from_data.rings {
            for (to, _) in &faces {
                let (from, to) = (*from, *to);
                calls.push(RowCall {
                    door: "ring_move",
                    faces: vec![from, to],
                    run: Box::new(move |b| b.ring_move(ring, to)),
                });
                calls.push(RowCall {
                    door: "ring_move_minting",
                    faces: vec![from, to],
                    run: Box::new(move |b| b.ring_move_minting(ring, to, tol())),
                });
            }
            let from = *from;
            calls.push(RowCall {
                door: "mfkrh",
                faces: vec![from],
                run: Box::new(move |b| b.mfkrh(ring, FaceSurface::Inherit).map(|_| ())),
            });
            if let LoopBoundary::Cycle { first: ring_first } = body.get_loop(ring).map_or(
                LoopBoundary::Empty {
                    vertex: Default::default(),
                },
                |l| l.boundary,
            ) && let Some(Some(target_walk)) =
                body.get_loop(from_data.outer).map(|l| match l.boundary {
                    LoopBoundary::Cycle { first } => body.loop_cycle(first),
                    LoopBoundary::Empty { .. } => None,
                })
            {
                for target in target_walk {
                    calls.push(RowCall {
                        door: "mekr_chord",
                        faces: vec![from],
                        run: Box::new(move |b| {
                            b.mekr_chord(
                                MekrSite::Cycles {
                                    target,
                                    ring: ring_first,
                                },
                                tol(),
                            )
                            .map(|_| ())
                        }),
                    });
                }
            }
        }
    }
    for &he in &halves {
        let Some(face) = face_of(he) else { continue };
        let faces_here: Vec<FaceKey> = [Some(face), body.mate(he).and_then(face_of)]
            .into_iter()
            .flatten()
            .collect();
        if let Some(p) = body.half_edge_start_point(he) {
            let tip = Point3::new(p.x * 0.95 + 0.01, p.y * 0.95 + 0.01, p.z + 0.05);
            calls.push(RowCall {
                door: "mev_line",
                faces: faces_here.clone(),
                run: Box::new(move |b| {
                    b.mev_line(MevSite::Fan { he1: he, he2: he }, tip, tol())
                        .map(|_| ())
                }),
            });
        }
        calls.push(RowCall {
            door: "kef",
            faces: faces_here,
            run: Box::new(move |b| b.kef(he).map(|_| ())),
        });
        let parent = body.get_half_edge(he).unwrap().parent_loop;
        for &he2 in &halves {
            if he2 != he && body.get_half_edge(he2).unwrap().parent_loop == parent {
                calls.push(RowCall {
                    door: "mef_chord",
                    faces: vec![face],
                    run: Box::new(move |b| {
                        b.mef_chord(MefSite::Chords { he1: he, he2 }, tol())
                            .map(|_| ())
                    }),
                });
            }
        }
    }
    for (face, data) in &faces {
        let Some(surface) = body.get_surface(data.surface).cloned() else {
            continue;
        };
        let (face, sense) = (*face, data.sense);
        calls.push(RowCall {
            door: "set_face_surface",
            faces: vec![face],
            run: Box::new(move |b| {
                b.lifting_rechart_refusals_for_tests(|b| {
                    b.set_face_surface(
                        face,
                        FaceSurface::New {
                            surface: surface.clone(),
                            sense,
                        },
                    )
                    .map(|_| ())
                })
            }),
        });
    }
    calls
}

/// Calls, `Err`s, row-4 panics, and `Ok`s that changed a row of a
/// half-edge on a face the call does not name, per door, over one or two
/// [`Tear::NextForeign`] tears of each body at each seed.
type RowTable = std::collections::BTreeMap<&'static str, [usize; 4]>;

/// A sweep's table, and every outcome [`unannounced`] reports, with the
/// seed and tear count that reproduce it.
struct RowSweep {
    table: RowTable,
    unannounced: Vec<String>,
}

fn row_walk_rows(seeds: &[u64]) -> RowSweep {
    let bodies: Vec<(&str, BuildFixture)> = FIXTURES.iter().copied().chain([SHEET]).collect();
    row_walk_rows_on(seeds, &bodies)
}

/// The split sheet as a [`row_walk_rows_on`] body.
const SHEET: (&str, BuildFixture) = ("split sheet", |_| sheet().body);

/// [`row_walk_rows`] over the named bodies.
fn row_walk_rows_on(seeds: &[u64], bodies: &[(&str, BuildFixture)]) -> RowSweep {
    use test_utils::fuzz::Rng;
    let mut table = RowTable::new();
    let mut unannounced_outcomes = Vec::new();
    let capture = PanicCapture::install();
    for &seed in seeds {
        for tears in [1, 2] {
            for &(name, build) in bodies {
                let mut body = build(tol());
                let mut rng = Rng::from_seed(seed);
                for _ in 0..tears {
                    plant(
                        &mut body,
                        Tear::NextForeign,
                        &mut rng,
                        HalfEdgeKey::default(),
                    );
                }
                let face_before: Vec<(HalfEdgeKey, Option<FaceKey>, String)> = body
                    .half_edges()
                    .map(|(he, d)| {
                        let face = body.get_loop(d.parent_loop).map(|l| l.face);
                        (he, face, format!("{:?}", body.pcurve(he)))
                    })
                    .collect();
                for call in row_calls(&body) {
                    let cells = table.entry(call.door).or_insert([0; 4]);
                    let mut trial = body.clone();
                    let outcome = match capture.run(|| (call.run)(&mut trial.begin_surgery())) {
                        Ok(returned) => Outcome::Returned(returned),
                        Err(report) => Outcome::Panicked(report),
                    };
                    cells[0] += 1;
                    if let Some(why) = unannounced(call.door, &outcome) {
                        unannounced_outcomes
                            .push(format!("{name}, seed {seed}, {tears} tears: {why}"));
                    }
                    match outcome {
                        Outcome::Returned(Ok(())) => {}
                        Outcome::Returned(Err(_)) => {
                            cells[1] += 1;
                            continue;
                        }
                        Outcome::Panicked(_) => {
                            cells[2] += 1;
                            continue;
                        }
                    }
                    let foreign = face_before.iter().any(|(he, face, row)| {
                        trial.get_half_edge(*he).is_some()
                            && !face.is_some_and(|f| call.faces.contains(&f))
                            && format!("{:?}", trial.pcurve(*he)) != *row
                    });
                    cells[3] += usize::from(foreign);
                }
            }
        }
    }
    RowSweep {
        table,
        unannounced: unannounced_outcomes,
    }
}

/// No call of the sweep returned `Ok` having changed another face's
/// rows, and every refusal and panic it met was announced.
fn assert_no_foreign_row_written(sweep: &RowSweep, context: &str) {
    let written: Vec<_> = sweep.table.iter().filter(|(_, c)| c[3] > 0).collect();
    assert!(
        written.is_empty(),
        "a door returned Ok having changed a row of a face it does not name: {written:?} ({context})"
    );
    assert!(
        sweep.unannounced.is_empty(),
        "unannounced outcomes ({context}):\n{}",
        sweep.unannounced.join("\n")
    );
}

/// [`row_walk_rows`] on a few seeds, asserting no `Ok` changed another
/// face's rows and every refusal and panic was announced: a
/// counterexample search on the shared fuzz seed and effort dial.
#[test]
fn row_walks_on_a_few_torn_bodies() {
    let mut rng = test_utils::fuzz::start("row_walks_on_a_few_torn_bodies");
    let seeds: Vec<u64> = (0..test_utils::fuzz::scaled(4))
        .map(|_| rng.next_u64())
        .collect();
    assert_no_foreign_row_written(&row_walk_rows(&seeds), &test_utils::fuzz::replay());
}

/// [`row_walk_rows_on`] the split sheet at four fixed seeds, asserting
/// no `Ok` changed another face's rows and every refusal and panic was
/// announced. The seeds are fixture
/// identifiers, not a search: their `NextForeign` tears route the wall's
/// walk through the split face, so between them `kfmrh`,
/// `kfmrh_minting`, `mef_chord`, `mev_line` and lifted `set_face_surface`
/// each reach a foreign row through the walk they decide their rows by,
/// and a walk that does not prove its members writes it.
#[test]
fn row_walks_on_seeds_that_divert_the_sheet() {
    let sweep = row_walk_rows_on(&[5, 14, 97, 150], &[SHEET]);
    for door in [
        "kfmrh",
        "kfmrh_minting",
        "mef_chord",
        "mev_line",
        "set_face_surface",
    ] {
        assert!(
            sweep.table.get(door).is_some_and(|c| c[0] > 0),
            "{door} is called on the torn sheet"
        );
    }
    assert_no_foreign_row_written(&sweep, "split sheet, seeds 5, 14, 97, 150");
}

/// **Evidence, not a gate**: [`row_walk_rows`] on seeds `1..=2000`,
/// run by hand, which prints the table and asserts the written column
/// is 0 and every outcome announced.
///
/// `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false cargo test --release -p
/// topo --lib -- --ignored --nocapture
/// row_walk_proofs::row_walks_on_torn_bodies`
#[test]
#[ignore = "evidence: the row walks' tear measurement, run by hand"]
#[cfg(not(debug_assertions))]
fn row_walks_on_torn_bodies() {
    let seeds: Vec<u64> = (1..=2000).collect();
    let sweep = row_walk_rows(&seeds);
    println!("| door | calls | `Err` | row-4 panic | `Ok`, another face's row changed |");
    println!("|---|---|---|---|---|");
    for (door, [calls, err, panicked, written]) in &sweep.table {
        println!("| `{door}` | {calls} | {err} | {panicked} | {written} |");
    }
    assert_no_foreign_row_written(&sweep, "seeds 1..=2000");
}

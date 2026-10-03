//! **A face the section crosses leaves the join minted whole.** The
//! boolean and splitting pipelines hang null edges on the operand's
//! faces (`Body::mev_null`), join them with chords (`mef`, `mekr`) and
//! kill them undescribed (`kef`, `kemr`). A null edge's halves have no
//! carrier, so while one holds a loop open that loop cannot be minted;
//! the Euler operators that rewire a loop out from under it mint it
//! there, at the mint site (the site mint, `topo::pcurves`).
//!
//! Each row runs a pipeline on a minted curved operand up to the end
//! of its join — every null edge killed, before the finish and the
//! closing mint — and reads every face that stores a row: it is
//! complete, and its rows are the minting pass's, byte for byte. At
//! this unit's merge base the join left the cut wall half-minted: the
//! chord halves the join's `mef`s added went rowless, because the face
//! they landed on was missing its null halves' rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::test_support::{boolean_through_the_join, split_through_the_join};

use crate::common::operands::m5_boss;
use topo::{Body, BooleanOp, FaceKey, HalfEdgeKey, LoopBoundary};

fn tol() -> Tol {
    Tol::witness()
}

/// The rod: radius `0.5` about `y` over `y ∈ [0, 4]`, revolved, so its
/// wall is a minted cylinder chart.
fn rod() -> Body<f64> {
    let profile = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(0.5, 0.0),
        Point2::new(0.5, 4.0),
        Point2::new(0.0, 4.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![profile])
        .validate(tol())
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .unwrap()
    .body
}

fn halves_of(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
            LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
            LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .collect()
}

/// Every stored row of `face`, with its interval, image and
/// certificate, sorted.
fn rows_of(body: &Body<f64>, face: FaceKey) -> Vec<String> {
    let mut out: Vec<String> = halves_of(body, face)
        .into_iter()
        .filter_map(|he| {
            let c = body.pcurve(he)?;
            Some(format!(
                "{he:?} {:?} {:?} {:?}",
                c.params(),
                c.pcurve(),
                c.certificate()
            ))
        })
        .collect();
    out.sort();
    out
}

/// What the join leaves on `body`: no null edge, and every face storing
/// a row complete, with the rows the minting pass derives for it.
/// Returns how many faces store a row.
fn every_minted_face_is_the_passs(body: &Body<f64>, what: &str) -> usize {
    let nulls = body
        .edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .is_some_and(|c| c.null_scaffold().is_some())
        })
        .count();
    assert_eq!(nulls, 0, "{what}: the join kills every null edge");
    let mut minted = 0;
    for (face, _) in body.faces() {
        let rows = rows_of(body, face);
        if rows.is_empty() {
            continue;
        }
        minted += 1;
        let halves = halves_of(body, face).len();
        assert_eq!(
            rows.len(),
            halves,
            "{what}: face {face:?} stores {} of its {halves} rows after the join",
            rows.len()
        );
        let mut pass = body.clone();
        topo::mint_pcurves_of(&mut pass, &[face], tol()).unwrap();
        assert_eq!(
            rows,
            rows_of(&pass, face),
            "{what}: face {face:?}'s rows are the minting pass's, byte for byte"
        );
    }
    minted
}

/// **The split.** The rod split by the plane through `y = 2` tilted 20°
/// about `x`: the section ellipse crosses the wall and its seam, and
/// the wall's two pieces leave the join minted whole.
#[test]
fn an_oblique_split_leaves_the_cut_wall_minted_whole_at_the_join() {
    let theta = 20f64.to_radians();
    let body = split_through_the_join(
        &rod(),
        &topo::test_support::split_plane(
            Point3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, theta.cos(), theta.sin()),
            geom_core::Tol::witness(),
        ),
        tol(),
    )
    .expect("the oblique split joins");
    let minted = every_minted_face_is_the_passs(&body, "the oblique split");
    assert!(minted >= 2, "the cut wall's pieces are minted: {minted}");
}

/// **The boolean.** A plate with the three-arc boss unioned on — its
/// walls minted cylinder charts — unioned again with a slab across the
/// boss above the plate: the slab's faces cut every boss wall along a
/// circle arc, and the walls' pieces leave the join minted whole. The
/// slab is a planar brick, whose chart mints nothing, so it stores no
/// row for the join to leave half-minted and is not read.
#[test]
fn a_slab_across_a_minted_boss_leaves_its_walls_minted_whole_at_the_join() {
    let plate = finished(
        "the plate",
        brick((0.0, 3.0), (0.0, 3.0), (0.0, 0.8), tol()),
        tol(),
    );
    let boss = finished("the boss", m5_boss(3, 0.3, 1.3), tol());
    let first = topo::union(&plate, &boss, tol())
        .expect("the boss unions on")
        .body()
        .expect("a body remains")
        .body
        .clone();
    let slab = brick((0.0, 3.0), (0.0, 3.0), (1.0, 1.15), tol());
    let (a, _) = boolean_through_the_join(BooleanOp::Union, &first, &slab, tol())
        .expect("the pipeline reaches its join")
        .expect("the slab joins");
    let minted = every_minted_face_is_the_passs(&a, "the bossed plate");
    assert!(minted >= 6, "every cut boss wall is minted: {minted}");
}

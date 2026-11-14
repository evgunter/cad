//! **An aimed ray keeps its line over a box** (`topo`'s side reader,
//! `side_reader::aimed_ray`): a slab extruded by a boxed depth at the
//! symbolic box scalar reads which side holds its material by its rays,
//! and a ray whose reading is not the same for every member of the box
//! is set aside.
//!
//! The slab is the unit square extruded along `+z`. Its bottom cap's aim
//! is `(0.191, 0.382, 0)` and its ray runs up at 10° to the cap's normal,
//! toward the wall at `x = 0`, which it reaches at height `1.469`: below
//! that depth the ray leaves through the top cap, above it through the
//! wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::{Band, Bounds, Interval, ParamSymbol, Point2, Real, Sym, SymBudget, Tol};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

type Boxed = Sym<Interval>;

/// The slab's leaf, as the extrude door hands it on, over a depth in
/// `[lo, hi]`.
fn slab_leaf(lo: f64, hi: f64, tol: Tol) -> topo::Replay<Boxed> {
    let at = |x: f64, y: f64| Point2::new(Boxed::from_f64(x), Boxed::from_f64(y));
    let square = RawLoop::new([
        (at(0.0, 0.0), Segment::Line),
        (at(1.0, 0.0), Segment::Line),
        (at(1.0, 1.0), Segment::Line),
        (at(0.0, 1.0), Segment::Line),
    ]);
    let profile = Profile::new(SketchPlane::<Boxed>::xy(), vec![square])
        .validate(tol)
        .expect("the square validates");
    let depth = Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id("slab_depth")),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    );
    let extrusion = Extrusion::Distance {
        depth,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, extrusion, tol)
        .expect("the slab extrudes over the box")
        .body
}

/// Runs `read` on the slab's leaf under a symbolic session.
fn over_slab<R>(lo: f64, hi: f64, read: impl FnOnce(topo::Replay<Boxed>, Tol, Band) -> R) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    geom_core::sym::with_session(budget, || read(slab_leaf(lo, hi, tol), tol, band)).0
}

/// What the ray aimed at the slab's bottom cap reads, alone.
fn bottom_cap_ray(lo: f64, hi: f64) -> String {
    over_slab(lo, hi, |leaf, _, band| {
        let body = leaf.into_body();
        let bottom = body
            .faces()
            .map(|(face, _)| face)
            .find(|&face| match *body.face(face).unwrap().surface() {
                geom::Surface::Plane { normal, .. } => normal.z.hi() < -0.5,
                _ => false,
            })
            .expect("the slab has a cap facing down");
        let prefix = format!("{bottom:?}: ");
        topo::test_support::each_aim(&body, band)
            .into_iter()
            .find_map(|line| line.strip_prefix(&prefix).map(str::to_owned))
            .expect("the bottom cap offers an aim")
    })
}

/// **A slab whose depth is a box a twentieth of it wide reads its side
/// by its rays, and its taker finishes it**: the extrude door's leaf at
/// the symbolic box scalar, depth `1 ± 0.025`, read by the side reader's
/// rays alone (no volume), and taken through tier 3. The plane normals
/// are the extrude's own. Red against a ray rebuilt from a start point
/// derived from its aim: every ray is then set aside at this width and
/// the reading falls to the volume.
#[test]
fn a_boxed_slab_reads_its_side_by_its_rays_and_is_taken() {
    let (rays, taken) = over_slab(0.975, 1.025, |leaf, tol, band| {
        let rays = topo::test_support::aimed_rays(&leaf.clone().into_body(), band);
        (rays, topo::take::<Boxed>(&Arc::new(leaf), tol).map(|_| ()))
    });
    assert_eq!(rays, ["Ok(Side(Out))"], "the slab's rays read its side");
    taken.unwrap_or_else(|e| panic!("the taker finishes the slab: {e}"));
}

/// **A ray whose exit moves from one face to its neighbour across the
/// box still reads**: over depths that hold `1.469` the bottom cap's
/// ray leaves through the top cap for some members and through the wall
/// for others. Its count turns at its aim, before the exit, and the exit
/// is one leaving crossing whichever face holds it: the edge between
/// them, met in band, stands for both. Red against a walk that faults on
/// an in-face reading past the crossing that reads.
#[test]
fn a_ray_whose_exit_moves_across_an_edge_over_the_box_still_reads() {
    for (what, lo, hi) in [
        ("leaving through the top cap", 1.40, 1.45),
        ("leaving through either", 1.44, 1.50),
        ("leaving through the wall", 1.49, 1.54),
    ] {
        assert_eq!(
            bottom_cap_ray(lo, hi),
            "Ok(Side(Out))",
            "depth in [{lo}, {hi}], {what}"
        );
    }
}

/// What a ray aimed straight down at the middle of the slab's top edge
/// over `x = x_edge` reads, the edge taken at depth 1.
fn top_edge_ray(x_edge: f64, lo: f64, hi: f64) -> String {
    over_slab(lo, hi, |leaf, _, band| {
        let body = leaf.into_body();
        let face = body.faces().next().expect("the slab has faces").0;
        let c = Boxed::from_f64;
        let aim = geom_core::Point3::new(c(x_edge), c(0.5), c(1.0));
        let up = geom_core::Vec3::new(c(0.0), c(0.0), c(1.0));
        topo::test_support::ray_at(&body, face, aim, up, band)
    })
}

/// **A ray that crosses the slab for some members of the box and misses
/// it for others reads nothing.** Two rays come down at the aimed
/// incidence onto the points where the top cap's edges over `x = 0` and
/// `x = 1` lie at depth 1, both drifting the same way across `x`. One
/// drifts into the slab: it enters through the wall or the top cap,
/// whichever the depth puts there, and reads `Out` over the whole box.
/// The other drifts out of it: where the slab is deeper than 1 it clips
/// the corner under that edge, and where it is shallower it passes above
/// the slab and crosses nothing, so over a box holding both no member's
/// crossing is every member's, and it reads nothing. Over depths a
/// tenth and more above 1 the same ray clips the corner for every
/// member, and reads.
/// Red against a ray that reads the crossing one member has.
#[test]
fn a_ray_that_only_some_members_of_the_box_cross_reads_nothing() {
    let over = |lo: f64, hi: f64| [0.0, 1.0].map(|x_edge| top_edge_ray(x_edge, lo, hi));
    let straddling = over(0.975, 1.025);
    let leaving = straddling
        .iter()
        .position(|read| read != "Ok(Side(Out))")
        .unwrap_or_else(|| panic!("one ray drifts out of the slab: {straddling:?}"));
    assert_eq!(
        straddling[1 - leaving],
        "Ok(Side(Out))",
        "the ray drifting into the slab enters it for every member"
    );
    assert!(
        straddling[leaving].starts_with("Ok(SetAside("),
        "the ray only some members cross reads nothing: {}",
        straddling[leaving]
    );
    assert_eq!(
        over(1.1, 1.14)[leaving],
        "Ok(Side(Out))",
        "the same ray, where every member's corner is clipped"
    );
}

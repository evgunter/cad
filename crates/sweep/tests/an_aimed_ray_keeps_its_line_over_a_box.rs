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
/// box is set aside**: over depths that hold `1.469` the bottom cap's
/// ray leaves through the top cap for some members and through the wall
/// for others, and for one it runs through the edge between them. How
/// far it passes that edge is then no one reading (`side_at_infinity_edge`,
/// an enclosure from zero up), and an edge met on such a gap would stand
/// for crossings only some members make. Over a box a hundredth as wide,
/// wholly below, every member leaves through the top cap clear of the
/// edge and the ray reads `Out`. Red against a walk that reads an
/// undecided gap as an edge met in band.
#[test]
fn a_ray_whose_exit_moves_across_an_edge_over_the_box_is_set_aside() {
    assert_eq!(
        bottom_cap_ray(1.44, 1.445),
        "Ok(Side(Out))",
        "depth in [1.44, 1.445], leaving through the top cap"
    );
    let straddling = bottom_cap_ray(1.44, 1.50);
    assert!(
        straddling.starts_with("Ok(SetAside(Crossing(")
            && straddling.contains("side_at_infinity_edge"),
        "depth in [1.44, 1.50], leaving through either: {straddling}"
    );
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
/// `x = 1` lie at depth 1, both drifting the same way across `x`: one
/// into the slab, one out of it. Where the slab is deeper than 1 the ray
/// drifting out clips the corner under its edge; where it is shallower
/// that ray passes above the slab and crosses nothing.
///
/// - Over depths wholly below 1 the ray drifting out reads nothing
///   (which says which of the two it is) and the other reads `Out`.
/// - Over depths wholly above 1 both read `Out`.
/// - Over depths holding 1 neither reads: each runs through its edge
///   for the member at depth 1, and the one drifting out has no
///   crossing that is every member's.
///
/// Red against a ray that reads the crossing one member has.
#[test]
fn a_ray_that_only_some_members_of_the_box_cross_reads_nothing() {
    let over = |lo: f64, hi: f64| [0.0, 1.0].map(|x_edge| top_edge_ray(x_edge, lo, hi));
    let below = over(0.9, 0.91);
    let leaving = below
        .iter()
        .position(|read| read == "Ok(SetAside(Graze))")
        .unwrap_or_else(|| panic!("one ray passes above the shallow slab: {below:?}"));
    assert_eq!(
        below[1 - leaving],
        "Ok(Side(Out))",
        "the ray drifting into the shallow slab enters it"
    );
    assert_eq!(
        over(1.1, 1.14),
        ["Ok(Side(Out))", "Ok(Side(Out))"],
        "both rays cross every member of the deep slab"
    );
    for read in over(0.975, 1.025) {
        assert!(
            read.starts_with("Ok(SetAside("),
            "over depths holding 1 neither ray reads: {read}"
        );
    }
}

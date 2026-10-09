//! **An annular tube through a plate.** Concentric circles, each with
//! its one vertex at its own azimuth, extruded into a tube and run
//! against the plate `(−3, 3)² × (0, 1)`. The plate's top face cuts
//! every wall across its seam: one-site loops in one face, which the
//! wrap-edge arm joins. Each section loop's region faces are then the
//! plate's disc or annulus between two circles, planar faces every
//! vertex and edge midpoint of which lies on the tube's walls, so only a
//! point of a region face's interior decides the loop's role
//! (`topo::stands`, rung 3). Where the circles' vertices sit at opposed
//! azimuths no chord between two vertices lies in the annulus, and the
//! disc has one vertex.
//!
//! Every row runs ∪, ∩ and both differences in both operand orders
//! (`differential::every_op_both_orders`), each held to its closed-form
//! volume within the quadrature's pad, read `SOUND` through
//! `differential::outcome`, and meshed and `check_mesh`ed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::brick;
use sweep::{ExtrudeSide, Extrusion, extrude};
use test_utils::fuzz;
use topo::Body;

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// One circle of a tube's profile: radius, first vertex's azimuth, and
/// how many equal arcs it is drawn in.
#[derive(Clone, Copy, Debug)]
struct Ring {
    r: f64,
    az: f64,
    arcs: usize,
}

fn ring(r: f64, az: f64) -> Ring {
    Ring { r, az, arcs: 1 }
}

/// `rings`, outermost first, as profile loops about `(cx, cy)`: the
/// first is the outer boundary and they alternate in sense inward.
fn loops(c: (f64, f64), rings: &[Ring]) -> Vec<ProfileLoop<f64>> {
    rings
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let sweep = if i % 2 == 0 { TAU } else { -TAU };
            let n = g.arcs as f64;
            RawLoop::new((0..g.arcs).map(|k| {
                let a = g.az + sweep * k as f64 / n;
                (
                    Point2::new(c.0 + g.r * a.cos(), c.1 + g.r * a.sin()),
                    Segment::Arc(Arc2 {
                        centre: Point2::new(c.0, c.1),
                        radius: g.r,
                        sweep: sweep / n,
                    }),
                )
            }))
        })
        .collect()
}

/// The profile's area: the disc of each even ring less the next one's.
fn area(rings: &[Ring]) -> f64 {
    rings
        .iter()
        .enumerate()
        .map(|(i, g)| if i % 2 == 0 { 1.0 } else { -1.0 } * PI * g.r * g.r)
        .sum()
}

/// The tube: `rings` about `c` extruded over `z ∈ z`.
fn tube(c: (f64, f64), rings: &[Ring], z: (f64, f64)) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops(c, rings))
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: z.1 - z.0,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z.0));
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

fn plate() -> Body<f64> {
    brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol())
}

/// Every op in both orders between `tube` (profile area `area`, over
/// `z`) and the plate, each at its closed form and meshed, and `SOUND`
/// but for a run `t3` names, whose result fails tier 3′ and nothing
/// else: it is two solids, one in the other's bore, which the census's
/// cross-solid backstop cannot separate
/// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
fn every_op(what: &str, tube: Body<f64>, area: f64, z: (f64, f64), t3: &[&str]) {
    let (va, vb, vab) = (
        area * (z.1 - z.0),
        36.0,
        area * (z.1.min(1.0) - z.0.max(0.0)),
    );
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let (t, p) = (fin("the tube", tube), fin("the plate", plate()));
    let mut bad = Vec::new();
    for (op, r, want) in every_op_both_orders(&t, &p, (va, vb, vab), tol()) {
        let row = format!("{what}: {op}");
        let res = r.as_ref().unwrap_or_else(|e| panic!("{row}: {e:?}"));
        let body = &res.body().unwrap_or_else(|| panic!("{row}: empty")).body;
        let mesh = mesh::tessellate(body, 5e-3, tol()).unwrap_or_else(|e| panic!("{row}: {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"));
        // The closed form within the quadrature's own certified pad,
        // which grows with ε; `outcome`'s fixed 1e-7 holds at the
        // default ε only, so it reads the measured volume.
        let m = topo::mass_properties(body, tol()).unwrap();
        assert!(
            (m.volume - want).abs() <= m.volume_pad + 1e-9 * want.max(1.0),
            "{row}: volume {} ± {}, closed form {want}",
            m.volume,
            m.volume_pad
        );
        let line = outcome(r, m.volume, tol());
        let want_line = if t3.contains(&op) {
            "OK BAD t2=true t3p=false cert=true operand=true"
        } else {
            "OK SOUND t2=true t3p=true cert=true operand=true"
        };
        if !line.starts_with(want_line) {
            bad.push(format!("{row}: {line}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// `rings` about `c` as a tube over `z`, run by [`every_op`].
fn every_op_on(what: &str, c: (f64, f64), rings: &[Ring], z: (f64, f64), t3: &[&str]) {
    every_op(what, tube(c, rings, z), area(rings), z, t3);
}

/// The witness tube: `r ∈ [0.5, 1]`, its walls' vertices at `ao` and
/// `ai`, over `z ∈ [0.5, 2.5]`, its foot sunk in the plate.
fn witness(ao: f64, ai: f64) {
    every_op_on(
        &format!("the annulus at ({ao}, {ai})"),
        (0.0, 0.0),
        &[ring(1.0, ao), ring(0.5, ai)],
        (0.5, 2.5),
        &[],
    );
}

/// **The four witness azimuth pairs**: aligned, near, opposed, and a
/// pair whose one vertex chord crosses the disc. Tube `1.5π`, overlap
/// `0.375π`.
#[test]
fn the_annulus_builds_at_every_witness_azimuth_pair() {
    for (ao, ai) in [(0.0, 0.0), (0.0, 1.0), (0.0, PI), (1.0, 4.0)] {
        witness(ao, ai);
    }
}

/// **Any azimuth pair** builds: a counterexample search over both
/// vertices' azimuths.
#[test]
fn the_annulus_builds_at_random_azimuth_pairs() {
    let mut rng = fuzz::start("annular_tube_azimuths");
    for _ in 0..fuzz::scaled(8) {
        let (ao, ai) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
        eprintln!("({ao}, {ai}) {}", fuzz::replay());
        witness(ao, ai);
    }
}

/// **Two nested annuli**, `r ∈ [0.75, 1]` and `[0.25, 0.5]`, one
/// tube's union with the other (a profile holds no island in a hole),
/// every pair of neighbouring vertices opposed: four one-site loops in
/// the plate's top face, whose regions are the inner disc and three
/// annuli. Where the plate is not in the result, the two rings are two
/// solids ([`every_op`]).
#[test]
fn two_nested_annuli_build() {
    let (outer, inner) = (
        [ring(1.0, 0.0), ring(0.75, PI)],
        [ring(0.5, 0.0), ring(0.25, PI)],
    );
    let z = (0.5, 2.5);
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let both = topo::union(
        &fin("the outer tube", tube((0.0, 0.0), &outer, z)),
        &fin("the inner tube", tube((0.0, 0.0), &inner, z)),
        tol(),
    )
    .unwrap();
    let both = both
        .body()
        .expect("two tubes unite")
        .body
        .clone()
        .into_body();
    let apart = ["A ∩ B", "B ∩ A", "A ∖ B"];
    every_op("two annuli", both, area(&outer) + area(&inner), z, &apart);
}

/// **The annulus off the plate's centre**, its vertices opposed.
#[test]
fn an_off_centre_annulus_builds() {
    every_op_on(
        "off centre",
        (1.2, -0.7),
        &[ring(1.0, 2.0), ring(0.5, 2.0 + PI)],
        (0.5, 2.5),
        &[],
    );
}

/// **The annulus through the plate**, `z ∈ [−0.5, 1.5]`: both of the
/// plate's faces cut both walls, two null faces each with an opposed
/// pair. The plate less the tube is two solids ([`every_op`]).
#[test]
fn an_annulus_through_the_plate_builds() {
    every_op_on(
        "through",
        (0.0, 0.0),
        &[ring(1.0, 0.0), ring(0.5, PI)],
        (-0.5, 1.5),
        &["B ∖ A"],
    );
}

/// **Two-arc circles agree with one-segment ones**: each wall drawn in
/// two arcs, then one of each, at the opposed pair.
#[test]
fn two_arc_circles_agree_with_one_segment_ones() {
    let two = |r, az| Ring { r, az, arcs: 2 };
    for (what, rings) in [
        ("both two-arc", [two(1.0, 0.0), two(0.5, PI)]),
        ("outer two-arc", [two(1.0, 0.0), ring(0.5, PI)]),
        ("inner two-arc", [ring(1.0, 0.0), two(0.5, PI)]),
    ] {
        every_op_on(what, (0.0, 0.0), &rings, (0.5, 2.5), &[]);
    }
}

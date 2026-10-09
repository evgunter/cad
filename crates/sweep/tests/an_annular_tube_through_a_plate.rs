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

/// What a row's result is held to besides its closed form.
#[derive(Clone, Copy, Default)]
struct Known<'a> {
    /// The runs whose result fails tier 3′ and nothing else: it is two
    /// solids, one in the other's bore, which the census's cross-solid
    /// backstop cannot separate
    /// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
    t3: &'a [&'a str],
    /// The runs whose result the mesher refuses `Triangulation`: a
    /// thin annulus whose two circles' vertices are opposed
    /// (`work/tess/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`).
    untriangulated: &'a [&'a str],
}

/// Every op in both orders between `tube` (profile area `area`, over
/// `z`) and the plate, each `SOUND` at its closed form and meshed, but
/// for the runs `known` names.
fn every_op(what: &str, tube: Body<f64>, area: f64, z: (f64, f64), known: Known<'_>) {
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
        match mesh::tessellate(body, 5e-3, tol()) {
            Ok(m) if !known.untriangulated.contains(&op) => {
                mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"));
            }
            Err(mesh::TessellateError::Triangulation { .. })
                if known.untriangulated.contains(&op) => {}
            m => bad.push(format!("{row}: the mesh {:?}", m.map(|_| "meshed"))),
        }
        let line = outcome(r, want, tol());
        let want_line = if known.t3.contains(&op) {
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

/// The tubes over `z`, each two rings about the origin, nested and
/// united (a profile holds no island in a hole), run by [`every_op`].
fn nested(what: &str, annuli: &[[Ring; 2]], z: (f64, f64), known: Known<'_>) {
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let mut all = tube((0.0, 0.0), &annuli[0], z);
    for a in &annuli[1..] {
        let r = topo::union(
            &fin("the tubes", all),
            &fin("a tube", tube((0.0, 0.0), a, z)),
            tol(),
        )
        .unwrap();
        all = r.body().expect("the tubes unite").body.clone().into_body();
    }
    let area = annuli.iter().map(|a| area(a)).sum();
    every_op(what, all, area, z, known);
}

/// `rings` about `c` as a tube over `z`, run by [`every_op`].
fn every_op_on(what: &str, c: (f64, f64), rings: &[Ring], z: (f64, f64), known: Known<'_>) {
    every_op(what, tube(c, rings, z), area(rings), z, known);
}

/// The witness tube: `r ∈ [0.5, 1]`, its walls' vertices at `ao` and
/// `ai`, over `z ∈ [0.5, 2.5]`, its foot sunk in the plate.
fn witness(ao: f64, ai: f64) {
    every_op_on(
        &format!("the annulus at ({ao}, {ai})"),
        (0.0, 0.0),
        &[ring(1.0, ao), ring(0.5, ai)],
        (0.5, 2.5),
        Known::default(),
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

/// **Two nested annuli**, `r ∈ [0.75, 1]` and `[0.25, 0.5]`, every
/// pair of neighbouring vertices opposed: four one-site loops in the
/// plate's top face, whose regions are the inner disc and three
/// annuli. Where the plate is not in the result, the two rings are two
/// solids.
#[test]
fn two_nested_annuli_build() {
    let known = Known {
        t3: &["A ∩ B", "B ∩ A", "A ∖ B"],
        ..Known::default()
    };
    nested(
        "two annuli",
        &[
            [ring(1.0, 0.0), ring(0.75, PI)],
            [ring(0.5, 0.0), ring(0.25, PI)],
        ],
        (0.5, 2.5),
        known,
    );
}

/// **Thin nested rings, thin gaps**: three annuli of width `w`, `w`
/// apart, every pair of neighbouring vertices opposed, so every
/// section loop's region faces are thin annuli whose vertex chords all
/// leave them. Each is witnessed by an inward line from an edge's
/// midpoint, whatever `w`.
#[test]
fn thin_nested_rings_build() {
    for w in [1e-3, 9e-4, 1e-4] {
        let r = |i: f64| 1.0 - i * w;
        let annuli: Vec<[Ring; 2]> = (0..3)
            .map(|k| {
                [
                    ring(r(2.0 * k as f64), 0.0),
                    ring(r(2.0 * k as f64 + 1.0), PI),
                ]
            })
            .collect();
        let known = Known {
            t3: &["A ∩ B", "B ∩ A", "A ∖ B"],
            untriangulated: &ALL_OPS,
        };
        nested(&format!("thin rings w = {w}"), &annuli, (0.5, 2.5), known);
    }
}

/// The six runs' names, as `every_op_both_orders` gives them.
const ALL_OPS: [&str; 6] = ["A ∪ B", "B ∪ A", "A ∩ B", "B ∩ A", "A ∖ B", "B ∖ A"];

/// **The annulus off the plate's centre**, its vertices opposed.
#[test]
fn an_off_centre_annulus_builds() {
    every_op_on(
        "off centre",
        (1.2, -0.7),
        &[ring(1.0, 2.0), ring(0.5, 2.0 + PI)],
        (0.5, 2.5),
        Known::default(),
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
        Known {
            t3: &["B ∖ A"],
            ..Known::default()
        },
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
        every_op_on(what, (0.0, 0.0), &rings, (0.5, 2.5), Known::default());
    }
}

/// **The uncut-shell witness reads a one-vertex disc**: a one-segment
/// plug of radius 0.5 flush in a bore through the plate, both vertices
/// at azimuth 0, the flush pairs declared. The plug is an uncut shell
/// every vertex and edge of which lies on the bore's wall, so only a
/// point inside a cap decides its side, and a cap is a disc of one
/// vertex. `∩` is empty and `∖` is either operand, in both orders, at
/// three heights. (`∪` is not this witness's: it stops in the seam
/// zip.)
#[test]
fn a_plug_flush_in_a_bore_is_sided_by_its_caps() {
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let bore = fin("the bore", tube((0.0, 0.0), &[ring(0.5, 0.0)], (-0.5, 1.5)));
    let bored = topo::subtract(&fin("the plate", plate()), &bore, tol()).unwrap();
    let bored = fin(
        "the bored plate",
        bored
            .body()
            .expect("a bored plate")
            .body
            .clone()
            .into_body(),
    );
    let vb = 36.0 - PI * 0.25;
    for z in [(0.0, 1.0), (0.25, 0.75), (0.0, 0.5)] {
        let plug = fin("the plug", tube((0.0, 0.0), &[ring(0.5, 0.0)], z));
        let vp = PI * 0.25 * (z.1 - z.0);
        for (order, x, y, vx) in [
            ("plug, plate", &plug, &bored, vp),
            ("plate, plug", &bored, &plug, vb),
        ] {
            let d = topo::test_support::flush_declarations(x, y, tol());
            let row = format!("the plug over z ∈ {z:?}, {order}");
            let meet = outcome(topo::intersect_with(x, y, &d, tol()), 0.0, tol());
            assert_eq!(meet, "EMPTY ok", "{row}: ∩");
            let less = outcome(topo::subtract_with(x, y, &d, tol()), vx, tol());
            assert!(less.starts_with("OK SOUND"), "{row}: ∖ {less}");
        }
    }
}

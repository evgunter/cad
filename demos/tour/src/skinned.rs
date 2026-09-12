//! The loft and the sweep — the tour's DEFINITIONAL stop (M5 PR 10;
//! frontier closed at M6-3), narration **and, since the montage
//! refresh, the skin scenes**.
//!
//! The narration stays beside the scenes ([`stops`]): it is the
//! *geometry* layer (`loft_geometry` / `sweep_geometry` — control
//! nets, weights, the MEASURED interpolation claim, the not-ruled
//! claim), which no render can show.
//!
//! # The scenes and the corpus (montage-v3 curation)
//!
//! - `lofts` — ONE cell carrying BOTH lofts, side by side, since the
//!   montage-v3 curation (Ev, 2026-08-30). Two adjacent cells were
//!   not showing a minimal pair: `compose_montage.py` trims and scales
//!   every cell independently, so the panels arrived at two different
//!   scales and the silhouette comparison was distorted by the
//!   composer. One frame gives them one camera AND one scale. The two
//!   bodies keep their own names, exports and narration lines.
//! - `loft_prism` — the same BODY as the corpus fixture
//!   `step-export/tests/common/mod.rs::loft_prism()` (recipe-layer
//!   twin: `editor-core/tests/corpus/loft_prism.rs`; acceptance + the
//!   derived V = 9 m³ bracket: `sweep/tests/m6_loft_body.rs`). Same
//!   sections, same placements, same degree — re-authored here rather
//!   than shared, and pinned by the volume its derivation produces
//!   (see [`stops`]), which is what the cross-link is actually for.
//! - `nonuniform_loft` — since montage-v2 the scene LEADS the corpus
//!   (the lily/s_duct precedent): the corpus fixture
//!   (`common/mod.rs::nonuniform_loft()`, #210/#207) keeps its
//!   z = 0/1/3 spacing, but at that spacing the pair's silhouettes
//!   are nearly indistinguishable — bulge peak at 48.8% vs 50% of
//!   height, peak half-width 1.415 vs 1.375, MEASURED. The SCENE
//!   re-places the same
//!   sections at z = 0/0.15/2 — same sections, same total height,
//!   ONLY the middle placement moves — driving the bulge to
//!   half-width 1.646 at 32.6% of height: silhouette-obvious. Rendered
//!   `LOFT_PAIR_GAP` along +x of its twin by `transform_rigid` on the
//!   BUILT body — the walls are described NURBS nets, which the rigid
//!   map carries by their control points.
//! - `s_duct` — standalone since montage-v2 (Ev, #218 follow-up:
//!   the S SOLID is two glued partial revolves, shape for shape, so
//!   as a cell it demonstrated the one-op path, not an unreachable
//!   shape). Still the fixture candidate for the next corpus fold;
//!   the corpus's sweep constant remains the quarter-arc
//!   `common/mod.rs::swept_elbow()`.
//! - `twisted_duct` — the sweep CELL since montage-v2: the twisted
//!   cubic (At, Bt², Ct³), the class NO assembly of revolves
//!   reaches — its torsion is nowhere zero (τ = 12ABC/|r′×r″|²,
//!   constant numerator), while a revolve's spine is a planar
//!   circular arc and gluing revolves only concatenates planar arcs.
//!   Two shadow-proof standalone renders ride beside it
//!   (`twisted_duct_shadow_{z,y}`, the silhouette3 pattern): the
//!   z-shadow is a parabola (no inflection), the y-shadow a cubic S
//!   (one inflection) — parallel projections of a PLANAR curve are
//!   all affine images of one another, and affine maps preserve
//!   inflection count, so no planar spine casts both. Fixture
//!   candidate alongside the S.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::polygon;
use pncad::geom_core::{Affine3, Point2, Point3, Vec3};
use pncad::prelude::{Open, Start, Via};
use pncad::sweep::skin::{Section, loft_geometry, sweep_geometry};
use pncad::sweep::{SketchSegment, segment_curve};

use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// A square-with-an-arc section, scaled by `s` (LIB-U3 profile
/// vocabulary: one loop, the arc as vertex 1's bulge).
fn chain(s: f64, tol: Tol) -> Section {
    // Lattice-authored since LIB-RETTAIL (raw `ProfileLoop` construction
    // is no longer presented surface, Ev's ruling on #413). The one
    // curved leg was a bulge of 0.25 on the vertex at (2, 0); the same
    // arc, said through the lattice, is `arc_to(Via { .. })` through the
    // apex the bulge implies. INVARIANT (why the point is exactly this):
    // for chord A->B of length L, apex = midpoint - n_hat * (L*b/2) with
    // n_hat the left normal, so b = 0.25 on the chord (2,0)->(2,1) puts
    // the apex at x = 2 + 0.125, y = 0.5 — and bulge_from_via returns
    // 0.25 back from it.
    let p = |x: f64, y: f64| Point2::new(x * s, y * s);
    let loop_ = Open
        .at(p(0.0, 0.0))
        .line_to(p(2.0, 0.0), tol)
        .and_then(|t| {
            t.arc_to(
                Via {
                    q: p(2.125, 0.5),
                    p: p(2.0, 1.0),
                },
                tol,
            )
        })
        .and_then(|t| t.line_to(p(0.0, 1.0), tol))
        .and_then(|t| t.line_to(Start, tol))
        .expect("the arc-and-lines section authors");
    vec![loop_.into()]
}

/// The tour's loft + sweep stop.
pub fn narration(tol: Tol) {
    println!("\n-- the loft and the sweep (M5 PR 10: definitional NURBS walls) --");

    // ---- The loft: three sections, the middle one scaled. ----
    let places = [0.0, 1.0, 2.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    let loft = loft_geometry(
        &[chain(1.0, tol), chain(1.6, tol), chain(1.0, tol)],
        &places,
        2,
        tol,
    )
    .expect("the three-section loft skins");
    println!(
        "== loft: 3 sections (1.0 / 1.6 / 1.0 scale) x 1 loop x {} segments ==",
        loft.walls[0].len()
    );
    for (j, wall) in loft.walls[0].iter().enumerate() {
        let (nu, nv) = wall.control_counts();
        println!(
            "   wall {j}: degree {}x{} NURBS, {nu}x{nv} control points, weights {}",
            wall.knots_u().degree(),
            wall.knots_v().degree(),
            if wall.weights().iter().all(|w| *w == 1.0) {
                "all 1 (integral)"
            } else {
                "rational (the arc is exact)"
            }
        );
    }

    // The interpolation claim, MEASURED (not asserted): the surface
    // reproduces each section at its own v-parameter.
    let mut worst = 0.0f64;
    for (j, wall) in loft.walls[0].iter().enumerate() {
        for (k, v) in loft.section_params.iter().enumerate() {
            let section = &loft.sections[0][j][k];
            let n = 8 * section.control().len();
            for i in 0..=n {
                #[allow(clippy::cast_precision_loss)]
                let u = i as f64 / n as f64;
                worst = worst.max(wall.eval(u, *v).distance(section.eval(u)));
            }
        }
    }
    println!(
        "   interpolation verified by EVALUATION at every section parameter: \
         worst deviation {worst:.3e} m (eps = {:.0e})",
        tol.eps()
    );

    // The middle is not the average of the ends: no wall is ruled.
    let wall = &loft.walls[0][0];
    let mid = wall.eval(0.5, loft.section_params[1]);
    let (a, b) = (wall.eval(0.5, 0.0), wall.eval(0.5, 1.0));
    let ruled = Point3::new(
        0.5f64.mul_add(b.x - a.x, a.x),
        0.5f64.mul_add(b.y - a.y, a.y),
        0.5f64.mul_add(b.z - a.z, a.z),
    );
    println!(
        "   the loft is NOT ruled: wall 0 bulges {:.4} m off the section-0/2 chord \
         at mid-height",
        mid.distance(ruled)
    );

    // ---- The sweep: the same profile carried along an arc path. ----
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(3.0, 3.0),
            bulge: 0.4,
        },
        Affine3::identity(),
    )
    .expect("the path converts");
    // The profile plane is normal to the path at its start — the
    // frame the sweep then carries along (`sweep_geometry` turns it by
    // the minimal rotation at each station).
    let place = normal_start_place(&path);
    let swept =
        sweep_geometry(&chain(1.0, tol), place, &path, 5, 3, tol).expect("the arc sweep skins");
    println!(
        "== sweep: 1 profile carried along an arc path at {} stations, v-degree 3 ==",
        swept.section_params.len()
    );
    println!(
        "   {} walls; the frame is path-FOLLOWING (each station turns the profile by \
         the minimal rotation carrying the path's start tangent to its own); a \
         reversing tangent refuses typed rather than picking an axis",
        swept.walls[0].len()
    );

    println!(
        "   the loft/sweep BODIES are scenes now (frontier fully retired): \
         lofts (loft_prism + nonuniform_loft in one cell), s_duct — \
         see the stops below"
    );
}

// ---- The scene constructions -------------------------------------
//
// THE CORPUS SHAPES ARE COPIED HERE, DELIBERATELY, AND NOTHING LINKS
// THE COPIES. Their other home is `step-export/tests/common/mod.rs`
// (and, for the elbow, `sweep::test_support`, which that fixture and
// the tessellation suites all delegate to), which is another crate's
// TEST-SUPPORT module: gated behind a dev-only feature, not
// published, and not something a user of this library could import. A
// demo exists to show the library the way a user would meet it, so
// reaching into a test module would make this file worse evidence,
// not better — and there is no public door that hands out corpus
// fixtures.
//
// What that costs is exactly one thing: these numbers can drift apart
// from the corpus's silently. So the cross-link is pinned where it is
// load-bearing rather than asserted in prose — `stops` checks the
// prism body against the volume `sweep/tests/m6_loft_body.rs` DERIVES
// for the fixture, and `loft_parameters` is ASKED rather than
// re-derived.

/// A closed four-line quad section (one loop) — the plainest
/// INTEGRAL profile: unit weights, no arc anywhere.
///
/// The same section as `common/mod.rs::quad`, by value; not the same
/// code. That one is a test fixture and spells its vertex table
/// directly, behind the door that exists for fixtures; this one goes
/// through the façade's polygon door, which classifies every corner at
/// authoring — the spelling this tour is here to show, and the only one
/// a consumer has.
fn quad(pts: [(f64, f64); 4], tol: Tol) -> Section {
    vec![polygon(&pts, tol).expect("the quad section")]
}

/// **The placement a path sweep starts from**: the plane through the
/// path's start point whose normal is the start TANGENT, with the
/// in-plane axes built off whichever world axis is least parallel to
/// it. `sweep_geometry`/`sweep_body` carry this frame along the path
/// by minimal rotation, so a section placed here stays normal to the
/// path — the first thing a real caller has to write, and the reason
/// both sweep cells below open with it.
///
/// The kernel's own suites share this recipe from
/// `sweep::test_support`'s neighbour in `sweep/tests/common`. The tour
/// cannot reach either: both are test-only homes behind a dev-only
/// feature, and this is a `src/` binary that links the façade as an
/// ordinary dependency. So it is a stated copy — and the fact that a
/// caller must write it at all is what the narration below is about.
fn normal_start_place(path: &pncad::geom::NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::unit_z()
    } else {
        Vec3::unit_x()
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    Affine3::from_frame(path.eval(lo), u, n.cross(u))
}

/// The prism's end sections (also `common/mod.rs::PRISM_SQUARE`).
const PRISM_SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
/// Its middle section: the NON-AFFINE trapezoid whose two bottom
/// corners flare by ±d, d = 0.375 (also `common/mod.rs::PRISM_TRAPEZOID`).
const PRISM_TRAPEZOID: [(f64, f64); 4] = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

/// How far along +x the non-uniform loft renders from its twin. The
/// prism reaches half-width 1.375 (its trapezoid) and the non-uniform
/// skin overshoots to 1.646, so 4 m leaves 4 − 1.375 − 1.646 ≈ 0.98 m
/// of clear air between the two silhouettes at the shared camera —
/// separated without either shrinking to make room.
const LOFT_PAIR_GAP: f64 = 4.0;

/// The S-duct's arc radius (scene-local; the corpus elbow's is
/// `sweep::test_support`'s `ELBOW_R` = 3 — see the S-path note below).
const S_R: f64 = 2.0;
/// The profile half-width. The same value as `sweep::test_support`'s
/// `ELBOW_H`, copied — that home is behind a dev-only feature this
/// binary cannot turn on, so the two are not linked and nothing would
/// notice if one moved.
const ELBOW_H: f64 = 0.25;

/// The twisted tube's stations. Six is enough for a degree-3 skin to
/// follow the spine and few enough that every one of them is a
/// number the note can print.
const TUBE_STATIONS: usize = 6;
/// The tube's outer half-width at the base: twice the neighbouring
/// sweep's [`ELBOW_H`], so that at montage scale the HOLE and the
/// taper are both legible on a 4.4 m spine. What the two cells share
/// is the spine, not the section.
const TUBE_OUT: f64 = 2.0 * ELBOW_H;
/// The inner half-width at the base. `TUBE_IN / TUBE_OUT` is 3/5, so
/// the hollow ratio `1 − λ²` is 16/25 — exact in binary, which is what
/// lets the volume oracle be an equality rather than a bracket.
const TUBE_IN: f64 = 0.3;
/// What the section tapers TO, as a fraction of the base.
const TUBE_TAPER_END: f64 = 0.5;
/// The authored roll, radians over the whole spine. Deliberately not a
/// multiple of a quarter turn: a square's axis is only defined mod
/// π/2, so a quarter-turn roll would be indistinguishable from none.
const TUBE_ROLL: f64 = 1.0;

/// The section's scale at station `i` — linear from 1 to
/// [`TUBE_TAPER_END`].
fn tube_taper(i: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let u = i as f64 / (TUBE_STATIONS - 1) as f64;
    (TUBE_TAPER_END - 1.0).mul_add(u, 1.0)
}

/// A centred square LOOP of half-width `h` — one loop, so an annular
/// section is two of them.
fn square(h: f64, tol: Tol) -> pncad::profile::ProfileLoop<f64> {
    polygon(&[(-h, -h), (h, -h), (h, h), (-h, h)], tol).expect("the square loop")
}

/// Station `i`'s placement on `path`: the plane normal to the spine's
/// tangent there, rolled `roll · u` about it.
///
/// The in-plane axes are built off whichever world axis is least
/// parallel to the tangent — [`normal_start_place`]'s recipe, applied
/// at every station rather than only the first, because a loft places
/// each section itself.
fn tube_place(path: &pncad::geom::NurbsCurve3<f64>, i: usize, roll: f64) -> Affine3<f64> {
    let (lo, hi) = path.domain();
    #[allow(clippy::cast_precision_loss)]
    let u = i as f64 / (TUBE_STATIONS - 1) as f64;
    let t = (hi - lo).mul_add(u, lo);
    let d = path.deriv(t);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let a = n.cross(helper);
    let a = a / a.norm();
    let b = n.cross(a);
    let (c, s) = (roll * u).sin_cos();
    let (sin, cos) = (c, s);
    Affine3::from_frame(path.eval(t), a * cos + b * sin, b * cos - a * sin)
}

/// The widths of a body's two END CAPS — the largest distance between
/// two vertices of one cap face, which for a square annulus is the
/// outer square's diagonal.
///
/// The caps are the two faces carrying a RING: a loft's walls have one
/// loop each and only an annular section's caps have two.
fn cap_widths(body: &pncad::topo::Body<f64>) -> (f64, f64) {
    let mut widths: Vec<f64> = body
        .faces()
        .filter(|(_, f)| f.rings.len() == 1)
        .map(|(k, _)| face_span(body, k))
        .collect();
    assert_eq!(
        widths.len(),
        2,
        "an annular loft has exactly two ringed caps"
    );
    widths.sort_by(|x, y| y.partial_cmp(x).expect("spans are finite"));
    (widths[0], widths[1])
}

/// The largest distance between two vertices of one face.
fn face_span(body: &pncad::topo::Body<f64>, face: pncad::topo::FaceKey) -> f64 {
    let ps = face_points(body, face);
    let mut best = 0.0f64;
    for (i, p) in ps.iter().enumerate() {
        for q in &ps[i + 1..] {
            best = best.max((*p - *q).norm());
        }
    }
    best
}

/// Every vertex position on a face, through its loops' cycles.
fn face_points(body: &pncad::topo::Body<f64>, face: pncad::topo::FaceKey) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    let f = body.get_face(face).expect("the face resolves");
    for &lp in std::iter::once(&f.outer).chain(f.rings.iter()) {
        let l = body.get_loop(lp).expect("the loop resolves");
        let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else {
            continue;
        };
        for he in body.loop_cycle(first).expect("the cycle walks") {
            let hed = body.get_half_edge(he).expect("the half-edge resolves");
            let v = body.get_vertex(hed.start).expect("the vertex resolves");
            out.push(*body.get_point(v.point).expect("the point resolves"));
        }
    }
    out
}

/// The body's `(v, e, f)` census and the genus the Euler-Poincaré
/// identity gives from it — the two neighbouring scenes' spelling,
/// scene-local as theirs are.
fn genus(body: &pncad::topo::Body<f64>) -> i64 {
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, x)| x.rings.len() as i64).sum();
    let s = body.shells().count() as i64;
    s - (v - e + f - r) / 2
}

/// The NARROW cap's own in-plane axis: the direction of its longest
/// vertex pair, sign-normalised so two frames a roll apart compare.
fn tip_axis(body: &pncad::topo::Body<f64>) -> Vec3<f64> {
    let (_, tip) = cap_widths(body);
    let face = body
        .faces()
        .filter(|(_, f)| f.rings.len() == 1)
        .find(|(k, _)| (face_span(body, *k) - tip).abs() < 1e-12)
        .expect("the narrow cap is one of the two")
        .0;
    let ps = face_points(body, face);
    let mut best = (0.0f64, Vec3::new(1.0, 0.0, 0.0));
    for (i, p) in ps.iter().enumerate() {
        for q in &ps[i + 1..] {
            let d = *p - *q;
            if d.norm() > best.0 {
                best = (d.norm(), d / d.norm());
            }
        }
    }
    let v = best.1;
    if v.x < 0.0 { v * -1.0 } else { v }
}

/// Section placements: pure translations up the world z-axis (also
/// `common/mod.rs::lofted_at_z`).
fn lofted_at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// The square/trapezoid/square section stack both loft scenes share —
/// the minimal pair's shared half.
/// The middle section's v-parameter at the montage spacing
/// (z = 0/0.15/2), `3√29/(3√29 + √5701)` — the pin the stop's note
/// narrates, checked against `loft_parameters` at build time.
// The shortest form that round-trips to the same f64 as the note's
// 0.17625368909901809 (that last digit is past f64's precision, which
// is why the narration keeps it and the constant does not).
const NONUNIFORM_T: f64 = 0.1762536890990181;

fn prism_sections(tol: Tol) -> Vec<Section> {
    vec![
        quad(PRISM_SQUARE, tol),
        quad(PRISM_TRAPEZOID, tol),
        quad(PRISM_SQUARE, tol),
    ]
}

/// The three skin scenes, in tour order: the two lofts as the
/// corpus's MINIMAL PAIR (same sections, same degree, same builder —
/// only the section spacing differs, so they share a camera and read
/// as a pair on the sheet), then the curved-path sweep.
pub fn stops(tol: Tol) -> Vec<Stop> {
    // Both lofts are 2 m tall columns flaring in x at one height, so
    // the story-bearing silhouette is the xz PROFILE: a near-face-on
    // ±y camera puts the ±x walls edge-on and the flare becomes a
    // bulge in the outline itself — the prism's symmetric peak
    // (half-width 1.375) at mid-height, the non-uniform's fatter peak
    // (half-width 1.646, wider than ANY authored section) at 32.6%
    // with its long upper taper — rather than a shading difference
    // (#218 review: the pair must be distinct in profile, not
    // shading). 10° of azimuth and elevation keep a sliver of side
    // wall and top for depth; cos 10° ≈ 0.985, so the profile stays
    // essentially unforeshortened. Shared by the pair on purpose (the
    // minimal-pair principle: same camera makes the difference
    // attributable to the geometry).
    let loft_view = || View {
        elev: 10.0,
        azim: -80.0,
        up: 'z',
    };

    let prism = pncad::sweep::loft_body::<f64>(
        &prism_sections(tol),
        &lofted_at_z(&[0.0, 1.0, 2.0]),
        2,
        tol,
    )
    .expect("shape (iii) loft builds")
    .body;
    // THE STOP'S OWN NARRATION, PINNED AGAINST THE KERNEL — the same
    // move `loft_parameters` gets twenty lines below, and the same
    // scope. **What this does NOT check, stated:** agreement with
    // `step-export/tests/common/mod.rs::loft_prism()`. Nothing here
    // reads that file, this demo cannot (it is another crate's
    // test-support module — see the copy note above), and two
    // different prisms can share a volume anyway. A section drifting
    // in the CORPUS leaves this green, and that gap is the price of
    // the copy, recorded rather than papered over.
    //
    // What it does check is the note's arithmetic against the kernel's
    // answer, with the note's number DERIVED from the sections rather
    // than typed: each slice is a trapezoid of area 4 + 2·d·λ(v) with
    // λ = 4v(1−v) and z = 2v exactly, so V = 8 + 8d/3, d being the
    // trapezoid's flare. Typing `9.0` would have let a section here
    // drift while the pin stayed green on a number that no longer
    // followed from it.
    let flare = PRISM_TRAPEZOID[1].0 - PRISM_SQUARE[1].0;
    let narrated = 8.0 + 8.0 * flare / 3.0;
    assert_eq!(
        narrated, 9.0,
        "the stop's note narrates V = 9 m³ exactly; these sections (flare d = {flare}) \
         now give {narrated}, so the note is wrong before the kernel is asked"
    );
    let prism_props = pncad::topo::mass_properties(&prism, tol).expect("the prism has a volume");
    // The enclosure is asked to BRACKET the derivation — and is
    // bounded from above first, because `volume_pad` is the props
    // door's own certified half-width with nothing constraining it:
    // `|V − v| ≤ pad` alone gets EASIER as the enclosure degrades, and
    // a bracket wide enough to swallow any answer proves nothing. The
    // door reports ~1e-13 on this body at this δ; 1e-9 leaves four
    // orders of headroom and still fails long before the quadrature
    // has stopped saying anything about a 9 m³ solid.
    const PRISM_PAD_MAX: f64 = 1e-9;
    assert!(
        prism_props.volume_pad <= PRISM_PAD_MAX,
        "loft_prism's certified volume enclosure widened to ± {} (> {PRISM_PAD_MAX:e}): \
         the bracket check below stops meaning anything at that width",
        prism_props.volume_pad
    );
    assert!(
        (prism_props.volume - narrated).abs() <= prism_props.volume_pad,
        "the skin changed: loft_prism's V = {} ± {} no longer brackets the {narrated} m³ \
         its own sections derive (the derivation is this stop's note, and \
         sweep/tests/m6_loft_body.rs derives the same number for the corpus fixture)",
        prism_props.volume,
        prism_props.volume_pad
    );
    // Montage-v2 spacing: z = 0/0.15/2, not the corpus fixture's
    // 0/1/3. Measured on the #218 sheet, 0/1/3 was invisible as a
    // pair member: its bulge peaks at 48.8% of height with half-width
    // 1.415 vs the prism's 50%/1.375 — the same silhouette, scaled.
    // Same sections at 0/0.15/2 keep the pair TRULY minimal (same
    // sections, same height, only the middle placement moves) and
    // the chord-length parameterization makes the skin overshoot
    // dramatically (numbers in the stop's note). The corpus fixture
    // keeps 0/1/3 — this scene now LEADS the corpus, the s_duct/lily
    // precedent.
    let nonuniform_places = lofted_at_z(&[0.0, 0.15, 2.0]);
    // The middle section's v-parameter, ASKED (LIB-U5 deliverable 1)
    // rather than re-derived: the note below narrates
    // t = 3√29/(3√29 + √5701) and every number downstream of it, so
    // the derivation is pinned against the kernel's own answer here.
    let params = pncad::sweep::loft_parameters(&prism_sections(tol), &nonuniform_places, 2, tol)
        .expect("the non-uniform sections skin");
    assert_eq!(
        params,
        vec![0.0, NONUNIFORM_T, 1.0],
        "the narrated v-parameterization is no longer what the skin chose"
    );
    let nonuniform_at_origin =
        pncad::sweep::loft_body::<f64>(&prism_sections(tol), &nonuniform_places, 2, tol)
            .expect("the non-uniform loft builds")
            .body;
    // The pair is placed the way a user places anything: BUILD the
    // body where its sections are authored, then MOVE it. The walls
    // are described NURBS nets and `transform_rigid` maps them by
    // their control points, so the rendered solid is the exact image
    // of the one every derivation below is stated about — the
    // placement is not part of the shape.
    let nonuniform = pncad::topo::transform_rigid(
        &nonuniform_at_origin,
        &Affine3::translation(Vec3::new(LOFT_PAIR_GAP, 0.0, 0.0)),
        tol,
    )
    .expect("the non-uniform loft is placed beside its twin");
    // ONE CELL, BOTH BODIES (montage-v3 curation, Ev 2026-08-30).
    // Two adjacent cells were not showing the pair the pair claims to
    // be: `compose_montage.py` trims and scales EVERY cell
    // independently, so the two panels arrived at two different scales
    // and the silhouette comparison — the whole content of a minimal
    // pair — was distorted by the composer. Side by side in ONE frame
    // they share a camera AND a scale, which is what "only the middle
    // placement moved" needs a reader to be able to see.

    // The S path (#218 review; DEMOTED to standalone at montage-v2):
    // two opposed quarter arcs of radius R in the world x = 0 plane,
    // sampled at 17 exact points and interpolated at degree 3 (the
    // sweep machinery consumes any NurbsCurve3, #210). Tangent runs
    // +z → +y → +z; never reversed, so the path-following frame is
    // total. As a SHAPE the S solid is two glued partial revolves
    // (each planar circular-arc sweep of the square IS a partial
    // revolve's orbit, and the halves glue at the inflection), so the
    // cell shows a one-op construction, not an unreachable shape
    // class. The
    // unreachable class needs a NON-PLANAR spine — `twisted_duct`
    // below, the sheet's sweep cell since montage-v2. The QUARTER-ARC
    // elbow stays the corpus/suite constant, built once in
    // `sweep::test_support` and delegated to by the STEP fixture and
    // the tessellation suites;
    // the S sweep remains a fixture CANDIDATE for the next corpus
    // fold.
    let s_points: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = core::f64::consts::FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_R * (1.0 - th.cos()), S_R * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = core::f64::consts::FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_R + S_R * ph.sin(), 2.0 * S_R - S_R * ph.cos())
        }))
        .collect();
    let path =
        pncad::geom::NurbsCurve3::interpolate(&s_points, 3).expect("the S path interpolates");
    let s_duct = pncad::sweep::sweep_body::<f64>(
        &quad(
            [
                (-ELBOW_H, -ELBOW_H),
                (ELBOW_H, -ELBOW_H),
                (ELBOW_H, ELBOW_H),
                (-ELBOW_H, ELBOW_H),
            ],
            tol,
        ),
        Affine3::identity(),
        &path,
        13,
        3,
        tol,
    )
    .expect("the S-path sweep body builds")
    .body;

    // Planar path, centroid ON the path, section symmetric about the
    // path plane: the curvature moment integrates to zero and the
    // continuum volume is A·L = (2h)²·(2·R·π/2).
    let a_times_l = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * (2.0 * S_R * core::f64::consts::FRAC_PI_2);

    let mut stops = vec![
        Stop {
            name: "lofts",
            caption: "the loft pair (same sections, only the middle spacing moves)".to_string(),
            montage: true,
            story: "R5 shape (iii) and its TRUE minimal pair, in one frame. Three \
                    polyline quad sections — squares at the ends, a trapezoid between \
                    — skinned at v-degree 2. The middle section is NOT an affine image \
                    of the squares (affine maps preserve parallelism; the trapezoid has \
                    a non-parallel pair), so the four walls are genuinely curved NURBS \
                    patches, not ruled strips. LEFT: placements z = 0/1/2. RIGHT: the \
                    SAME sections, the SAME 2 m height, and ONLY the middle placement \
                    moved, to z = 0/0.15/2 — the degree-2 skin interpolates through the \
                    crowded spacing and OVERSHOOTS, bulging to half-width 1.646, wider \
                    than any authored section (the trapezoid stops at 1.375), peaking \
                    at 32.6% of the height with a long taper above",
            ops: "sweep::loft_body(square, trapezoid, square, v_degree 2) twice — \
                  @ z = 0/1/2, and @ z = 0/0.15/2 -> topo::transform_rigid for the \
                  pair's +x offset",
            delta: 6e-3,
            note: Some(format!(
                "[loft_prism] the corpus fixture's body, section for section \
                 (step-export/tests/common/mod.rs::loft_prism, \
                 editor-core/tests/corpus/loft_prism.rs, sweep/tests/m6_loft_body.rs — \
                 and the volume below is checked against it); volume is DERIVED, not \
                 measured: the degree-2 skin through sections at (0, 1/2, 1) is the \
                 quadratic Lagrange interpolant, corner paths S + lambda(v)*D with \
                 lambda = 4v(1-v), z = 2v exactly, each slice a trapezoid of area \
                 4 + 2*d*lambda (d = 0.375) -> V = 8 + 8d/3 = 9 m^3 exactly; the walls \
                 RENDER through the trimmed-face NURBS tessellation lane (the M6-3 \
                 frontier's second half).\n   \
                 [nonuniform_loft] the scene LEADS the corpus since montage-v2 (the \
                 s_duct/lily precedent) — the corpus fixture keeps z = 0/1/3 \
                 (step-export/tests/common/mod.rs::nonuniform_loft, #210/#207), whose \
                 bulge (peak 48.8% of height, half-width 1.415) is visually the \
                 prism's silhouette rescaled; MEASURED before this re-spacing. \
                 Derivation at 0/0.15/2: skin_parameters averages cumulative CHORD \
                 lengths over the first strip's control rows (the flared bottom \
                 corners), so t = 3*sqrt(29)/(3*sqrt(29) + sqrt(5701)) = \
                 {NONUNIFORM_T} — which the scene ASKS the kernel for \
                 (sweep::loft_parameters) and pins this derivation against, rather \
                 than re-deriving it in prose; the corner flare is the quadratic \
                 Lagrange bump lambda(v) = v(1-v)/(t(1-t)), slice area 4 + 2d*lambda \
                 (d = 0.375), z(v) the quadratic through (0,0),(t,0.15),(1,2), and \
                 int v(1-v) z'(v) dv = H/6 for ANY quadratic z, so \
                 V = 4H + dH/(3t(1-t)) = 8 + 0.25/(t(1-t)) = 9.721901523222 m^3 \
                 (quadrature agrees at pad ~1e-13). Peak half-width \
                 1 + d/(4t(1-t)) = 1.6457 at z(1/2) = 0.6513 = 32.6% of height. A \
                 naive z-proportional parameterization (t = 0.075) would say \
                 11.604 m^3 — 19% off: the chord-length choice is load-bearing. \
                 Same skin-fit lane whose synthesized weight channel used to land an \
                 ulp off 1.0 on non-uniform spacings and refuse at assembly (#207); \
                 BUILT at the origin like its twin and then MOVED {LOFT_PAIR_GAP} m \
                 along +x by transform_rigid, which maps the described NURBS walls by \
                 their control points (knots and weights untouched), so the rendered \
                 solid is the exact image of the body every number above is stated \
                 about"
            )),
            view: loft_view(),
            bodies: vec![
                SceneBody::plain("loft_prism", [0.55, 0.72, 0.52], prism),
                SceneBody::plain("nonuniform_loft", [0.45, 0.62, 0.78], nonuniform),
            ],
        },
        Stop {
            name: "s_duct",
            caption: "s_duct (an S path in ONE sweep op)".to_string(),
            // Standalone since montage-v2 (Ev, #218 follow-up): the
            // S SOLID is glued-revolves-expressible, so the honest
            // not-a-revolve cell is `twisted_duct`; this scene stays
            // alive as the one-op planar-S construction and the
            // corpus-fold candidate.
            montage: false,
            story: "a 0.5 m square profile swept through an S: two OPPOSED quarter \
                    arcs of radius 2, 13 stations, v-degree 3. A single-axis revolve \
                    can only bend one way, so ONE revolve cannot make this — though \
                    two glued partial revolves could (each planar arc sweep is a \
                    partial revolve's orbit), which is why the montage's sweep cell \
                    is now the non-planar twisted_duct. The frame is path-following \
                    (each station turns the profile by the minimal rotation carrying \
                    the start tangent to its own; on a planar path that rotation axis \
                    is fixed, so the square never rolls)",
            ops: "sweep::sweep_body(square(h = 0.25), S path (two opposed R = 2 \
                  quarter arcs, degree-3 interpolant through 17 exact points), \
                  13 stations, v_degree 3)",
            delta: 5e-3,
            note: Some(format!(
                "the scene LEADS the corpus (the lily precedent): the round-trip \
                 corpus's sweep constant stays the revolve-expressible quarter-arc \
                 elbow (built once in sweep::test_support; the STEP fixture, the \
                 skin-integrality bracket and the tessellation rows all delegate to \
                 it), and this S sweep is the fixture \
                 CANDIDATE for the next corpus fold — sweep_body had ZERO successful \
                 curved-path callers before #207. The volume expectation is A*L = \
                 (2h)^2 * 2R * pi/2 = {a_times_l:.9} m^3 (planar path, centroid on \
                 path, symmetric section: the curvature moment cancels), approached \
                 through two discretizations — 13 stations and the path interpolant — \
                 not equalled",
            )),
            // The S lives in the world x = 0 plane, so the camera sits
            // near the +x axis: the double bend is the OUTLINE, not a
            // shading gradient (#218 review — same acceptance as the
            // loft pair). 15°/12° off pure profile keep the near cap
            // and one side wall lit for depth; cos 15° ≈ 0.97, the S
            // stays essentially unforeshortened.
            view: View {
                elev: 12.0,
                azim: 15.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("s_duct", [0.72, 0.45, 0.30], s_duct)],
        },
    ];

    // ---- The twisted duct: the sweep cell (montage-v2) ----------
    //
    // The twisted cubic r(t) = (At, Bt², Ct³), t ∈ [−1, 1] — THE
    // canonical nonzero-torsion curve. τ = 12ABC/|r′×r″|² has a
    // CONSTANT numerator, so the spine is nowhere-planar: no point
    // has an osculating plane the curve stays in, and its curvature
    // varies continuously too (no arc segment anywhere — the path is
    // the degree-3 interpolant through 33 exact points). A revolve's
    // spine is a planar circular arc; gluing revolves concatenates
    // planar arcs — nothing glued from revolves has a spine with
    // nonzero torsion. The twisted cubic is THIS cell because it is
    // the mathematically definitive nonzero-torsion demonstration.
    let (tc_a, tc_b, tc_c) = (2.2, 1.3, 1.5);
    let cubic_points: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0f64.mul_add(f64::from(k) / 32.0, -1.0);
            Point3::new(tc_a * t, tc_b * t * t, tc_c * t * t * t)
        })
        .collect();
    let cubic_path =
        pncad::geom::NurbsCurve3::interpolate(&cubic_points, 3).expect("the cubic interpolates");
    // Profile plane normal to the start tangent — the same recipe the
    // narration opens with, spelled once.
    let place = normal_start_place(&cubic_path);
    let twisted = pncad::sweep::sweep_body::<f64>(
        &quad(
            [
                (-ELBOW_H, -ELBOW_H),
                (ELBOW_H, -ELBOW_H),
                (ELBOW_H, ELBOW_H),
                (-ELBOW_H, ELBOW_H),
            ],
            tol,
        ),
        place,
        &cubic_path,
        17,
        3,
        tol,
    )
    .expect("the twisted-cubic sweep body builds")
    .body;

    // Continuum volume expectation A·L for ANY normal-section frame
    // of a centered symmetric profile (the curvature moment cancels
    // by symmetry and roll about the tangent drops out of the
    // Jacobian): L here is the interpolant's arc length, computed by
    // composite Simpson on ‖dC/dt‖ — no elementary closed form for
    // ∫√(A² + 4B²t² + 9C²t⁴) dt.
    let interp_len = {
        let (lo, hi) = cubic_path.domain();
        let n = 4096;
        let f = |t: f64| cubic_path.deriv(t).norm();
        let mut s = f(lo) + f(hi);
        for i in 1..n {
            let w = if i % 2 == 1 { 4.0 } else { 2.0 };
            #[allow(clippy::cast_precision_loss)]
            let t = ((hi - lo) / n as f64).mul_add(i as f64, lo);
            s += w * f(t);
        }
        #[allow(clippy::cast_precision_loss)]
        let h = (hi - lo) / n as f64;
        s * h / 3.0
    };
    let tc_al = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * interp_len;
    let tau0 = 3.0 * tc_c / (tc_a * tc_b);

    let twisted_color = [0.58, 0.42, 0.66];
    let shadow = |name: &'static str, caption: String, elev: f64, azim: f64| Stop {
        name,
        caption,
        montage: false,
        story: "shadow proof: the twisted duct viewed straight down one axis — \
                parallel projections of a PLANAR curve are all affine images of one \
                another, and affine maps preserve inflection count, so a parabola \
                (no inflection) down z and a cubic S (one inflection) down y prove \
                the spine is planar in NO plane",
        ops: "same body as twisted_duct; axis view",
        delta: 1e-2,
        note: None,
        view: View {
            elev,
            azim,
            up: 'z',
        },
        bodies: vec![SceneBody::plain(name, twisted_color, twisted.clone())],
    };

    // ---- The twisted TUBE: the same spine, said as a LOFT --------
    //
    // `sweep_body` above takes ONE profile and derives its own frame,
    // so there is no argument in which a taper or a roll could be
    // asked for. `loft_body` takes the sections and the placements as
    // TWO lists, so both are sayable — and a section may carry a HOLE,
    // which is the third thing this body has and its neighbour cannot.
    //
    // Sharing `cubic_path` is the point. The spine's nowhere-zero
    // torsion is asserted next door and is not re-derived here; what
    // this body adds on top of it is taper, roll and an annulus, none
    // of which any extrude or revolve reaches either. A prism has one
    // section; a solid of revolution has one axis, and a section whose
    // size varies along the spine has none.
    let tube_places: Vec<Affine3<f64>> = (0..TUBE_STATIONS)
        .map(|i| tube_place(&cubic_path, i, TUBE_ROLL))
        .collect();
    let unrolled_places: Vec<Affine3<f64>> = (0..TUBE_STATIONS)
        .map(|i| tube_place(&cubic_path, i, 0.0))
        .collect();
    let tube_sections: Vec<Section> = (0..TUBE_STATIONS)
        .map(|i| {
            let k = tube_taper(i);
            vec![square(TUBE_OUT * k, tol), square(TUBE_IN * k, tol)]
        })
        .collect();
    // The same stations with the hole left out: the volume oracle's
    // other operand, and nothing else.
    let solid_sections: Vec<Section> = (0..TUBE_STATIONS)
        .map(|i| vec![square(TUBE_OUT * tube_taper(i), tol)])
        .collect();

    let twisted_tube = pncad::sweep::loft_body::<f64>(&tube_sections, &tube_places, 3, tol)
        .expect("the annular sections skin along the twisted cubic")
        .body;
    let twisted_solid = pncad::sweep::loft_body::<f64>(&solid_sections, &tube_places, 3, tol)
        .expect("the same stations without the hole skin too")
        .body;
    let unrolled_tube = pncad::sweep::loft_body::<f64>(&tube_sections, &unrolled_places, 3, tol)
        .expect("the roll-free twin skins")
        .body;

    // **The hole is real**, and the census is where that is legible: a
    // capped tube is an annulus swept along an interval, so it is a
    // solid torus and its genus is 1. The solid twin over the same
    // stations is genus 0.
    assert_eq!(genus(&twisted_tube), 1, "a capped tube is a solid torus");
    assert_eq!(
        genus(&twisted_solid),
        0,
        "…and the hole is what makes it one"
    );
    assert_eq!(
        pncad::topo::validate_geometric(&twisted_tube, tol),
        Ok(()),
        "the twisted tube: tier 3"
    );

    // **The volume, in closed form against the solid twin.** Both
    // sections are centred and symmetric, so each one's curvature
    // moment vanishes and each body's volume is ∫A ds along the spine
    // (the same argument the neighbouring cell states for its own
    // A·L). Both loops taper by the SAME factor at every station, so
    // the ratio of the areas is constant along the spine and comes
    // straight out of the integral: 1 − (TUBE_IN/TUBE_OUT)² = 16/25,
    // whatever the spine does.
    let v_tube = pncad::topo::mass_properties(&twisted_tube, tol)
        .expect("the tube has a volume")
        .volume;
    let v_solid = pncad::topo::mass_properties(&twisted_solid, tol)
        .expect("the solid twin has a volume")
        .volume;
    let hollow_ratio = 1.0 - (TUBE_IN / TUBE_OUT) * (TUBE_IN / TUBE_OUT);
    assert!(
        ((v_tube - hollow_ratio * v_solid) / v_tube).abs() < 1e-12,
        "V_tube = {v_tube} against (1 − λ²)·V_solid = {}",
        hollow_ratio * v_solid
    );

    // **The taper, measured on the stored body.** The two end caps'
    // widths are their outer squares' diagonals, and a sweep could
    // produce neither from the other: one profile goes down the path.
    let (base_w, tip_w) = cap_widths(&twisted_tube);
    let want_base = TUBE_OUT * 2.0 * std::f64::consts::SQRT_2;
    let want_tip = want_base * TUBE_TAPER_END;
    assert!(
        (base_w - want_base).abs() < 1e-9 && (tip_w - want_tip).abs() < 1e-9,
        "cap widths {base_w} / {tip_w} against {want_base} / {want_tip}"
    );

    // **The roll, ISOLATED.** Comparing this body's own two ends mixes
    // the authored roll with the spine's own turn, so the comparison
    // is against the SAME body built with the roll set to zero:
    // identical spine, identical stations, identical sections, so the
    // two tip caps are coplanar and their frames differ by the roll
    // and nothing else. (The instrument is lily's, on a body whose
    // spine has torsion.)
    let rolled = tip_axis(&twisted_tube);
    let unrolled = tip_axis(&unrolled_tube);
    let measured_roll = rolled.dot(unrolled).clamp(-1.0, 1.0).acos();
    let quarter = std::f64::consts::FRAC_PI_2;
    let folded = (measured_roll % quarter).min(quarter - measured_roll % quarter);
    let want = (TUBE_ROLL % quarter).min(quarter - TUBE_ROLL % quarter);
    assert!(
        (folded - want).abs() < 1e-6,
        "the tip caps differ by {measured_roll} rad, folded to {folded} against the \
         authored {TUBE_ROLL} folded to {want} — a square's axis is only defined mod \
         a quarter turn, which is why both sides fold"
    );

    stops.push(Stop {
        name: "twisted_tube",
        caption: "THE SAME SPINE, HOLLOW AND TAPERING (a loft, not a sweep)".to_string(),
        montage: true,
        story: "the twisted cubic again — the spine whose torsion is nowhere zero, so \
                no assembly of revolves reaches it — but said as a LOFT rather than a \
                sweep. `sweep_body` takes ONE profile and derives its own frame, so \
                neither a taper nor a roll is sayable to it; `loft_body` takes the \
                sections and the placements as two lists, so the square shrinks to \
                half its width down the spine and turns 1 rad about it on the way. \
                And a section may carry a HOLE: this is a capped tube, genus 1, whose \
                volume is the solid twin's times 1 − (3/5)^2 = 16/25 exactly. Nothing \
                extruded reaches it (a prism has one section) and nothing revolved \
                does either (a section whose size varies along the spine has no axis)",
        ops: "NurbsCurve3::interpolate(33 points of (2.2t, 1.3t^2, 1.5t^3), degree 3) \
              -> 6 ANNULAR square sections tapering 1 -> 1/2, placed normal to the \
              spine and rolled 1 rad -> sweep::loft_body(v_degree 3). The roll is \
              measured against the SAME body built roll-free, so the spine's own turn \
              is differenced out",
        delta: 1e-2,
        note: Some(format!(
            "genus 1 (a capped tube is a solid torus; the solid twin over the same \
             stations is genus 0). V = {v_tube:.9} against the twin's {v_solid:.9} \
             times 16/25 — an equality, because both loops taper by the same factor \
             at every station, so the area ratio is constant along the spine and the \
             curvature moment of each centred symmetric section vanishes. Cap widths \
             {base_w:.6} -> {tip_w:.6} m (the outer squares' diagonals), the taper \
             read off the stored body. The tip caps of the rolled and roll-free twins \
             differ by {measured_roll:.6} rad, folded to {folded:.6} against the \
             authored {TUBE_ROLL} folded to {want:.6} — a square's axis is defined \
             only mod a quarter turn, so both sides fold"
        )),
        // Looking into the NARROW end, roughly down its own tangent:
        // the hole reads at the near cap, the section widens as the
        // body recedes, and the spine's S still crosses the frame.
        view: View {
            elev: 38.0,
            azim: 40.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain(
            "twisted_tube",
            [0.44, 0.55, 0.68],
            twisted_tube,
        )],
    });

    stops.push(Stop {
        name: "twisted_duct",
        caption: "twisted_duct (nowhere-zero torsion)".to_string(),
        // Montage cell RETIRED in favour of `twisted_tube` next door,
        // which is this body's spine with three things added that no
        // extrude or revolve reaches either — a taper, a roll and a
        // hole — and none of which a SWEEP can be asked for. The
        // torsion claim is not lost with the cell: it is this scene's,
        // it is asserted here, and `twisted_tube`'s story cites it
        // rather than re-deriving it. Standalone render and both
        // shadow proofs are untouched, which is where the spine's
        // non-planarity is looked at.
        montage: false,
        story: "a 0.5 m square swept along the TWISTED CUBIC (2.2t, 1.3t², 1.5t³), \
                17 stations, v-degree 3 — a spine with nowhere-zero TORSION and \
                continuously varying curvature, no arc anywhere. A revolve's spine \
                is a planar circular arc, and gluing revolves only concatenates \
                planar arcs, so NO assembly of revolves reaches this body — unlike \
                the planar S (s_duct, standalone), which two glued partial revolves \
                could fake. The square visibly rolls as the bend plane turns: the \
                path-following frame carries it through the spine's torsion",
        ops: "sweep::sweep_body(square(h = 0.25), twisted cubic (At, Bt^2, Ct^3), \
              A/B/C = 2.2/1.3/1.5, degree-3 interpolant through 33 exact points, \
              17 stations, v_degree 3)",
        delta: 5e-3,
        note: Some(format!(
            "torsion tau = 12ABC/|r' x r''|^2 — CONSTANT numerator 12ABC = \
             {:.2}, so tau > 0 everywhere (peak tau(0) = 3C/(AB) = {tau0:.4} \
             m^-1) — and curvature varies continuously with it; the shadow pair \
             (twisted_duct_shadow_z: a parabola; twisted_duct_shadow_y: a cubic S) \
             is the planarity REFUTATION, since parallel projections of a planar \
             curve are affine images of each other and cannot differ in inflection \
             count. Volume expectation A*L = {tc_al:.9} m^3 (L by quadrature over \
             the interpolant; centered symmetric section, so the curvature moment \
             cancels and frame roll drops out), approached through the two \
             discretizations, not equalled. Fixture CANDIDATE for the next corpus \
             fold, beside the S",
            12.0 * tc_a * tc_b * tc_c
        )),
        // The spine's biggest excursion is the cubic S in the xz
        // plane (down −y) with the parabolic bow in y adding depth;
        // 25° of azimuth off −y and a low elevation keep BOTH visible
        // as outline: the S reads directly, and the near end's roll
        // (the square's ridge lines turning) reads against it.
        view: View {
            elev: 14.0,
            azim: -65.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain(
            "twisted_duct",
            twisted_color,
            twisted.clone(),
        )],
    });
    stops.push(shadow(
        "twisted_duct_shadow_z",
        "z-shadow: a parabola (no inflection)".to_string(),
        90.0,
        -90.0,
    ));
    stops.push(shadow(
        "twisted_duct_shadow_y",
        "y-shadow: a cubic S (one inflection)".to_string(),
        0.0,
        -90.0,
    ));
    stops
}

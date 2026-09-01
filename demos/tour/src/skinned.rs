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
//!   montage-v3 curation (Evan, 2026-08-30). Two adjacent cells were
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
//!   `LOFT_PAIR_GAP` along +x of its twin (a rigid placement, applied
//!   after every assertion; volume is translation-invariant).
//! - `s_duct` — standalone since montage-v2 (Evan, #218 follow-up:
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

use pncad::geom_core::{Affine3, Point2, Point3, Vec3};
use pncad::prelude::{Open, Start, Via};
use pncad::profile::SketchPlane;
use pncad::sweep::skin::{Section, loft_geometry, sweep_geometry};
use pncad::sweep::{SketchSegment, segment_curve};

use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// A square-with-an-arc section, scaled by `s` (LIB-U3 profile
/// vocabulary: one loop, the arc as vertex 1's bulge).
fn chain(s: f64, tol: Tol) -> Section {
    // Lattice-authored since LIB-RETTAIL (raw `ProfileLoop` construction
    // is no longer presented surface, Evan's ruling on #413). The one
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
/// code. The corpus builds it with `ProfileLoop::polygon`, this builds
/// it through the PATHS lattice ([`crate::paths::path_polygon`]),
/// which is the spelling this tour is here to show.
fn quad(pts: [(f64, f64); 4], tol: Tol) -> Section {
    vec![crate::paths::path_polygon(&pts, tol)]
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
    SketchPlane::from_frame(path.eval(lo), u, n.cross(u)).placement
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

/// Section placements: pure translations up the world z-axis (also
/// `common/mod.rs::lofted_at_z`).
fn lofted_at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    lofted_at_x_z(0.0, zs)
}

/// The same stack of placements, the whole loft shifted `dx` along +x —
/// how the pair cell puts the two lofts side by side while
/// `transform_rigid` refuses a NURBS-walled body (#1346).
///
/// A COMMON offset on every placement is a rigid motion of the finished
/// solid: it moves the body and changes no section, no spacing and no
/// parameterization (`skin_parameters` reads chord lengths between
/// control rows, which a common translation leaves alone — pinned by
/// the `loft_parameters` assertion in [`stops`], which still gets the
/// same `t`).
fn lofted_at_x_z(dx: f64, zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(dx, 0.0, *z)))
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
    let nonuniform_places = lofted_at_x_z(LOFT_PAIR_GAP, &[0.0, 0.15, 2.0]);
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
    let nonuniform =
        pncad::sweep::loft_body::<f64>(&prism_sections(tol), &nonuniform_places, 2, tol)
            .expect("the non-uniform loft builds")
            .body;
    // ONE CELL, BOTH BODIES (montage-v3 curation, Evan 2026-08-30).
    // Two adjacent cells were not showing the pair the pair claims to
    // be: `compose_montage.py` trims and scales EVERY cell
    // independently, so the two panels arrived at two different scales
    // and the silhouette comparison — the whole content of a minimal
    // pair — was distorted by the composer. Side by side in ONE frame
    // they share a camera AND a scale, which is what "only the middle
    // placement moved" needs a reader to be able to see.
    //
    // GAP RECORDED, NOT WORKED AROUND (`memories/demo-purpose.md`):
    // the natural spelling here is `twopeg`'s / the teapot lid's one
    // — build the body, then place it with `transform_rigid` — and it
    // REFUSES on this body: `TransformError::NurbsPlaceholder`, from an
    // arm that matches the `Surface::Nurbs` VARIANT while its own text
    // describes only the placeholder ("unimplemented geometry evaluates
    // to poison"). These walls are described degree-1×2 and 2×2 nets
    // that evaluate, tessellate, integrate and export; nothing about
    // them is poison, and `NurbsSurface::is_placeholder()` is public on
    // both halves. So NO loft, sweep or skinned body in this kernel can
    // be moved — filed as #1346, which is strictly less work than the
    // Approx arm beside it (#1020): a rigid map of a described net is
    // the control-point map, weights and knots unchanged, no
    // certificate to re-derive.
    //
    // Until it lands, the pair is placed by AUTHORING the second loft's
    // three placements at the offset rather than by moving the built
    // body. That is a rigid motion of the whole loft — every section
    // shifted by the same vector — so it changes the body's position
    // and nothing else, and the derivations below still hold of the
    // body that renders. What it does NOT do is exercise the door a
    // user would reach for, which is the finding.

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
                  @ z = 0/1/2, and @ z = 0/0.15/2 with every placement carrying the \
                  pair's +x offset (transform_rigid REFUSES a NURBS-walled body, #1346)",
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
                 authored {LOFT_PAIR_GAP} m along +x of its twin (a COMMON offset on \
                 all three placements — a rigid motion of the whole loft, leaving \
                 every number above invariant). It is authored rather than \
                 transformed because `transform_rigid` REFUSES a NURBS-walled body: \
                 the arm matches the `Surface::Nurbs` variant while its reason \
                 describes only the placeholder, so no loft, sweep or skinned body \
                 in this kernel can be moved — #1346"
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
            // Standalone since montage-v2 (Evan, #218 follow-up): the
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

    stops.push(Stop {
        name: "twisted_duct",
        caption: "twisted_duct (nowhere-zero torsion)".to_string(),
        montage: true,
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

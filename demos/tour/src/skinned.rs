//! The loft and the sweep — the tour's DEFINITIONAL stop (M5 PR 10;
//! frontier closed at M6-3), narration **and, since the montage
//! refresh, three scenes**.
//!
//! # The narration's frontier, fully retired
//!
//! The §10.3 skin and the §10.4 sweep are real geometry; since M6-3
//! the loft BODY assembles and validates at tier 3, and the
//! retire-on-closure panic that pinned that frontier fired as
//! designed. The half still outstanding — "the full `Stop` with a
//! `SceneBody`", i.e. the curved-wall RENDER through the trimmed-face
//! tessellation of NURBS patches — lands HERE, as [`stops`]. The
//! narration stays beside the scenes: it is the *geometry* layer
//! (`loft_geometry` / `sweep_geometry` — control nets, weights, the
//! MEASURED interpolation claim, the not-ruled claim), which no
//! render can show.
//!
//! # The three scenes ARE the corpus
//!
//! Every construction here is a corpus fixture, constant for
//! constant, so the montage cell and the fixture that guards it are
//! the same body:
//!
//! - `loft_prism` — `step-export/tests/common/mod.rs::loft_prism()`
//!   (recipe-layer twin: `editor-core/tests/corpus/loft_prism.rs`;
//!   acceptance + the derived V = 9 m³ bracket:
//!   `sweep/tests/m6_loft_body.rs`).
//! - `nonuniform_loft` —
//!   `step-export/tests/common/mod.rs::nonuniform_loft()` (#210/#207).
//! - `swept_elbow` — `step-export/tests/common/mod.rs::swept_elbow()`,
//!   itself `sweep/tests/m7_skin_integral.rs`'s elbow (#210/#207).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use profile::SketchPlane;
use sweep::skin::{SectionSegments, loft_geometry, sweep_geometry};
use sweep::{SketchSegment, segment_curve};

use crate::{SceneBody, Stop, View};

/// A square-with-an-arc section, scaled by `s`.
fn chain(s: f64) -> SectionSegments {
    let p = |x: f64, y: f64| Point2::new(x * s, y * s);
    vec![vec![
        SketchSegment::Line {
            a: p(0.0, 0.0),
            b: p(2.0, 0.0),
        },
        SketchSegment::Arc {
            a: p(2.0, 0.0),
            b: p(2.0, 1.0),
            bulge: 0.25,
        },
        SketchSegment::Line {
            a: p(2.0, 1.0),
            b: p(0.0, 1.0),
        },
        SketchSegment::Line {
            a: p(0.0, 1.0),
            b: p(0.0, 0.0),
        },
    ]]
}

/// The tour's loft + sweep stop.
pub fn narration() {
    println!("\n-- the loft and the sweep (M5 PR 10: definitional NURBS walls) --");

    // ---- The loft: three sections, the middle one scaled. ----
    let places = [0.0, 1.0, 2.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    let loft = loft_geometry(&[chain(1.0), chain(1.6), chain(1.0)], &places, 2)
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
        Tolerance::get().eps
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
    let t0 = path.deriv(0.0);
    let n = t0 / t0.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::unit_z()
    } else {
        Vec3::unit_x()
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let place = SketchPlane::from_frame(path.eval(0.0), u, n.cross(u)).placement;
    let swept = sweep_geometry(&chain(1.0), place, &path, 5, 3).expect("the arc sweep skins");
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
         loft_prism, nonuniform_loft, swept_elbow — see the stops below"
    );
}

// ---- The scene constructions (corpus fixtures, constant for
// constant — `step-export/tests/common/mod.rs`) -------------------

/// A closed four-segment polyline section (one loop, four lines) —
/// the plainest INTEGRAL profile: unit weights, no arc anywhere.
/// `common/mod.rs::quad`, verbatim.
fn quad(pts: [(f64, f64); 4]) -> SectionSegments {
    let seg = |a: (f64, f64), b: (f64, f64)| SketchSegment::Line {
        a: Point2::new(a.0, a.1),
        b: Point2::new(b.0, b.1),
    };
    vec![vec![
        seg(pts[0], pts[1]),
        seg(pts[1], pts[2]),
        seg(pts[2], pts[3]),
        seg(pts[3], pts[0]),
    ]]
}

/// The prism's end sections (`common/mod.rs::PRISM_SQUARE`).
const PRISM_SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
/// Its middle section: the NON-AFFINE trapezoid whose two bottom
/// corners flare by ±d, d = 0.375 (`common/mod.rs::PRISM_TRAPEZOID`).
const PRISM_TRAPEZOID: [(f64, f64); 4] = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

/// The elbow's path radius (`m7_skin_integral.rs::ELBOW_R`).
const ELBOW_R: f64 = 3.0;
/// The elbow's profile half-width (`m7_skin_integral.rs::ELBOW_H`).
const ELBOW_H: f64 = 0.25;

/// Section placements: pure translations up the world z-axis
/// (`common/mod.rs::lofted_at_z`).
fn lofted_at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// The square/trapezoid/square section stack both loft scenes share —
/// the minimal pair's shared half.
fn prism_sections() -> Vec<SectionSegments> {
    vec![
        quad(PRISM_SQUARE),
        quad(PRISM_TRAPEZOID),
        quad(PRISM_SQUARE),
    ]
}

/// The three skin scenes, in tour order: the two lofts as the
/// corpus's MINIMAL PAIR (same sections, same degree, same builder —
/// only the section spacing differs, so they share a camera and read
/// as a pair on the sheet), then the curved-path sweep.
pub fn stops() -> Vec<Stop> {
    // Both lofts are 2 m and 3 m tall columns flaring at one height;
    // the camera looks at the flared −y wall (normal −y, the one the
    // ±0.375 corners move) across one side wall, so the flare reads
    // as a bulge in the vertical silhouette rather than head-on.
    let loft_view = || View {
        elev: 18.0,
        azim: -55.0,
        up: 'z',
    };

    let prism = sweep::loft_body::<f64>(&prism_sections(), &lofted_at_z(&[0.0, 1.0, 2.0]), 2)
        .expect("shape (iii) loft builds")
        .body;
    let nonuniform = sweep::loft_body::<f64>(&prism_sections(), &lofted_at_z(&[0.0, 1.0, 3.0]), 2)
        .expect("the non-uniform loft builds")
        .body;

    // `common/mod.rs::swept_elbow`, constant for constant. The sketch
    // arc runs (0,0) → (R,R) with bulge = tan(π/8) — a 90° turn; the
    // placement rotates the sketch plane by −π/2 about the world
    // y-axis, sending sketch (x, y) to world (0, y, x). The path
    // therefore leaves the origin with tangent +z and arrives at
    // (0, 3, 3) with tangent +y, and the identity-placed square
    // profile (world XY plane) is already normal to its start.
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(ELBOW_R, ELBOW_R),
            bulge: (core::f64::consts::PI / 8.0).tan(),
        },
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -core::f64::consts::FRAC_PI_2,
        ),
    )
    .expect("the elbow path is a well-formed quarter arc");
    let elbow = sweep::sweep_body::<f64>(
        &quad([
            (-ELBOW_H, -ELBOW_H),
            (ELBOW_H, -ELBOW_H),
            (ELBOW_H, ELBOW_H),
            (-ELBOW_H, ELBOW_H),
        ]),
        Affine3::identity(),
        &path,
        9,
        3,
    )
    .expect("the curved-path sweep body builds")
    .body;

    let pappus = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * ELBOW_R * core::f64::consts::FRAC_PI_2;

    vec![
        Stop {
            name: "loft_prism",
            caption: "loft_prism (3 sections, non-affine middle)".to_string(),
            montage: true,
            story: "R5 shape (iii): three polyline quad sections — squares at z = 0 and \
                    z = 2, a trapezoid at z = 1 — skinned at v-degree 2. The middle \
                    section is NOT an affine image of the squares (affine maps preserve \
                    parallelism; the trapezoid has a non-parallel pair), so the four \
                    walls are genuinely curved NURBS patches, not ruled strips",
            ops: "sweep::loft_body(square, trapezoid, square @ z = 0/1/2, v_degree 2)",
            delta: 6e-3,
            note: Some(format!(
                "the corpus fixture VERBATIM (step-export/tests/common/mod.rs::loft_prism, \
                 editor-core/tests/corpus/loft_prism.rs, sweep/tests/m6_loft_body.rs); \
                 volume is DERIVED, not measured: the degree-2 skin through sections at \
                 (0, 1/2, 1) is the quadratic Lagrange interpolant, corner paths \
                 S + lambda(v)*D with lambda = 4v(1-v), z = 2v exactly, each slice a \
                 trapezoid of area 4 + 2*d*lambda (d = 0.375) -> \
                 V = 8 + 16d/3 = 9 m^3 exactly; the walls RENDER here through the \
                 trimmed-face NURBS tessellation lane (the M6-3 frontier's second half)"
            )),
            view: loft_view(),
            bodies: vec![SceneBody::plain("loft_prism", [0.55, 0.72, 0.52], prism)],
        },
        Stop {
            name: "nonuniform_loft",
            caption: "nonuniform_loft (the minimal pair: z = 0/1/3)".to_string(),
            montage: true,
            story: "loft_prism's OWN three sections re-placed at z = 0, 1, 3 — spacing \
                    1 : 2 instead of 1 : 1, and nothing else changed. The flare sits a \
                    third of the way up instead of at mid-height, and the skin fit's \
                    synthesized weight channel has to survive a non-uniform \
                    parameterization: until #207 its LU round-trip landed an ulp off \
                    1.0 on exactly this input, the walls came out bitwise RATIONAL, and \
                    the body refused at assembly",
            ops: "sweep::loft_body(square, trapezoid, square @ z = 0/1/3, v_degree 2)",
            delta: 6e-3,
            note: Some(format!(
                "the corpus fixture VERBATIM \
                 (step-export/tests/common/mod.rs::nonuniform_loft, #210/#207) — the \
                 minimal pair with loft_prism, shown WITH it and under the same camera. \
                 The v-parameterization is NOT [0, 1/3, 1]: skin_parameters averages \
                 cumulative CHORD lengths, and the trapezoid's flare lengthens the \
                 first chord, so t = sqrt(73)/(sqrt(73) + sqrt(265)) = \
                 0.34419950074181277 and the derived volume is \
                 V = 12 + 0.375/(t(1-t)) = 12.75 + 126.75/sqrt(19345) = \
                 13.661304680798798 m^3 (the naive 1/3 would say 13.6875 — out by \
                 1.9e-3 relative, 1.6e8 times the certified pad)"
            )),
            view: loft_view(),
            bodies: vec![SceneBody::plain(
                "nonuniform_loft",
                [0.45, 0.62, 0.78],
                nonuniform,
            )],
        },
        Stop {
            name: "swept_elbow",
            caption: "swept_elbow (first CURVED-path sweep body)".to_string(),
            montage: true,
            story: "the tour's first body swept along a CURVED path: a 0.5 m square \
                    profile carried through a 90-degree arc of radius 3 at nine \
                    stations, skinned at v-degree 3. The frame is path-following — each \
                    station turns the profile by the minimal rotation carrying the \
                    path's start tangent to its own — so the square stays normal to the \
                    arc all the way round and the four walls are one NURBS patch each",
            ops: "sweep::sweep_body(square(h = 0.25), quarter arc R = 3, 9 stations, \
                  v_degree 3)",
            delta: 3e-3,
            note: Some(format!(
                "the corpus fixture VERBATIM \
                 (step-export/tests/common/mod.rs::swept_elbow = \
                 sweep/tests/m7_skin_integral.rs's elbow, #210/#207); sweep_body had \
                 ZERO successful callers anywhere in the tree before #207 — every \
                 curved path drove the same synthesized-weight drift the non-uniform \
                 loft did. The volume claim is CONVERGENCE to Pappus, not equality: \
                 the stations lie exactly on the quarter torus of square cross-section \
                 whose Pappus volume is (2h)^2 * R * pi/2 = {pappus:.9} m^3, but the \
                 walls are the degree-3 interpolation THROUGH nine such stations, so \
                 the ~4e-6 relative gap is discretization (it fell ~1.8 decades when \
                 the station count went 5 -> 9) and sits four orders WIDER than the \
                 certified enclosure pad"
            )),
            // The elbow lives in the world YZ plane (x is its thin
            // 0.5 m dimension), so the montage's best angle is the
            // one that shows all three of what the stop is about:
            // the arc's turn, the square section, and the walls'
            // ruling direction. Camera at azim 38 / elev 22 sees the
            // +y end cap (the profile, head-on-ish), the convex
            // outer wall along the whole turn, and one side wall in
            // grazing light — while the start cap (normal −z) hides,
            // which is what keeps the arc reading as a turn rather
            // than as a symmetric ribbon.
            view: View {
                elev: 22.0,
                azim: 38.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("swept_elbow", [0.72, 0.45, 0.30], elbow)],
        },
    ]
}

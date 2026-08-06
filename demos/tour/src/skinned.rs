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
//! - `s_duct` — the one scene that LEADS the corpus rather than
//!   citing it (the lily precedent; #218 review): the corpus's sweep
//!   constant, `step-export/tests/common/mod.rs::swept_elbow()`
//!   (`sweep/tests/m7_skin_integral.rs`'s quarter-arc elbow), is
//!   revolve-expressible, so the CELL carries an S path no revolve
//!   can orbit and stands as the fixture candidate for the next
//!   corpus fold. The elbow remains the tested constant in the mesh
//!   and sweep suites.

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
         loft_prism, nonuniform_loft, s_duct — see the stops below"
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

/// The S-duct's arc radius (scene-local; the corpus elbow's is
/// `m7_skin_integral.rs::ELBOW_R` = 3 — see the S-path note below).
const S_R: f64 = 2.0;
/// The profile half-width (`m7_skin_integral.rs::ELBOW_H`, shared).
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
    // Both lofts are 2 m and 3 m tall columns flaring in x at one
    // height, so the story-bearing silhouette is the xz PROFILE: a
    // near-face-on ±y camera puts the ±x walls edge-on and the flare
    // becomes a bulge in the outline itself — the prism's peak at
    // mid-height, the non-uniform's at one third with its long taper —
    // rather than a shading difference (#218 review: the pair must be
    // distinct in profile, not shading). 10° of azimuth and elevation
    // keep a sliver of side wall and top for depth; cos 10° ≈ 0.985,
    // so the profile stays essentially unforeshortened. Shared by the
    // pair on purpose (the minimal-pair principle: same camera makes
    // the difference attributable to the geometry).
    let loft_view = || View {
        elev: 10.0,
        azim: -80.0,
        up: 'z',
    };

    let prism = sweep::loft_body::<f64>(&prism_sections(), &lofted_at_z(&[0.0, 1.0, 2.0]), 2)
        .expect("shape (iii) loft builds")
        .body;
    let nonuniform = sweep::loft_body::<f64>(&prism_sections(), &lofted_at_z(&[0.0, 1.0, 3.0]), 2)
        .expect("the non-uniform loft builds")
        .body;

    // The S path (#218 review): the corpus's quarter-arc elbow is
    // revolve-expressible — a square swept along ONE planar arc is a
    // partial revolve's orbit, so its cell demonstrated nothing a
    // revolve couldn't and sat next to the (deliberately torus-class)
    // tube cell looking like its sibling. A single-axis revolve can
    // only bend one way; this path bends BOTH ways: two opposed
    // quarter arcs of radius R in the world x = 0 plane, sampled at 17
    // exact points and interpolated at degree 3 (the path is the
    // cubic interpolant through the S — the sweep machinery consumes
    // any NurbsCurve3, #210). Tangent runs +z → +y → +z; never
    // reversed, so the path-following frame is total. The QUARTER-ARC
    // elbow stays the corpus/suite constant
    // (step-export/tests/common/mod.rs::swept_elbow,
    // sweep/tests/m7_skin_integral.rs, mesh/tests/m7_nurbs_trimmed.rs);
    // this scene LEADS the corpus (the lily precedent) — the S sweep
    // is a fixture CANDIDATE for the next corpus fold.
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
        geom_curves::NurbsCurve3::interpolate(&s_points, 3).expect("the S path interpolates");
    let s_duct = sweep::sweep_body::<f64>(
        &quad([
            (-ELBOW_H, -ELBOW_H),
            (ELBOW_H, -ELBOW_H),
            (ELBOW_H, ELBOW_H),
            (-ELBOW_H, ELBOW_H),
        ]),
        Affine3::identity(),
        &path,
        13,
        3,
    )
    .expect("the S-path sweep body builds")
    .body;

    // Planar path, centroid ON the path, section symmetric about the
    // path plane: the curvature moment integrates to zero and the
    // continuum volume is A·L = (2h)²·(2·R·π/2).
    let a_times_l = (2.0 * ELBOW_H) * (2.0 * ELBOW_H) * (2.0 * S_R * core::f64::consts::FRAC_PI_2);

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
            note: Some(
                "the corpus fixture VERBATIM (step-export/tests/common/mod.rs::loft_prism, \
                 editor-core/tests/corpus/loft_prism.rs, sweep/tests/m6_loft_body.rs); \
                 volume is DERIVED, not measured: the degree-2 skin through sections at \
                 (0, 1/2, 1) is the quadratic Lagrange interpolant, corner paths \
                 S + lambda(v)*D with lambda = 4v(1-v), z = 2v exactly, each slice a \
                 trapezoid of area 4 + 2*d*lambda (d = 0.375) -> \
                 V = 8 + 16d/3 = 9 m^3 exactly; the walls RENDER here through the \
                 trimmed-face NURBS tessellation lane (the M6-3 frontier's second half)"
                    .to_string(),
            ),
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
            note: Some(
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
                    .to_string(),
            ),
            view: loft_view(),
            bodies: vec![SceneBody::plain(
                "nonuniform_loft",
                [0.45, 0.62, 0.78],
                nonuniform,
            )],
        },
        Stop {
            name: "s_duct",
            caption: "s_duct (a sweep no revolve can orbit)".to_string(),
            montage: true,
            story: "a 0.5 m square profile swept through an S: two OPPOSED quarter \
                    arcs of radius 2, 13 stations, v-degree 3. A single-axis revolve \
                    can only bend one way around its axis — a path whose curvature \
                    changes sign is the shape class only the sweep machinery reaches, \
                    and the reversal is in the silhouette itself. The frame is \
                    path-following (each station turns the profile by the minimal \
                    rotation carrying the start tangent to its own; on a planar path \
                    that rotation axis is fixed, so the square never rolls)",
            ops: "sweep::sweep_body(square(h = 0.25), S path (two opposed R = 2 \
                  quarter arcs, degree-3 interpolant through 17 exact points), \
                  13 stations, v_degree 3)",
            delta: 5e-3,
            note: Some(format!(
                "the scene LEADS the corpus (the lily precedent): the round-trip \
                 corpus's sweep constant stays the revolve-expressible quarter-arc \
                 elbow (step-export/tests/common/mod.rs::swept_elbow = \
                 sweep/tests/m7_skin_integral.rs's, also \
                 mesh/tests/m7_nurbs_trimmed.rs's), and this S sweep is the fixture \
                 CANDIDATE for the next corpus fold — sweep_body had ZERO successful \
                 curved-path callers before #207, and #218's review asked for a cell a \
                 revolve could NOT have produced. The volume expectation is A*L = \
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
    ]
}

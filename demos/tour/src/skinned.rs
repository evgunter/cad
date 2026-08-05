//! The loft and the sweep — the tour's DEFINITIONAL stop (M5 PR 10;
//! frontier closed at M6-3), narration only.
//!
//! # Why narration and not a scene (updated at M6-3)
//!
//! The §10.3 skin and the §10.4 sweep are real geometry, and since
//! M6-3 the loft BODY assembles and validates at tier 3 — the
//! retire-on-closure panic that pinned the old frontier fired as
//! designed and its narration retired into
//! [`lofted_solid_narration`], which builds and validates the solid
//! live. What still keeps this a narration rather than a scene: the
//! curved-wall RENDER (trimmed-face tessellation of NURBS patches) —
//! the full `Stop` with a `SceneBody` lands with it (tracked in the
//! M6-3 PR as the retirement's second half).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use profile::SketchPlane;
use sweep::skin::{SectionSegments, loft_geometry, sweep_geometry};
use sweep::{SketchSegment, segment_curve};
use topo::validate_geometric;

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

    lofted_solid_narration();
}

/// The frontier CLOSED (M6-3) and the retire-on-closure panic that
/// stood here fired as designed: `sweep::loft_body` assembles the
/// shape-(iii) solid and it validates at tier 3 — described NURBS
/// walls pass check 1, the iso seams certify, the patch flux feeds
/// +V. This narration builds and validates it live; the full render
/// Stop (SceneBody + registration next to `curvedcut`) is the
/// follow-up half of the retirement, tracked in the M6-3 PR.
fn lofted_solid_narration() {
    let quad = |pts: [(f64, f64); 4]| -> SectionSegments {
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
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    let lofted = sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("the shape (iii) loft assembles (M6-3)");
    validate_geometric(&lofted.body).expect("the loft body validates at tier 3");
    let m = topo::props::mass_properties(&lofted.body).expect("mass properties");
    println!(
        "   the frontier CLOSED (M6-3): the shape-(iii) loft BODY assembles and passes \
         tier 3 — {} faces ({} NURBS walls), volume {:.9} ± {:.2e} m^3 (derived exact: 9)",
        lofted.body.faces().count(),
        lofted.side_faces.iter().map(Vec::len).sum::<usize>(),
        m.volume,
        m.volume_pad
    );
}

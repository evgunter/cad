//! The loft and the sweep — the tour's DEFINITIONAL stop (M5 PR 10),
//! narration only.
//!
//! # Why narration and not a scene
//!
//! The §10.3 skin and the §10.4 sweep this PR ships are real geometry:
//! the walls exist, they are exact rational NURBS, and they pass
//! through their sections to rounding. What does NOT exist yet is a
//! B-rep SOLID built out of them — tier 3 refuses `Surface::Nurbs` by
//! kind and `EdgeCurve::certify` refuses NURBS carriers outright, so
//! there is no body to census, mesh, export, or render. The
//! certification flip is M5 PR 9's charter (its spec: "`EdgeCurve::
//! certify`'s Nurbs-carrier refusal FLIPS — this PR mints the kernel's
//! first rung-3 edges at rest"), and the curved-wall RENDER waits on
//! PR 11's trimmed-face tessellation regardless.
//!
//! So this stop narrates what is TRUE today, measures it, and pins the
//! frontier with a retire-on-closure panic — the `curvedcut` pattern.
//! Deliberately not a silent skip: a demo that quietly draws nothing is
//! how a frontier stops being visible.
//!
//! **PR 9/11's flip, in one place.** When a NURBS-walled solid builds,
//! [`narration`]'s retire panic fires with instructions: replace the
//! wall measurements with a real `Stop` carrying the lofted body, and
//! register it in `main`'s ladder next to `curvedcut`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use geom_surfaces::Surface;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::skin::{SectionSegments, lift_surface, loft_geometry, sweep_geometry};
use sweep::{Extrusion, SketchSegment, extrude, segment_curve};
use topo::{FaceSurface, validate_geometric};

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

    pin_frontier(&loft);
}

/// Retire-on-closure: the day a NURBS-walled face passes tier 3, this
/// panics with instructions. See the module docs.
fn pin_frontier(loft: &sweep::LoftGeometry) {
    let square = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(0.0, 1.0),
        ])],
    )
    .validate(Tolerance::get())
    .expect("the square validates");
    let built = extrude(&square, Extrusion::Distance(2.0)).expect("extrudes");
    let mut body = built.body;
    let wall = lift_surface::<f64>(&loft.walls[0][1]).expect("lifts");
    let face = built.side_faces[0][1];
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(wall.into())))
        .expect("the arena takes a real NURBS surface");
    match validate_geometric(&body) {
        Err(errors)
            if errors.iter().any(|e| {
                matches!(e, topo::ValidationError::UncertifiableSurface { face: f } if *f == face)
            }) =>
        {
            println!(
                "   frontier, pinned: tier 3 refuses a real NURBS wall by KIND \
                 (UncertifiableSurface). The certification flip is M5 PR 9's \
                 charter; the curved-wall RENDER is PR 11's."
            );
        }
        other => panic!(
            "the NURBS-wall frontier has CLOSED ({other:?}) — retire this narration: \
             build the lofted solid, make it a real Stop with a SceneBody, and \
             register it in main's ladder next to curvedcut"
        ),
    }
}

//! **A STEP placement is adopted at the file's band, its re-mint
//! reported, and one off orthogonal beyond that band at the session box
//! refuses, naming its transform** (`RigidMotion::adopt`,
//! `FileCoincidence::adoption_band`, `NormalizationKind::PlacementReMinted`).
//! A cylinder of radius 2.5 m, exported at the origin in a file that
//! declares a coincidence distance ε_in, is placed by an
//! `ITEM_DEFINED_TRANSFORMATION` turning it about `z` and standing it at
//! `(700, 700, 0)`, about 990 m out. The placement's axis direction is
//! off unit, and within ε_in the reader adopts it verbatim, so the map's
//! second and third columns carry the defect.
//!
//! The adoption band is ε_in, at least the run's ε and at most its
//! `K·ε`. A stretch of 0.4 of that band's zero at the box's extent is
//! beyond rounding and inside the band: the import re-mints the
//! placement, records the re-minted map, reports the re-mint with its
//! displacement bound, and the placed cylinder passes the shared gate,
//! whose check 1 reads its frame's unit-ness at the radius. A hundred
//! bands at the box's extent refuses `PlacementMap`, naming the
//! transform. A file declaring ε_in = 100 m is adopted at `K·ε`, so a
//! placement 4% off unit refuses rather than being snapped by metres.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::tolerance::SESSION_BOX_EXTENT;
use geom_core::{Arc2, Point2, RigidMotionError, Tol};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use step_import::{ImportOptions, NormalizationKind, StepImport, StepImportError, import_step};
use sweep::{ExtrudeSide, Extrusion, extrude};

const RADIUS: f64 = 2.5;

fn tol() -> Tol {
    Tol::witness()
}

/// The file's declared coincidence distance.
const EPS_IN: f64 = 1e-6;

/// The band a placement in a file declaring [`EPS_IN`] is adopted at.
fn band() -> geom_core::Band {
    geom_core::FileCoincidence::new(EPS_IN)
        .adoption_band(tol())
        .unwrap()
}

/// The cylinder at the origin, written as a STEP document declaring
/// [`EPS_IN`].
fn exported() -> String {
    exported_at(EPS_IN)
}

/// The cylinder at the origin, written as a STEP document declaring
/// `eps_in`.
fn exported_at(eps_in: f64) -> String {
    let rim = RawLoop::new([(
        Point2::new(RADIUS, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: RADIUS,
            sweep: TAU,
        }),
    )]);
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![rim])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: 1.0,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let options = step_export::StepOptions {
        uncertainty_m: Some(eps_in),
        ..step_export::StepOptions::default()
    };
    step_export::step_string(&body, &options, tol()).unwrap()
}

/// `text`'s representation placed into a fresh root by a turn of 0.3
/// rad about `z`, the axis direction `1 + stretch` long, standing at
/// `(700, 700, 0)`.
fn placed(text: &str, stretch: f64) -> String {
    let line = text
        .lines()
        .find(|l| l.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"))
        .unwrap();
    let rep = line.split_whitespace().next().unwrap();
    let inner = &line[line
        .find("((")
        .map_or_else(|| line.find("(#").unwrap(), |i| i + 1)..];
    let ids: Vec<&str> = inner
        .split(|c: char| !(c == '#' || c.is_ascii_digit()))
        .filter(|t| t.starts_with('#'))
        .collect();
    let (axis, ctx) = (ids[0], ids[ids.len() - 1]);
    let (s, c) = 0.3_f64.sin_cos();
    let z = 1.0 + stretch;
    let block = format!(
        "#9000 = SHAPE_REPRESENTATION('', ({axis}), {ctx});\n\
         #9001 = ( REPRESENTATION_RELATIONSHIP('','',{rep},#9000) \
         REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#9002) SHAPE_REPRESENTATION_RELATIONSHIP() );\n\
         #9002 = ITEM_DEFINED_TRANSFORMATION('','',{axis},#9003);\n\
         #9003 = AXIS2_PLACEMENT_3D('',#9004,#9005,#9006);\n\
         #9004 = CARTESIAN_POINT('',(700.0,700.0,0.0));\n\
         #9005 = DIRECTION('',(0.,0.,{z:.17e}));\n\
         #9006 = DIRECTION('',({c:.17e},{s:.17e},0.));\n"
    );
    let at = text.rfind("ENDSEC;").unwrap();
    format!("{}{block}{}", &text[..at], &text[at..])
}

#[test]
fn a_step_placement_inside_the_band_is_re_minted_reported_and_imports_its_cylinder() {
    let stretch = 0.4 * band().zero() / SESSION_BOX_EXTENT;
    let text = placed(&exported(), stretch);
    match import_step(&text, &ImportOptions::default(), tol()) {
        Ok(imported @ StepImport::Solid { .. }) => {
            let StepImport::Solid { body, .. } = &imported else {
                unreachable!()
            };
            let reach = body
                .points()
                .map(|(_, p)| (*p - geom_core::Point3::origin()).norm())
                .fold(0.0_f64, f64::max);
            assert!(
                (985.0..1000.0).contains(&reach),
                "the cylinder stands about 990 m out: {reach}"
            );
            let reported: Vec<_> = imported
                .normalizations()
                .iter()
                .filter_map(|n| match n.kind {
                    NormalizationKind::PlacementReMinted {
                        transform,
                        moved_by,
                    } => Some((transform, moved_by)),
                    _ => None,
                })
                .collect();
            assert_eq!(
                reported,
                vec![(
                    9002,
                    band().zero() * geom_core::RigidMotion::<f64>::RE_MINT_BANDS
                )],
                "the re-mint is reported once, with its displacement bound"
            );
            let applied = imported
                .instances()
                .iter()
                .find_map(|i| i.placement)
                .expect("the instance records the map applied");
            for (k, c) in applied.linear.cols().into_iter().enumerate() {
                let defect = (c.dot(c) - 1.0).abs();
                assert!(
                    defect <= 4.0 * f64::EPSILON,
                    "column {k} of the recorded map is {defect:e} off unit: it is not the re-mint"
                );
            }
        }
        Ok(StepImport::Wireframe { .. }) => panic!("expected a solid"),
        Err(e) => panic!("a placement {stretch:e} off unit refused: {e}"),
    }
}

#[test]
fn a_step_placement_beyond_the_band_refuses_naming_its_transform() {
    let stretch = 100.0 * band().zero() / SESSION_BOX_EXTENT;
    let text = placed(&exported(), stretch);
    match import_step(&text, &ImportOptions::default(), tol()) {
        Err(StepImportError::PlacementMap {
            transform: 9002,
            source: RigidMotionError::NotRigid { check },
        }) => assert_eq!(check, "transform_rigid_col1_unit"),
        other => panic!("a placement {stretch:e} off unit: {other:?}"),
    }
}

/// **A file's coincidence distance is no snap past the run's `K·ε`**: a
/// file declaring ε_in = 100 m would adopt a placement 4% off unit,
/// moving a point at the box's edge by metres; adopted at `K·ε` it
/// refuses.
#[test]
fn a_files_coincidence_distance_is_bounded_by_the_runs_k_eps() {
    let text = placed(&exported_at(100.0), 0.04);
    match import_step(&text, &ImportOptions::default(), tol()) {
        Err(StepImportError::PlacementMap {
            transform: 9002,
            source: RigidMotionError::NotRigid { .. },
        }) => {}
        other => panic!("a placement 4% off unit in a 100 m file: {other:?}"),
    }
}

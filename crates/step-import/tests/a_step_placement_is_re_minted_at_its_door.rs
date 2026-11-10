//! **A STEP placement is adopted at the file's band and re-minted at
//! the decided door, and one off orthogonal beyond that band at the
//! session box refuses there, naming its transform**
//! (`RigidMotion::check_rigid`, `FileCoincidence::adoption_band`). A
//! cylinder of radius 2.5 m, exported at the origin in a file that
//! declares a coincidence distance ε_in of 1e-6 m, is placed by an
//! `ITEM_DEFINED_TRANSFORMATION` turning it about `z` and standing it
//! at `(700, 700, 0)`, about 990 m out. The placement's axis direction
//! is off unit, and within ε_in the reader adopts it verbatim, so the
//! map's second and third columns carry the defect.
//!
//! A column 1e-10 off unit moves a point at the box's edge by 1e-7 m,
//! inside ε_in: the map is admitted as the rigid motion nearest it and
//! the placed cylinder imports, passing the shared gate, whose check 1
//! reads its frame's unit-ness at the radius. A column 0.4·ε_in off
//! unit would move a point at the box's edge by hundreds of ε_in, and
//! refuses `PlacementMap`, naming the transform.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::tolerance::SESSION_BOX_EXTENT;
use geom_core::{Arc2, Point2, RigidMotionError, Tol};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use step_import::{ImportOptions, StepImport, StepImportError, import_step};
use sweep::{ExtrudeSide, Extrusion, extrude};

const RADIUS: f64 = 2.5;

fn tol() -> Tol {
    Tol::witness()
}

/// The file's declared coincidence distance.
const EPS_IN: f64 = 1e-6;

/// The cylinder at the origin, written as a STEP document declaring
/// [`EPS_IN`].
fn exported() -> String {
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
        uncertainty_m: Some(EPS_IN),
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
fn a_step_placement_inside_the_levered_band_imports_its_cylinder() {
    let stretch = 1e-10;
    assert!(
        2.0 * stretch * SESSION_BOX_EXTENT < EPS_IN.max(tol().eps()),
        "the row's stretch is inside the file's band at the box's extent"
    );
    let text = placed(&exported(), stretch);
    match import_step(&text, &ImportOptions::default(), tol()) {
        Ok(StepImport::Solid { body, .. }) => {
            let reach = body
                .points()
                .map(|(_, p)| (*p - geom_core::Point3::origin()).norm())
                .fold(0.0_f64, f64::max);
            assert!(
                (985.0..1000.0).contains(&reach),
                "the cylinder stands about 990 m out: {reach}"
            );
        }
        Ok(StepImport::Wireframe { .. }) => panic!("expected a solid"),
        Err(e) => panic!("a placement {stretch:e} off unit refused: {e}"),
    }
}

#[test]
fn a_step_placement_beyond_the_levered_band_refuses_naming_its_transform() {
    let stretch = 0.4 * EPS_IN.max(tol().eps());
    let text = placed(&exported(), stretch);
    match import_step(&text, &ImportOptions::default(), tol()) {
        Err(StepImportError::PlacementMap {
            transform: 9002,
            source: RigidMotionError::NotRigid { check },
        }) => assert_eq!(check, "transform_rigid_col1_unit"),
        other => panic!("a placement {stretch:e} off unit: {other:?}"),
    }
}

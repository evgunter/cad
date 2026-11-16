//! **A STEP placement is built by its axes, and a move beyond rounding
//! is reported** (ISO 10303-42's `build_axes` as
//! `RigidMotion::from_axis_and_reference`,
//! `NormalizationKind::PlacementReMinted`). A cylinder of radius 2.5 m,
//! exported at the origin in a file that declares a coincidence distance
//! ε_in, is placed by an `ITEM_DEFINED_TRANSFORMATION` turning it about
//! `z` and standing it at `(700, 700, 0)`, about 990 m out. The
//! placement's axis direction is off unit, and within ε_in the reader
//! adopts it verbatim, so the frame as stated carries the defect.
//!
//! Every placement frame is minted from its fields by `build_axes` (the
//! axis normalized, the reference projected and normalized), so the map
//! the import records and places by is rigid by construction whatever
//! the fields say. Where the stated frame is rigid within its own
//! rounding nothing is reported; where it is not, the move `build_axes`
//! made is reported with the farthest a point of the session box moved.
//! A file declaring ε_in = 100 m adopts an axis 4% off unit verbatim,
//! and the metres-long move its build makes is reported, not hidden
//! (`work/onehome/a-step-files-input-tolerance-has-no-bound-the-ratified-text-states.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::tolerance::SESSION_BOX_EXTENT;
use geom_core::{Arc2, Point2, Tol};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};
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

/// The import of `text`, a solid, and its placement reports
/// ([`NormalizationKind::PlacementReMinted`]) as `(transform, moved_by)`.
fn imported(text: &str) -> (StepImport, Vec<(u64, f64)>) {
    let imported = match import_step(text, &ImportOptions::default(), tol()) {
        Ok(imported @ StepImport::Solid { .. }) => imported,
        Ok(StepImport::Wireframe { .. }) => panic!("expected a solid"),
        Err(e) => panic!("the placed cylinder refused: {e}"),
    };
    let reported = imported
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
    (imported, reported)
}

#[test]
fn a_placement_beyond_rounding_is_built_by_its_axes_reported_and_imports_its_cylinder() {
    let stretch = 0.4 * EPS_IN;
    let (imported, reported) = imported(&placed(&exported(), stretch));
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
    // The stated frame's second and third axes are `1 + stretch` long;
    // built, they are unit. A corner of the box moves by about the
    // stretch times its distance from the frame's axis line and along it.
    let [(transform, moved_by)] = reported.as_slice() else {
        panic!("one report for the one placement: {reported:?}");
    };
    assert_eq!(*transform, 9002);
    let e = SESSION_BOX_EXTENT;
    assert!(
        (0.5 * stretch * e..=4.0 * stretch * e).contains(moved_by),
        "the move is the stretch at the box's corners: {moved_by:e} for {stretch:e}"
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
            "column {k} of the recorded map is {defect:e} off unit: it is not the built frame"
        );
    }
}

#[test]
fn a_placement_within_rounding_reports_nothing() {
    let (_, reported) = imported(&placed(&exported(), 0.0));
    assert_eq!(
        reported,
        vec![],
        "a frame rigid within its rounding moves nothing worth a report"
    );
}

/// **A file's coincidence distance bounds no move a build makes**: a
/// file declaring ε_in = 100 m adopts an axis 4% off unit verbatim, and
/// `build_axes` moves the box's corners by tens of metres — reported,
/// so the move is in the import's own record.
#[test]
fn a_files_slack_axis_is_built_and_its_metres_long_move_reported() {
    let (_, reported) = imported(&placed(&exported_at(100.0), 0.04));
    let [(9002, moved_by)] = reported.as_slice() else {
        panic!("one report naming the transform: {reported:?}");
    };
    assert!(
        *moved_by >= 0.02 * SESSION_BOX_EXTENT,
        "the move is reported at its size: {moved_by}"
    );
}

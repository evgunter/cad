//! Review probes for PR 6393.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::{Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{ExtrudeError, ExtrudeSide, Extrusion, Revolution, RevolveAxis, RevolveError};

#[test]
fn review_6393_sweep_doors() {
    let tol = Tol::witness();
    let none: Vec<ProfileLoop<f64>> = Vec::new();
    let path = NurbsCurve3::<f64>::interpolate(
        &[
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.5, 2.0),
            Point3::new(0.0, 1.0, 3.0),
        ],
        3,
    )
    .unwrap();
    let r = sweep::sweep_body::<f64>(&none, SketchPlane::xy(), &path, 5, 3, tol);
    eprintln!(
        "PROBE sweep_body: {:?}",
        r.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );
    let g = sweep::skin::sweep_geometry(&none, SketchPlane::xy(), &path, 5, 3, tol);
    eprintln!(
        "PROBE sweep_geometry: {:?}",
        g.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );
    let sections: Vec<sweep::Section> = vec![vec![], vec![]];
    let places = [
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
    ];
    let g = sweep::skin::loft_geometry(&sections, &places, 1, &[0.0, 1.0], tol);
    eprintln!(
        "PROBE loft_geometry: {:?}",
        g.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );
    let g = sweep::skin::loft_parameters(&sections, &places, 1, tol);
    eprintln!(
        "PROBE loft_parameters: {:?}",
        g.as_ref().map(|_| ()).map_err(|e| e.to_string())
    );

    // Vector extrusion and a partial revolve.
    let empty = Profile::<f64>::new(SketchPlane::xy(), vec![])
        .validate(tol)
        .unwrap();
    let e = sweep::extrude(&empty, Extrusion::Vector(Vec3::new(0.0, 0.0, 1.0)), tol).map(|_| ());
    assert_eq!(e, Err(ExtrudeError::EmptyProfile));
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let rv = sweep::revolve(&empty, axis, Revolution::Partial(1.0), tol).map(|_| ());
    eprintln!("PROBE partial revolve: {rv:?}");
    // A degenerate axis and an empty profile: which refuses first?
    let bad = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 0.0),
    };
    let rv = sweep::revolve(&empty, bad, Revolution::Full, tol).map(|_| ());
    eprintln!("PROBE degenerate axis over empty: {rv:?}");
    let e = sweep::extrude(&empty, Extrusion::Vector(Vec3::new(1.0, 0.0, 0.0)), tol).map(|_| ());
    eprintln!("PROBE in-plane vector over empty: {e:?}");

    // Interval lane.
    let ei = Profile::<Interval>::new(SketchPlane::xy(), vec![])
        .validate(tol)
        .unwrap();
    let e = sweep::extrude(
        &ei,
        Extrusion::Distance {
            depth: Interval::from_f64(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|_| ());
    eprintln!("PROBE interval extrude: {e:?}");
    let rv = sweep::revolve(
        &ei,
        RevolveAxis {
            origin: Point2::new(Interval::from_f64(0.0), Interval::from_f64(0.0)),
            dir: geom_core::Vec2::new(Interval::from_f64(0.0), Interval::from_f64(1.0)),
        },
        Revolution::Full,
        tol,
    )
    .map(|_| ());
    assert_eq!(
        rv.map_err(|e| matches!(e, RevolveError::EmptyProfile)),
        Err(true)
    );
    let lofted = sweep::loft_profiles::<Interval>(
        &[
            ei.clone(),
            ei.clone().on_plane(SketchPlane::xy_at(Point3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(1.0),
            ))),
        ],
        1,
        tol,
    );
    eprintln!(
        "PROBE interval loft_profiles: {:?}",
        lofted.map(|_| ()).map_err(|e| e.to_string())
    );
}

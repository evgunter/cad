//! Review probe for PR 4815: a piece whose seam the split now stores as
//! a chart image, through STEP export and import. Scratch, not for
//! merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn tol() -> Tol {
    Tol::witness()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).expect("finished")
}

fn shoulder(h: f64) -> topo::AtRestBody<f64> {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let outline = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.0, 2.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.0), bulge),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outline])
        .validate(tol())
        .unwrap();
    at_rest(
        extrude(
            &profile,
            Extrusion::Distance {
                depth: h,
                side: ExtrudeSide::Along,
            },
            tol(),
        )
        .unwrap()
        .body,
    )
}

fn cut(body: &topo::AtRestBody<f64>, at: Point3<f64>, n: Vec3<f64>) -> [topo::Body<f64>; 2] {
    let r = topo::split(body, &topo::test_support::split_plane(at, n, tol()), tol()).unwrap();
    [
        r.above.body().unwrap().clone(),
        r.below.body().unwrap().clone(),
    ]
}

fn trip(what: &str, body: &topo::Body<f64>) -> String {
    let seams: Vec<String> = body
        .edges()
        .filter_map(|(_, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match c.description() {
                geom_brep::EdgeDescription::TangentIntersection { .. } => Some("Tangent".into()),
                geom_brep::EdgeDescription::Chart(ch) => {
                    let face = body.face_of_half_edge(e.he_plus).unwrap();
                    let fs = body.get_face(face).unwrap().surface;
                    let fm = body.face_of_half_edge(e.he_minus).unwrap();
                    let fms = body.get_face(fm).unwrap().surface;
                    (fs != fms).then(|| {
                        format!("Chart({:?})", body.get_surface(ch.surface).unwrap().kind())
                    })
                }
                _ => None,
            }
        })
        .collect();
    let text = match step_export::step_string(body, &step_export::StepOptions::default(), tol()) {
        Ok(t) => t,
        Err(e) => return format!("{what}: seams {seams:?}: EXPORT refused {e:?}"),
    };
    match import_step(&text, &ImportOptions::default(), tol()) {
        Ok(StepImport::Solid { body: back, .. }) => {
            let v0 = topo::mass_properties(body, tol()).unwrap().volume;
            let v1 = topo::mass_properties(&back, tol()).map(|p| p.volume);
            format!("{what}: seams {seams:?}: imports, volume {v0:e} -> {v1:?}")
        }
        Ok(other) => format!("{what}: seams {seams:?}: imports as {other:?}"),
        Err(e) => {
            let s = format!("{e:?}");
            format!(
                "{what}: seams {seams:?}: IMPORT refused {}",
                &s[..s.len().min(600)]
            )
        }
    }
}

#[test]
fn probe_step_round_trip_of_the_pieces() {
    let eps = tol().eps();
    let unit = (2.0 * eps).sqrt();
    let wall = at_rest(
        topo::split(
            &shoulder(1.0),
            &topo::test_support::split_plane(
                Point3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                tol(),
            ),
            tol(),
        )
        .unwrap()
        .above
        .body()
        .unwrap()
        .clone(),
    );
    let mut lines = vec![trip("the wall piece (parent)", &wall)];
    for c in [0.25, 0.99, 4.0] {
        let d = c * unit;
        let [above, below] = cut(&wall, Point3::new(0.0, 0.0, d), Vec3::new(0.0, 0.0, 1.0));
        lines.push(trip(&format!("c={c} long piece"), &above));
        lines.push(trip(&format!("c={c} short piece"), &below));
    }
    // The filed gap's own native case, for comparison: the shoulder cut
    // at its seam with a depth under √(2ε) stores a chart image.
    let thin = shoulder(0.25 * unit);
    let [a, b] = cut(&thin, Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
    lines.push(trip("native thin shoulder, wall side (filed gap)", &a));
    lines.push(trip("native thin shoulder, other side", &b));
    for l in &lines {
        eprintln!("{l}");
    }
}

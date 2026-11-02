//! Review probes for the chart-singularity pass (PR 6007): faces the
//! listed fixtures do not reach. A sphere face holding BOTH poles (one
//! bounding circle that does not separate them), a cone cap on each
//! nappe, and every one of them with its curved face's `same_sense`
//! flipped. Each file either imports to the closed-form body (tier 3,
//! volume, every singular point a vertex of two edges) or refuses
//! typed; none ships a wrong body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point3, Tol};
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// A two-face solid: a curved face (`surface`, entities #25..#29 stated
/// by the caller) bounded by ONE closed circle, and the plane disc that
/// circle bounds. The circle is centred at `c`, its axis `n` and
/// reference `x`, radius `r`; the curved face uses it forward, the
/// plane (normal −`n`) reversed.
fn two_face(surface: &str, sense: bool, c: [f64; 3], n: [f64; 3], x: [f64; 3], r: f64) -> String {
    let v = [c[0] + r * x[0], c[1] + r * x[1], c[2] + r * x[2]];
    let f = |p: [f64; 3]| format!("({:?},{:?},{:?})", p[0], p[1], p[2]);
    let s = if sense { ".T." } else { ".F." };
    let m = [-n[0], -n[1], -n[2]];
    format!(
        "ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('review probe'),'2;1');
FILE_NAME('probe','2026-10-29T00:00:00',('r'),('r'),'r','r','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));
ENDSEC;
DATA;
#1 = APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2);
#2 = APPLICATION_CONTEXT('core data for automotive mechanical design processes');
#3 = SHAPE_DEFINITION_REPRESENTATION(#4,#10);
#4 = PRODUCT_DEFINITION_SHAPE('','',#5);
#5 = PRODUCT_DEFINITION('design','',#6,#9);
#6 = PRODUCT_DEFINITION_FORMATION('','',#7);
#7 = PRODUCT('probe','probe','',(#8));
#8 = PRODUCT_CONTEXT('',#2,'mechanical');
#9 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');
#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#14,#15),#47);
#11 = CARTESIAN_POINT('',(0.,0.,0.));
#12 = DIRECTION('',(0.,0.,1.));
#13 = DIRECTION('',(1.,0.,0.));
#14 = AXIS2_PLACEMENT_3D('',#11,#12,#13);
#15 = MANIFOLD_SOLID_BREP('',#16);
#16 = CLOSED_SHELL('',(#33,#42));
#17 = CARTESIAN_POINT('',{v});
#18 = VERTEX_POINT('',#17);
#19 = CARTESIAN_POINT('',{c});
#20 = DIRECTION('',{n});
#21 = DIRECTION('',{x});
#22 = AXIS2_PLACEMENT_3D('',#19,#20,#21);
#23 = CIRCLE('',#22,{r:?});
#24 = EDGE_CURVE('',#18,#18,#23,.T.);
{surface}
#30 = ORIENTED_EDGE('',*,*,#24,.T.);
#31 = EDGE_LOOP('',(#30));
#32 = FACE_OUTER_BOUND('',#31,.T.);
#33 = ADVANCED_FACE('',(#32),#29,{s});
#34 = CARTESIAN_POINT('',{c});
#35 = DIRECTION('',{m});
#36 = DIRECTION('',{x});
#37 = AXIS2_PLACEMENT_3D('',#34,#35,#36);
#38 = PLANE('',#37);
#39 = ORIENTED_EDGE('',*,*,#24,.F.);
#40 = EDGE_LOOP('',(#39));
#41 = FACE_OUTER_BOUND('',#40,.T.);
#42 = ADVANCED_FACE('',(#41),#38,.T.);
#43 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );
#44 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );
#45 = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );
#46 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#43,'distance_accuracy_value','confusion accuracy');
#47 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#46)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#43,#44,#45)) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') );
ENDSEC;
END-ISO-10303-21;
",
        v = f(v),
        c = f(c),
        n = f(n),
        x = f(x),
        m = f(m),
    )
}

const SPHERE: &str = "#25 = CARTESIAN_POINT('',(0.,0.,0.));
#26 = DIRECTION('',(0.,0.,1.));
#27 = DIRECTION('',(1.,0.,0.));
#28 = AXIS2_PLACEMENT_3D('',#25,#26,#27);
#29 = SPHERICAL_SURFACE('',#28,10.);";

/// A cone through the origin's circle of radius 5 at half-angle 0.5,
/// opening along `axis`.
fn cone(axis: [f64; 3]) -> String {
    format!(
        "#25 = CARTESIAN_POINT('',(0.,0.,0.));
#26 = DIRECTION('',({:?},{:?},{:?}));
#27 = DIRECTION('',(1.,0.,0.));
#28 = AXIS2_PLACEMENT_3D('',#25,#26,#27);
#29 = CONICAL_SURFACE('',#28,5.,0.5);",
        axis[0], axis[1], axis[2]
    )
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// What one file did: `Ok(volume)` for a body that passed every check,
/// `Err(text)` for a typed refusal.
fn outcome(name: &str, text: &str) -> Result<(f64, usize), String> {
    let tol = Tol::witness();
    let imported = import_step(text, &ImportOptions::default(), tol).map_err(|e| format!("{e}"))?;
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = imported
    else {
        panic!("{name} imports as a solid");
    };
    let singular_records = normalizations
        .iter()
        .filter(|n| matches!(n.kind, NormalizationKind::ChartSingularityVertex { .. }))
        .count();
    let b = body.body();
    assert_eq!(topo::validate_geometric(b, tol), Ok(()), "{name}: tier 3");
    let volume = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", b.clone(), tol),
        tol,
    )
    .volume;
    // Every singular point of every curved face lying on the body is a
    // vertex with two edges at it; one lying in a face's closure with
    // no vertex is the defect the pass exists to remove.
    let points: Vec<_> = b.vertex_points().collect();
    for (_, f) in b.faces() {
        let s: Vec<Point3<f64>> = match *b.get_surface(f.surface).unwrap() {
            Surface::Sphere {
                center,
                radius,
                axis,
                ..
            } => vec![center + axis * radius, center - axis * radius],
            Surface::Cone { apex, .. } => vec![apex],
            _ => continue,
        };
        for s in s {
            if let Some((v, _)) = points.iter().find(|(_, p)| (*p - s).norm() <= 1e-9) {
                let ends = b
                    .edges()
                    .flat_map(|(_, e)| [e.he_plus, e.he_minus])
                    .filter(|h| b.get_half_edge(*h).unwrap().start == *v)
                    .count();
                assert_eq!(ends, 2, "{name}: singular point {s:?} ends {ends} edges");
            }
        }
    }
    Ok((volume, singular_records))
}

fn ball_less_cap(c: f64) -> f64 {
    let r = 0.010;
    4.0 * PI * r * r * r / 3.0 - cap(r, r - c)
}

/// A ball less the cap x > 6 mm: ONE sphere face holding both poles,
/// bounded by one circle whose vertex sits on the meridian through
/// u_ref. Two iterations: the north pole interior, then the south pole
/// reached by the north's second meridian and completed as a strut tip.
#[test]
fn a_sphere_face_holding_both_poles_imports_as_the_ball_less_its_cap() {
    let text = two_face(SPHERE, true, [6., 0., 0.], [-1., 0., 0.], [0., 0., 1.], 8.);
    let (volume, records) = outcome("both poles", &text).unwrap_or_else(|e| panic!("{e}"));
    let want = ball_less_cap(0.006);
    assert!(
        (volume - want).abs() <= 1e-9 * want,
        "volume {volume} vs {want}"
    );
    assert_eq!(records, 2, "both poles re-minted");
}

/// The same ball, the circle's vertex placed at the equator: the
/// meridian through it is TANGENT to the hole there, so the cut meets
/// the vertex from both poles.
#[test]
fn a_sphere_face_holding_both_poles_with_its_vertex_at_the_equator() {
    let text = two_face(SPHERE, true, [6., 0., 0.], [-1., 0., 0.], [0., 1., 0.], 8.);
    match outcome("both poles, vertex at equator", &text) {
        Ok((volume, records)) => {
            let want = ball_less_cap(0.006);
            assert!(
                (volume - want).abs() <= 1e-9 * want,
                "volume {volume} vs {want}"
            );
            assert_eq!(records, 2, "both poles re-minted");
        }
        Err(e) => panic!("a valid ball less a cap refuses: {e}"),
    }
}

/// An oblique cut that misses both poles, the vertex off every axis
/// plane.
#[test]
fn a_sphere_face_holding_both_poles_cut_obliquely() {
    let k = 1.0 / 3.0_f64.sqrt();
    // A plane at distance 6 along (1,1,1)/√3; reference ⟂ to it.
    let (n, c) = ([-k, -k, -k], [6. * k, 6. * k, 6. * k]);
    let s = 1.0 / 2.0_f64.sqrt();
    let x = [s, -s, 0.];
    let text = two_face(SPHERE, true, c, n, x, 8.);
    let (volume, records) = outcome("oblique", &text).unwrap_or_else(|e| panic!("{e}"));
    let want = ball_less_cap(0.006);
    assert!(
        (volume - want).abs() <= 1e-9 * want,
        "volume {volume} vs {want}"
    );
    assert_eq!(records, 2, "both poles re-minted");
}

/// Every probe with its curved face's `same_sense` flipped: an
/// inside-out file. It may refuse typed, or import a body that measures
/// the closed form; it must not ship a different body.
#[test]
fn a_flipped_sense_never_ships_a_wrong_body() {
    let k = 1.0 / 3.0_f64.sqrt();
    let s = 1.0 / 2.0_f64.sqrt();
    let h = 5.0 / 0.5_f64.tan() / 1000.0;
    let cone_v = PI * 0.005 * 0.005 * h / 3.0;
    let r = 0.010;
    let rows: Vec<(&str, String, f64)> = vec![
        (
            "both poles",
            two_face(SPHERE, false, [6., 0., 0.], [-1., 0., 0.], [0., 0., 1.], 8.),
            ball_less_cap(0.006),
        ),
        (
            "oblique",
            two_face(
                SPHERE,
                false,
                [6. * k, 6. * k, 6. * k],
                [-k, -k, -k],
                [s, -s, 0.],
                8.,
            ),
            ball_less_cap(0.006),
        ),
        // The north cap above z = 6 and its tilted twin.
        (
            "north cap",
            two_face(SPHERE, false, [0., 0., 6.], [0., 0., 1.], [1., 0., 0.], 8.),
            cap(r, 0.004),
        ),
        (
            "south cap",
            two_face(
                SPHERE,
                false,
                [0., 0., -6.],
                [0., 0., -1.],
                [1., 0., 0.],
                8.,
            ),
            cap(r, 0.004),
        ),
        // Cone caps: opening up (apex below), opening down (apex
        // above), and the far nappe (the face beyond the apex).
        (
            "cone up",
            two_face(
                &cone([0., 0., 1.]),
                false,
                [0., 0., 0.],
                [0., 0., -1.],
                [1., 0., 0.],
                5.,
            ),
            cone_v,
        ),
        (
            "cone down",
            two_face(
                &cone([0., 0., -1.]),
                false,
                [0., 0., 0.],
                [0., 0., 1.],
                [1., 0., 0.],
                5.,
            ),
            cone_v,
        ),
        (
            "cone far nappe",
            two_face(
                &cone([0., 0., 1.]),
                false,
                [0., 0., -2.0 * h * 1000.0],
                [0., 0., 1.],
                [1., 0., 0.],
                5.,
            ),
            cone_v,
        ),
    ];
    let mut report = Vec::new();
    for (name, text, want) in &rows {
        // The unflipped file first: it must import to the closed form.
        let right = text.replace("#29,.F.);", "#29,.T.);");
        let ok = outcome(name, &right);
        report.push(format!("{name} as stated right: {ok:?} (want {want})"));
        if let Ok((v, _)) = ok {
            assert!(
                (v - want).abs() <= 1e-9 * want,
                "{name}: right-side-out volume {v} vs {want}"
            );
        }
        let flipped = outcome(name, text);
        report.push(format!("{name} flipped: {flipped:?}"));
        if let Ok((v, _)) = flipped {
            assert!(
                (v - want).abs() <= 1e-9 * want,
                "{name}: a flipped file shipped a body of volume {v}, the closed form {want}"
            );
        }
    }
    eprintln!("{}", report.join("\n"));
}

/// Open CASCADE's full cone cut by an OBLIQUE plane: the lateral face
/// one loop, a seam generator from the apex (a valence-1 vertex) to the
/// rim and back, the rim an ellipse whose major axis lies in the seam's
/// meridian plane. Answers the file and its closed-form volume (m³).
fn oblique_cone(slope: f64) -> (String, f64) {
    let (t, m) = (0.5_f64.tan(), slope);
    let h = 5.0 / t;
    let x1 = h * t / (1.0 - m * t);
    let x2 = -h * t / (1.0 + m * t);
    let p1 = [x1, 0.0, m * x1];
    let xc = 0.5 * (x1 + x2);
    let c = [xc, 0.0, m * xc];
    let a = 0.5 * ((x1 - x2) * (x1 - x2) * (1.0 + m * m)).sqrt();
    let b = (((m * xc + h) * t).powi(2) - xc * xc).sqrt();
    let q = (1.0 + m * m).sqrt();
    let n = [-m / q, 0.0, 1.0 / q];
    let rf = [1.0 / q, 0.0, m / q];
    let apex = [0.0, 0.0, -h];
    let len = ((p1[0] - apex[0]).powi(2) + (p1[2] - apex[2]).powi(2)).sqrt();
    let sd = [(p1[0] - apex[0]) / len, 0.0, (p1[2] - apex[2]) / len];
    let f = |p: [f64; 3]| format!("({:?},{:?},{:?})", p[0], p[1], p[2]);
    let text = format!(
        "ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('review probe'),'2;1');
FILE_NAME('probe','2026-10-29T00:00:00',('r'),('r'),'r','r','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));
ENDSEC;
DATA;
#1 = APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2);
#2 = APPLICATION_CONTEXT('core data for automotive mechanical design processes');
#3 = SHAPE_DEFINITION_REPRESENTATION(#4,#10);
#4 = PRODUCT_DEFINITION_SHAPE('','',#5);
#5 = PRODUCT_DEFINITION('design','',#6,#9);
#6 = PRODUCT_DEFINITION_FORMATION('','',#7);
#7 = PRODUCT('probe','probe','',(#8));
#8 = PRODUCT_CONTEXT('',#2,'mechanical');
#9 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');
#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#14,#15),#60);
#11 = CARTESIAN_POINT('',(0.,0.,0.));
#12 = DIRECTION('',(0.,0.,1.));
#13 = DIRECTION('',(1.,0.,0.));
#14 = AXIS2_PLACEMENT_3D('',#11,#12,#13);
#15 = MANIFOLD_SOLID_BREP('',#16);
#16 = CLOSED_SHELL('',(#42,#51));
#17 = CARTESIAN_POINT('',{apex});
#18 = VERTEX_POINT('',#17);
#19 = CARTESIAN_POINT('',{p1});
#20 = VERTEX_POINT('',#19);
#21 = CARTESIAN_POINT('',{apex});
#22 = DIRECTION('',{sd});
#23 = VECTOR('',#22,1.);
#24 = LINE('',#21,#23);
#25 = EDGE_CURVE('',#18,#20,#24,.T.);
#26 = CARTESIAN_POINT('',{c});
#27 = DIRECTION('',{n});
#28 = DIRECTION('',{rf});
#29 = AXIS2_PLACEMENT_3D('',#26,#27,#28);
#30 = ELLIPSE('',#29,{a:?},{b:?});
#31 = EDGE_CURVE('',#20,#20,#30,.T.);
#32 = CARTESIAN_POINT('',(0.,0.,0.));
#33 = DIRECTION('',(0.,0.,1.));
#34 = DIRECTION('',(1.,0.,0.));
#35 = AXIS2_PLACEMENT_3D('',#32,#33,#34);
#36 = CONICAL_SURFACE('',#35,5.,0.5);
#37 = ORIENTED_EDGE('',*,*,#25,.T.);
#38 = ORIENTED_EDGE('',*,*,#31,.F.);
#39 = ORIENTED_EDGE('',*,*,#25,.F.);
#40 = EDGE_LOOP('',(#37,#38,#39));
#41 = FACE_OUTER_BOUND('',#40,.T.);
#42 = ADVANCED_FACE('',(#41),#36,.T.);
#43 = CARTESIAN_POINT('',{c});
#44 = DIRECTION('',{n});
#45 = DIRECTION('',{rf});
#46 = AXIS2_PLACEMENT_3D('',#43,#44,#45);
#47 = PLANE('',#46);
#48 = ORIENTED_EDGE('',*,*,#31,.T.);
#49 = EDGE_LOOP('',(#48));
#50 = FACE_OUTER_BOUND('',#49,.T.);
#51 = ADVANCED_FACE('',(#50),#47,.T.);
#56 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );
#57 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );
#58 = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );
#59 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#56,'distance_accuracy_value','confusion accuracy');
#60 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#59)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#56,#57,#58)) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') );
ENDSEC;
END-ISO-10303-21;
",
        apex = f(apex),
        p1 = f(p1),
        sd = f(sd),
        c = f(c),
        n = f(n),
        rf = f(rf),
    );
    (text, PI * a * b * (h / q) / 3.0 * 1e-9)
}

/// The oblique cone's rim is an ellipse, a carrier the pass reads no
/// crossing of: it refuses typed, naming that (a spec-listed refusal;
/// main refuses the same files at its LINE-edge gate). Read at a
/// coaxial cut (slope 0, an ellipse with equal axes) and an oblique one.
#[test]
fn an_obliquely_cut_open_cascade_cone_refuses_typed() {
    for slope in [0.0, 0.2] {
        let (text, _) = oblique_cone(slope);
        match outcome("oblique cone", &text) {
            Err(e) => assert!(e.contains("reads no crossing of"), "slope {slope}: {e}"),
            Ok(got) => panic!("slope {slope}: imported {got:?}"),
        }
    }
}

/// A north cap whose rim vertex sits `delta` metres round the rim from
/// where the u_ref meridian lands: the landing splits the rim `delta`
/// from the vertex. Reports what ships (the shortest edge) per delta.
#[test]
fn a_meridian_landing_beside_a_rim_vertex() {
    let tol = Tol::witness();
    let mut report = Vec::new();
    for delta in [0.5e-9_f64, 1.5e-9, 3e-9, 9e-9, 2e-8] {
        let phi = delta / 0.008;
        let text = two_face(
            SPHERE,
            true,
            [0., 0., 6.],
            [0., 0., 1.],
            [phi.cos(), phi.sin(), 0.],
            8.,
        );
        match import_step(&text, &ImportOptions::default(), tol) {
            Ok(StepImport::Solid { body, .. }) => {
                let b = body.body();
                let pts: std::collections::BTreeMap<_, _> = b.vertex_points().collect();
                let shortest = b
                    .edges()
                    .map(|(_, e)| {
                        let h = b.get_half_edge(e.he_plus).unwrap();
                        let t = b.get_half_edge(e.he_minus).unwrap();
                        let (p, q) = (pts[&h.start], pts[&t.start]);
                        (p - q).norm()
                    })
                    .filter(|l| *l > 0.0)
                    .fold(f64::INFINITY, f64::min);
                report.push(format!(
                    "delta {delta:e}: ships {} edges, shortest chord {shortest:e} (eps {:e})",
                    b.edges().count(),
                    tol.eps()
                ));
                assert!(
                    shortest >= tol.eps(),
                    "an edge shorter than ε ships: {shortest:e}"
                );
            }
            Ok(_) => panic!("a solid"),
            Err(e) => {
                let text = format!("{e}");
                report.push(format!("delta {delta:e}: {text}"));
                // The file states entities #1..#47; a refusal naming a
                // higher id names topology the import minted, which the
                // user cannot find in the file.
                let named: Vec<u64> = text
                    .split('#')
                    .skip(1)
                    .filter_map(|t| t.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok())
                    .collect();
                assert!(
                    named.iter().all(|id| *id <= 47),
                    "delta {delta:e}: a refusal of a valid cap names a minted entity: {text}"
                );
            }
        }
    }
    eprintln!("{}", report.join("\n"));
    // Pinned at the default band, (1e-9, 1e-8): within its zero the
    // landing is the rim vertex, past its escalation the rim is split
    // there (and the vertex, left with one carrier through it, joined),
    // and inside it the landing is undecided and refuses, naming the
    // face the file states.
    if (0.99e-9..=1.01e-9).contains(&tol.eps()) {
        let lines: Vec<&str> = report.iter().map(String::as_str).collect();
        for (i, want) in [
            "delta 5e-10: ships 4 edges",
            "delta 1.5e-9: step import: face #33: ",
            "delta 3e-9: step import: face #33: ",
            "delta 9e-9: step import: face #33: ",
            "delta 2e-8: ships 4 edges",
        ]
        .into_iter()
        .enumerate()
        {
            assert!(lines[i].starts_with(want), "row {i}: {}", lines[i]);
        }
        for line in &lines[1..4] {
            assert!(
                line.contains("is undecided: margin")
                    && line.contains("tighten the tolerance below"),
                "{line}"
            );
        }
    }
}

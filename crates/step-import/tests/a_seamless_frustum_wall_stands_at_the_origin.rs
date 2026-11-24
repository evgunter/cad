//! **A frustum wall stated as a STEP file states it, with no seam**: a
//! conical face bounded by two ring loops, each one closed circle edge
//! winding the axis, and the two discs they bound. Its cone's apex is
//! wherever the taper puts it, and a slender wall's apex is past the
//! session box. The face lies between its two rings on one nappe, so it
//! does not hold its apex (`topo::cover`'s cone arm reads that, and
//! `topo`'s `a_seamless_frustum_band_does_not_hold_its_apex` pins it on a
//! body Euler operators build).
//!
//! The import door recognizes a seamless band on a cylinder or a torus
//! only, and re-mints it seamed. On a cone it refuses the face typed
//! before any body exists, so no imported part reaches the box with this
//! face today.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// The seamless frustum, millimetres: rings of radius `r0` at `z = 0`
/// and `r1` at `z = h`, the cone between them, the discs closing it.
fn seamless_frustum(r0: f64, r1: f64, h: f64) -> String {
    let semi = ((r0 - r1) / h).atan();
    let f = |x: f64| format!("{x:?}");
    format!(
        "ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('a seamless frustum'),'2;1');
FILE_NAME('frustum','2026-11-21T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));
ENDSEC;
DATA;
#1 = APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2);
#2 = APPLICATION_CONTEXT('core data for automotive mechanical design processes');
#3 = SHAPE_DEFINITION_REPRESENTATION(#4,#10);
#4 = PRODUCT_DEFINITION_SHAPE('','',#5);
#5 = PRODUCT_DEFINITION('design','',#6,#9);
#6 = PRODUCT_DEFINITION_FORMATION('','',#7);
#7 = PRODUCT('frustum','frustum','',(#8));
#8 = PRODUCT_CONTEXT('',#2,'mechanical');
#9 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');
#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#14,#15),#47);
#11 = CARTESIAN_POINT('',(0.,0.,0.));
#12 = DIRECTION('',(0.,0.,1.));
#13 = DIRECTION('',(1.,0.,0.));
#14 = AXIS2_PLACEMENT_3D('',#11,#12,#13);
#15 = MANIFOLD_SOLID_BREP('',#16);
#16 = CLOSED_SHELL('',(#30,#40,#46));
#17 = CARTESIAN_POINT('',({r0},0.,0.));
#18 = VERTEX_POINT('',#17);
#19 = CIRCLE('',#14,{r0});
#20 = EDGE_CURVE('',#18,#18,#19,.T.);
#21 = CARTESIAN_POINT('',({r1},0.,{h}));
#22 = VERTEX_POINT('',#21);
#23 = CARTESIAN_POINT('',(0.,0.,{h}));
#24 = AXIS2_PLACEMENT_3D('',#23,#12,#13);
#25 = CIRCLE('',#24,{r1});
#26 = EDGE_CURVE('',#22,#22,#25,.T.);
#27 = DIRECTION('',(0.,0.,-1.));
#28 = AXIS2_PLACEMENT_3D('',#11,#27,#13);
#29 = CONICAL_SURFACE('',#28,{r0},{semi});
#30 = ADVANCED_FACE('',(#33,#36),#29,.T.);
#31 = ORIENTED_EDGE('',*,*,#20,.T.);
#32 = EDGE_LOOP('',(#31));
#33 = FACE_OUTER_BOUND('',#32,.T.);
#34 = ORIENTED_EDGE('',*,*,#26,.F.);
#35 = EDGE_LOOP('',(#34));
#36 = FACE_BOUND('',#35,.T.);
#37 = ORIENTED_EDGE('',*,*,#20,.F.);
#38 = EDGE_LOOP('',(#37));
#39 = FACE_OUTER_BOUND('',#38,.T.);
#40 = ADVANCED_FACE('',(#39),#41,.F.);
#41 = PLANE('',#14);
#42 = ORIENTED_EDGE('',*,*,#26,.T.);
#43 = EDGE_LOOP('',(#42));
#44 = FACE_OUTER_BOUND('',#43,.T.);
#45 = PLANE('',#24);
#46 = ADVANCED_FACE('',(#44),#45,.T.);
#48 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );
#49 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );
#50 = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );
#51 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#48,'distance_accuracy_value','confusion accuracy');
#47 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#51)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#48,#49,#50)) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') );
ENDSEC;
END-ISO-10303-21;
",
        r0 = f(r0),
        r1 = f(r1),
        h = f(h),
        semi = f(semi),
    )
}

/// **The seamless frustum imports to a body the box admits at the
/// origin, or is refused typed as a band the door does not recognize**,
/// whatever its taper: 0.1 m over 10 m (apex 100 m off) and 1 mm over
/// 10 m (apex 10 km off, past the box). Today both refuse as an
/// unrecognized band. *Breaks if* the door comes to import such a wall
/// and the box refuses it, or the door refuses it for another reason.
#[test]
fn a_seamless_frustum_wall_imports_inside_the_box_or_refuses_as_an_unrecognized_band() {
    let tol = Tol::witness();
    for (what, r1) in [("tapered 0.1 m", 900.0), ("tapered 1 mm", 999.0)] {
        let text = seamless_frustum(1000.0, r1, 10_000.0);
        match import_step(&text, &ImportOptions::default(), tol) {
            Ok(StepImport::Solid { body, .. }) => assert_eq!(
                topo::validate_geometric(&body, tol),
                Ok(()),
                "{what}: the wall at the origin is a finished body"
            ),
            Ok(other) => panic!("{what}: imports as a solid or refuses, got {other:?}"),
            Err(e) => {
                let said = e.render();
                assert!(
                    said.contains("is not a recognized periodic band"),
                    "{what}: refused as an unrecognized band: {said}"
                );
            }
        }
    }
}

/// [`seamless_frustum`] seamed: one cone face bounded by one loop, the
/// bottom ring, a seam up the generator at `x = r0`, the top ring back
/// and the seam down. The seam is a `LINE`, or the same segment as a
/// degree-1 `B_SPLINE_CURVE_WITH_KNOTS`.
fn seamed_frustum(r0: f64, r1: f64, h: f64, spline_seam: bool) -> String {
    let f = |x: f64| format!("{x:?}");
    let seam = if spline_seam {
        "#60 = B_SPLINE_CURVE_WITH_KNOTS('',1,(#17,#21),.UNSPECIFIED.,.F.,.F.,(2,2),(0.,1.),.UNSPECIFIED.);"
            .to_owned()
    } else {
        format!(
            "#61 = DIRECTION('',({},0.,{}));\n#62 = VECTOR('',#61,1.);\n#60 = LINE('',#17,#62);",
            f(r1 - r0),
            f(h)
        )
    };
    let loops = "#30 = ADVANCED_FACE('',(#33),#29,.T.);
#64 = ORIENTED_EDGE('',*,*,#63,.T.);
#65 = ORIENTED_EDGE('',*,*,#63,.F.);
#32 = EDGE_LOOP('',(#31,#64,#34,#65));
#33 = FACE_OUTER_BOUND('',#32,.T.);";
    let text = seamless_frustum(r0, r1, h);
    let start = text.find("#30 = ADVANCED_FACE").expect("the cone face");
    let end = text.find("#37 = ").expect("the bottom disc's edge");
    let kept: String = text[start..end]
        .lines()
        .filter(|l| l.starts_with("#31 ") || l.starts_with("#34 "))
        .map(|l| format!("{l}\n"))
        .collect();
    format!(
        "{}{seam}\n#63 = EDGE_CURVE('',#18,#22,#60,.T.);\n{kept}{loops}\n{}",
        &text[..start],
        &text[end..]
    )
}

/// **A seamed slender frustum imports to a body the box admits at the
/// origin, its apex far past the box**, whether its seam is a line or
/// a degree-1 B-spline: rings of 100 mm and 99.95 mm, 1 m apart, so the
/// cone's apex is 2 km off (inside the rounding reach at every ε the
/// suite runs; at ε = 1e-12, about 5 km, an apex 10 km off refuses the
/// import's seam residual). The cone face's lift is decided, so its
/// cover is its boundary's hull and holds no apex (`topo::cover`'s cone
/// arm: the apex joins only a face whose lift or apex incidence is not
/// decided). *Breaks if* the cone arm keeps the apex of such a face
/// (then the box refuses it as the limit 1 km past), or the door comes
/// to refuse the file.
#[test]
fn a_seamed_slender_frustum_imports_inside_the_box_with_its_apex_far_past_it() {
    let tol = Tol::witness();
    for (what, spline) in [("a line seam", false), ("a spline seam", true)] {
        let text = seamed_frustum(100.0, 99.95, 1000.0, spline);
        let StepImport::Solid { body, .. } = import_step(&text, &ImportOptions::default(), tol)
            .unwrap_or_else(|e| panic!("{what}: imports: {}", e.render()))
        else {
            panic!("{what}: imports as a solid");
        };
        assert_eq!(
            topo::validate_geometric(&body, tol),
            Ok(()),
            "{what}: the wall at the origin is a finished body"
        );
    }
}

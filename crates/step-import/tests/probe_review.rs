//! Adversarial review probes for M7-2 (review/m7-2 branch only).
mod common;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/freecad/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// A3: at eps=1e-7 cone_trunc IMPORTS but tier 3 goes red in-band.
#[test]
fn a3_cone_trunc_imports_then_fails_tier3_at_1e7() {
    let eps = geom_core::Tolerance::get().eps;
    if eps != 1e-7 {
        println!("a3: SKIP (needs CAD_TOLERANCE_EPS=1e-7, have {eps:e})");
        return;
    }
    let imp = import_step(&fixture("cone_trunc"), &ImportOptions::default()).unwrap();
    let StepImport::Solid { body, .. } = imp else {
        panic!("wireframe")
    };
    assert_eq!(topo::validate(&body), Ok(()));
    assert_eq!(topo::validate_closed(&body), Ok(()));
    let g = topo::validate_geometric(&body);
    println!("a3 tier3 result: {g:?}");
    assert!(g.is_err(), "expected the reported in-band tier-3 failure");
}

/// A3 sweep: every fixture, all three tiers, report refusals/tier-fails.
#[test]
fn a3_sweep_all_tiers() {
    let eps = geom_core::Tolerance::get().eps;
    let names = [
        "box",
        "cylinder",
        "cone_trunc",
        "cone_apex",
        "sphere",
        "torus",
        "box_hole",
        "fuse_boxes",
        "box_fillet_edge",
        "box_fillet_corner",
        "compound_two",
        "box_importexport",
        "twobody_importexport",
    ];
    for name in names {
        match import_step(&fixture(name), &ImportOptions::default()) {
            Ok(StepImport::Solid { body, .. }) => {
                let t1 = topo::validate(&body).is_ok();
                let t2 = topo::validate_closed(&body).is_ok();
                let t3 = topo::validate_geometric(&body);
                if !(t1 && t2 && t3.is_ok()) {
                    println!("a3sweep eps={eps:e} {name}: t1={t1} t2={t2} t3={t3:?}");
                } else {
                    println!("a3sweep eps={eps:e} {name}: ALL GREEN");
                }
            }
            Ok(_) => println!("a3sweep eps={eps:e} {name}: WIREFRAME"),
            Err(e) => {
                let s = e.to_string();
                let short: String = s.chars().take(90).collect();
                println!("a3sweep eps={eps:e} {name}: REFUSED {short}");
            }
        }
    }
}

/// A1: an inside-out torus (face sense flipped) must NOT be laundered
/// to a positive volume by the FullPeriodTorus normalization.
#[test]
fn a1_inside_out_torus_cannot_slip_through() {
    let text = fixture("torus").replace(
        "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let g = topo::validate_geometric(&body);
            println!("a1 flipped-sense torus tier3: {g:?}");
            assert!(
                g.is_err(),
                "an inside-out torus certified positive: laundered!"
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 flipped-sense torus refused: {e}"),
    }
}

/// A1: same with the loop reversed via FACE_BOUND .F. instead.
#[test]
fn a1_reversed_bound_torus() {
    let text = fixture("torus").replace(
        "#18 = FACE_BOUND('',#19,.T.);",
        "#18 = FACE_BOUND('',#19,.F.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let g = topo::validate_geometric(&body);
            println!("a1 reversed-bound torus tier3: {g:?}");
            assert!(
                g.is_err(),
                "reversed-bound torus certified positive: laundered!"
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 reversed-bound torus refused: {e}"),
    }
}

/// A1 control: flip one box face's sense — what does the kernel do
/// with a genuinely mis-oriented NON-normalized face?
#[test]
fn a1_control_box_face_sense_flip() {
    let text = fixture("box").replace(
        "#17 = ADVANCED_FACE('',(#18),#52,.F.);",
        "#17 = ADVANCED_FACE('',(#18),#52,.T.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            println!(
                "a1 box-flip: imported; t2={:?} t3={:?}",
                topo::validate_closed(&body),
                topo::validate_geometric(&body)
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 box-flip refused: {e}"),
    }
}

/// A1 control: sphere with flipped face sense (EdgeFreeSphere path).
#[test]
fn a1_control_sphere_sense_flip() {
    let text = fixture("sphere").replace(
        "#17 = ADVANCED_FACE('',(#18),#22,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#22,.F.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            println!(
                "a1 sphere-flip: imported; t3={:?}",
                topo::validate_geometric(&body)
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 sphere-flip refused: {e}"),
    }
}

/// A1: measure the flipped torus's certified volume directly.
#[test]
fn a1_flipped_torus_volume() {
    for (label, from, to) in [
        (
            "sense",
            "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
            "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
        ),
        (
            "bound",
            "#18 = FACE_BOUND('',#19,.T.);",
            "#18 = FACE_BOUND('',#19,.F.);",
        ),
    ] {
        let text = fixture("torus").replace(from, to);
        let Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) = import_step(&text, &ImportOptions::default())
        else {
            println!("a1vol {label}: refused/other");
            continue;
        };
        let props = topo::mass_properties(&body);
        println!(
            "a1vol {label}: norms={} volume={:?}",
            normalizations.len(),
            props.map(|p| p.volume)
        );
    }
}

/// A1: consistently inverted torus — sense AND bound both flipped.
#[test]
fn a1_double_flipped_torus() {
    let text = fixture("torus")
        .replace(
            "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
            "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
        )
        .replace(
            "#18 = FACE_BOUND('',#19,.T.);",
            "#18 = FACE_BOUND('',#19,.F.);",
        );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) => {
            let props = topo::mass_properties(&body);
            println!(
                "a1 double-flip: norms={} vol={:?} t3={:?}",
                normalizations.len(),
                props.map(|p| p.volume),
                topo::validate_geometric(&body)
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 double-flip refused: {e}"),
    }
}

/// A1: does the imported flipped torus body still carry sense=false?
#[test]
fn a1_flipped_torus_body_sense() {
    for (label, text) in [
        ("orig", fixture("torus")),
        (
            "sense-flip",
            fixture("torus").replace(
                "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
                "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
            ),
        ),
    ] {
        let Ok(StepImport::Solid { body, .. }) = import_step(&text, &ImportOptions::default())
        else {
            panic!()
        };
        let senses: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
        println!("a1sense {label}: senses={senses:?}");
    }
}

/// A1: bound-flip torus — body face senses.
#[test]
fn a1_bound_flip_torus_senses() {
    let text = fixture("torus").replace(
        "#18 = FACE_BOUND('',#19,.T.);",
        "#18 = FACE_BOUND('',#19,.F.);",
    );
    let Ok(StepImport::Solid { body, .. }) = import_step(&text, &ImportOptions::default()) else {
        panic!()
    };
    let senses: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
    println!("a1bf senses={senses:?}");
}

/// A1: is the bound-flip torus body's half-edge structure IDENTICAL to
/// the original's (reversal lost) or reversed (props blind)?
#[test]
fn a1_torus_halfedge_diff() {
    let mk = |text: &str| {
        let Ok(StepImport::Solid { body, .. }) = import_step(text, &ImportOptions::default())
        else {
            panic!()
        };
        let mut sig = Vec::new();
        for (_, lp) in body.loops() {
            let topo::LoopBoundary::Cycle { first } = lp.boundary else {
                continue;
            };
            let cycle = body.loop_cycle(first).unwrap();
            let dirs: Vec<String> = cycle
                .iter()
                .map(|&he| {
                    let h = body.get_half_edge(he).unwrap();
                    format!("{:?}:{:?}", h.edge, h.start)
                })
                .collect();
            sig.push(dirs.join(","));
        }
        sig
    };
    let orig = mk(&fixture("torus"));
    let flip = mk(&fixture("torus").replace(
        "#18 = FACE_BOUND('',#19,.T.);",
        "#18 = FACE_BOUND('',#19,.F.);",
    ));
    println!("a1he orig: {orig:?}");
    println!("a1he flip: {flip:?}");
    println!("a1he identical: {}", orig == flip);
}

/// A1 control: reversed FACE_BOUND on a box face must be caught.
#[test]
fn a1_control_box_bound_flip() {
    let text = fixture("box").replace(
        "#18 = FACE_BOUND('',#19,.F.);",
        "#18 = FACE_BOUND('',#19,.T.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            println!(
                "a1bb imported; t2={:?} t3={:?}",
                topo::validate_closed(&body),
                topo::validate_geometric(&body)
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("a1bb refused: {}", &s[..s.len().min(120)]);
        }
    }
}

/// A1: inside-out cone_apex plants (sense flip; bound flip).
#[test]
fn a1_inside_out_cone_apex() {
    let orig = fixture("cone_apex");
    for (label, from, to) in [
        (
            "sense",
            "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
            "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
        ),
        (
            "bound",
            "#18 = FACE_BOUND('',#19,.T.);",
            "#18 = FACE_BOUND('',#19,.F.);",
        ),
    ] {
        let text = orig.replace(from, to);
        assert_ne!(text, orig, "{label}: replacement must hit");
        match import_step(&text, &ImportOptions::default()) {
            Ok(StepImport::Solid { body, .. }) => {
                let v = topo::mass_properties(&body).map(|p| p.volume);
                println!(
                    "a1ca {label}: imported t3={:?} vol={:?}",
                    topo::validate_geometric(&body),
                    v
                );
            }
            Ok(_) => panic!("wireframe"),
            Err(e) => {
                let s = e.to_string();
                println!("a1ca {label} refused: {}", &s[..s.len().min(110)]);
            }
        }
    }
}

/// A1: sense-flipped cone_apex — do body senses carry the flip?
#[test]
fn a1_cone_apex_flip_senses() {
    let text = fixture("cone_apex").replace(
        "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
    );
    let Ok(StepImport::Solid { body, .. }) = import_step(&text, &ImportOptions::default()) else {
        panic!()
    };
    let senses: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
    println!("a1cas senses={senses:?}");
}

/// A1 control: sense-flipped cylinder lateral face (no normalization).
#[test]
fn a1_control_cylinder_sense_flip() {
    let orig = fixture("cylinder");
    let text = orig.replace(
        "#17 = ADVANCED_FACE('',(#18),#45,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#45,.F.);",
    );
    assert_ne!(text, orig);
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let v = topo::mass_properties(&body).map(|p| p.volume);
            let senses: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
            println!(
                "a1cyl imported t3={:?} vol={:?} senses={senses:?}",
                topo::validate_geometric(&body),
                v
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("a1cyl refused: {}", &s[..s.len().min(110)]);
        }
    }
}

/// A5(a): edge #206 between COINCIDENT planes replaced by a circular
/// arc that leaves the plane (endpoints unchanged). If the MappedCurve
/// rung launders it, the gate's claim is false.
#[test]
fn a5_off_locus_arc_between_coincident_planes() {
    let text = fixture("fuse_boxes").replace(
        "#209 = LINE('',#210,#211);",
        "#209 = CIRCLE('',#900,0.25);\n\
         #900 = AXIS2_PLACEMENT_3D('',#901,#902,#903);\n\
         #901 = CARTESIAN_POINT('',(0.75,1.,1.));\n\
         #902 = DIRECTION('',(0.,1.,0.));\n\
         #903 = DIRECTION('',(1.,0.,0.));",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let v = topo::mass_properties(&body).map(|p| p.volume);
            println!(
                "a5a LAUNDERED: t3={:?} vol={:?}",
                topo::validate_geometric(&body),
                v
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("a5a refused: {}", &s[..s.len().min(160)]);
        }
    }
}

/// A5(b): nearly-coincident distinct planes (1e-8 m apart, 10x eps).
#[test]
fn a5_nearly_coincident_planes_refuse() {
    let text = fixture("fuse_boxes").replace(
        "#335 = CARTESIAN_POINT('',(0.5,0.5,1.));",
        "#335 = CARTESIAN_POINT('',(0.5,0.5,1.00001));",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { .. }) => println!("a5b LAUNDERED: imported"),
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("a5b refused: {}", &s[..s.len().min(160)]);
        }
    }
}

fn own_fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/../step-export/tests/fixtures/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// A4: planted WRONG outer designation — the inner pip ring marked
/// FACE_OUTER_BOUND and the true outer demoted to FACE_BOUND. The
/// cross-check must refuse typed.
#[test]
fn a4_wrong_outer_designation_refuses() {
    let text = own_fixture("die_pips")
        .replace(
            "#39 = FACE_OUTER_BOUND('', #38, .T.);",
            "#39 = FACE_BOUND('', #38, .T.);",
        )
        .replace(
            "#59 = FACE_BOUND('', #58, .T.);",
            "#59 = FACE_OUTER_BOUND('', #58, .T.);",
        );
    match import_step(&text, &ImportOptions::default()) {
        Ok(_) => println!("a4wo LAUNDERED: imported with wrong outer designation"),
        Err(e) => {
            let s = e.to_string();
            println!("a4wo refused: {}", &s[..s.len().min(180)]);
        }
    }
}

/// A4: BOTH rings demoted to plain FACE_BOUND on the own-dialect pip
/// face: inference alone must pick the true outer (and the body
/// certify), or refuse — never pick the pip.
#[test]
fn a4_inference_on_demoted_bounds() {
    let text = own_fixture("die_pips").replace(
        "#39 = FACE_OUTER_BOUND('', #38, .T.);",
        "#39 = FACE_BOUND('', #38, .T.);",
    );
    match import_step(&text, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            println!("a4inf imported t3={:?}", topo::validate_geometric(&body));
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => {
            let s = e.to_string();
            println!("a4inf refused: {}", &s[..s.len().min(150)]);
        }
    }
}

/// A6: prefixed ANGLE refuses end-to-end; other length prefixes scale
/// (the "prefix is data" claim); a second, different length scale
/// refuses.
#[test]
fn a6_unit_edges() {
    let orig = fixture("box");
    // (a) prefixed radian
    let t = orig.replace("SI_UNIT($,.RADIAN.)", "SI_UNIT(.MILLI.,.RADIAN.)");
    assert_ne!(t, orig);
    match import_step(&t, &ImportOptions::default()) {
        Err(e) => {
            let s = e.to_string();
            println!("a6 angle refused: {}", &s[..s.len().min(110)]);
        }
        Ok(_) => println!("a6 angle LAUNDERED"),
    }
    // (b) MICRO length: volume must scale by 1e-18 vs MILLI's 1e-9.
    let t = orig.replace("SI_UNIT(.MILLI.,.METRE.)", "SI_UNIT(.MICRO.,.METRE.)");
    assert_ne!(t, orig);
    match import_step(&t, &ImportOptions::default()) {
        Ok(StepImport::Solid { body, .. }) => {
            let v = topo::mass_properties(&body).unwrap().volume;
            println!("a6 micro volume={v:e} (expect 1e-18)");
        }
        Ok(_) => panic!(),
        Err(e) => println!("a6 micro refused: {e}"),
    }
}

/// A6: a second, different length scale in one file must refuse.
#[test]
fn a6_two_length_scales_refuse() {
    let orig = fixture("box");
    let t = orig.replace(
        "#167 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );",
        "#167 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.CENTI.,.METRE.) );",
    );
    assert_ne!(t, orig);
    match import_step(&t, &ImportOptions::default()) {
        Err(e) => {
            let s = e.to_string();
            println!("a6two refused: {}", &s[..s.len().min(120)]);
        }
        Ok(_) => println!("a6two LAUNDERED: two length scales accepted"),
    }
}

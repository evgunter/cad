//! Adversarial review probes for M7-2, adopted by merge from the
//! `review/m7-2` branch (authorship kept). The two `a1_*torus*` probes
//! that failed deliberately there now assert the fix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/freecad/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// A3, **retired** (the #89 in-band landing): at eps=1e-7 cone_trunc
/// imports AND all three tiers are green. The landing was a
/// dimensional-metering defect, not a scale/ε fact: `du_of_rims`
/// metered the cone's rim-level difference — already a slant LENGTH —
/// by `× arm`, manufacturing an area-dimensioned margin
/// (5.590169943747308e-7 = separation √5/2 mm × arm 0.5 mm, m²) that
/// sat inside `Band { 1e-7, 1e-6 }`. Metered honestly (bare), the
/// margin is the rim separation itself, ≈1.118e-3 m — decisively out
/// of that band. Scale-linearity of the fixed margin is pinned at
/// `geom-brep/tests/rim_dim_scale_twins.rs`.
#[test]
fn a3_cone_trunc_all_tiers_green_at_1e7_landing_retired() {
    let eps = geom_core::Tol::witness().get().eps;
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
    let g = topo::validate_geometric(&body, Tol::witness());
    println!("a3 tier3 result: {g:?}");
    assert_eq!(g, Ok(()), "the in-band landing is retired: tier 3 is green");
}

/// A3 sweep: every fixture, all three tiers, report refusals/tier-fails.
#[test]
fn a3_sweep_all_tiers() {
    let eps = geom_core::Tol::witness().get().eps;
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
                let t3 = topo::validate_geometric(&body, Tol::witness());
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

/// A1, **resolved**: an inside-out torus (face sense flipped, loop
/// left alone) is REFUSED typed. The normalization now reads the
/// loop's cyclic order — the only place a fundamental polygon's
/// winding lives — instead of its reversal-invariant flag multiset,
/// so the inversion is detected instead of being minted away.
#[test]
fn a1_inside_out_torus_cannot_slip_through() {
    let text = fixture("torus").replace(
        "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
    );
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("an inside-out torus must not import at all");
    let text = err.to_string();
    assert!(
        text.contains("ORIENTATION-INVERTED") && text.contains("#17"),
        "the refusal must name the inversion and the face: {text}"
    );
    assert!(
        text.contains("same_sense"),
        "and the kernel limitation that blocks an honest inverted import: {text}"
    );
}

/// A1, **resolved**: the same inversion stated the other way — loop
/// reversed via `FACE_BOUND .F.`, sense left alone — refuses too.
#[test]
fn a1_reversed_bound_torus() {
    let text = fixture("torus").replace(
        "#18 = FACE_BOUND('',#19,.T.);",
        "#18 = FACE_BOUND('',#19,.F.);",
    );
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("a reversed-bound torus is inside out and must not import");
    assert!(err.to_string().contains("ORIENTATION-INVERTED"), "{err}");
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
                topo::validate_geometric(&body, Tol::witness())
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
                topo::validate_geometric(&body, Tol::witness())
            );
        }
        Ok(_) => panic!("wireframe"),
        Err(e) => println!("a1 sphere-flip refused: {e}"),
    }
}

/// A1, **resolved**: neither single flip reaches a volume at all now —
/// there is no body to measure, which is the point. (Before the fix
/// both produced `+1.2337005501361696e-9`, bit-identical to the
/// correct torus.)
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
        let err = import_step(&text, &ImportOptions::default())
            .err()
            .unwrap_or_else(|| panic!("a1vol {label}: imported, so it was laundered"));
        println!("a1vol {label}: refused — {err}");
    }
}

/// A1, **resolved**: the CONSISTENTLY inverted statement — sense AND
/// bound both flipped — is ISO 10303-42's other legal encoding of the
/// SAME right-side-out solid, and it must import, not refuse. It does,
/// with the correct positive volume and a green tier 3.
///
/// This is the assertion that keeps the fix from being "refuse
/// anything unusual": the two encodings that mean a right-side-out
/// ring both adopt, and the two that mean an inside-out one both
/// refuse.
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
    let Ok(StepImport::Solid {
        body,
        normalizations,
        ..
    }) = import_step(&text, &ImportOptions::default())
    else {
        panic!("the equivalent re-encoding of a valid torus must import");
    };
    assert_eq!(normalizations.len(), 1, "still one reported normalization");
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    assert!(
        v > 0.0,
        "a right-side-out ring has positive volume, got {v}"
    );
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
}

/// A1, **resolved**: the original imports; the sense-flip does not.
#[test]
fn a1_flipped_torus_body_sense() {
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&fixture("torus"), &ImportOptions::default())
    else {
        panic!("the stated torus imports")
    };
    let senses: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
    println!("a1sense orig: senses={senses:?}");
    let flipped = fixture("torus").replace(
        "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
    );
    assert!(
        import_step(&flipped, &ImportOptions::default()).is_err(),
        "the sense-flip has no body to carry a sense at all"
    );
}

/// A1, **resolved**: the bound-flip torus has no body either.
#[test]
fn a1_bound_flip_torus_senses() {
    let text = fixture("torus").replace(
        "#18 = FACE_BOUND('',#19,.T.);",
        "#18 = FACE_BOUND('',#19,.F.);",
    );
    assert!(import_step(&text, &ImportOptions::default()).is_err());
}

/// A1, **resolved** — the finding's own probe, inverted. It used to
/// show that the bound-flipped torus built a body HALF-EDGE-IDENTICAL
/// to the original: the reversal had been discarded. Now the
/// bound-flip has no body at all, and the structure it used to
/// produce belongs to the file that genuinely means it — the
/// double-flip re-encoding, whose half-edge signature must equal the
/// original's because it states the same solid.
#[test]
fn a1_torus_halfedge_diff() {
    let sig = |text: &str| {
        let Ok(StepImport::Solid { body, .. }) = import_step(text, &ImportOptions::default())
        else {
            panic!("expected a body")
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
    let orig = sig(&fixture("torus"));
    let double = sig(&fixture("torus")
        .replace(
            "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
            "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
        )
        .replace(
            "#18 = FACE_BOUND('',#19,.T.);",
            "#18 = FACE_BOUND('',#19,.F.);",
        ));
    assert_eq!(
        orig, double,
        "the two legal encodings of one solid must reconstruct to one body"
    );
    // And the single flips, which mean the OTHER solid, reach no body.
    for (from, to) in [
        (
            "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
            "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
        ),
        (
            "#18 = FACE_BOUND('',#19,.T.);",
            "#18 = FACE_BOUND('',#19,.F.);",
        ),
    ] {
        assert!(
            import_step(
                &fixture("torus").replace(from, to),
                &ImportOptions::default()
            )
            .is_err(),
            "a single flip means an inside-out ring and must refuse"
        );
    }
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
                topo::validate_geometric(&body, Tol::witness())
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
                let v = topo::mass_properties(&body, Tol::witness()).map(|p| p.volume);
                println!(
                    "a1ca {label}: imported t3={:?} vol={:?}",
                    topo::validate_geometric(&body, Tol::witness()),
                    v
                );
                panic!("a1ca {label}: an inside-out cone_apex must refuse pre-body (M6-6 rider)");
            }
            Ok(_) => panic!("wireframe"),
            Err(e) => {
                let s = e.to_string();
                println!("a1ca {label} refused: {}", &s[..s.len().min(110)]);
                // PINNED (M6-6): the sense flip refuses with the
                // orientation-inverted diagnosis, pre-body. (The bound
                // flip already refused structurally — traversal parity
                // — before the rider; that arm stays a typed refusal.)
                if label == "sense" {
                    assert!(
                        s.contains("ORIENTATION-INVERTED"),
                        "sense flip must carry the orientation-inverted diagnosis: {s}"
                    );
                }
            }
        }
    }
}

/// A1: sense-flipped cone_apex — the body never exists to carry the
/// flip: since the M6-6 rider the degenerate-apex normalization
/// cross-checks the stated sense against the loop's derived winding
/// (the torus pattern) and refuses PRE-BODY.
#[test]
fn a1_cone_apex_flip_senses() {
    let text = fixture("cone_apex").replace(
        "#17 = ADVANCED_FACE('',(#18),#38,.T.);",
        "#17 = ADVANCED_FACE('',(#18),#38,.F.);",
    );
    let err = import_step(&text, &ImportOptions::default())
        .err()
        .unwrap_or_else(|| panic!("an inside-out cone_apex must refuse pre-body (M6-6 rider)"));
    let s = err.to_string();
    println!("a1cas refused: {}", &s[..s.len().min(110)]);
    assert!(
        s.contains("ORIENTATION-INVERTED") && s.contains("#17"),
        "the refusal must name the face and the inversion: {s}"
    );
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
    // PINNED (M6-6): the live hole this control found — a flipped
    // cylinder lateral imported tier-3 GREEN with positive volume —
    // is closed from both sides: the kernel's check-6 curved arm
    // refuses the body, and the import rider refuses pre-body.
    let err = import_step(&text, &ImportOptions::default())
        .err()
        .unwrap_or_else(|| panic!("an inside-out cylinder wall must refuse pre-body (M6-6 rider)"));
    let s = err.to_string();
    println!("a1cyl refused: {}", &s[..s.len().min(110)]);
    assert!(
        s.contains("ORIENTATION-INVERTED") && s.contains("#17"),
        "the refusal must name the face and the inversion: {s}"
    );
}

/// A5, **resolved**: an edge whose carrier leaves the coincident plane
/// no longer adopts through the conventional rung. The rung now gates
/// on the CURVE as well as the surface pair, so `import_step` refuses
/// typed instead of returning a body only tier 3 would catch.
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
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("an off-locus carrier must not adopt as a conventional curve");
    let s = err.to_string();
    assert!(
        s.contains("edge #206") && s.contains("no intensional description certifies"),
        "expected the typed adoption refusal naming the edge: {s}"
    );
    println!("a5a refused: {}", &s[..s.len().min(200)]);
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
            println!(
                "a4inf imported t3={:?}",
                topo::validate_geometric(&body, Tol::witness())
            );
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
            let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
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

/// ADVERSARIAL REVIEW PROBE (branch review/rim-dim): the FULL refusal
/// error for the cylinder fixture at eps=1e-7 — which predicate the
/// CORPUS_EPS_CEILING refusals actually rest on (F5 linkage).
#[test]
fn review_f5_cylinder_refusal_predicate() {
    let eps = geom_core::Tol::witness().get().eps;
    if eps != 1e-7 {
        println!("SKIP (needs 1e-7, have {eps:e})");
        return;
    }
    let got = import_step(&fixture("cylinder"), &ImportOptions::default());
    match got {
        Err(e) => println!("FULL REFUSAL: {e:?}"),
        Ok(_) => println!("IMPORTED (no refusal)"),
    }
}

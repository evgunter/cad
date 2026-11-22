//! **Records follow the file** (`docs/DESIGN.md` `[import]`): a surface
//! entity is one record, worn by every face of one shell that cites it,
//! and two entities are two records unless one-record's reported act
//! joins them. No comparison of printed values decides which record a
//! face wears.
//!
//! One row per sharing pattern a file can state by reference within one
//! shell. Each states the sharing by reference (the writer cites one
//! surface entity per surface key, so a native body's export states its
//! own sharing), and holds the import to its native twin, built through
//! the named public verb: the import wears the records the file cites as
//! the twin wears its keys, and takes the twin's at-rest verdict. The
//! pinch has no native twin and is held to its tier-3 refusal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Point2, Tol, Vec2};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use step_export::{StepOptions, step_string};
use step_import::{ImportOptions, NormalizationKind, StepImport, StepImportError, import_step};
use topo::{Body, BooleanError, BooleanResult, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

/// Each surface key of `body` worn by more than one face: its kind and
/// its wearers' senses, sorted.
fn shared(body: &Body<f64>) -> Vec<(&'static str, Vec<bool>)> {
    let mut by: BTreeMap<_, (&'static str, Vec<bool>)> = BTreeMap::new();
    for (k, f) in body.faces() {
        let kind = body.face_surface(k).unwrap().kind().name();
        by.entry(f.surface)
            .or_insert((kind, Vec::new()))
            .1
            .push(f.sense);
    }
    let mut out: Vec<_> = by
        .into_values()
        .filter(|(_, senses)| senses.len() > 1)
        .map(|(kind, mut senses)| {
            senses.sort_unstable();
            (kind, senses)
        })
        .collect();
    out.sort_unstable();
    out
}

/// The surface entity each `ADVANCED_FACE` of `text` cites.
fn cited(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| l.contains("= ADVANCED_FACE("))
        .map(|l| {
            let at = l.rfind('#').unwrap();
            l[at..].split(',').next().unwrap().trim().to_owned()
        })
        .collect()
}

/// How many surface entities `text` cites from more than one face.
fn cited_by_several(text: &str) -> usize {
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for s in cited(text) {
        *by.entry(s).or_default() += 1;
    }
    by.values().filter(|&&n| n > 1).count()
}

fn written(body: &Body<f64>) -> String {
    step_string(
        &crate::common::at_rest(body),
        &StepOptions::default(),
        tol(),
    )
    .expect("the native twin writes")
}

fn imported(what: &str, text: &str) -> Result<Body<f64>, StepImportError> {
    healed(what, text).map(|(body, _)| body)
}

/// The import of `text`, and how many one-record joins healing reported.
fn healed(what: &str, text: &str) -> Result<(Body<f64>, usize), StepImportError> {
    match import_step(text, &ImportOptions::default(), tol()) {
        Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) => {
            let joins = normalizations
                .iter()
                .filter(|n| matches!(n.kind, NormalizationKind::OneRecord { .. }))
                .count();
            Ok((body.into_body(), joins))
        }
        Ok(_) => panic!("{what}: a solid file imports as a solid"),
        Err(e) => Err(e),
    }
}

/// Tier 3's verdict, as the kinds of what it refuses.
fn verdict(body: &Body<f64>) -> Result<(), Vec<String>> {
    topo::validate_geometric(body, tol()).map_err(|errs| {
        errs.iter()
            .map(|e| {
                format!("{e:?}")
                    .split([' ', '{', '('])
                    .next()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    })
}

/// **The row**: `native`'s export states its sharing by reference, and
/// its import wears the cited records as `native` wears its keys, with
/// no join of healing's, and takes `native`'s at-rest verdict. Answers
/// the import.
fn held_to_its_twin(what: &str, native: &Body<f64>) -> Body<f64> {
    let want = shared(native);
    assert!(
        !want.is_empty(),
        "{what}: the native twin wears a key on several faces"
    );
    let text = written(native);
    assert_eq!(
        cited_by_several(&text),
        want.len(),
        "{what}: the file cites one entity per shared key"
    );
    let (body, joins) = healed(what, &text).unwrap_or_else(|e| {
        panic!(
            "{what}: the twin is {:?} at rest, the import refuses: {e}",
            verdict(native)
        )
    });
    assert_eq!(
        shared(&body),
        want,
        "{what}: the import wears the file's records"
    );
    assert_eq!(
        joins, 0,
        "{what}: the records are the file's, none joined by healing"
    );
    assert_eq!(
        verdict(&body),
        verdict(native),
        "{what}: the at-rest verdict"
    );
    body
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::AtRestBody<f64> {
    topo::test_support::finished(
        "a brick",
        topo::test_support::brick::<f64>(x, y, z, tol()),
        tol(),
    )
}

fn result(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> Body<f64> {
    match r {
        Ok(BooleanResult::Body(bb)) => bb.body.into_body(),
        other => panic!("{what}: the boolean builds, got {:?}", other.map(|_| ())),
    }
}

/// **Faces no edge joins, on one record.** A slot cut across a bar's top
/// divides it into two faces; both keep the bar's record.
#[test]
fn a_record_worn_by_faces_no_edge_joins_imports_as_the_boolean_that_divides_a_face() {
    let bar = brick((0.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let slot = brick((1.0, 2.0), (-1.0, 2.0), (0.5, 2.0));
    let native = result("the slotted bar", topo::subtract(&bar, &slot, tol()));
    held_to_its_twin("the slotted bar", &native);
}

/// **One record worn both ways in one solid.** `topo::union` of
/// `[0,2]×[−1,0]×[0,1]` and `[1,3]×[0,1]×[0,1]` leaves both `y = 0`
/// faces on one key with opposite senses. Two entities printed alike
/// stay two records, as an extrude of the same outline leaves them.
#[test]
fn a_record_worn_both_ways_imports_as_the_union_leaves_it() {
    let low = brick((0.0, 2.0), (-1.0, 0.0), (0.0, 1.0));
    let high = brick((1.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let native = result("the Z-step union", topo::union(&low, &high, tol()));
    assert!(
        shared(&native).contains(&("plane", vec![false, true])),
        "the union leaves both y = 0 faces on one key, worn both ways: {:?}",
        shared(&native)
    );
    held_to_its_twin("the Z-step union", &native);

    // The same file with the second y = 0 face citing a copy of the
    // entity, printed alike: two records, the extrude's state.
    let text = written(&native);
    let mut by: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in text.lines().filter(|l| l.contains("= ADVANCED_FACE(")) {
        let s = line[line.rfind('#').unwrap()..]
            .split(',')
            .next()
            .unwrap()
            .to_owned();
        by.entry(s).or_default().push(line.to_owned());
    }
    let (plane, faces) = by.iter().find(|(_, f)| f.len() == 2).unwrap();
    let stated = text
        .lines()
        .find(|l| l.starts_with(&format!("{plane} = PLANE(")))
        .unwrap();
    let copy = "#999999";
    let twice = text
        .replacen(
            &faces[1],
            &faces[1].replace(&format!("{plane},"), &format!("{copy},")),
            1,
        )
        .replacen(
            stated,
            &format!("{stated}\n{}", stated.replacen(plane.as_str(), copy, 1)),
            1,
        );
    let body = imported("the Z-step on two printed-alike planes", &twice)
        .expect("the Z-step on two entities imports");
    assert!(
        !shared(&body).iter().any(|(kind, _)| *kind == "plane"),
        "two entities printed alike, facing opposite ways, are two records: {:?}",
        shared(&body)
    );
    assert_eq!(
        verdict(&body),
        Ok(()),
        "the extrude's state is valid at rest"
    );
}

/// **Edge-adjacent faces of one curved record.** A disc revolved a
/// quarter turn: its wall is two faces on one torus key, meeting along
/// the equators.
#[test]
fn edge_adjacent_faces_of_one_curved_record_import_as_the_revolve_leaves_them() {
    let lp = bulge_loop(vec![
        (Point2::new(4.0, 0.0), 1.0),
        (Point2::new(6.0, 0.0), 1.0),
    ]);
    let vp = profile::Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let native = sweep::revolve(&vp, axis, sweep::Revolution::Partial(FRAC_PI_2), tol())
        .expect("the quarter turn revolves")
        .body
        .into_body();
    assert!(
        shared(&native).contains(&("torus", vec![true, true])),
        "the wall is two faces of one torus key: {:?}",
        shared(&native)
    );
    held_to_its_twin("the quarter torus", &native);
}

/// **Edge-adjacent faces of one plane, facing one way.** An axis-touching
/// square revolved by π: its two caps across the axis edge wear one
/// record. The import and its twin share the at-rest verdict, and both
/// are refused as boolean operands, `NonMaximalFaces`
/// (`work/export/an-imported-union-ships-coplanar-same-record-neighbours-no-boolean-takes.md`).
#[test]
fn coplanar_neighbours_of_one_record_import_as_the_half_revolve_leaves_them() {
    let lp = bulge_loop(
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .to_vec(),
    );
    let vp = profile::Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let native = sweep::revolve(&vp, axis, sweep::Revolution::Partial(PI), tol())
        .expect("the half turn revolves")
        .body
        .into_body();
    assert!(
        shared(&native)
            .iter()
            .any(|(kind, s)| *kind == "plane" && s[0] == s[1]),
        "the caps wear one plane key, facing one way: {:?}",
        shared(&native)
    );
    let import = held_to_its_twin("the half revolve", &native);
    let far = brick((10.0, 11.0), (0.0, 1.0), (0.0, 1.0));
    for (what, body) in [("native", native), ("import", import)] {
        let operand = topo::test_support::finished(what, body, tol());
        match topo::union(&operand, &far, tol()) {
            Err(BooleanError::NonMaximalFaces { .. }) => {}
            other => panic!("{what}: want NonMaximalFaces, got {:?}", other.map(|_| ())),
        }
    }
}

/// **A pinch**: opposite facings meeting at an edge. The ball's two
/// bands on one sphere, one cited reversed: a lamina at their shared
/// edges, refused at tier 3 on its geometry. Stated on two records of
/// one sphere, the edge between them refuses first, at the build.
#[test]
fn a_pinch_is_refused_at_tier_3_on_one_record_or_two() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/ball.step"
    ))
    .unwrap();
    let faces: Vec<&str> = text
        .lines()
        .filter(|l| l.contains("= ADVANCED_FACE("))
        .collect();
    let [first, _] = faces[..] else {
        panic!("the ball is two bands: {faces:?}")
    };
    let sphere = cited(&text)[0].clone();
    assert_eq!(
        cited(&text),
        [sphere.clone(), sphere.clone()],
        "one sphere entity, cited twice"
    );
    let flip = |s: &str| {
        if s.ends_with(".T.);") {
            s.replace(".T.);", ".F.);")
        } else {
            s.replace(".F.);", ".T.);")
        }
    };
    let pinched = text.replacen(first, &flip(first), 1);
    let stated = text
        .lines()
        .find(|l| l.starts_with(&format!("{sphere} = SPHERICAL_SURFACE(")))
        .unwrap();
    let copy = "#999999";
    let pinched_on_two = pinched
        .replacen(
            &flip(first),
            &flip(first).replace(&format!("{sphere},"), &format!("{copy},")),
            1,
        )
        .replacen(
            stated,
            &format!("{stated}\n{}", stated.replacen(sphere.as_str(), copy, 1)),
            1,
        );
    match imported("one record", &pinched) {
        Err(StepImportError::TierInvalid { errors, .. }) => assert!(
            !errors.is_empty()
                && errors
                    .iter()
                    .all(|e| matches!(e, ValidationError::LaminaWedge { .. })),
            "one record: the pinch is a lamina: {errors:?}"
        ),
        other => panic!(
            "one record: want the tier-3 lamina refusal, got {:?}",
            other.map(|_| ())
        ),
    }
    // Two records facing opposite ways stay two, and the edge between two
    // records of one sphere refuses at the build, typed, before tier 3.
    match imported("two records", &pinched_on_two) {
        Err(StepImportError::NotYetSupported {
            entity: step_import::BuiltEntity::Edge,
            ..
        }) => {}
        other => panic!(
            "two records: want the edge's typed refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// `loft_prism`'s twisted wall (`#111`, ruled between a quadratic
/// column and a straight one) split along its `v = ½` ruling into two
/// faces that both cite `#111`: its side edges `#100` and `#119` each
/// split at the ruling's ends into their two Bézier halves, and the
/// ruling minted as the edge between the two faces.
fn split_loft_wall() -> String {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/loft_prism.step"
    ))
    .unwrap();
    let quadratic = |id: u64, [a, b, c]: [&str; 3]| {
        format!(
            "#{} = CARTESIAN_POINT('', ({a}));\n#{} = CARTESIAN_POINT('', ({b}));\n\
             #{} = CARTESIAN_POINT('', ({c}));\n#{id} = B_SPLINE_CURVE_WITH_KNOTS('', 2, \
             (#{}, #{}, #{}), .UNSPECIFIED., .U., .U., (3, 3), (0.0, 1.0), .UNSPECIFIED.);",
            id + 1,
            id + 2,
            id + 3,
            id + 1,
            id + 2,
            id + 3
        )
    };
    let swaps = [
        (
            "#100 = EDGE_CURVE('', #65, #17, #99, .T.);".to_owned(),
            format!(
                "{}\n{}\n#100 = EDGE_CURVE('', #65, #2001, #2200, .T.);\n\
                 #2100 = EDGE_CURVE('', #2001, #17, #2210, .T.);",
                quadratic(
                    2200,
                    ["1.0, -1.0, 0.0", "1.375, -1.0, 0.5", "1.375, -1.0, 1.0"]
                ),
                quadratic(
                    2210,
                    ["1.375, -1.0, 1.0", "1.375, -1.0, 1.5", "1.0, -1.0, 2.0"]
                ),
            ),
        ),
        (
            "#119 = EDGE_CURVE('', #57, #25, #118, .T.);".to_owned(),
            format!(
                "{}\n{}\n#119 = EDGE_CURVE('', #57, #2003, #2220, .T.);\n\
                 #2119 = EDGE_CURVE('', #2003, #25, #2230, .T.);",
                quadratic(2220, ["1.0, 1.0, 0.0", "1.0, 1.0, 0.5", "1.0, 1.0, 1.0"]),
                quadratic(2230, ["1.0, 1.0, 1.0", "1.0, 1.0, 1.5", "1.0, 1.0, 2.0"]),
            ),
        ),
        (
            "(#88, #94, #95, #101)".to_owned(),
            "(#88, #94, #95, #101, #2101)".to_owned(),
        ),
        (
            "#132 = ORIENTED_EDGE('', *, *, #119, .F.);".to_owned(),
            "#132 = ORIENTED_EDGE('', *, *, #2119, .F.);\n\
             #2132 = ORIENTED_EDGE('', *, *, #119, .F.);"
                .to_owned(),
        ),
        (
            "(#131, #132, #133, #139)".to_owned(),
            "(#131, #132, #2132, #133, #139)".to_owned(),
        ),
        (
            "#123 = ADVANCED_FACE('', (#122), #111, .T.);".to_owned(),
            "#2000 = CARTESIAN_POINT('', (1.375, -1.0, 1.0));\n\
             #2001 = VERTEX_POINT('', #2000);\n\
             #2002 = CARTESIAN_POINT('', (1.0, 1.0, 1.0));\n\
             #2003 = VERTEX_POINT('', #2002);\n\
             #2101 = ORIENTED_EDGE('', *, *, #2100, .T.);\n\
             #2010 = CARTESIAN_POINT('', (1.375, -1.0, 1.0));\n\
             #2011 = DIRECTION('', (-0.18428853505018536, 0.9828721869343219, 0.0));\n\
             #2012 = VECTOR('', #2011, 2.0348525745124633);\n\
             #2013 = LINE('', #2010, #2012);\n\
             #2014 = EDGE_CURVE('', #2001, #2003, #2013, .T.);\n\
             #2020 = ORIENTED_EDGE('', *, *, #2014, .F.);\n\
             #2023 = EDGE_LOOP('', (#113, #114, #120, #2020));\n\
             #2024 = FACE_OUTER_BOUND('', #2023, .T.);\n\
             #123 = ADVANCED_FACE('', (#2024), #111, .T.);\n\
             #2040 = ORIENTED_EDGE('', *, *, #2100, .F.);\n\
             #2041 = ORIENTED_EDGE('', *, *, #2014, .T.);\n\
             #2042 = ORIENTED_EDGE('', *, *, #2119, .T.);\n\
             #2043 = EDGE_LOOP('', (#112, #2040, #2041, #2042));\n\
             #2044 = FACE_OUTER_BOUND('', #2043, .T.);\n\
             #2045 = ADVANCED_FACE('', (#2044), #111, .T.);"
                .to_owned(),
        ),
    ];
    let mut out = text;
    for (old, new) in swaps {
        assert_eq!(
            out.matches(old.as_str()).count(),
            1,
            "the fixture states {old} once"
        );
        out = out.replacen(old.as_str(), &new, 1);
    }
    out.replacen("#123, #142", "#123, #2045, #142", 1)
}

/// **Our own split spline faces round-trip by reference.** `c_prism`'s
/// top is split at its crease knot into the C's back and its two arms;
/// the arms lie past the knot line, apart, on the one sub-patch record
/// the split mints for that side. Written back, that key is one entity
/// cited by both arms, and imports again as one record.
#[test]
fn a_spline_face_split_on_one_key_round_trips_by_reference() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/c0-knot/c_prism.step"
    ))
    .unwrap();
    let split = imported("c_prism", &text).expect("c_prism imports");
    assert!(
        shared(&split).iter().any(|(kind, _)| *kind == "nurbs"),
        "the two arms wear one spline key: {:?}",
        shared(&split)
    );
    held_to_its_twin("c_prism's split top", &split);
}

/// **One spline stated as two entities with an edge between refuses**,
/// typed, at that edge: one-record joins no spline pair, and the section
/// of two records of one spline is no curve the kernel builds. Stated as
/// one entity, the edge is a curve inside one spline that is none of
/// its net's boundary rows, and refuses too.
#[test]
fn one_spline_stated_twice_with_an_edge_between_refuses_typed() {
    let once = split_loft_wall();
    let stated = once
        .lines()
        .find(|l| l.starts_with("#111 = B_SPLINE_SURFACE_WITH_KNOTS("))
        .unwrap();
    let twice = once
        .replacen(
            stated,
            &format!("{stated}\n{}", stated.replacen("#111", "#999999", 1)),
            1,
        )
        .replacen(
            "#2045 = ADVANCED_FACE('', (#2044), #111, .T.);",
            "#2045 = ADVANCED_FACE('', (#2044), #999999, .T.);",
            1,
        );
    for (what, text, lane) in [
        (
            "once",
            &once,
            "a construction of a curve inside one surface",
        ),
        ("twice", &twice, "a section construction for this pair"),
    ] {
        match imported(what, text) {
            Err(e @ StepImportError::NotYetSupported { id: 2014, .. }) => assert!(
                e.render().contains(lane),
                "{what}: the edge refuses naming {lane:?}: {}",
                e.render()
            ),
            other => panic!(
                "{what}: want the edge's typed refusal, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}

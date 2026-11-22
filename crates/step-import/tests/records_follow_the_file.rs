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

use std::collections::{BTreeMap, BTreeSet};

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Point2, Tol, Vec2};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use step_export::{StepOptions, step_string};
use step_import::{ImportOptions, NormalizationKind, StepImport, StepImportError, import_step};
use topo::{Body, BooleanError, BooleanResult, ValidationError};

use crate::common::{corner_names, micrometres, shared_surfaces};

fn tol() -> Tol {
    Tol::witness()
}

/// A face named by its corners ([`corner_names`]) and its sense.
type Named = (Vec<[i64; 3]>, bool);

/// **Which faces share a surface**: each surface key of `body` worn by
/// several faces, as the set of its wearers by name. Two bodies whose
/// faces share alike compare equal, whatever their keys.
fn partition(body: &Body<f64>) -> BTreeSet<Vec<Named>> {
    let names = corner_names(body);
    shared_surfaces(body)
        .into_iter()
        .map(|wearers| {
            let mut named: Vec<Named> = wearers
                .into_iter()
                .map(|(f, sense)| (names[&f].clone(), sense))
                .collect();
            named.sort_unstable();
            named
        })
        .collect()
}

/// Each surface key worn by several faces, as its kind and its wearers'
/// senses, sorted: what a twin's pattern is checked against.
fn kinds(body: &Body<f64>) -> Vec<(&'static str, Vec<bool>)> {
    let mut out: Vec<_> = shared_surfaces(body)
        .into_iter()
        .map(|wearers| {
            let kind = body.face_surface(wearers[0].0).unwrap().kind().name();
            let mut senses: Vec<bool> = wearers.iter().map(|&(_, s)| s).collect();
            senses.sort_unstable();
            (kind, senses)
        })
        .collect();
    out.sort_unstable();
    out
}

/// The entities of a STEP text, by id.
fn entities(text: &str) -> BTreeMap<&str, &str> {
    text.lines()
        .filter_map(|l| l.split_once(" = "))
        .map(|(id, rhs)| (id, rhs.trim_end_matches(';')))
        .collect()
}

/// The `#` references of a record, in order.
fn refs(rhs: &str) -> Vec<&str> {
    rhs.match_indices('#')
        .map(|(i, _)| {
            let len = rhs[i + 1..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rhs.len() - i - 1);
            &rhs[i..=i + len]
        })
        .collect()
}

/// Each `ADVANCED_FACE` of `text`: its line, and the surface entity it
/// cites.
fn faces_citing(text: &str) -> Vec<(&str, &str)> {
    text.lines()
        .filter(|l| l.contains("= ADVANCED_FACE("))
        .map(|l| (l, *refs(l.split_once(" = ").unwrap().1).last().unwrap()))
        .collect()
}

/// **Which faces the file says share a surface**: each surface entity
/// cited by several faces, as the set of its citing faces, each named by
/// the points of its bounds' vertices and its `same_sense`.
fn cited_partition(text: &str) -> BTreeSet<Vec<Named>> {
    let all = entities(text);
    let point = |id: &str| -> [i64; 3] {
        let xyz = tuple(all[refs(all[id])[0]]);
        micrometres([xyz[0], xyz[1], xyz[2]])
    };
    let mut by: BTreeMap<&str, Vec<Named>> = BTreeMap::new();
    for (line, surface) in faces_citing(text) {
        let rhs = line.split_once(" = ").unwrap().1;
        let face_refs = refs(rhs);
        let mut corners = Vec::new();
        for bound in &face_refs[..face_refs.len() - 1] {
            let lp = all[refs(all[bound])[0]];
            if lp.starts_with("VERTEX_LOOP(") {
                corners.push(point(refs(lp)[0]));
                continue;
            }
            for oe in refs(lp) {
                let edge = all[refs(all[oe])[0]];
                for v in &refs(edge)[..2] {
                    corners.push(point(v));
                }
            }
        }
        corners.sort_unstable();
        corners.dedup();
        let sense = rhs.trim_end_matches(';').ends_with(".T.)");
        by.entry(surface).or_default().push((corners, sense));
    }
    by.into_values()
        .filter(|faces| faces.len() > 1)
        .map(|mut faces| {
            faces.sort_unstable();
            faces
        })
        .collect()
}

/// The numbers of a record's last parenthesised tuple.
fn tuple(rhs: &str) -> Vec<f64> {
    let open = rhs.rfind('(').unwrap();
    let close = rhs[open..].find(')').unwrap() + open;
    rhs[open + 1..close]
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

/// `text` with face `line` citing a copy of the surface entity it cites,
/// printed alike under the id `copy`.
fn on_a_copy(text: &str, line: &str, copy: &str) -> String {
    let all = entities(text);
    let surface = *refs(line.split_once(" = ").unwrap().1).last().unwrap();
    let stated = format!("{surface} = {};", all[surface]);
    text.replacen(
        line,
        &line.replacen(&format!("{surface},"), &format!("{copy},"), 1),
        1,
    )
    .replacen(
        &stated,
        &format!("{stated}\n{}", stated.replacen(surface, copy, 1)),
        1,
    )
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
    let want = partition(native);
    assert!(
        !want.is_empty(),
        "{what}: the native twin wears a key on several faces"
    );
    let text = written(native);
    assert_eq!(
        cited_partition(&text),
        want,
        "{what}: the file cites one entity per shared key, from the faces that wear it"
    );
    let (body, joins) = healed(what, &text).unwrap_or_else(|e| {
        panic!(
            "{what}: the twin is {:?} at rest, the import refuses: {e}",
            verdict(native)
        )
    });
    assert_eq!(
        partition(&body),
        want,
        "{what}: the import's faces share as the file's do"
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
        kinds(&native).contains(&("plane", vec![false, true])),
        "the union leaves both y = 0 faces on one key, worn both ways: {:?}",
        kinds(&native)
    );
    held_to_its_twin("the Z-step union", &native);

    // The same file with the second y = 0 face citing a copy of the
    // entity, printed alike: two records, the extrude's state.
    let text = written(&native);
    let citing = faces_citing(&text);
    let twice_cited: Vec<&str> = citing
        .iter()
        .filter(|(_, s)| citing.iter().filter(|(_, t)| t == s).count() == 2)
        .map(|&(line, _)| line)
        .collect();
    let [_, second] = twice_cited[..] else {
        panic!("one entity cited by the two y = 0 faces: {twice_cited:?}")
    };
    let twice = on_a_copy(&text, second, "#999999");
    let body = imported("the Z-step on two printed-alike planes", &twice)
        .expect("the Z-step on two entities imports");
    assert!(
        partition(&body).is_empty(),
        "two entities printed alike, facing opposite ways, are two records: {:?}",
        kinds(&body)
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
        kinds(&native).contains(&("torus", vec![true, true])),
        "the wall is two faces of one torus key: {:?}",
        kinds(&native)
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
        kinds(&native)
            .iter()
            .any(|(kind, s)| *kind == "plane" && s[0] == s[1]),
        "the caps wear one plane key, facing one way: {:?}",
        kinds(&native)
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

/// The shell each face of `body` lies in.
fn shell_of(body: &Body<f64>) -> BTreeMap<topo::FaceKey, topo::ShellKey> {
    body.shells()
        .flat_map(|(k, shell)| shell.faces.iter().map(move |&f| (f, k)))
        .collect()
}

/// Whether every surface key of `body` lies in one shell.
fn no_key_crosses_a_shell(body: &Body<f64>) -> bool {
    let of = shell_of(body);
    shared_surfaces(body)
        .iter()
        .all(|wearers| wearers.iter().all(|&(f, _)| of[&f] == of[&wearers[0].0]))
}

/// **Import never shares a record across solids.** A slot cut through a
/// bar parts it in two solids whose tops, bottoms and sides keep the
/// bar's keys, so the file cites each of those entities from both
/// solids; each solid is built alone, and no key of the import crosses
/// from one to the other.
#[test]
fn a_surface_two_solids_cite_is_two_records() {
    let bar = brick((0.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let slot = brick((1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0));
    let native = result("the parted bar", topo::subtract(&bar, &slot, tol()));
    assert_eq!(native.solids().count(), 2, "the slot parts the bar");
    assert!(
        !no_key_crosses_a_shell(&native),
        "the twin's keys cross its solids"
    );
    let text = written(&native);
    assert!(
        !cited_partition(&text).is_empty(),
        "the file cites the bar's entities from both solids"
    );
    let body = imported("the parted bar", &text).expect("the parted bar imports");
    assert_eq!(body.solids().count(), 2, "two solids");
    assert!(
        no_key_crosses_a_shell(&body),
        "no surface key crosses a solid: {:?}",
        kinds(&body)
    );
}

/// **Import never shares a record across a solid's shells.** A bar with
/// two cavities, the second cavity's top re-cited onto the `PLANE` entity
/// of the first's (one plane, `z = 1.5`): the two voids are two shells,
/// each built alone, and their tops stay two records.
#[test]
fn a_surface_two_voids_cite_is_two_records() {
    let bar = brick((0.0, 5.0), (0.0, 2.0), (0.0, 2.0));
    let one = match topo::subtract(&bar, &brick((1.0, 2.0), (0.5, 1.5), (0.5, 1.5)), tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the first cavity cuts, got {:?}", other.map(|_| ())),
    };
    let native = result(
        "the two-cavity bar",
        topo::subtract(&one, &brick((3.0, 4.0), (0.5, 1.5), (0.5, 1.5)), tol()),
    );
    assert_eq!(native.shells().count(), 3, "an outer shell and two voids");
    let text = written(&native);
    let all = entities(&text);
    let tops: Vec<(&str, &str)> = faces_citing(&text)
        .into_iter()
        .filter(|&(_, plane)| {
            let rhs = all[plane];
            rhs.starts_with("PLANE(") && {
                let at = refs(all[refs(rhs)[0]]);
                let (origin, normal) = (tuple(all[at[0]]), tuple(all[at[1]]));
                origin[2] == 1.5 && normal[0] == 0.0 && normal[1] == 0.0
            }
        })
        .collect();
    let [(_, first), (second, other)] = tops[..] else {
        panic!("the two cavities' tops: {tops:?}")
    };
    assert_ne!(first, other, "the twin states the two tops on two entities");
    let recited = text.replacen(
        second,
        &second.replacen(&format!("{other},"), &format!("{first},"), 1),
        1,
    );
    assert_eq!(
        cited_partition(&recited).len(),
        1,
        "the file cites one plane from both tops"
    );
    let body = imported("the re-cited cavities", &recited).expect("the cavities import");
    assert_eq!(body.shells().count(), 3, "three shells");
    assert!(
        no_key_crosses_a_shell(&body),
        "no surface key crosses a shell: {:?}",
        kinds(&body)
    );
}

/// **A pinch**: opposite facings meeting at an edge. The ball's two
/// bands on one sphere, one cited reversed: a lamina at their shared
/// edges, refused at tier 3 on its geometry. Stated on two records of
/// one sphere, the edge between them refuses first, at the build.
#[test]
fn a_pinch_is_refused_at_tier_3_on_one_record_or_two() {
    // The first of the two meridians the bands meet along.
    const BAND_EDGE: u64 = 15;
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/ball.step"
    ))
    .unwrap();
    let citing = faces_citing(&text);
    let [(first, sphere), (_, again)] = citing[..] else {
        panic!("the ball is two bands: {citing:?}")
    };
    assert_eq!(sphere, again, "one sphere entity, cited twice");
    let flip = |s: &str| {
        if s.ends_with(".T.);") {
            s.replace(".T.);", ".F.);")
        } else {
            s.replace(".F.);", ".T.);")
        }
    };
    let pinched = text.replacen(first, &flip(first), 1);
    let pinched_on_two = on_a_copy(&pinched, &flip(first), "#999999");
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
    // records of one sphere refuses at the build, typed, before tier 3:
    // the section value decides the two records one sphere.
    match imported("two records", &pinched_on_two) {
        Err(StepImportError::NotYetSupported {
            entity: step_import::BuiltEntity::Edge,
            id,
            lane,
            ..
        }) if id == BAND_EDGE
            && lane.starts_with("one record of a surface stated twice that one_record") => {}
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
        kinds(&split).iter().any(|(kind, _)| *kind == "nurbs"),
        "the two arms wear one spline key: {:?}",
        kinds(&split)
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

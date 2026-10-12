//! **A finished body can wear one chart with both senses within one
//! solid, and `shell` refuses it `ChartSenseMixed`.** The import adopts
//! surfaces by bit signature per solid and keeps each face's own
//! `same_sense`, so a file whose two faces cite one plane, one of them
//! reversed, imports as one surface key worn both ways. The Z-step
//! prism's two faces on y = 0 face opposite ways; written on separate
//! planes it shells, and written on one plane it is still a finished
//! body, which the verb takes and refuses typed: a chart moves as one,
//! and a chart worn both ways has no one inward.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::Tol;
use step_export::{StepOptions, step_string};
use step_import::{ImportOptions, StepImport, import_step};
use topo::{AtRestBody, Body, FaceKey, ShellError};

/// The Z-step profile, counterclockwise: `[0,2]×[−1,0] ∪ [1,3]×[0,1]`.
const Z_STEP: [(f64, f64); 8] = [
    (0.0, -1.0),
    (2.0, -1.0),
    (2.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 0.0),
    (0.0, 0.0),
];

/// The record `#id = …;` of `text`, its right-hand side.
fn record<'a>(text: &'a str, id: &str) -> &'a str {
    let head = format!("{id} = ");
    text.lines()
        .find_map(|l| l.strip_prefix(head.as_str()))
        .unwrap_or_else(|| panic!("no record {id}"))
}

/// The `#` references in a record, in order.
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

/// The numbers in a record's last parenthesised tuple.
fn tuple(rhs: &str) -> Vec<f64> {
    let open = rhs.rfind('(').unwrap();
    let close = rhs[open..].find(')').unwrap() + open;
    rhs[open + 1..close]
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

/// `text` with its face on the plane y = 0 whose normal is +y re-cited
/// onto the plane of its face whose normal is −y, reversed.
fn share_the_y0_plane(text: &str) -> String {
    let mut by_normal: BTreeMap<i8, (String, String)> = BTreeMap::new();
    for line in text.lines() {
        let Some((id, rhs)) = line.split_once(" = ") else {
            continue;
        };
        if !rhs.starts_with("ADVANCED_FACE(") {
            continue;
        }
        let plane = *refs(rhs).last().unwrap();
        let Some(placement) = record(text, plane).strip_prefix("PLANE(") else {
            continue;
        };
        let at = refs(record(text, refs(placement)[0]));
        let origin = tuple(record(text, at[0]));
        let normal = tuple(record(text, at[1]));
        if origin[1] == 0.0 && normal[0] == 0.0 && normal[2] == 0.0 {
            by_normal.insert(normal[1] as i8, (id.to_owned(), plane.to_owned()));
        }
    }
    let ((up, up_plane), (_, down_plane)) = (&by_normal[&1], &by_normal[&-1]);
    let old = format!("{up} = {}", record(text, up));
    assert!(old.ends_with(&format!("{up_plane}, .T.);")), "{old}");
    let new = old.replace(
        &format!("{up_plane}, .T.);"),
        &format!("{down_plane}, .F.);"),
    );
    text.replace(&old, &new)
}

fn imported(text: &str) -> Body<f64> {
    match import_step(text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the Z-step imports as a solid, got {:?}", other.map(|_| ())),
    }
}

/// The charts of `body` worn by more than one face, each wearer with
/// its sense.
fn shared_charts(body: &Body<f64>) -> Vec<Vec<(FaceKey, bool)>> {
    let mut by: BTreeMap<_, Vec<(FaceKey, bool)>> = BTreeMap::new();
    for (k, f) in body.faces() {
        by.entry(f.surface).or_default().push((k, f.sense));
    }
    by.into_values().filter(|w| w.len() > 1).collect()
}

#[test]
fn a_chart_worn_both_ways_is_finished_and_shell_refuses_it() {
    let tol = Tol::witness();
    let text = step_string(
        &topo::test_support::prism::<f64>(&Z_STEP, 1.0, tol).body,
        &StepOptions::default(),
        tol,
    )
    .unwrap();

    let apart = imported(&text);
    assert!(shared_charts(&apart).is_empty(), "control: no chart shared");
    let apart = AtRestBody::validate(apart, tol).expect("control: the Z-step is finished");
    topo::shell(&apart, 0.1, tol).expect("control: the Z-step on two planes shells");

    let shared = imported(&share_the_y0_plane(&text));
    let charts = shared_charts(&shared);
    assert!(
        charts.len() == 1 && charts[0].len() == 2 && charts[0][0].1 != charts[0][1].1,
        "one chart, worn by the two y = 0 faces with opposite senses: {charts:?}"
    );
    assert_eq!(shared.solids().count(), 1, "one solid wears it");
    let wearers = [charts[0][0].0, charts[0][1].0];
    let shared = AtRestBody::validate(shared, tol)
        .unwrap_or_else(|e| panic!("the shared-chart Z-step is a finished body: {e:?}"));
    match topo::shell(&shared, 0.1, tol) {
        Err(ShellError::ChartSenseMixed { face, other }) => assert!(
            wearers.contains(&face) && wearers.contains(&other) && face != other,
            "the refusal names the two wearers {wearers:?}, got {face:?}, {other:?}"
        ),
        other => panic!(
            "want ChartSenseMixed, got {:?}",
            other.map(|s| s.body.faces().count())
        ),
    }
}

//! **An imported body measures through its own certificate and no
//! other** (PR 5741's review, M1).
//!
//! The import hands out the body and the gate's certificate of it as one
//! value ([`topo::CertifiedBody`]), so the review's probe — the
//! certificate of `box.step` measured against the same file scaled ×2,
//! whose face keys are the same because the two are built the same way,
//! answering the small box's volume for the big one — has no spelling:
//! `measure` takes no body, and the body comes out only by shared borrow
//! or with the certificate dropped (`topo::CertifiedBody`'s
//! `compile_fail` rows). What is left to run is that each of the twins
//! measures its own volume, that changing a copy of the body changes
//! nothing about the measurement, and that the door's own refusals reach
//! through it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use quantity::Length;
use step_import::{ImportOptions, StepImport, import_step};
use topo::{CertifiedBody, MassPropsError, Settle};

/// `box.step`, the review's subject.
fn box_step() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .expect("box.step reads")
}

/// `text` with every cartesian point scaled by `k` (the review's probe):
/// the same entities, so the same arena order and the same face keys.
fn scaled(text: &str, k: f64) -> String {
    text.lines()
        .map(|l| {
            let Some(i) = l.find("CARTESIAN_POINT('',(") else {
                return l.to_string();
            };
            let start = i + "CARTESIAN_POINT('',(".len();
            let end = l[start..].find(')').unwrap() + start;
            let nums: Vec<String> = l[start..end]
                .split(',')
                .map(|s| format!("{:?}", s.trim().parse::<f64>().unwrap() * k))
                .collect();
            format!("{}{}{}", &l[..start], nums.join(","), &l[end..])
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn imported(text: &str, tol: Tol) -> CertifiedBody {
    match import_step(text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the box must import as a solid: {other:?}"),
    }
}

/// The review's twins: the small box and the box twice its size, built
/// alike, each measured through its own certificate — and each answer is
/// that body's, held to the body measured fresh and to the ×8 ratio of
/// the two closed forms.
#[test]
fn twins_built_alike_each_measure_their_own_volume() {
    let tol = Tol::witness();
    let text = box_step();
    let small = imported(&text, tol);
    let big = imported(&scaled(&text, 2.0), tol);
    let keys = |b: &CertifiedBody| b.body().faces().map(|(f, _)| f).collect::<Vec<_>>();
    assert_eq!(
        keys(&small),
        keys(&big),
        "the probe's premise: twins built alike share their face keys"
    );
    let settle = || topo::test_support::pinned_resolution::<f64>(tol);
    let own = |b: &CertifiedBody| b.measure(settle()).expect("a box measures").enclosure;
    let fresh = |b: &CertifiedBody| {
        let finished = topo::test_support::finished("the box", b.body().clone(), tol);
        topo::mass_properties(&finished, tol, settle())
            .expect("a box measures fresh")
            .enclosure
    };
    for (what, b) in [("small", &small), ("big", &big)] {
        assert_eq!(
            format!("{:?}", own(b)),
            format!("{:?}", fresh(b)),
            "{what}: the certified body measures the body it holds"
        );
    }
    let (v_small, v_big) = (own(&small).volume, own(&big).volume);
    assert!(
        (v_big - 8.0 * v_small).abs() <= 1e-12 * v_big,
        "the big box encloses eight small ones: {v_big:e} against 8 × {v_small:e}"
    );
}

/// A body changed after import is a different body, and the certified
/// value cannot be it: the body leaves the pair by copy or with the
/// certificate dropped, so the certificate goes on measuring the body it
/// was derived over.
#[test]
fn a_copy_changed_after_import_leaves_the_measurement_as_it_was() {
    let tol = Tol::witness();
    let certified = imported(&box_step(), tol);
    let settle = || topo::test_support::pinned_resolution::<f64>(tol);
    let before = certified.measure(settle()).unwrap().enclosure;
    let mut copy = certified.body().clone();
    let face = copy.faces().next().map(|(f, _)| f).expect("a face");
    topo::replace_face_offset(&mut copy, face, -0.25 * 1e-3, tol).expect("a box face moves");
    let moved = topo::test_support::pinned_volume(
        "the moved box",
        &topo::test_support::finished("the moved box", copy, tol),
        tol,
    )
    .volume;
    assert!(
        (moved - before.volume).abs() > 1e-3 * before.volume,
        "the changed copy is a different box: {moved:e} against {:e}",
        before.volume
    );
    assert_eq!(
        format!("{:?}", certified.measure(settle()).unwrap().enclosure),
        format!("{before:?}"),
        "the certified body's measurement does not move with a copy"
    );
}

/// The door's own refusals reach through the certified body: a
/// resolution finer than ε is refused typed (review n5).
#[test]
fn a_resolution_finer_than_eps_is_refused_through_the_certified_body() {
    let tol = Tol::witness();
    let certified = imported(&box_step(), tol);
    let finer = Settle::Resolution(Length::from_meters(0.5 * tol.eps()));
    assert_eq!(
        certified.measure(finer).map(|m| m.stop),
        Err(MassPropsError::ResolutionFinerThanEps {
            resolution: 0.5 * tol.eps(),
            eps: tol.eps(),
        }),
    );
}

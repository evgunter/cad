//! The tour's one integration-test binary: every `tests/*.rs` suite is a
//! module of it (`autotests = false` in Cargo.toml), so the release build
//! links one test binary and instantiates the kernel's generics once for
//! all of them rather than once per suite. A suite's tests are named
//! `<suite>::<test>` under binary `all`.
//!
//! A suite file that is not listed here does not compile and does not run;
//! `every_suite_is_a_module` fails when one is missing.

mod common;

mod blend1_r1_wall6_probes;
mod eps_regression;
mod k_probe_brackets;
mod review_blend2_r1_probes;
mod teapot_document;
mod verbs_demo2_r1_probes;
mod verbs_teapot;
mod verbs_teapot_r1_probes;
mod verbs_teapot_r2_probes;
mod verbs_torus_vessel;

#[test]
fn every_suite_is_a_module() {
    let here = include_str!("all.rs");
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("read tests/") {
        let path = entry.expect("tests/ entry").path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let is_suite = path.extension().is_some_and(|e| e == "rs") && stem != "all";
        if is_suite && !here.lines().any(|l| l == format!("mod {stem};")) {
            missing.push(stem.to_owned());
        }
    }
    assert!(
        missing.is_empty(),
        "suites not declared in tests/all.rs: {missing:?}"
    );
}

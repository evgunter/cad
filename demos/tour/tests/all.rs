//! The tour's one integration-test binary: every `tests/*.rs` suite is
//! mounted here by `#[path]` (`autotests = false` in Cargo.toml), so the
//! release build links one test binary and instantiates the kernel's
//! generics once for all of them rather than once per suite. A suite's
//! tests are named `<suite>::<test>` under binary `all`.
//!
//! The helpers the suites share are declared once, as `mod common;`, and a
//! suite says `use crate::common::<helper>;`. A suite file that is not
//! mounted here does not compile and does not run;
//! `every_suite_file_is_aggregated` (the kernel crates' row, from
//! `test_utils`) fails when one is missing.

mod common;

#[path = "blend1_r1_wall6_probes.rs"]
mod blend1_r1_wall6_probes;
#[path = "bracket.rs"]
mod bracket;
#[path = "chaintol.rs"]
mod chaintol;
#[path = "eps_regression.rs"]
mod eps_regression;
#[path = "gallery.rs"]
mod gallery;
#[path = "heatsink.rs"]
mod heatsink;
#[path = "k_probe_brackets.rs"]
mod k_probe_brackets;
#[path = "klein.rs"]
mod klein;
#[path = "lily.rs"]
mod lily;
#[path = "mate7a_r2_probes.rs"]
mod mate7a_r2_probes;
#[path = "mcplate.rs"]
mod mcplate;
#[path = "projectbox.rs"]
mod projectbox;
#[path = "review_blend2_r1_probes.rs"]
mod review_blend2_r1_probes;
#[path = "scene_body_delta.rs"]
mod scene_body_delta;
#[path = "skinned.rs"]
mod skinned;
#[path = "snowman.rs"]
mod snowman;
#[path = "teapot.rs"]
mod teapot;
#[path = "teapot_document.rs"]
mod teapot_document;
#[path = "tolerance.rs"]
mod tolerance;
#[path = "torusvessel.rs"]
mod torusvessel;
#[path = "twopeg.rs"]
mod twopeg;
#[path = "verbs_demo2_r1_probes.rs"]
mod verbs_demo2_r1_probes;
#[path = "verbs_teapot.rs"]
mod verbs_teapot;
#[path = "verbs_teapot_r1_probes.rs"]
mod verbs_teapot_r1_probes;
#[path = "verbs_teapot_r2_probes.rs"]
mod verbs_teapot_r2_probes;
#[path = "verbs_torus_vessel.rs"]
mod verbs_torus_vessel;

test_utils::every_suite_file_is_aggregated!();

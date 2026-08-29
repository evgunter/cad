//! Aggregated integration-test binary for `editor-core`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! The files themselves are untouched: each keeps its own `//!` docs, its inner
//! attributes (`#![cfg(feature = "interval")]` and friends work as
//! module-level attributes), and its own `mod <helper>;` lines — a
//! `#[path]` module's child modules resolve against the DIRECTORY
//! CONTAINING the path file, i.e. `tests/`, exactly as when each file was
//! its own crate root.
//!
//! WHY ONE BINARY: on the CI runner (2 vCPU) the per-binary codegen+link
//! constant dominated the workspace build job — the suites are small, so
//! that constant was the bill. The figures are deliberately NOT restated
//! here: they were measured once, nothing in the repo re-takes them, and
//! the LINK/DEBUGINFO note in .github/workflows/ci.yml is the one place
//! that carries them with their date, their provenance run and the record
//! of what has since changed.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run — `every_suite_file_is_
//! aggregated` below fails loudly if you forget.
//!
//! Test IDs gain a module prefix (`export::round_trip` rather than
//! `round_trip`, under binary `all` rather than binary `export`); the set
//! of tests is otherwise identical.

// Each suite keeps its own verbatim `mod <helper>;`, so a shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

#[path = "asm1_identity_pins.rs"]
mod asm1_identity_pins;
#[path = "asm2a_instantiate.rs"]
mod asm2a_instantiate;
#[path = "asm2b_multisolid.rs"]
mod asm2b_multisolid;
#[path = "asm4_split_inline.rs"]
mod asm4_split_inline;
#[path = "asm_r2a_mate_solve.rs"]
mod asm_r2a_mate_solve;
#[path = "asm_r2a_schema_v13.rs"]
mod asm_r2a_schema_v13;
#[path = "asm_r2b_assembly.rs"]
mod asm_r2b_assembly;
#[path = "asm_r2b_schema_v14.rs"]
mod asm_r2b_schema_v14;
#[path = "asm_roots.rs"]
mod asm_roots;
#[path = "asm_upd_pin_update.rs"]
mod asm_upd_pin_update;
#[path = "asm_upd_schema_v10.rs"]
mod asm_upd_schema_v10;
#[path = "boolean_op_wire.rs"]
mod boolean_op_wire;
#[path = "dsc_checks.rs"]
mod dsc_checks;
#[path = "e4_dual_door.rs"]
mod e4_dual_door;
#[path = "gui1_pick.rs"]
mod gui1_pick;
#[path = "gui1_pick_r2.rs"]
mod gui1_pick_r2;
#[path = "lbret_schema_v8.rs"]
mod lbret_schema_v8;
#[path = "lib_doors_node_result.rs"]
mod lib_doors_node_result;
#[path = "lib_g14_split_walls.rs"]
mod lib_g14_split_walls;
#[path = "lib_placedunion.rs"]
mod lib_placedunion;
#[path = "lib_sel1_geoselect.rs"]
mod lib_sel1_geoselect;
#[path = "lib_sel2_flush.rs"]
mod lib_sel2_flush;
#[path = "lib_u5_interrogate.rs"]
mod lib_u5_interrogate;
#[path = "lib_u7_select.rs"]
mod lib_u7_select;
#[path = "m10_1_analysis.rs"]
mod m10_1_analysis;
#[path = "m10_1_r2_probes.rs"]
mod m10_1_r2_probes;
#[path = "m10_1_schema_v15.rs"]
mod m10_1_schema_v15;
#[path = "m4_pr1_dims.rs"]
mod m4_pr1_dims;
#[path = "m4_pr1_doc.rs"]
mod m4_pr1_doc;
#[path = "m4_pr1_eval.rs"]
mod m4_pr1_eval;
#[path = "m4_pr1_paths.rs"]
mod m4_pr1_paths;
#[path = "m4_pr2_eval.rs"]
mod m4_pr2_eval;
#[path = "m4_pr2_eval_interval.rs"]
mod m4_pr2_eval_interval;
#[path = "m4_pr2_wire.rs"]
mod m4_pr2_wire;
#[path = "m4_pr3_names.rs"]
mod m4_pr3_names;
#[path = "m4_pr3_names_bool.rs"]
mod m4_pr3_names_bool;
#[path = "m4_pr3_names_ci.rs"]
mod m4_pr3_names_ci;
#[path = "m4_pr3_names_interval.rs"]
mod m4_pr3_names_interval;
#[path = "m4_pr3_names_rework.rs"]
mod m4_pr3_names_rework;
#[path = "m4_pr4_appearance_hook.rs"]
mod m4_pr4_appearance_hook;
#[path = "m4_pr4_banked.rs"]
mod m4_pr4_banked;
#[path = "m4_pr4_ci.rs"]
mod m4_pr4_ci;
#[path = "m4_pr4_diff.rs"]
mod m4_pr4_diff;
#[path = "m4_pr4_edits.rs"]
mod m4_pr4_edits;
#[path = "m4_pr4_hit.rs"]
mod m4_pr4_hit;
#[path = "m4_pr4_interval.rs"]
mod m4_pr4_interval;
#[path = "m4_pr4_resolve.rs"]
mod m4_pr4_resolve;
#[path = "m4_pr5_declare.rs"]
mod m4_pr5_declare;
#[path = "m4_pr6_eps_diff.rs"]
mod m4_pr6_eps_diff;
#[path = "m4_pr6_floats.rs"]
mod m4_pr6_floats;
#[path = "m4_pr6_golden.rs"]
mod m4_pr6_golden;
#[path = "m4_pr6_refusal.rs"]
mod m4_pr6_refusal;
#[path = "m4_pr6_review_probes.rs"]
mod m4_pr6_review_probes;
#[path = "m4_pr6_roundtrip.rs"]
mod m4_pr6_roundtrip;
#[path = "m4_pr6_roundtrip_interval.rs"]
mod m4_pr6_roundtrip_interval;
#[path = "m4_pr7_appearance.rs"]
mod m4_pr7_appearance;
#[path = "m4_pr7_appearance_interval.rs"]
mod m4_pr7_appearance_interval;
#[path = "m4_pr8_corpus.rs"]
mod m4_pr8_corpus;
#[path = "m4_pr8_corpus_interval.rs"]
mod m4_pr8_corpus_interval;
#[path = "m4_pr8_k_probe.rs"]
mod m4_pr8_k_probe;
#[path = "m4_pr8_latency.rs"]
mod m4_pr8_latency;
#[path = "m5_pr10_nodes.rs"]
mod m5_pr10_nodes;
#[path = "m5_pr10_schema_v2.rs"]
mod m5_pr10_schema_v2;
#[path = "m5_pr11_corpus_curved.rs"]
mod m5_pr11_corpus_curved;
#[path = "m5_pr12_fillet_node.rs"]
mod m5_pr12_fillet_node;
#[path = "m5_pr5_corpus.rs"]
mod m5_pr5_corpus;
#[path = "m5_pr5_corpus_probe.rs"]
mod m5_pr5_corpus_probe;
#[path = "m5_pr6_pcurve_persistence.rs"]
mod m5_pr6_pcurve_persistence;
#[path = "m5_pr8_bvh_diff.rs"]
mod m5_pr8_bvh_diff;
#[path = "m5_s1_rest_declare.rs"]
mod m5_s1_rest_declare;
#[path = "m6_5_downstream.rs"]
mod m6_5_downstream;
#[path = "m6_5_schema_v3.rs"]
mod m6_5_schema_v3;
#[path = "m6_5_selection_refusals.rs"]
mod m6_5_selection_refusals;
#[path = "m6_composed_node.rs"]
mod m6_composed_node;
#[path = "m9_1_declare_classes.rs"]
mod m9_1_declare_classes;
#[path = "m9_1_schema_v11.rs"]
mod m9_1_schema_v11;
#[path = "m9_d1_r1_probes.rs"]
mod m9_d1_r1_probes;
#[path = "m9_d1_r2_probes.rs"]
mod m9_d1_r2_probes;
#[path = "placedunion_schema_v12.rs"]
mod placedunion_schema_v12;
#[path = "respell_schema_v9.rs"]
mod respell_schema_v9;
#[path = "review_gui1_r1.rs"]
mod review_gui1_r1;
#[path = "review_m4_pr1.rs"]
mod review_m4_pr1;
#[path = "review_m4_pr1_die.rs"]
mod review_m4_pr1_die;
#[path = "review_m4_pr2.rs"]
mod review_m4_pr2;
#[path = "review_m5_pr10_schema.rs"]
mod review_m5_pr10_schema;
#[path = "review_m5_pr10_sweep_node.rs"]
mod review_m5_pr10_sweep_node;
#[path = "review_m5_pr1_e2e_interval.rs"]
mod review_m5_pr1_e2e_interval;
#[path = "review_m5_pr9_doc_probe.rs"]
mod review_m5_pr9_doc_probe;
#[path = "review_m6_5_pr2_probes.rs"]
mod review_m6_5_pr2_probes;
#[path = "ring_r1_names_probe.rs"]
mod ring_r1_names_probe;
#[path = "schema_ledger.rs"]
mod schema_ledger;
#[path = "switch_display_units.rs"]
mod switch_display_units;
#[path = "switch_dump.rs"]
mod switch_dump;
#[path = "switch_naming.rs"]
mod switch_naming;
#[path = "switch_plate_param.rs"]
mod switch_plate_param;
#[path = "switch_program_key.rs"]
mod switch_program_key;
#[path = "switch_program_vocabulary.rs"]
mod switch_program_vocabulary;
#[path = "switch_slots.rs"]
mod switch_slots;
#[path = "u8a_parse.rs"]
mod u8a_parse;

/// Guards the `autotests = false` hazard: a suite file added to `tests/`
/// but not declared above would silently stop being compiled and run.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let src = include_str!("all.rs");
    let mut missing: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("tests/ is readable") {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let name = path
            .file_name()
            .expect("file has a name")
            .to_string_lossy()
            .to_string();
        if name == "all.rs" {
            continue;
        }
        if !src.contains(&format!("#[path = \"{name}\"]")) {
            missing.push(name);
        }
    }
    missing.sort();
    assert!(
        missing.is_empty(),
        "tests/*.rs suites are not declared in tests/all.rs, so `autotests = false` \
         is silently dropping them: {missing:?}. Add a `#[path]` line for each."
    );
}

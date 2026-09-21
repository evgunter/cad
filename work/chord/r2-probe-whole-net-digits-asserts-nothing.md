---
id: r2-probe-whole-net-digits-asserts-nothing
kind: issue
title: nurbs_cert::tests::r2_probe_whole_net_digits is a non-ignored #[test] that asserts nothing
status: open
opened: 2026-09-18
---

## Finding

`nurbs_cert::tests::r2_probe_whole_net_digits`
(`crates/mesh/src/nurbs_cert.rs`) is a non-ignored `#[test]` whose body is a
loop of `println!` — it prints `whole_net_bound`'s five figures at
`{:.17e}` for three fixtures and asserts nothing. Its only failure mode is
an `unwrap` on `whole_net_bound`, which `cert10`'s rows already exercise on
the same fixtures. `memories/test-suite-cost.md`: *"A test that asserts
nothing is never a gate."*

Its sibling `cert10_fold_cost_table` in the same file is the shape to copy
if the printout is still wanted: `#[ignore = "measurement harness: …"]`.
Otherwise delete it, or turn the digits it prints into pins.

Found while sweeping `crates/mesh` for illegible domination messages (PR
2848); not touched there.

---
id: census-files-each-cut-a-files-shipped-part-by-hand
kind: issue
title: Eight source censuses each cut a file's shipped part by hand; one cut at the first cfg(test) line hid four copysign sites
status: open
opened: 2026-11-09
priority: P1
cost: M
---



A census that counts what SHIPPED code holds has to cut a file's
test-only code away first, and eight of them each spell that cut:

- `crates/geom-core/tests/certified_endpoint_census.rs` — `production_sources` and `production`, which mirror `scripts/gates/lib.sh`'s `gate_production_sources` and `--skip-cfg-test` line for line (test-only `mod` mounts dropped, every test-only item skipped by brace depth). The fullest of the eight.
- `crates/geom-core/tests/sym_rule_f_rows.rs` — `the_copysign_mint_sites_the_tree_holds_are_these` cut each file at its first line reading `#[cfg(test)]`. `crates/topo/src/sector_face.rs` carries a `#[cfg(test)]` accessor above its cone-apex readers, so the census never saw four `.copysign(` sites there (PR 5956 onward). Rerouted onto the endpoint census's view in `onecopy/main-reds-censuses`.
- `crates/editor-core/tests/msolve7_member_residue.rs` — `code.find("#[cfg(test)]")`, the same first-hit cut.
- `crates/mesh/tests/all.rs` — a line equal to `#[cfg(test)]` that mounts no module.
- `crates/editor-core/tests/node_standing.rs`, `crates/profile/tests/recourse_roster.rs`, `crates/topo/src/boolean/offer_rows.rs`, `crates/topo/src/tier3_tests.rs`, `crates/geom-brep/tests/kstats_escalation_channel.rs`, `tools/k-lint/tests/predicate_roster.rs` — each blanks `#[cfg(test)]` items its own way, some through `test_utils::source::attributed_extent`.

Swept with `grep -rn '"#\[cfg(test)\]"'` over `crates/*/tests`, `crates/*/src` and `tools`. It cannot see a census that spells the attribute another way (a `cfg(all(test, …))` match, a regex), or one that keeps test code without cutting it at all.

**What closing this needs**: one production view in `test_utils::source` — the endpoint census's two functions moved there, since they are the one copy held to the gate's reader — with every census above reading it. Some of them may want a narrower view on purpose (the first-hit cuts are not that); each keeps its own cut only with that stated.

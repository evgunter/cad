---
id: census-files-each-cut-a-files-shipped-part-by-hand
kind: issue
title: Nine source censuses each cut a file's shipped part by hand; a tenth, the copysign census, cut at the first cfg(test) line and hid four sites until PR 6610
status: open
opened: 2026-11-09
priority: P1
cost: M
---



A census that counts what SHIPPED code holds has to cut a file's
test-only code away first. Ten census files spelled that cut, each its
own way; PR 6610 rerouted one (the copysign census) onto another (the
endpoint census), so nine spellings stand:

- `crates/geom-core/tests/certified_endpoint_census.rs` — `production_sources` and `production`, which mirror `scripts/gates/lib.sh`'s `gate_production_sources` and `--skip-cfg-test` line for line (test-only `mod` mounts dropped, every test-only item skipped by brace depth). The fullest of the ten, and the home the copysign census now reads.
- `crates/geom-core/tests/sym_rule_f_rows.rs` — `the_copysign_mint_sites_the_tree_holds_are_these` cut each file at its first line reading `#[cfg(test)]`. `crates/topo/src/sector_face.rs` carries a `#[cfg(test)]` accessor above its cone-apex readers, so the census never saw four `.copysign(` sites there (PR 5956 onward). Rerouted onto the endpoint census's view in `onecopy/main-reds-censuses`.
- `crates/editor-core/tests/msolve7_member_residue.rs` — `code.find("#[cfg(test)]")`, the same first-hit cut.
- `crates/mesh/tests/all.rs` — a line equal to `#[cfg(test)]` that mounts no module.
- `crates/editor-core/tests/node_standing.rs`, `crates/profile/tests/recourse_roster.rs`, `crates/topo/src/boolean/offer_rows.rs`, `crates/topo/src/tier3_tests.rs`, `crates/geom-brep/tests/kstats_escalation_channel.rs`, `tools/k-lint/tests/predicate_roster.rs` — each blanks `#[cfg(test)]` items its own way, some through `test_utils::source::attributed_extent`.

**How each was found.** `grep -rln '"#\[cfg(test)\]"'` over `crates/*/tests`, `crates/*/src` and `tools`, less `test_utils::source`'s own unit rows, returned nine files at PR 6610's base (`79044571`): the eight in the last three bullets and `sym_rule_f_rows.rs`, whose cut was `l.trim() == "#[cfg(test)]"`. On main after PR 6610 it returns the eight. `certified_endpoint_census.rs` is not a hit: it matches the attribute as `"#[cfg("` and parses the body (`is_test_only_cfg`), and it was found by reading it as the census the copysign census was rerouted onto. So the grep cannot see a census that spells the attribute another way (a `cfg(` prefix match, a `cfg(all(test, …))` match, a regex), and neither can this list. Nor can it see a census that keeps test code without cutting it at all.

**What closing this needs**: one production view in `test_utils::source` — the endpoint census's two functions moved there, since they are the one copy held to the gate's reader — with every census above reading it. Some of them may want a narrower view on purpose (the first-hit cuts are not that); each keeps its own cut only with that stated.

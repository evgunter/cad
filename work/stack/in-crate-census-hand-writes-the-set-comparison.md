---
id: in-crate-census-hand-writes-the-set-comparison
kind: issue
title: stackup.rs's one-example-per-Unavailable-arm census hand-writes the roster comparison instead of test_utils::census::set_difference
status: open
opened: 2026-09-28
priority: P4
cost: E
---

Filed by GATHER's name-edge-home lane (PR 3321, review item S4), on
STACK's slate because `crates/editor-core/src/stackup.rs` is STACK's
ground (`work.py territory`: props, stack; STACK's `paths` names only
this file).

**The finding.** `stackup.rs`, `every_arm` in the test module (~1898-1922),
builds one example per `Unavailable` arm and compares them with the
`UNAVAILABLE` census's roster by hand. It sorts two `Vec<String>`s and
`assert_eq!`s them, with the message "one example per Unavailable arm, no
more" (~1912-1920). `test_utils::census::set_difference`
(`crates/test-utils/src/census.rs`) exists for this shape. It prints only
the failing direction, and each direction has its own recourse.
`crates/test-utils/src/f6.rs` (~213-216) calls a re-spelled pair "a second
copy of the mechanism". A hand-written `assert_eq!` on sorted vectors
prints both whole lists. It also reports a duplicated example as a
mismatch without saying which side is short.

**The sweep.** In-crate censuses that read `variant_identifier` or a
census's `.identifiers()`, across `crates/`, `tools/` and `demos/`:

| site | comparison |
|---|---|
| `crates/editor-core/src/doc.rs` (~1429, `CARRIER`) | uses `set_difference` |
| `crates/editor-core/src/persist/check.rs` (~1800 `SNAPSHOT_ERROR`, ~1816 `WALK`) | uses `set_difference` |
| `crates/editor-core/src/names/geompred.rs` (`mod census`) | uses `set_difference` (PR 3321's fix pass) |
| `crates/editor-core/src/stackup.rs` (~1912-1920, `UNAVAILABLE`) | **hand-written sorted `assert_eq!`: this row** |
| `crates/topo/tests/display_contract.rs` (~113), `crates/viewer/tests/panel_edits.rs` (~846) | read `.identifiers()` as a ban list, not a set comparison |
| `crates/editor-core/tests/display_contract.rs` (~2420) | `variant_identifier` on single values, not a roster comparison |

**What would close it.** Replace the sort-and-`assert_eq!` in `every_arm`
with `test_utils::census::set_difference(UNAVAILABLE.identifiers(), &made, …)`
and a `panic!` on its report. Say in the report which side is missing what.

---
id: source-readers-list-the-item-visibilities-they-have-met
kind: issue
title: Source readers that find an item by listing the visibilities they have met (pub, pub(crate)) do not see one declared pub(super) or pub(in ..) - six sites, modules and fns
status: open
opened: 2026-11-15
priority: P4
cost: E
---



Filed by GREEN's `green/nightly-1115` lane (PR 6985), from the sibling
sweep of the defect it fixed in `crates/profile/tests/raw_door_census.rs`:
that census found a file's test module by `mod`, `pub mod` or
`pub(crate) mod`, PR 6950 declared `mesh::trimmed`'s test module
`pub(super) mod tests`, and the census read that module's rows as
production source (red on the nightlies of 2026-11-14 and 11-15).

**The home now exists.** PR 6985 adds
`test_utils::source::after_visibility` (an item's text past whatever
visibility it carries) and reads it in `source::test_mounted` and in the
profile census. The readers below still spell the list by hand.

Modules:

- `crates/topo/src/boolean/offer_rows.rs` (near `strip_prefix("mod ")`):
  strips `pub(crate) ` then `pub `.
- `crates/mesh/tests/all.rs`, `mounts_a_module`: `mod ` and `pub mod `
  only.
- `crates/topo/src/link.rs`: finds its own test module by the exact text
  `"\n#[cfg(test)]\nmod tests {"`.
- `crates/pncad/tests/all.rs` (the façade's shadow modules):
  `strip_prefix("mod ")` on a trimmed line, so only a private `mod x {}`.
  That is what it means to find, so it may be right as it stands; say so
  in place if it is.

Functions, the same class one item kind over:

- `crates/topo/src/pcurves/staleness_posture.rs`: `pub(crate) fn ` or
  `fn ` (also a hand-rolled reader:
  `work/charttail/the-ring-promotion-posture-row-reads-euler-kill-rs-by-hand.md`).
- `crates/viewer/src/widgets.rs`: `pub(crate) fn ` or `pub fn `.

Hits of the sweep that are NOT this defect:

- `crates/topo/src/face_normal.rs`: `find("mod tests {")` is a substring
  search, so it matches at any visibility.
- `crates/test-utils/src/source.rs` `file_module_decls`: a token scan
  that takes any visibility.
- `tools/k-lint/tests/predicate_roster.rs`, `crates/geom-core/tests/flagged_census.rs`,
  `crates/geom-brep/tests/kstats_escalation_channel.rs`,
  `crates/pncad-py/src/tests.rs`, and every `.py` and `.sh` hit: fixture
  text a self-test writes, not a reader.

Each live site fails open: an item it does not see is one it does not
check, or (as in the profile census) production source it over-reads.

Swept with
`grep -rn '"mod tests\|"mod "\|pub(crate) mod ")\|"pub(crate) fn \|"pub fn '`
over `.rs`, `.py` and `.sh` in `crates tools demos scripts`. That cannot
match a reader that finds items by regex or through the shared lexer's
item walk, and it was not widened to `struct`, `enum`, `const` or
`impl`.

Moved from TCOST to GREEN on 2026-11-20, by `git mv`, when TCOST merged into GREEN: TCOST held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GREEN carries.

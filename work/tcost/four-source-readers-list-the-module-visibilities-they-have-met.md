---
id: four-source-readers-list-the-module-visibilities-they-have-met
kind: issue
title: Four more source readers find a module declaration by listing the visibilities they have met, so a pub(super) or pub(in ..) module is invisible to them
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

The same shape, not red today:

- `crates/test-utils/src/source.rs`, `file_module_decls`: strips
  `pub(crate) ` then `pub ` before `mod `. It is the shared door, so
  every caller inherits the gap.
- `crates/topo/src/boolean/offer_rows.rs` (near `strip_prefix("mod ")`):
  the same two prefixes.
- `crates/mesh/tests/all.rs`, `mounts_a_module`: `mod ` and `pub mod `
  only.
- `crates/topo/src/link.rs`: finds its own test module by the exact text
  `"\n#[cfg(test)]\nmod tests {"`.

Each fails open: a module it does not see is one it does not check, or
(as in the profile census) production source it over-reads. The fix
there is `after_mod_keyword`, eight lines; the natural home for one copy
is `test_utils::source`.

Swept with
`grep -rn '"mod tests\|"mod "\|pub(crate) mod ")' crates tools demos scripts`
over `.rs`, `.py` and `.sh`. That cannot match a reader that finds
modules by regex or through the shared lexer's item walk; the `.py` and
`.sh` hits were all fixtures written by gate self-tests, not readers.

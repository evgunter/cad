---
id: prose-census-negative-reads-are-guarded-only-by-a-non-empty-allowlist
kind: issue
title: prose_census's raise-site scan and census() read the tree through equalities an empty scan passes only while an allowlist happens to be non-empty; refuse at the scan
status: open
opened: 2026-09-25
priority: P3
cost: E
---


## Finding

Raised by the style review of PR 3141 (GATHER's E-row batch). That PR
decided `a-source-census-scan-that-matches-nothing-should-refuse-at-the-scan`
for `test_utils::source`: a scan whose file is required to carry its
needle refuses at the read (`required_matches`, `sentinel_region`). The
same shape stands one crate over, in `crates/pncad-py/src/prose_census.rs`,
which is LIB's ground.

- `raise_sites_rendering_debug` is guarded by
  `no_raise_site_composes_a_debug_rendering_into_its_message`, which
  compares its findings against `KNOWN_DEBUG_RAISES` for equality. That
  comparison can see a scan that read nothing only while
  `KNOWN_DEBUG_RAISES` has at least one entry. Repair its last entry, and
  a scan that silently stopped matching passes as a clean tree.
- `census()` is guarded against an empty read by
  `the_census_reads_a_tree_and_not_an_empty_one`, a `> 200` floor in a
  separate test, and by `KNOWN_BRACED` / `UNDECIDED` equalities, which
  have the same dependence on their allowlists being non-empty. The floor
  sits at the consumer, not at the read.

## What a taker owes

For each scan, decide whether its input is required to produce sites.
If it is, refuse inside the scan, naming the needle, so the guard stops
depending on an allowlist that is meant to shrink to empty. If it is
not, say why at the scan.

**Not read for this shape:**
`crates/topo/tests/certified_enclosure_impl_census.rs` and
`crates/test-utils/tests/deny_unknown_fields_census.rs`. Sweep them with
the same question.

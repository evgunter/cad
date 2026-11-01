---
id: topo-tests-notched-beams-fixture-is-verbatim-in-two-files
kind: issue
title: The four-box notched_beams fixture is verbatim in crosslap_rest and m5_s1_rest_zip
status: open
opened: 2026-09-16
priority: P4
cost: E
---

## Finding

- **Where**: `crates/topo/tests/crosslap_rest.rs`'s `notched_beams`
  against the body of `m5_s1_rest_zip.rs`'s
  `rest_subtract_and_intersect_resolve_structurally`.
- **Importance**: medium
- **Confidence**: sure
- **Raised by**: the style review of the `dup-brick` lane's PR (S-DUP),
  2026-09-16

Two beams, each notched by subtracting a cutter: same four coordinate
sets, same z ranges, same variable names (`beam_a`, `cut_a`, `beam_b`,
`cut_b`), same `"notch A yields a body"` / `"notch B yields a body"`
panic strings. One is a named `fn notched_beams() -> (Body, Body)`; the
other is the same eight statements inlined into a `#[test]`.

It needs a home decision — `crosslap_rest.rs`'s version returns the pair
and asserts the exact dyadic volume of each notched beam before handing
them over, which is a fixture assertion the other file does not want
where it sits — so it is a row and not a mechanical fix.

**The `dup-brick` PR touched both halves**: all eight box constructions
in the pair were converted from `prism_z` with a literal rectangle
profile to `common::brick`. That made the two copies *more* alike, not
less — they are still verbatim to each other — so the citation above is
against the post-merge text and the copy is unaffected either way.

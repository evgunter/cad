---
id: genus-rings-helper-spelled-nine-times
kind: issue
title: Test-only genus/rings helper is spelled nine times across five crates
status: closed
opened: 2026-08-28
closed: 2026-09-12
github: 1123
refs: [1082, 1099]
---

## From GitHub issue 1123

Opened 2026-08-28; 0 comments.

`rings(&body)` (sum of `Face::rings.len()`) and `genus(&body)` (the
Euler–Poincaré identity with a parity check before halving) are
hand-copied into nine test files across five crates:

- `crates/topo/src/review_m1_pr3.rs`, `crates/topo/src/review_m1_pr4.rs`
- `crates/sweep/tests/verbs_shell.rs`,
  `crates/sweep/tests/verbs_shell_r2_probes.rs`,
  `crates/sweep/tests/verbs_shell_r2b.rs`,
  `crates/sweep/tests/shellfix1_r1_probes.rs`
- `demos/tour/src/teapot.rs`, `demos/tour/tests/verbs_teapot.rs`,
  `demos/tour/tests/verbs_teapot_r2_probes.rs`

Two of those copies were added by the #1082 repair (PR #1099) and two
more arrived with its review probe branches, which is what surfaced
this: the class grows one copy per suite that ever asks a body its
genus, and it will keep growing.

**Why it is not free.** `genus` is three lines of a published
identity, but the copies are not identical in the part that matters:
some assert `chi % 2 == 0` before halving and some do not, and an odd
`v − e + f − r` is a census that does not satisfy Euler–Poincaré at
all — halving it silently produces a plausible number. A drift in that
guard is a drift in what a green row means.

**Why it was not simply collapsed here.** `demos/tour` is a separate
workspace and an integration test cannot import a binary's module, so
no single existing home covers all nine; `topo`'s `test_support`
feature covers the kernel crates and `demos/tour` would still need its
own. The fix is a shared test-support surface (probably
`topo::test_support`, re-exported through `pncad` for the tour) plus
one deletion pass — a small, mechanical change that wants doing in one
go rather than one file at a time.

Until then each copy carries a pointer to this issue at its
definition, so a reader who finds one knows the other eight exist.

Found by: VERBS-SHELLFIX PR-1 review (ordinal 101), R1 + R2 NOTE.

## Home

A duplicated-spelling structural finding — the code-quality register's ground; a live finding no row cites is a `kind: issue` file there.

## Re-homed to SUITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

SUITE collects the test-side rows — the suites, the fixtures and the
shared helpers — that S-TINT's and S-TCOST's own boards did not carry.
This row is one of them.

Its class at the cut was **M** — mechanical deletions, but the shared
home must cross a separate workspace; copies now 11, not 9. The class is
a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Closed by the census door (2026-09-12, PR 2131)

`topo::readback::euler_counts` / `EulerCounts::genus` is the shared home
the row asked for — a public kernel door rather than a test-support
surface, so it reaches the detached `demos/tour` workspace through
`pncad::topo` with nothing re-exported. PR 2131 delegates or deletes
every copy the row names, and the two the re-home counted on top:

- `crates/topo/src/review_m1_pr3.rs` — `GenusInputs`/`genus_inputs`/
  `genus` folded onto `EulerCounts`/`euler_counts`/`.genus()`.
- `crates/topo/src/review_m1_pr4.rs` — its two per-BODY copies read the
  door (`mfkrh_on_a_planted_ring_disconnects_the_shell_not_negative_genus`
  asserts `genus() == Ok(-1)`, `genus_two_double_hole_body_tears_down_to_nothing`
  asserts `Ok(2)`). `ShellComponent::genus`, the per-component
  inherent method on the component walk, is not one of the nine — it is
  not the whole-body identity — and stays.
- `crates/sweep/tests/verbs_shell.rs`, `verbs_shell_r2_probes.rs`,
  `verbs_shell_r2b.rs`, `shellfix1_r1_probes.rs` — `rings_of`/`genus_of`
  keep their names and are one-line delegates onto the door; the
  "one of NINE copies" pointer is gone with the copies.
- `demos/tour/src/teapot.rs` — `genus` deleted, call sites ask the door.
- `demos/tour/tests/verbs_teapot.rs`, `verbs_teapot_r2_probes.rs` —
  `rings`/`genus` delegate; `verbs_teapot_r1_probes.rs`'s `rings` and its
  hand identity likewise.
- The two the re-home counted beyond nine: `demos/tour/src/skinned.rs`
  (`genus` deleted) and `demos/tour/src/torusvessel.rs` (`genus`
  deleted, `census` reads the door).

The parity guard the row worried about now lives once, typed
(`EulerParityError`), so a copy can no longer drop it. Nothing the row
names remains, so it closes.

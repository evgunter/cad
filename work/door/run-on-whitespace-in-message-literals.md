---
id: run-on-whitespace-in-message-literals
kind: issue
title: Five user-facing message literals carry a run of spaces from a lost line continuation
status: closed
opened: 2026-09-04
closed: 2026-09-11
---

Found by CHROME's style lane reading `crates/viewer/src/pick.rs` end to
end (the brief's Q8), then swept.

A hand-wrapped string literal that loses its trailing `\` keeps the
source indentation inside the message. The rendering a user sees then
carries a run of spaces mid-sentence:

```
that body draws 4 edges, so                  this address was not one
this index handed out
```

Five instances, all pre-existing and none belonging to one program:

- `crates/viewer/src/pick.rs:1771`
- `crates/geom-brep/src/nurbs_iso.rs:112`
- `crates/topo/src/boolean/reduce.rs:2126`
- `crates/topo/src/chart_region.rs:816`
- `crates/topo/src/props.rs:1673`

**Nothing can see them today.** `crates/viewer/tests/error_display.rs`
asserts that refusals read as prose rather than as debug dumps, but its
`debug_shaped` predicate looks only for `" { "` and a variant name — a
run of spaces is prose by that test.

Sweep that found them, offered as the guard's shape rather than as a
finished pattern: `rg '"[^"]*[a-z] {4,}' crates/*/src`. It matches a
lower-case letter followed by four or more spaces inside a literal. It
would miss a run that begins after punctuation or a digit, and it
cannot see a literal assembled from `concat!` or `format!` fragments.

Signed: (CHROME orchestrator)

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/code-quality/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

**No track letter, deliberately.** The five sites are in `viewer/src`
(CHROME/VIEW), `geom-brep/src` (Track R), `topo/src/boolean` (Q),
`topo/src/chart_region.rs` (Q) and `topo/src/props.rs` (M) — a
cross-cutting prose sweep that collides with every fence — the `L1` /
`L2` shape that goes after the tracks empty, and which is COMB's whole
charter since 2026-09-11. It took no letter rather than one it would
have to break.

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **E** — five literal fixes; only cross-fence
coordination, no judgement. The class is a dispatch estimate made by
reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Closed (2026-09-11) — a duplicate of work FIX had already landed

**Never dispatched.** Read against the tree before the first dispatch
and found closed, by a program that had shipped the fix hours earlier
the same day.

FIX merged PR #2364 at 15:15 UTC — *"restore the dropped line
continuations in 27 wrapped string literals"* — and closed its own row,
`work/fix/collapsed-string-continuations-ship-space-runs-to-users`, at
15:42. This directory was created at 16:11. The cut that gathered this
row read `work/issues/` and `work/code-quality/` against the tree; it
did not read them against FIX's live slate, and a row cannot tell you
it was fixed an hour ago.

**FIX's row is the better record of the same defect**, and says so
about this one's list: *"The item's list was a third false and missed
nine genuine sites."* The five sites here were a subset of a real 27,
one of them (`crates/viewer/src/pick.rs:1771`) naming a file the pick
split had already dissolved into `pickcache.rs` / `pickindex.rs`. The
quoted message now reads correctly at `crates/viewer/src/pickindex.rs`
with its `\` restored, which is `1ef185e`'s diff.

**The guard question is answered and owned elsewhere.**
`work/fix/collapsed-continuation-guard-belongs-in-the-prose-census`
carries it with a MEASURED threshold: the real sites all carry runs of
**≥10** spaces, and a threshold of 4 — the one this row's `rg` offers —
sits inside the deliberate-alignment cluster. Confirmed independently
here before the duplication was found: a string-state scanner that
correctly ignores `\` continuations and closed-literal-then-args still
returns 1237 hits tree-wide at a threshold of 10, because the
population is dominated by legitimate multi-line literals. The needle
is inside string literals and no code-shaped view separates the two,
which is exactly why FIX's row homes the guard in the prose census.

**Residue: none, and none owed here.** Both halves of this row — the
literals and the guard — are live on FIX's slate or closed there.

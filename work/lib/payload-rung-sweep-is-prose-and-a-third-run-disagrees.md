---
id: payload-rung-sweep-is-prose-and-a-third-run-disagrees
kind: issue
title: the payload-rung sweep is a prose pattern re-implemented each run, and its curated set is three of the facade's four lists
status: closed
opened: 2026-09-08
closed: 2026-09-09
refs: [LIB-CUR7, LIB-SWEEP, payload-rung-re-sweep-finds-six-uncurated-profile-discriminants]
---


The payload-rung sweep exists only as a paragraph of prose. Each unit
that runs it re-implements the paragraph, and no two implementations
have yet agreed on a number.

| run | curated names | declared types | raw hits | narrowed |
| --- | --- | --- | --- | --- |
| LIB-CUR5 | 525 | 796 | 115 | 22 rows |
| LIB-CUR6 | 402 | 812 | 137 | 19 rows / 18 names |
| LIB-CUR7 | 406 | 813 | 208 | 17 rows / 16 names |

Some columns now reconcile and some do not, which is worse than a
clean disagreement because it reads like agreement.

- `curated names` RECONCILES EXACTLY. 402 -> 406 is the four names
  carried between the two merge bases and nothing else, checked by
  re-reading both trees: `AttrKind`, `ExprPath`, `NamingError`,
  `ProgramRefusal`, added, none removed.
- `declared types` reconciles to one name (812 -> 813).
- `raw hits` does not: 137 -> 208, with no change in the tree that
  accounts for seventy-one. Two implementations of "every type
  identifier appearing in the declaration's body" are counting
  different things, and the narrowing is what hides it.
- `curated names` between the first two runs (525 -> 402) has never
  been explained at all.

The NARROWED sets agree, and the way they agree is the finding.
The 17 rows this run reports are exactly the 18 rows the previous
run's own hit-list table ENUMERATES, minus `AttrKind`, which was
carried in between. But that table enumerates 18 rows over 17 names
while the prose above it claims 19 over 18 — so the run before this
one already disagreed with itself by one row, in one document, and
nothing noticed. A count nobody can re-derive is not a measurement.

What a unit closing this would deliver: the sweep as a committed
script, run rather than re-derived, with its blind spots as comments
beside the code that has them. Then a count that moves is a diff.

## The blind spot this run adds

**The curated set is three of the façade's FOUR curated lists.** The
pattern says "on no curated list" and every implementation has read
`crates/pncad/src/{document,select,prelude}.rs`, because that is the
set `crates/pncad-py/tests/test_binding_census.py` reads
(`FACADE_FILES`) and the set the pattern's prose names. The façade
curates a fourth by hand: `crates/pncad/src/profile.rs`, whose
completeness is guarded by
`crates/pncad/tests/all.rs::every_profile_layer_root_export_is_carried_or_listed`
against an EMPTY `PROFILE_NOT_CARRIED` — so every root export of the
profile layer is on it.

Every one of the six hits LIB-CUR7 settled was on that list already:

| name | on `profile.rs` at |
| --- | --- |
| `ContactKind` | `crates/pncad/src/profile.rs:98` |
| `EscalationSite` | `crates/pncad/src/profile.rs:98` |
| `FilletLeg` | `crates/pncad/src/profile.rs:98` |
| `FilletLegCarrier` | `crates/pncad/src/profile.rs:98` |
| `NoCornerReason` | `crates/pncad/src/profile.rs:98` |
| `Step` | `crates/pncad/src/profile.rs:74` |

That did not make the six false positives — the question a curated
list asks is whether a refusal it carries is MATCHABLE THROUGH it, and
`ProfileError` is on the prelude while its payloads were not — but the
scan cannot tell "uncurated" from "curated on a list I do not read",
and it reported the six as the former. A run over a name curated only
at `profile.rs` and carried by nothing on the other three would be a
false positive outright, and nothing in the pattern would catch it.

The fix is not simply to add the file. The four lists are not one
surface: `profile.rs` is the profile layer's whole presented root and
the prelude is the glob surface, so a name on one and not the other is
a real state with a real question attached. A sweep that reads all
four has to report WHICH list, or it trades false positives for false
negatives.


## Closed

LIB-SWEEP (branch `lib/sweep`). The sweep is
`scripts/payload-rung-sweep.py`, run by both halves of CI, and a count
that moves is now a diff.

**The drift, bounded.** Three of the four columns are settled and the
fourth is not, and which is which is the deliverable.

| run | curated | declared | raw | narrowed |
| --- | --- | --- | --- | --- |
| LIB-CUR5 | 525 | 796 | 115 | 22 rows |
| LIB-CUR6 | 402 | 812 | 137 | 19 rows / 18 names |
| LIB-CUR7 | 406 | 813 | 208 | 17 rows / 16 names |
| LIB-SWEEP, three lists | 413 | 821 | 121 | 11 |
| LIB-SWEEP, four lists | 457 | 821 | 113 | 10 |

- **`curated` — settled, INCLUDING CUR5's 525**, which had never been
  explained. The leaf reader over the three lists gives 413 at this
  merge base, which reconciles with 402 and 406 as tree growth. The
  same reader over ALL ELEVEN files in `crates/pncad/src/` gives 536.
  CUR5 counted the façade's whole source directory, not the three
  curated lists; the 536/525 gap over that interval is the same order
  as the 413/402 gap. The loosest reader over three files — every
  identifier in every `pub use` statement, module segments included —
  gives 432, so no reading of three files reaches 525 and the
  hypothesis has no competitor.
- **`declared` — settled.** 796 → 812 → 813 → 821 is monotone with the
  tree, and the script's index and CUR6's agree to one name at the
  merge base they share.
- **`raw` — NOT settled, and bounded instead.** Re-running this
  script's own scan with each closed blind spot re-opened gives, at
  this tree: as specified 121, with (f) open 146, with (g) open 160,
  with both open 185, and with CUR5's eleven-file curated set and both
  open 188. The three reported values straddle that interval on both
  sides — 115 below it, 208 above it — so the blind spots do not
  account for the spread: the difference is in what an implementation
  counted as ONE hit, which no surviving artefact records. Bounding it
  is where this stops.
- **`narrowed` — settled, and it reconciles EXACTLY.** CUR7's 16 names
  minus the six it settled is ten, and the four-list run's narrowed set
  is those ten names over ten rows. The three-list run adds exactly one
  row, `Step` under `ClosedLoop`, and that single row is the whole
  effect of the fourth list on this column: both are on
  `crates/pncad/src/profile.rs` (`:71`, `:74`), so `Step` is curated
  BESIDE its carrier and is not a rung there — while CUR7's argued
  non-carriage on the prelude stands untouched at
  `crates/pncad/src/prelude.rs:243`.

**The blind spot this item added is closed.** The scan reads all four
lists and reports which list each side of a row is on, so "uncurated"
and "curated on a list I do not read" are two different answers. It
did not trade false positives for false negatives, because a payload
curated on a list that does NOT carry its carrier is its own reported
category rather than a silent drop: four such rows exist at this merge
base and are filed as
`cross-list-payload-rungs-under-document-only-carriers`.

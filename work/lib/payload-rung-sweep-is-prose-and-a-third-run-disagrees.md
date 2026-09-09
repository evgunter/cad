---
id: payload-rung-sweep-is-prose-and-a-third-run-disagrees
kind: issue
title: the payload-rung sweep is a prose pattern re-implemented each run, and its curated set is three of the facade's four lists
status: open
opened: 2026-09-08
refs: [LIB-CUR7, payload-rung-re-sweep-finds-six-uncurated-profile-discriminants]
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

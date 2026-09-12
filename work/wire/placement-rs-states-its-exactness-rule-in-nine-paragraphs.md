---
id: placement-rs-states-its-exactness-rule-in-nine-paragraphs
kind: issue
title: placement.rs asserts exact / by bits / D9-deterministic in nine separate doc paragraphs, none the authority for any other, and is 60% prose
status: open
opened: 2026-09-11
refs: [2375]
---


## Finding

From the full review of PR 2375 (S6 `likely`, S8 `unsure`, S9 `unsure`),
which read `crates/editor-core/src/placement.rs` end to end under Q8 —
392 lines. Three findings, one file, filed together because they are the
same accumulation seen three ways.

**S6 — nine spellings of one rule.** After PR 2375 the file states
"this is exact / by bits / D9-deterministic" at `:103-112`, `:126-128`,
`:165-169`, `:227-229`, `:234-236`, `:245-252`, `:263-266`, `:271-272`
and `:296-300` — nine doc paragraphs, nine different sentences, **none
of which is the authority for any other**. That is the shape
`docs/prompts/reviewer-style-lane.md` Q2 names: a comment existing to
reconcile two spellings of one rule is evidence the rule needs one home.
PR 2375 added two of the nine, which is how the file got here — no
single unit added an unreasonable paragraph.

The reviewer's taste finding, recorded as such: one paragraph on `Frame`
and cross-references from the methods.

**S8 — `#[must_use]` is inconsistent inside the file.** None of `Frame`'s
methods carry it (`:187`, `:198`, `:204`, `:216`, `:230`, `:237`, `:253`,
`:267`, `:273`) while `AxisRefusal::kind` and `::carried` at `:34` and
`:41` in the same file do, and `Mat3::map` / `Affine3::map` — which the
new bodies now call — do. Pure-query methods on a `Copy` struct are the
canonical case. The reviewer flags the **inconsistency**, not the rule,
and does not know whether the project has ruled on it; a taker should
find out before changing anything.

**S9 — the ratio.** The file is 392 lines and about 60% prose.
`rotate_then_translate` (`:100-159`) is 38 doc lines over a 21-line body,
four titled paragraphs deep, arguing its bit-agreement with the transform
node. **Nothing in it is wrong** — that is the point of the finding. It
is the accumulation Q8 exists to surface, and the reviewer notes that
PR 2375's new test module is the first thing in the file that would catch
any of the nine claims going false.

## What this row is not

It is not "delete the prose". Every paragraph is individually defensible
and several are load-bearing arguments about bit-exactness that this
project genuinely wants written down. The row is about **where** the rule
lives, not whether it is stated: one home, cross-referenced, so the tenth
unit does not add a tenth sentence.

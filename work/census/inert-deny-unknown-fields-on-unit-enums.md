---
id: inert-deny-unknown-fields-on-unit-enums
kind: unit
title: Inert #[serde(deny_unknown_fields)] on unit-only enums — nine sites in role.rs, and a ledger entry that believed it load-bearing
status: closed
opened: 2026-08-30
github: 1308
refs: [1301]
branch: census/inert-deny
pr: 2634
closed: 2026-09-15
---

## From GitHub issue 1308

Opened 2026-08-30; 0 comments.

**Raised by BLEND-5's review round** (PR #1301), established by execution: removing `#[serde(deny_unknown_fields)]` from `RimSupport` left every row of the v18 break suite green. An externally-tagged enum rejects unknown variants unconditionally; the attribute governs struct-like fields, and every tag enum in `crates/editor-core/src/names/role.rs` is unit-only — so the attribute is inert at all nine sites (`role.rs:41, 93, 132, 150, 166, 187, 235` and the two locator structs; the struct sites are the only ones where it can do work).

The cost is not the attribute — it's what the project has come to believe about it: the v18 ledger entry originally credited `deny_unknown_fields` with the serde-death that justifies the schema break, when the operative machinery is (a) the version door (`SchemaTooOld` / `UnknownSchema` before serde is ever reached) and (b) the enum's own unconditional unknown-variant refusal. BLEND-5's fix pass corrects that ledger entry; this issue is the sweep obligation for the class: every `#[serde(deny_unknown_fields)]` on a fieldless enum in `editor-core` (and anywhere else in the workspace), each either removed or kept with one sentence saying it is a habit-guard for a future non-unit variant, so no future doc reasons from an attribute that does nothing.

## Specced as CENSUS's first unit (2026-09-15)

`docs/CENSUS-INERT-DENY-SPEC.md` binds it; the spec is deleted at merge
and this section is the part that survives. Three corrections to the
text above, measured against the tree on 2026-09-15 and recorded here
because they change the population:

- **"73 sites" counts grep hits.** 73 lines under `crates/` match
  `deny_unknown_fields` in a `.rs` file; **59 are the attribute, 14 are
  prose** — module docs, doc-comments and test comments reasoning about
  it. The prose is the half this row is really about, and it was not
  separated out when the row was written. Two further prose sites are
  in MARKDOWN under `crates/` (`profile/README.md`,
  `verbs/README.md`), which a `.rs`-only sweep does not see.
- **"unit-vs-struct" is the wrong rule.** The attribute needs a NAMED
  field to deny, so it is inert on a unit enum, a **tuple-variant-only
  enum**, a **tuple struct** and a unit struct alike. The coarse rule
  finds 14 inert sites; the named-field rule finds **22**. The eight it
  misses are `Attr`, `WireTarget`, `WireMeasureExpr`, `DocParamValue`,
  `PartSelect`, `MetaValue` and the tuple structs `ParamName` and
  `RecipeNodeId` — inert for a reason the row does not state. The tree
  holds no unit-struct site; the fourth shape is covered by the rule
  and has no member.
- **"every tag enum in `role.rs` is unit-only" is false.** `role.rs`
  carries 11 attribute sites — 8 enums, 3 structs — and two of the eight
  enums carry data (`Qualifier`, `RoleSeg`). Six are unit-only, not
  nine. The row's `role.rs:41, 93, …` citations are all rotted; the
  sites are at `:255`–`:566` today. `Qualifier` and `RoleSeg` are NOT
  inert: each has struct variants (`Qualifier::OrderAlong { rank, of }`
  and fourteen of `RoleSeg`'s), so the attribute governs at both and
  they are untouched.

The title still says "nine sites in role.rs"; ids and titles are stable
and it is left as the row was cited, with the correction here.

The class estimate **M** is right and stands: multi-file, no design
call in the sweep, and an instrument to build first.

## What landed

All 22 inert sites removed, none kept: a keep needs an exemption in the
instrument, and an exemption list is a hand-written list of
declarations — this program's own class. Five prose sites named the
attribute as machinery it is not and were rewritten to name what
refuses; eleven were true and are untouched.

The instrument is `crates/test-utils/tests/deny_unknown_fields_census.rs`
— a repo-wide walk through `test_utils::source`, with its line in
`crates/test-utils/tests/reader_census.rs`. Its blind spots are stated
in its own header, not only in the PR.

`work/msolve/mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses.md`
is the complement the sweep turned up: a field-bearing wire type with
no attribute, filed rather than fixed because the fix changes what a
document accepts.

## Home

`work/code-quality/` — a workspace-wide sweep of an attribute that does nothing, plus the doc that reasoned from it: the register's own subject (code that does not look like the way you would do it, and the prose debt around it).

## Re-homed to CENSUS (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CENSUS collects the rows of one class: a vocabulary spelled by hand in
several places, and the census or instrument that cannot see one of the
spellings. This row is a member of that class.

Its class at the cut was **M** — workspace sweep of 73 sites, but each
decided mechanically by unit-vs-struct. The class is a dispatch estimate
made by reading the row against the tree on 2026-09-11, not a verdict on
the finding, and a lane that finds it wrong says so in its PR. The id,
the `track:` letter where the row carries one, and the body above are
unchanged by the move.

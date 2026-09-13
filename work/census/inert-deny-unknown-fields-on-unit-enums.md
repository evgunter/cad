---
id: inert-deny-unknown-fields-on-unit-enums
kind: issue
title: Inert #[serde(deny_unknown_fields)] on unit-only enums — nine sites in role.rs, and a ledger entry that believed it load-bearing
status: open
opened: 2026-08-30
github: 1308
refs: [1301]
---

## From GitHub issue 1308

Opened 2026-08-30; 0 comments.

**Raised by BLEND-5's review round** (PR #1301), established by execution: removing `#[serde(deny_unknown_fields)]` from `RimSupport` left every row of the v18 break suite green. An externally-tagged enum rejects unknown variants unconditionally; the attribute governs struct-like fields, and every tag enum in `crates/editor-core/src/names/role.rs` is unit-only — so the attribute is inert at all nine sites (`role.rs:41, 93, 132, 150, 166, 187, 235` and the two locator structs; the struct sites are the only ones where it can do work).

The cost is not the attribute — it's what the project has come to believe about it: the v18 ledger entry originally credited `deny_unknown_fields` with the serde-death that justifies the schema break, when the operative machinery is (a) the version door (`SchemaTooOld` / `UnknownSchema` before serde is ever reached) and (b) the enum's own unconditional unknown-variant refusal. BLEND-5's fix pass corrects that ledger entry; this issue is the sweep obligation for the class: every `#[serde(deny_unknown_fields)]` on a fieldless enum in `editor-core` (and anywhere else in the workspace), each either removed or kept with one sentence saying it is a habit-guard for a future non-unit variant, so no future doc reasons from an attribute that does nothing.

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

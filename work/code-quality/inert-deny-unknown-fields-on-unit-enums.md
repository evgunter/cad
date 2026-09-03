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

---
id: main-red-prose-census-cannot-decide-the-glue-record-broken-surfaces
kind: issue
title: main red: the prose census cannot decide GlueRecordBroken's surface pair in merge_faces.rs (pncad-py prose_census, every ε), since PR 6079
status: open
opened: 2026-10-30
priority: P3
cost: E
---


**What fails.** `pncad-py` `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason` (`crates/pncad-py/src/prose_census.rs`, the `UNDECIDED` assertion) is red on origin/main `2e1c1bff` (2026-10-30, run locally, every ε alike since the census reads source) on two sites `UNDECIDED` did not name, beside the CLEAR and SPLICE rows nightly 77's triage (#6082) filed:

- `("crates/topo/src/merge_faces.rs", "MergeCoplanarError", "a"): 1`
- `("crates/topo/src/merge_faces.rs", "MergeCoplanarError", "b"): 1`

Both are `MergeCoplanarError::GlueRecordBroken { surfaces: (a, b) }`'s `Display` arm, `"the Boolean's glue record for surfaces {a:?} and {b:?} …"` (`crates/topo/src/merge_faces.rs`, `impl Display for MergeCoplanarError`).

**Attribution (read, not bisected).** The arm arrived with PR 6079 (`contacthold/glued-pair-reads-its-record`, merge `7f30969f`), which retired `DeclaredPairUnread { f1, f2, kind }` for it; no later merge touches `merge_faces.rs`. The merge landed after nightly 77's main (`39bbdfc777`), so nightly 77 does not show it.

**Reproduce.** `cargo nextest run -p pncad-py -E 'test(=prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason)'`

**What closing it takes.** Bind the pair through named fields the census can type (or render each `SurfaceKey` through a door the census reads), and strike the two `UNDECIDED` lines that cite this row. Or keep the lines with the reason they carry, stated as accepted.

## 2026-10-30 (GREEN, lane `green/nightly-77-pins`)

Pinned: the two sites are named in `prose_census.rs`'s `UNDECIDED`, citing this row, with the reason the census could not decide them. The other entries keep the row honest: the census still reds on any site that arrives unnamed, and on these two lines once the arm is rewritten. Un-pinning (striking the lines) owes nothing beyond the census's own run: the census reads source and stops on nothing.

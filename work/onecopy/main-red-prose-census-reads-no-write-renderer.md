---
id: main-red-prose-census-reads-no-write-renderer
kind: issue
title: Main red: pncad-py's prose census reads 138 sites, under its 200 floor, and its undecided roster drifted: it never read a renderer written in an inherent fn write
status: open
opened: 2026-11-09
priority: P1
cost: M
---


**What fails.** Two pncad-py slow-set rows in `crates/pncad-py/src/prose_census.rs` are red on origin/main at every ε (the census reads source):
- `prose_census::tests::the_census_reads_a_tree_and_not_an_empty_one`: "the same vacuity one level in: 138 sites";
- `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`: the `UNDECIDED` roster no longer matches.

The small-sweeps lane reported them on 2026-11-09.

**Cause** (bisected over main's first parents):
- **PR 6430** (`b84ea1f43e`, 2026-11-03, `pcert/display-payload-only`) is the first red: the site count drops from over 200 to 134. It moved each refusal's sentence out of `impl Display` into an inherent `fn write(&self, f, at)`, and `geom_brep::ended_by_write!` now generates the `Display`. The census finds bodies by the literal heads `Display for` / `Say for`, so it stopped reading every refusal on the macro, and every hand-written twin (`OffsetFitError::write_payload`, `topo::shell`'s `AsShelled::write`). This is `work/census/prose-census-finds-sentence-impls-by-their-trait-names.md`'s predicted failure.
- **Roster drift on top of it**, unseen because the rows stood red:
  - PR 6448 (2026-11-03) re-rendered one `CertifyError` `check` read through `check.words()`;
  - PR 6549 (2026-11-06) replaced `profile`'s `ReplayError` with `Refused`/`Takes`, and moved `ProgramFault`'s `verb` rendering into them;
  - PR 6579 (2026-11-08) added `FaceUncovered`'s two renderings.

**Asked.** Read renderers by what they do, then re-pin the roster to the real drift.

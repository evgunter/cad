---
id: main-red-prose-census-cannot-decide-the-selection-refusal-face-arm
kind: issue
title: main red: the prose census cannot decide SelectionRefusal's face arm in clearance.rs (pncad-py prose_census, every ε)
status: open
opened: 2026-10-30
priority: P3
cost: E
---


**What fails.** `pncad-py` `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason` (`crates/pncad-py/src/prose_census.rs`, the `UNDECIDED` assertion, `:2291`) is red at every ε in nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467) (main `39bbdfc777`), and was green on 2026-10-29 (run 40823227622, `5648e577eb`). The census finds two sites it cannot decide that `UNDECIDED` does not name:

- `("crates/editor-core/src/clearance.rs", "SelectionRefusal", "<positional>")`
- `("crates/editor-core/src/clearance.rs", "SelectionRefusal", "k")`

Both are the face arm of `SelectionRefusal`'s `Say` impl, `Some(k) => write!(f, "{sep}face {k:?} of body {}", c.body)?` (`crates/editor-core/src/clearance.rs:728`).

The same row is also red on two `CertifyError` sites in geom-brep, filed on SPLICE as `work/splice/main-red-prose-census-cannot-decide-certify-errors-check.md`. The test goes green when both rows close.

**Attribution (read, not bisected).** The arm arrived with PR 5977 (`clear/cleartext`, merge `2f9b55f2`). It is the only change to the `Say` arm in the window, and no later merge touches it.

**Reproduce.** `cargo nextest run -p pncad-py -E 'test(=prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason)'`

**What closing it takes.** Rewrite the arm so the census can read it (its preferred remedy), or give each site its `UNDECIDED` line with the reason it cannot be decided.

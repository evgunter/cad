---
id: main-red-prose-census-cannot-decide-certify-errors-check
kind: issue
title: main red: the prose census cannot decide CertifyError's two check reads in geom-brep certify.rs (pncad-py prose_census, every ε)
status: open
opened: 2026-10-30
priority: P3
cost: E
---


**What fails.** `pncad-py` `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason` (`crates/pncad-py/src/prose_census.rs`, the `UNDECIDED` assertion, `:2291`) is red at every ε in nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467) (main `39bbdfc777`), and was green on 2026-10-29 (run 40823227622, `5648e577eb`). The census finds a site it cannot decide that `UNDECIDED` does not name: `("crates/geom-brep/src/certify.rs", "CertifyError", "check"): 2`. These are the two `check` reads in `CertifyError`'s refused-arm mapping, `Self::Image(check)` and `Self::ImageEscalated { check, cause }` (`crates/geom-brep/src/certify.rs:747` and `:751`), which read `check` through `arc_rim_check` and `map_or`.

The same row is also red on two `SelectionRefusal` sites in editor-core, filed on CLEAR as `work/clear/main-red-prose-census-cannot-decide-the-selection-refusal-face-arm.md`. The test goes green when both rows close.

**Attribution (read, not bisected).** PR 5959 (`splice/structural-rim`, merge `dd2a8b45`) is the only merge in the window that touches `certify.rs`, and it adds these arms.

**Reproduce.** `cargo nextest run -p pncad-py -E 'test(=prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason)'`

**What closing it takes.** Rewrite the two reads so the census can decide them, or give the site its `UNDECIDED` line with the reason.

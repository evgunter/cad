---
id: main-red-the-prose-census-reads-a-debug-rendering-in-selects-message
kind: issue
title: Main red (slow set): pncad-py's prose census finds a Debug rendering in select.rs's message since PR 6555
status: closed
closed: 2026-11-12
opened: 2026-11-11
priority: P1
cost: E
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`), at default ε and 1e-6. Filed on bind, which owns `crates/pncad-py/src/py/select.rs` (`work.py territory`); the PR that added the site, `reads/select-indexed-member`, has no program directory left.

**What fails.** `pncad-py prose_census::tests::no_raise_site_composes_a_debug_rendering_into_its_message` (`crates/pncad-py/src/prose_census.rs`): `left: {("crates/pncad-py/src/py/select.rs", "message")}`, `right: {}`.

**The site.** PR 6555 (merge `a5e6a785`) added to `select.rs`'s `message` the text `"the read holds no member at {member:?}: read a member the family has, …"`. That builds a raise site's human message out of a `Debug` rendering, the route to the panic the census guards.

**What closing it takes.** Render `member` through its `Display` (or a named field), or, if a `Debug` rendering is the right message here, give the site its allowance with the reason. The census keys the allowance on the site, not on the file.

The class (slow-set source censuses no PR gate selects) is on green's `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.

## Closed

A duplicate of REFS' `main-red-select-no-member-raise-composes-a-debug-rendering` (filed 2026-11-10 by ONEHOME's stage-2 lane), which closes in the same PR (`reads/no-member-prose`): `NoMember`'s raise spells its index values joined, not through `Debug`, and `KNOWN_DEBUG_RAISES` stays empty. The class stays open on GREEN's `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.

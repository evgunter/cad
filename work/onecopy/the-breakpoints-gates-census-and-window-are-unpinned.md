---
id: the-breakpoints-gates-census-and-window-are-unpinned
kind: issue
title: breakpoints-by-hand derives its plants from its arrays, so the census itself, the window bound and each entry's call are unguarded
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6616 (`onecopy/variety-gaps`, 94036920 → 257f0049). The PR merged with these left open. All are NOTE-grade.

1. **The census is unguarded.** The plants are derived from the arrays, so the self-test proves the matcher equals the arrays, never that the arrays equal the intended list. Deleting an entry, or moving `fold` from REFUSED to PASSED, survives (11/11 such mutations). The change is at least a visible one-line diff. The gate header should say so.
2. **The window bound is pinned on neither side.** Window 2, 3 or 40 survives. The only plant puts the dedup two statements on. Add a plant at four statements on (red) and one at five (quiet).
3. **An entry's name and its planted call are independent fields.** `'try_fold~~map(|(v, _)| v)'` passes the self-test while `try_fold` is never exercised. Derive the call from the name, or check that it contains the name.
4. **Recall gaps:**
   - `try_fold` and `reduce` take the closure's value, which fits REFUSED's own definition, but neither is listed.
   - These are blind and not named under BLIND SPOTS: `.map(|&x| x).collect()`, `copied().filter(..).collect()`, and a field `k.knots.to_vec()`.
5. **A false positive missing from CRIES WOLF:** `k.runs().count() + o.iter().map(|(x, _)| x).count()` fires, because `[^;]*` crosses into another iterator's chain.

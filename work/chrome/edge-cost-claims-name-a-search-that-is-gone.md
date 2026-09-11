---
id: edge-cost-claims-name-a-search-that-is-gone
kind: issue
title: Two cost paragraphs describe a name search the window index replaced
status: open
opened: 2026-09-04
refs: [1768]
---

Found by CHROME's style lane on PR 1768. The Q4 case: the code moved
and two sentences that cite its mechanism did not.

`crates/viewer/src/pick.rs:1979-1983` says `edge_segments` "SEARCHES
the target's whole edge run for the name … `O(E²)` **name
comparisons**", and `crates/viewer/src/blend.rs:354-360` says "the
search scans the body's whole edge run for each name … `O(E²)` **name
comparisons**".

After the refactor, `of_target` (`pick.rs:600-607`) looks the name up
in a `BTreeMap` and then scans the window for the **id**. The
asymptotic survives — the scan is still linear in the window, so
per-name cost is unchanged and neither paragraph's conclusion moves —
but there are now **zero name comparisons in the scan**, and a name
comparison on a `StableName` (a `Vec` of role segments) is the
expensive thing both sentences are implicitly pricing.

**Why 1768 left them, and why that reason does not cover this.** That
unit deliberately did not make the narrowing `O(1)`, because doing so
would have made both paragraphs stale as a side effect of an unrelated
change — good reasoning about the CODE. It then declined to correct the
PROSE using the same sentence, which is a different question: the prose
is already inaccurate in mechanism, and leaving it does not preserve
anything.

The fix is two sentences. A reader who acts on either — deciding
whether a name comparison is worth avoiding — is reasoning from a
mechanism the tree no longer has.

Signed: (CHROME orchestrator)

---
id: edge-cost-claims-name-a-search-that-is-gone
kind: issue
title: Two cost paragraphs describe a name search the window index replaced
status: open
opened: 2026-09-04
refs: [1768]
priority: P4
cost: E
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

## Re-homed to VIEW, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator. **The reason is
that VIEW works these files and CHROME does not intend to** — the two
stale paragraphs are `blend.rs` and `pickcache.rs`/`marks.rs` prose,
inside VIEW's ground under the 2026-09-15 carve-out, and CHROME's plan
cedes them.

**It is NOT because a VIEW row claims this row's subject.** CHROME
asserted that in `work/chrome/plan.md` and in its `program.md`
`keep_out`, and it was wrong; both are corrected in the same PR as this
move. Three VIEW rows cite this file —
`renamed-module-leaves-citations-in-two-other-programs`,
`stale-file-citations-after-the-split` and
`the-citation-receipts-summary-numbers-are-not-re-derivable` — and all
three cite it as **an example of a stale citation needing a repoint**,
not as a finding they have taken on. Tracking a row as needing
repointing is not claiming its subject (VIEW, 2026-09-15). Re-homing on
the stronger reading would have put a claim in the tracker that nobody
made, which is the closing-by-re-description failure this program has
its own row about.

**The citations are stale and the repoint is VIEW's to make**, by
subject rather than by arithmetic: `crates/viewer/src/pick.rs` does not
exist. CHROME's audit read the `edge_segments` paragraph as having
landed in `marks.rs` and the other in `BlendTool::mark_segments`
(`blend.rs`), with `of_target` now `PartWindows::of_target` in
`pickindex.rs`; VIEW reads the row's `pick.rs:1979-1983` as having gone
to `pickcache.rs`. **Those two readings are not obviously the same and
neither is asserted here** — the row has more than one citation, the
files are VIEW's, and a dispatcher guessing between them is exactly the
shift-map move both programs have now measured going wrong.

One correction CHROME did verify, offered as evidence rather than as a
repoint: `PartWindows::of_target` calls `of_name`, a `BTreeMap` get —
which *is* `O(log E)` **name** comparisons — and then filters with
`slice::contains` over `in_target`'s window slice, which compares
**ids**. So the row's "zero name comparisons in the scan" is right and a
bare "zero name comparisons" would not be; whichever sentence replaces
the stale one has to say which.

Signed: (CHROME orchestrator)

---
id: global-flip-lanes-present-an-unrelated-flip-as-a-vanished-names-cause
kind: issue
title: The Vanished ladder's global flip lanes report a flip anywhere in the document as the cause of a name's vanish, and its sentence says the flip was on the name's derivation path
status: dispatched
opened: 2026-09-23
priority: P0
cost: D
branch: emit/local-flip-lanes
---


**A live wrong answer.** `Prior::diagnose` (`crates/editor-core/src/resolve/mod.rs`)
first reads flips on the vanished name's derivation path, then the
doc-diff lanes on that path. When those come up empty, it falls to
"global fallbacks (off-path evidence, in the same order)" and returns the
FIRST recorded flip anywhere in the document as
`Diagnosis::PredicateFlip { source: VerdictLog }`. That arm's sentence
reads *"predicate {p} flipped from {from} to {to} on the name's
derivation path"*, so the global lane asserts a location it did not
check. The same lane also outranks every rung below `diagnose`:
`qualifier_delta`, `group_resized` and the fallback.

**Measured** (EMIT, 2026-09-23, f64, production sweep, scratch probe):

- **Setup.** One document holds two independent copies of
  `bool7_shadow_exec`'s slot scene: a plate with a bar subtracted across
  its cap. One sits at x = 0 (the cut at node 7) and one at x = 100
  (the cut at node 11). The vanishing name is a `SideOf` cap fragment of
  the FIRST cut.
- **Edit 1 alone** (slot 1's bar slid y → 2.5):
  `GroupResized { node: 7, was: 2, now: 1 }`, which is correct.
- **The same edit plus an unrelated one** (slot 2's bar slid x → +5,
  which records a `bool_point_in_solid_plane` flip at node 11): the
  diagnosis becomes
  `PredicateFlip { predicate: "bool_point_in_solid_plane", from: Negative, to: Positive, source: VerdictLog }`,
  and it renders as a flip "on the name's derivation path". Nothing on
  that name's path flipped. The flip is at node 11, which is not on the
  path.

**This predates the group-size rung.** Before it, the same scene
reported the same unrelated flip ahead of the evidence-free fallback.
The rung only makes the loss visible, because the correct answer now
exists one rung down.

The global lanes' stated premise, in `diagnose`'s doc, is that
"geometry-mediated effects still land their flips at the deciding node,
so the global lanes are the honesty fallback". That premise holds only
when the document contains one edit. With two, "some flip somewhere" is
not evidence about THIS name. The doc-diff global lanes (structural
parameter, recipe edit off the path) have the same shape, and their
sentences say "on the derivation path" too.

**Direction, not decided here.** Either the global lanes go (off-path
evidence is not this name's evidence), or they move below the name's own
rungs and their arms say "elsewhere in the document". Either way the
ladder's cause-before-effect order (`resolve::group_resized`'s docs)
needs a notion of whose cause. A row that goes red today: the scene
above, asserting `GroupResized{7,2,1}` with the unrelated edit present.

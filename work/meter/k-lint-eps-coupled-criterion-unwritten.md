---
id: k-lint-eps-coupled-criterion-unwritten
kind: issue
title: What makes a predicate eps-coupled is nowhere written, so k-lint's roster cannot be pinned against the kernel in the ADDED direction
status: open
opened: 2026-09-07
refs: [k-lint-predicate-roster-unpinned]
---


## What

**`k-lint-predicate-roster-unpinned` asks for a pin, and a pin can only
close half of what that item found.** `tools/k-lint/src/lib.rs:283`
holds `EPS_COUPLED_PREDICATES = ["props_quad_converged"]`, a roster of
the kernel's vocabulary held in a workspace-excluded consumer, and the
item names two silent directions:

- a predicate the kernel **renames** leaves the roster naming nothing;
- a predicate the kernel **adds** to that class is absent from the
  roster and is judged by the wrong rule.

An `include_str!` pin over `crates/geom-brep/src/props/quad.rs` — the
`D204` shape, whose machinery already exists at
`tools/tess-meter/tests/derivations.rs:239` — closes the first
direction completely: the name is either still minted at the cited site
or the pin reds. **It cannot close the second at all.** Catching an
ADDED ε-coupled predicate needs a statement of what makes a predicate
ε-coupled that a test can evaluate over the kernel's minted names, and
no such statement exists in the tree. `tools/k-lint/src/lib.rs:30` and
`:126` describe the class as one "whose margin is a headroom against
ε", which is a description of the roster's one member rather than a
criterion anything can apply.

## Finding

**The item's own framing hides this, which is why it gets a file.**
`k-lint-predicate-roster-unpinned` says "nothing pins the two together
in either direction" and then reaches for `D204`'s remedy, which was
adequate there because `CHART_TAGS`' producing half is a closed match
on `Chart::tag` — the roster and the vocabulary are the same finite
set, so pinning one pins both directions. `EPS_COUPLED_PREDICATES` is
not that shape: it is a SUBSET selected from an open and growing
vocabulary by a property, and a subset selected by an unwritten
property cannot be checked for completeness.

So the pinning lane should be dispatched knowing it will close one
direction and disclose the other, rather than discovering at review
that its "pinned in one direction" claim is narrower than the item it
cites. Whether writing the criterion is worth a unit is a separate
question and probably reaches `crates/geom-brep/src/props/*` (PROPS'
territory), which is why this is an issue and not a rider.

**Confidence:** sure that no criterion is stated in `tools/k-lint` or
in the `props/quad.rs` mint sites; unsure whether one exists somewhere
in `docs/` under a name the sweep below could not match.

**Sweep and its blind spot.** Grepped `EPS_COUPLED_PREDICATES` across
the tree and read every hit, and read `tools/k-lint/src/lib.rs`'s
module docs for a definition of the class. The pattern was the
CONSTANT's name and the phrase "ε-coupled"; it cannot match a criterion
written down without either — a rule stated in `docs/K-REPORT.md` about
which margins are headroom, say, in prose that never uses the roster's
spelling.

## Was

Raised by the METER orchestrator's difficulty pass over this program's
slate (2026-09-07), reading `k-lint-predicate-roster-unpinned` against
`tools/k-lint`'s manifest and the `D204` precedent it cites.

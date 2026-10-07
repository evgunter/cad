---
id: self-overlapping-spines-build-and-validate
kind: issue
title: A loft or sweep whose spine revisits itself (a planar arc past a full turn) builds a self-overlapping body and every validation tier says Ok
status: parked
opened: 2026-09-16
refs: [2752, 368]
priority: P0
cost: H
blocked_on: [SHELL-3, window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch, self-intersection-drops-every-vertex-sharing-face-pair-globally]
---

Found by both of BOOL-6's reviews (PR 2752) and filed by the S-BOOL
orchestrator on sweep ground (`crates/sweep/src/loft.rs`). The
per-slab stacking fold correctly admits every spine whose per-slab
turn stays under π, which includes a unit-radius planar arc curled
9.0 or 13.0 rad over 17 stations — past one and two full turns — where
stations three or more apart come within less than the section's own
width. The body builds; `validate`, `validate_closed` and
`validate_geometric` all return Ok; only `LevelIndex::contains`
notices ("2 level rings claim … the body overlaps itself there"). The
old end-to-end π wall excluded every self-overlapping circular spine
by accident; nothing downstream refuses one now, and the kernel has
never claimed a self-intersection gate. BOOL-6 pins the fact in one
named row (`the suite's self-overlap row`) and says so in its docs
rather than presenting the builds as wins. The question for the owner
is whether a loft/sweep door should refuse a self-overlapping spine
(a per-station clearance decide against the section's extent, or a
tier-3 self-intersection check) or whether self-overlap is a legal
body the certifying door must name. Measured, not acted on;
difficulty M.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to BLEND (the sweep crate and the profile fillet door are BLEND's charter; crates/sweep/src/loft.rs passes to BLEND at this exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Weighed (2026-10-06): a loft promises an embedded boundary, certified at its door

Weighed with `two-section-loft-with-an-inverted-top-normal-builds` by
two designers (an Opus and a Fable lane), who were given the problem
and not the options. They took three rounds. Round 1 crossed on the
reversed list, so a second round followed, and they converged. The
blinding is on `analysis/design-fork/carve-2026-10-06`. Nothing here
went to Ev.

- **The contract.** The walls and caps a loft or sweep builds form an
  embedded closed surface (the boundary meets itself only along its own
  edges), and the cross-section at each section's parameter is the
  authored section. That is certified on the BUILT body, not inferred
  from the stations: the skin is the definition (Q8), and no rule on
  the stations bounds the surface between them. Both designers
  exhibited embedded lofts that every closed-form slab test refuses (a
  sharp elbow, and a y-prism with a 60° tilted section). Station proxies
  and "self-overlap is a legal body" are both rejected.
- **The instrument.** It is the clearance engine's body-level half in
  `topo` (`work/clear/SHELL-3.md`), at a certifying scalar, called at
  the loft door. That is D1's door clause, Ev's B at #1737, applied as
  written. It refuses typed, naming the two faces that meet and the
  v-range of the sections between which they meet. Where a plane faces
  back along the stack, the refusal may say so as a diagnosis that
  decides nothing. It has three prerequisites, none built:
  - SHELL-3;
  - `work/clear/window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch.md`;
  - `work/clear/self-intersection-drops-every-vertex-sharing-face-pair-globally.md`.
    The certificate means "embedded" only with this one: without it, a
    triangle-section loft has no examined pair, and every cap is
    excluded against every wall.
- **The f64 lane** certifies by lifting its built body exactly to
  Interval. A loft's structure is chosen at f64 and lifted exactly
  (`loft.rs`, "Scalar posture"). An f64 lane that returns `Ok` where
  the Interval lane refuses is the direction the central commitment
  does not tolerate. The price, one interval subdivision per rebuild,
  is measured before this lands, on the tour's lofts and
  `m8_14_long_turn_sweep`'s helices. If it is prohibitive, the f64 door
  takes SHELL-4's `None` shape and says so. This sits beside Ev's #1737
  ruling for `shell`, so Ev is told when the certificate unit is
  specified.
- **The per-slab stacking fold retires** when the certificate lands. It
  is S-BOOL's Q2 shape: an agent recommendation in #1373, built in
  #2752, with no wording of Ev's on it. Its three arms each get a home:
  - the fold-back goes to the certificate;
  - the sliver goes to the skin's banded coincident-section decide
    (`skin-coincident-section-check-is-an-unbanded-f64-compare`);
  - the reversed list BUILDS: the order sets v and which cap is bottom,
    and inside is derived. `ReversedStacking`'s own text says the
    reversed list "lofts the same solid", and `Node::Loft` says "order
    is data: reversing it reverses the produced surface's
    v-direction".
- **How inside is derived** is the one detail decided at the
  certificate unit, not now. One mechanism reads slab 0's common sign
  and traverses every ring the other way when it is Negative; it works
  at every scalar, with no new mechanism. The other reads the certified
  signed volume and calls `revert` on a negative one. The first builds
  the y-prism inside out: section 0 is the unit square in z = 0 facing
  +z, and section 1 is a half-width-0.5 square centred at (3, 0.5, 0.3)
  with normal (−0.87, 0, 0.5). That body is embedded, but its
  cross-section winds clockwise. The second is sound only once the body
  is certified.
- **Interim, built now** as `carve/fold-reads-the-far-normal`. The fold
  also decides section k+1's normal against slab k's displacement,
  under the same `loft_stacking` band. Every refusal it adds is either
  a fold (by the turning-number argument: rings that wind oppositely
  about the stack must pass through a singular or non-simple ring) or
  an embedded inside-out body, which the fold refuses while it stands.
  It closes the downward top and an interior back-facing section
  (z = 0, 1, 0.5 with normals +z, −z, −z). The docs say plainly that
  embedding is not yet certified. It retires with the fold.

This row stays open as the door-certificate unit, parked on the three
CLEAR prerequisites.

## When the fold retires (from PR 4186's review, 2026-10-06)

Whether two adjacent sections are apart is decided today as the Zero arm
of the fold's centroid-along-the-base-normal margin (`stacking_fold`,
called from `build` in `crates/sweep/src/loft.rs` after `validate_loft`
and before `skin_validated`). It must stay BEFORE the skin, while the
embedding certificate needs the walls and so runs after it. The unit
that retires the fold therefore splits that arm out as its own decide at
the same call site. Otherwise the skin's `NoParameterStep` becomes the
coincidence door again, against its own doc.

## The interim's known cost (from PR 4188's review, 2026-10-06)

The far-normal check the designers agreed as the interim
(`LoftError::FarSectionNotForward`) is CONSERVATIVE. The turning-number
argument holds for rings projected along the stack, not for the 3-D
rings, which can turn edge-on to it and stay simple. So it refuses some
embedded, correctly oriented bodies. Two rows in
`crates/sweep/tests/bool6_per_slab_stacking.rs` pin them, and each
should BUILD once this unit's certificate retires the fold:
`an_embedded_hood_with_a_downturned_top_is_refused_by_the_far_decide`
(volume +227.5 at the old base) and
`an_embedded_oblique_arc_sweep_is_refused_by_the_far_decide`. The old
near check has the same hole on the mirror side.

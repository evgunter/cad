---
id: a-placement-is-the-bundle-of-mates
kind: issue
title: D10 stage 3 PR B: Place { shapes, constraints } owns today's mates, the world is a node, an instance is a placement, spaces are kinds; gauges, offsets, PlaceInWorld, the spanning tree and roots retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [a-mate-reads-face-variables, an-unattributed-contact-at-rest-is-a-finding]
refs: [intent-stage3-is-built, mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion]
---

INTENT stage 3, PR B. Spec: `docs/INTENT-STAGE3-SPEC.md` §3. Built on FORK-S3M (fork log row 97, PR 4326) and FORK-S3O (row 96, PR 4325).

`Place { shapes, constraints }` reads a list of shapes of one space, as a union reads its operands, and defines one copy of each. Its constraints are addressed by (placement, minted id), never by position. In B a constraint is today's mate payload, numbers included, moved under the placement unchanged. That is a migration interim, not a final state: C (`a-mate-relates-two-poses`) ends it by re-typing each constraint into poses and values. That is why B comes before C: a value is a placement's constraint, so it cannot exist before `Place` does.

The world is one undeletable node that defines no variable. Only a placement's constraints and export read it. An instance of a part is a placement, with one output per world placement of the part, keyed by that placement, and no `frame` port. Spaces are kinds decided from the recipe, and a read across two spaces refuses `SpaceMismatch`. Until D, every construction reading no body and every absolute datum is of the document's one kind (a named interim), so today's combines of datum-built bodies still evaluate. D lands "a body's kind is its root". A copy whose bundle pins less than its body needs is a loose copy, not a refusal. A mate beyond its bundle's pin refuses `Overconstrained`; nothing verifies and mints it. F generalises the rule to "pinned or not".

Retired here:

- `PlaceInWorld` (if stage 2 C landed it), `Node::Gauge`, `InstantiatePart.gauge`/`offset`;
- `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`;
- `check_offsets` and `OffsetDisagrees`/`OffsetUnchecked`;
- the spanning tree, roots and `MateRole`.

The refactor doors are rewritten over placements.

Migration check: every corpus copy's pose and the product digests are bit-equal. A gauge root becomes one mate to the world over a temporary `FrameBase::World`. Tree mates go to the child's bundle. Declaring mates become assertions where a stage 2 D measure states them, and are otherwise dropped and named.

Closes `a-declaring-mates-alignment-is-never-read`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `a-placer-row-states-what-a-poisoned-row-cannot` and the offset half of `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`.

Waits on stage 2 F (`a-mate-reads-face-variables`: mate sides read `Face` variables). It also waits on stage 4 J (`an-unattributed-contact-at-rest-is-a-finding`). B drops declaring mates, so their contacts are unattributed, and A5's hard error would refuse those products; J turns that error into a finding first.

**A host construction never reads a part's world** (from the second review of INTENT stage 2 C, PR #4359, NOTE-F). In stage 2 an instance's one `body` output is its part's whole product, every copy at its pose (`eval/parts.rs`), so a host boolean reading an instance depends on the part's placement poses: the pose is read "by the gather" only in the letter. Stage 2 C refuses the visible edges of this (`Uncarried::Posed` and `SplitError::RemainderReadUncarried` where split or inline would carry such a read across the seam), but the read itself stands. This unit's instance ports must close it: no port a host construction reads carries a part's world placements.

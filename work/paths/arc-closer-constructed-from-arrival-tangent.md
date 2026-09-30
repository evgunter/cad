---
id: arc-closer-constructed-from-arrival-tangent
kind: issue
title: paths: an arc_to mode that fixes the ARRIVAL direction — the arc from the tip to p whose end tangent is authored (its Start row is the construct-from-arrival tangent closer)
status: open
opened: 2026-09-02
github: 1578
refs: [1573, 433]
priority: P1
cost: D
---

## From GitHub issue 1578

Opened 2026-09-02; 0 comments.

**Filed from BOOL-12 (PR [#1573](https://github.com/evgunter/cad/pull/1573)) as the schedule for a disclosed deviation.**

BOOL-12's spec asked for the tangent seam member as a CONSTRUCTION: the circular arc through the departure point and `Start` whose end tangent is `Start.dir`, with a sharp departure. The unit built the declare-and-CHECK form on the existing `tangent_arc_to` construction instead (`tangent_arc_to(Start.arrives_tangent())`), for a measured reason: under the construction form the stadium — the member's canonical fixture and an acceptance criterion — cannot close (the constructed cap's start tangent is the incoming straight, so the derived departure junction is an undeclared tangency).

**What remains un-authorable without this form:** an author who knows where the seam is and how the closing arc must ARRIVE, but not the departure angle, must solve for the angle by hand. The construct-from-arrival form would remove that friction: `.angle(θ)` is then unnecessary for the arrival-declared case.

**Shape:** a new arc MODE on the point state (§2c family growth, arc-construction vocabulary, not seam vocabulary), so it lands on the ground BOOL-10 is redesigning (`arc_continue`'s retirement and the declared-subdivision arc form). Take it there or as a small S-BOOL unit after BOOL-10; the both-ends-tangent case stays the seam fillet's.

Refs issue 433, BOOL-12 (`docs/BOOL-12-SPEC.md` deliverable 3), BOOL-10.

## Home

`work/bool/` — the PATHS lattice work the Q1 ruling chain produced is S-BOOL's charter, and the issue names BOOL-10's `arc_continue` retirement as the ground it lands on.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Widened to interior targets (2026-09-18)

Ev, authoring a `B` in the viewer's path form, hit the same gap away from the seam. The first bump is `.angle(θ).tangent_arc_to(p)` with θ perpendicular to the spine; the second bump should arrive at the bottom of the spine perpendicular to it as well, and that is the fact the author KNOWS. The joint at the middle of the `B` is a sharp corner of no particular angle, so the departure direction is exactly the number nobody has. Today the second bump has to be spelled with a derived number: a departure `.angle(…)` into `tangent_arc_to`, a `Bulge` computed from the arrival angle, or a `Via` point picked on the arc.

**What is owed** is the general form, and the Start-targeting form this item was filed for is one row of it: a point-state `arc_to` mode carrying an endpoint `p` and an authored END direction (angle or director, the way `.angle` and `.toward` bind the departure), which determines the arc: it is `tangent_arc_to` reversed. The chord `p − tip` and the end tangent `t₁` fix the sweep, `θ = 2·∠(chord, t₁)`, so the bulge is `tan(θ/4)` of a derived angle and the step stores only authored data (PROFILES-V2 §V1). The refusals are the mirror of `tangent_arc_to`'s: `t₁` along the chord is a line, not an arc, and `t₁` against the chord is a full turn. The joint at the tip is a sharp junction checked like any other arc leg's start tangent. With `p: Start`, the end direction is `Start.dir` and is not authored, which is the closer this item describes.

**Where it lands**: a new `ArcData` mode on the point rows (`ARC_TO_POINT` in `crates/profile/src/path/program.rs`, and probably `FUSED_POINT` and `FUSED_LEG_END`), declared in `arc_modes!`. A mode is a vocabulary change to the ratified `docs/PATHS-DESIGN.md` §2c family, so the spelling is an `[ev]` PR. Once it lands, `arc_specs_at` and the viewer's mode picker offer it with no further edit, and `tests/arc_spec_census.rs` walks its cells.


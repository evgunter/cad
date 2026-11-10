---
id: carriers-are-identical-reads-carrier-identity-at-the-lattice
kind: issue
title: family.rs's carriers_are_identical (key path_carrier_identity) decides whether two carriers are the same inside the fillet family's arc extension — a lattice question the sixth-round ruling retires
status: closed
opened: 2026-09-08
refs: [2135]
priority: P1
cost: H
pr: 6682
branch: round/extensions-are-their-own-segments
closed: 2026-11-10
---

Found by BOOL-10's seal measurement (PR 2135): after `arc_continue`'s
removal, the remaining chain-side readers of `Incoming.carrier` are the
fillet family's — `resolve_fillet`'s Positive-fit arm, `family::merge_of`,
and `carriers_are_identical` (the §2c dissolution amendment's
arc-extension vertex-move choice, under the key `path_carrier_identity`).
The last is exactly an "are the carriers the same" question, which the
Q1 sixth-round ruling (Ev, 2026-09-02) says the lattice never asks. It is
ruling-adjacent and outside BOOL-10's fence, so it is filed rather than
deleted: the unit that takes it decides whether the arc-extension choice
re-spells as a declared thing (the vertex moves because the author said
so) or whether the amendment's case is one the ruling did not cover, and
puts that to Ev. `verbs.rs`'s header names the readers as of BOOL-10.
Difficulty S/M. Not scheduled.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Conditioning, found beside `PendingRunOut::rides` (2026-09-26)

Whatever the ruling decides, the measurement is ill-conditioned when a
side is rebuilt from a chord. `family.rs`'s `FusedIncoming::FromTip`
arm compares `inc.carrier` with the derived circle; an arc leg's
`Incoming.carrier` is `path.rs`'s `arc_carrier(at, p, bulge)`, whose
centre carries a rounding error of about ε_mach·R²/chord. At R = 1 a
chord of ~1e-7 m puts that error at the linear band, so the d + |Δr|
margin can escalate or answer "different carrier" for a leg that is on
the circle. `PendingRunOut::rides` (PR 3266) hit exactly this with its
first spelling — the reviewer's `line_arc_internal` repro escalated at
a 1e-7 rad run out and misnamed one at 3e-8 — and now measures point
deviations (the end's and the arc midpoint's radial misses) instead,
which round at ε·R whatever the chord. This arm was not reproduced
end-to-end; a unit that keeps the identity question should measure it
the same way or say why a short incoming arc cannot reach it.

## Ruled (Ev, 2026-11-09, PR 6542) — now a build unit

Both merges retire: an arc extension's and a ray extension's incoming
run is always its own segment, with a constructed tangent joint at the
tip; no emission asks whether two carriers are the same. Ev asked first
what had forced the August compromise; the answer (PR 6542's thread):
the merges worked around a same-carrier `TangencyContradicted` refusal
the sixth round retired, plus OnArc's bit-identity condition; neither
binds today. A run on main with all three merge sites disabled: no
validation refusal; four pinned rows move; the tip joint after an arc
extension comes out decided, not constructed (`family.rs` constructs it
only when not merging).

Spec: delete `extend_leg_to`, `extend_arc_to`, `merge_of`, the `merge`
arm of `emit_fillet_in`, `PendingMeta::extends_carrier` and
`extends_straight_leg` (PR 6545's forward-only guard), the merge role of
`ByTangent`, `carriers_are_identical` and its gate row; construct the
tip joint unconditionally; shrink `Incoming.carrier` if nothing reads it;
re-baseline the pinned rows and the stored-loop corpus hash, saying what
moved; re-word the code docs citing the dead "§4 item 4 exemption".
Loft run-awareness is LATHE's `loft-corresponds-sections-by-segment-not-by-run`.

## Closed (PR 6682)

The decision is gone rather than measured better.
- **Deleted:** `carriers_are_identical` with its
  `path_carrier_identity` gate, `merge_of`, `extend_leg_to`,
  `extend_arc_to`, `ByTangent` and `Incoming.carrier`.
- **The tip.** A fused or ray incoming's tip is always a vertex, with a
  constructed tangent joint, for every `r`.
- **The fit gate.** The merge used to absorb the run lengths between
  the fit gate's band and validation's. The fit now reads the run on
  validation's band; that closes
  `a-fillet-run-the-fit-gate-passes-validation-calls-degenerate`.
- **What stays.** At r = R in the band, the simplicity pass's
  `carrier_circles_identity` still escalates at the tip pair. That
  question is real: `sweep::swept::cosurface` reads it to choose one
  wall or two. PATHS-DESIGN documents it.
- **Filed for follow-up:**
  - the span window just past the band,
    `a-fillet-run-just-past-the-band-builds-and-then-escalates-a-span`;
  - DOCTAIL's strand-report row, raised to P0 because this change
    widens it to the ordinary fillet insert.

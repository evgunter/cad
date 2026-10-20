---
id: a-stored-arcs-end-is-never-checked-against-its-authored-vertex
kind: issue
title: paths: nothing checks a stored arc's end against the end vertex it was authored to, so a construction whose arithmetic misses passes validation silently (path.rs's 'verified-never-trusted' promise)
status: closed
closed: 2026-10-20
branch: paths/stored-arc-landing-check
opened: 2026-10-19
priority: P2
pr: 5449
---


Found by #5403's review (Q5), the class behind both
`tangent-arc-near-a-full-turn-loses-its-end-vertex` and
`a-center-arc-near-a-full-turn-loses-its-end-vertex`.

**The gap.** `path.rs`'s crate docs promise "verified-never-trusted;
nothing is trusted because the algebra produced it". A tangent or
`Via` arc's endpoint facts are `Facts::Registered`. They are theorems
of the reals, which `ConstructedProfile::validate` does not re-read
(D1). A `Center` arc's facts are `Decided`: equidistance is decided at
the door, but nothing reads the stored SWEEP against the end vertex.
So an arithmetic miss in the sweep went through `validate` with no
error at all:
- the tangent arc at R = 1.36e4 missed by 3.4e-8;
- the `Center` arc at R = 1.37e6 missed by 2.1e-4.

Both are fixed at the construction now, but the next construction
whose arithmetic misses will pass the same way.

**Why P2.** Once #5403 lands no silent miss past ε is known: the one
live miss its re-review's generator found is the hairpin fillet's,
about 7e-11 m (`a-hairpin-line-fillets-half-tangent-cancels`, which
carries its own derived worst case). This row is the guard against the
class's next instance, not a fix for a live one, so it sits below a
live silent wrong answer (P1) and above a cosmetic one.

**Shape of the check.** At storage time, decide
`‖landing(a) − b‖` (the `arc_landing` margin that `seg.rs` already
names for table arcs) against the band, and refuse it typed:
- definitely off: refuse;
- in band: escalate;
- past the scene's resolution: `ArcBelowSceneResolution`.

Questions for whoever takes it:
- whether a registered arc's landing is a decision at all, or a
  `debug_assert!`-class witness check (the registrants' pattern);
- what it costs the symbolic tier, since the landing's forms are new
  per arc;
- whether it supersedes `Facts::Registered` for the endpoint at the
  far end.

## Outcome

#5449 (`paths/stored-arc-landing-check`).

**The questions.**
- *Decision or witness check?* Neither, as the row framed them. It
  is a cross-check of the arithmetic, run on the witness lane only. A
  registered landing is a theorem of the reals, and the two misses
  were arithmetic losing the digits that theorem is about. A `Center`
  arc's landing follows from its decided equidistance the same way.
  So the check does not decide anything about the loop, and it is not
  a verdict: it goes through `k_stats::check_unlogged` (audit row F23).
  It is not a `debug_assert!` either. At an inexact witness an
  assertion is ruled out (SYM-6: a `Disputed` answer is never
  asserted on), so the loud answer is a typed refusal,
  `PathError::ArcMissesItsEnd`, which ends in the kernel-defect ending.
- *Where?* At storage: `Core::build`, before the chain is finished,
  and `circle_loop`. Every constructed arc passes through one of those
  two, at every scalar and every replay. It is not in
  `ConstructedProfile::validate`, which keeps D1's "not decided
  again", and not per construction, since the class lives at storage.
- *Cost?* On the symbolic tier, none: `Sym<Interval>` has an exact
  witness, so the check does not run there and mints no node. On
  `Sym<f64>` (coincide replay) it is one unlogged check per arc. The
  K stream gains one `path_arc_landing` sample per constructed arc at
  the `Probe` lane. A `path_arc_landing_resolution` sample appears only
  behind a Positive landing, so a sound arc never adds one. No verdict
  rows, no stored bits and no goldens move.
- *Does it supersede `Facts::Registered`?* No. The registration is
  what the symbolic tier believes and what the exact witness checks;
  the cross-check is the witness lane's reading of the same fact.
  Both stay.

**Why not at `Interval`.** An exact scalar's stored arc encloses the
arc its construction proved. A cancelling spelling makes that
enclosure wide; it does not make it wrong. Reading the landing there
could confirm Zero or escalate on the width, and it could never catch
a miss (the D1 page's own argument, which holds there). Both
mutants' `Interval` arcs do enclose their end vertex: that is pinned
by `a_cancelling_arc_at_interval_encloses_its_end_vertex`. They are
loud at `Interval` all the same, but through validation's
`arc_span` escalation, and that is also what the fixed code answers on
those probes (`a-near-full-arcs-span-reads-a-chord-scale-apex-at-interval`).

**Teeth.** `sugar::split_half_tan` was mutated back to its first arm,
which is both pre-#5403 spellings (the tangent arc's
`across/(|d| + along)` and the `Center` arc's
`σ(L/2)/(r + σ·offset)`). On the row's probes at R = 1.36e4, 1.37e5
and 1.37e6 the `f64` replay refuses `ArcMissesItsEnd` in all six cases
(4.7e-8, 4.8e-8, 4.9e-7, 5.0e-6, 1.6e-4 and 2.1e-4 m). Before this
change all six built and validated silently. Under the mutant,
`tests/tangent_arc_near_a_full_turn.rs` goes red with the new refusal
on the tangent, `Center` and near-behind rows. The kept rows are
`path::stored_arc_landing`, which spell both cancelling arcs by hand
at every ε row.

**The blind spot.** A lowered arc (chord + X) lands by construction,
so a cancelling X moves its carrier rather than its end. Under the
mutant, the `Via` row's arc missed its through point `q` by 1.4e-5 m
to 382 m and validated silently, with its landing within 4e-10.
Filed as
`a-via-arcs-carrier-is-never-checked-against-its-authored-through-point`
(P2).

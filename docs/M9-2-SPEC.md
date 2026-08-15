# M9-2 — the A5 at-rest door (spec)

Orchestrator work order for M9-PLAN item 2 (RATIFIED #509).
Substrates: the M9-1 exploration (2026-08-15) and the dedicated
chart-region exploration (same day). Owner ruling applies: built
here, consumed by ASM (R2-b cited below). ONE unit, TWO PRs:

- **PR-1 — the chart-region overlap predicate**: file-disjoint
  from M9-1's in-flight lane, no dependency on its types —
  DISPATCHES IMMEDIATELY, in parallel with M9-1.
- **PR-2 — the census door**: the tier-3′ arms, the import-side
  declaration channel, #382 half-2, the pin retirements. AFTER
  M9-1 PR-1 merges (consumes its record types).

## PR-1 — chart-region overlap (binding)

New module (topo/src/chart_region.rs or boolean/uv_contain.rs)
owning UV loop extraction, 2-D containment, segment crossing, and
the area margin. Reads geom-brep's Pcurve/ChartWindow and
geom-core's doors; **reads containment.rs/contain.rs for METHOD,
never refactors them** (no shared-code merge surface with M9-1).

1. **The planar trim inventory in (u,v), defined structurally**:
   `IsoLine`, `IsoArc` (its UV image is a straight segment), the
   derive-on-demand affine images of PLANAR faces, and `Harmonic`
   whose trig channels are exact-f64-structural zeros — the C6
   pattern (props.rs:632-641 precedent): structure read as
   structure, a numerically-almost-zero channel REFUSES typed. No
   scalar zero-test on T ever (the IsoLine-variant rationale is
   the binding statement). Everything else — sinusoid Harmonics
   (tilted cylinder cuts), `Fitted`, conic trims — refuses typed:
   the F5 envelope discipline moved to (u,v), stated in-doc with
   the tiltedcut corpus named as the honest exclusion.
2. **The certified lane is same-chart by construction**: the
   at-rest site (one body, shared `SurfaceKey` ⇒ identical chart)
   and cross-body rung 2 (same `GeomSource` ⇒ bit-identical
   descriptions ⇒ identical chart, the N6 theorem). **Rung-3
   (declared) pairs ESCALATE** — C2 itself says two descriptions
   of one locus may differ as charts (CONTACT-DESIGN:151-155), so
   "exact in chart space" is unachievable there; the honest
   posture is a typed escalation naming the chart divergence, not
   a margined pseudo-exact test. (A one-sentence clarification to
   C3's invariant recording this cross-rung scope goes in the PR
   for Evan's eye — it elaborates C2's own caveat, it does not
   re-litigate C3.)
3. **Containment/crossing machinery**: port point_in_loop's
   METHOD to 2-D — ray parity, a fixed 2-D direction schedule,
   the four named trileans re-derived (new predicate names = new
   K rows, margins re-metered); segment×segment crossing on the
   over_lever determinant/straddle-height form. Loop points come
   from PCURVE ENDPOINTS gated on variant — never the chord-
   polygon read that would silently accept a curved loop.
4. **Area, metered honestly**: chart-space shoelace (with IsoArc
   segments exact) → model area via chart arms → margin
   `over_lever(A, P)` (mean width, the split_section_area
   precedent, derivation in-doc). **The certified lane is
   restricted to charts with EXACT CONSTANT arms** (plane (1,1),
   cylinder (r,1)) — `chart_arms`' documented over-statement is
   safe for escape-metering and UNSAFE for a positive-area claim;
   lower stretch bounds (inf|S_u|, inf|S_v|) do not exist. NURBS/
   sphere/torus/cone charts refuse the area claim typed, and the
   inf-bounds extension is filed as a named follow-up issue at PR
   time.
5. **Seam branches**: a pair whose loops sit on different periodic
   branches REFUSES typed (no region representation crosses a
   seam; ChartWindow::hull is branchless min/max) — branch
   normalization is a possible later rung, not this unit's.
6. Determinism: fixed schedules (D9); every decision through
   named decide/Margin doors; ledger rows for any decide_flagged
   (expected: none — this unit should need no flagged site).

## PR-2 — the census door (binding; dispatch after M9-1 PR-1)

1. **Census arms beyond exact-on-planar**: CurveContact certified
   per C3's jet schedule (M9-1's certifier); PatchContact
   certified through PR-1's predicate; `Declared::index` grows
   face-granularity keys; `confirm_declarations` (two-directional)
   extends verbatim; `UndeclaredContact` reach extended with the
   kernel finding type (M9-1's layering ruling).
2. **The import-side declaration channel** (D7 step 4's residue):
   adoption can attach declarations so a touching import
   certifies at 3′ under the SHARED gate — no import-only
   validity path (the #276/#260 one-gate ruling binds).
3. **#382 half-2 executes here**: inter-instance overlap between
   disjoint-keyed solids surfaces as undeclared contact / C6's
   recorded-gate-skips posture through the SAME census arms — not
   an ad-hoc graft check; the #382 issue closes with this PR
   citing the half-1 doc language it makes true.
4. **Pin retirements (acceptance)**: the KISS-ASSEMBLY pin
   (review_r1_tier_gate_probes.rs:601) and the boss_union
   "refuses HERE" pin (m5_pr9_boss_union.rs:127) both execute
   their own retirement text; tier_gate.rs:46's stale "M8 contact
   program" label truthed in passing.
5. R2-b consumption points cited in-code where the records
   surface (ASM-R2-SPEC-DRAFT:39-58): same currency, no adapter.

## Acceptance

1. PR-1: overlap/containment rows on constructed planar and
   loft-wall (IsoLine/IsoArc) fixtures — overlap certifies
   definitely-positive with the mean-width margin; disjoint
   regions answer empty (⇒ stale at the consumer); an in-band
   sliver ESCALATES (three-outcome, never silent); the
   structural-inventory gate proven by a red-then-green mutant
   that lets a sinusoid Harmonic through; the seam-branch refusal
   pinned; the tiltedcut exclusion pinned as typed.
2. PR-2: kiss assembly certifies WITH declarations and refuses
   WITHOUT under the shared gate (the pin's inversion, exact);
   boss_union's curved touching result validates at 3′ with its
   declared Rest and still refuses undeclared; a two-instance
   overlapping placement (the #382 fixture class) refuses as
   undeclared contact naming the guilty pair; crosslap
   unregressed; ε-row three-outcome honesty on every new row.
3. Hosted CI fully green both PRs; the M9-1→PR-2 serialization
   holds (PR-2's branch merges M9-1 PR-1 before opening).

## Process

Unit protocol: implementer arm = next block position at dispatch
(v4, block M8-15 continues; positions consume in dispatch order).
Difficulty pre-dispatch: PR-1 **M**, unit **L**; task-class
NUMERIC. One blinded reviewer + fix pass at the unit level (row
at PR-2, the two-PR precedent) unless PR-1's findings warrant an
early round — orchestrator's call at PR-1 completion. Review
ordinal claimed from the ledger ON MAIN at review dispatch.
Standard brief lines (foreground discipline, no trailers,
invariant comments, lane-private publish paths, ε honesty,
k-lint discipline, merge-main + union).

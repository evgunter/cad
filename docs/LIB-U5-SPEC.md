# LIB-U5 spec — read-back/interrogation doors (binding)

Mandate: LIBRARY-DESIGN §L5 U5 — the model answers what the
parameterization chose — killing the P3/P4 restatement class.
Measured basis: `~/.local/share/cad-work/u5-census.md` (executed
2026-08-09; cite, don't re-survey). Scoped by three ratified
rules the census pinned:

1. **Values, never verdicts** (LB7's line): a door returns "this
   face's plane is (o,n)"; it never returns "is this planar" —
   geometric predicates stay deferred.
2. **The pad rule** (Q8, the MassProperties::volume_pad
   precedent): any answer from an approximating/quadrature
   source carries its certified residual; definitional-surface
   read-back is a re-read of authored data and needs none.
3. **The oracle discriminator**: restatements of a KERNEL CHOICE
   migrate to queries; INDEPENDENT derivations of an invariant
   (az's exact fractions, the Pappus closed forms, dyadic
   volumes) are tests and MUST NOT be replaced. The census §1
   table is the disposition list.

**LB12 (ruled at spec time): the EntityRef leak SEALS.** U7's
`pncad::select` export of `EntityRef`/`Entry` contradicts
table.rs's own arena-keys-never-leave-editor-core rule (G1).
U5 removes both from the pncad surface (clean break — they are
days old) and replaces them with name→geometry doors; `Entry`'s
tie information surfaces through a typed result instead
(`Unique`/`Tied` without keys). Nothing else may re-expose keys.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Slot rules: `local-scripts/with-build-slot.sh --
cargo ...`; `--express SECS` for ≤10-min rows; long rows default
mutex with BLOCKING foreground waits (re-issue timeouts; setsid +
foreground-poll past the harness cap); NEVER park. Cold-clippy
verification (cargo clean -p touched crates first) at default AND
--features interval + discipline greps BEFORE opening. Commit AND
push per chunk. NO Co-Authored-By, no model names. Merge
origin/main before opening (SWITCH-P #273 may merge mid-unit —
crates/profile moves; your footprint is disjoint but re-merge);
confirm checks STARTED.

## 1. Fence

In scope: `crates/sweep` (return-type field + Section-shaped
front doors), `crates/editor-core` (names/interrogation module,
additive), `crates/pncad` (doors + the LB12 seal), `demos/tour` +
`crates/step-export/tests` (the migration below). OUT:
`crates/profile` (SWITCH-P's lane), predicates/verdicts of any
kind, per-face area/centroid (census (c): needs per-face
quadrature pads — report-only), schema/persist, CI, docs/M*.

## 2. Deliverables

1. **Skin-parameter doors** (census (a), the cheap kills):
   `Lofted` gains `section_params: Vec<f64>` (the value is
   computed at loft.rs:555 and currently discarded); a
   `loft_parameters(&[Section], v_degree) -> Vec<f64>`-shaped
   front door taking what authors hold (internally the existing
   make_compatible → skin_parameters path — no new math);
   `sweep_places` re-fronted the same way if the census's
   Section-shaped gap applies. Migration: skinned.rs:362-373 and
   step-export common:799-820 replace their hand-derived t with
   the query + an assertion against the SAME asserted value (the
   printed narration constants stay as pins — SAID changes, no
   geometry).
2. **Name→geometry doors** (census (b), the unit's weight): in
   editor-core, `face_frame(&ev, node, &name)`, `edge_frame`,
   `vertex_position` — returning a designed `Pose`/frame type
   (analytic surfaces read their stored origin+axis — Q8
   definitional re-read; NURBS faces: state the convention or
   refuse typed `NoCanonicalFrame` — your measured call,
   REPORTED, with the §2b-register doc stating which); plus
   `entity_ref`'s sealed replacement per LB12. pncad exports the
   doors; `EntityRef`/`Entry` leave the prelude and select.rs
   surface.
3. **Cap/joint frames**: the tube/swept cap-plane read (lily's
   assert_cap hand-scan at :793-805 becomes a door); the P4 pin
   table (lily.rs:779-789 turtle transcriptions) migrates to
   queries + assertions where the door answers it — sites the
   doors CANNOT yet answer stay with a gap comment naming what
   door is missing (that list feeds U4/follow-ups; do not build
   new kernel computation for them).
4. **Profile fillet-at-corner**: the "which segment is the
   fillet at corner k" door (kills rocker's find-by-radius scan
   at :226-229) — profile-level, keyed by the corner's authored
   position or index, returning the arc's SegmentKind data.
5. **Doctests** on every new door (the pncad convention).

## 3. Acceptance

- Byte-identity: tour + step-export exports at 3 ε rows vs your
  own base build (this unit changes how values are OBTAINED,
  never the values; narration text may change only where a
  hand-derivation comment becomes a query mention — list every
  narration line that changes).
- The migrated sites assert the same constants they asserted
  before (the pins survive as pins).
- Full batteries on touched crates; zero new [[test]] binaries.
- The LB12 seal proven: grep pin — no EntityRef/EntityKey
  nameable from pncad.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-u5-report.md`, per-phase figures,
gaps annotated. Open, do NOT merge. Final message: PR number +
report path only. Genuine forks (the NURBS-frame convention and
the Pose type's shape are the likely spots): report, smallest
faithful reading, flag.

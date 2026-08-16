# LIB-U4B spec — the frame-constructor family in geom-core (constructors only; binding)

Mandate: the ratified LQ3(c) (LIBRARY-DESIGN §L7, #362 + Evan's
resonance amendment): frame CONSTRUCTORS in geom-core — point-at,
mirror, and the P1 path-start frame with a stated degenerate-axis
policy — consumed as plain Affine3 values. SCOPE IS CONSTRUCTORS
ONLY: this unit adds the family + its own tests; NO demo/corpus
migration (that rides later consumers — keeps this unit
crate-disjoint from the in-flight RETTAIL/RESPELL work).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding
(local-scripts/with-build-slot.sh, foreground one-at-a-time,
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge, checks
STARTED, cold clippy CI scope, k-lint discipline, invariant
comments).

## 1. Deliverables

1. **The family, in geom-core** (a new module beside the Affine3
   machinery): (a) `point_at` — a frame whose axis aims at a
   target point, with the roll convention STATED and pinned
   (P6's lily evidence is the acceptance context); (b) `mirror`
   — reflection across a stated plane/line, orientation
   consequence documented; (c) `path_start_frame` — the P1
   Gram–Schmidt recipe written ONCE, with the degenerate-axis
   policy STATED (the skinned.rs `n.z.abs()<0.9` dodge becomes a
   designed policy: name the fallback axis rule and refuse typed
   at the truly-degenerate case rather than dodging).
2. **Resonance (Evan's #362 amendment, REQUIRED)**: the
   vocabulary uses the SAME TERMS as the PATHS placement family
   (`nurbs`/`nurbs_reversed`/`nurbs_mirrored` semantics — mirror
   means reflection with the stated orientation consequence;
   placement means rigid, no scale/deform). The unit's spec-level
   duty: state in module docs which PATHS term each constructor
   resonates with; unify outright ONLY if it falls out naturally
   (Evan's guess: not worth forcing) — SAY which happened.
3. **Tests in geom-core only**: orientation pins (12-entry
   bitwise rows per the ASM Frame precedent if apt), the
   degenerate-axis refusal rows, mirror orientation flips,
   point-at roll convention pins, round-trips through Affine3.
4. **Findings**: compare each constructor against its hand-rolled
   demo twin (skinned.rs:476-488, lily.rs:248-276, diefillet's
   axis-angle) BY READING (no demo edits) — report divergences as
   numbered findings; the future migration consumes them.

## 2. Fence

OUT: any demo/corpus/tour edit, sweep/loft signature changes,
SketchPlane changes, editor-core/schema/Expr (VQ8 stays
deferred), Python bindings, U4a path legs, CI structure. This
unit touches crates/geom-core ONLY. Anything missing: REPORT.

## 3. Acceptance

cargo test -p geom-core green (delta stated); cold clippy CI
scope; zero new [[test]] binaries; hosted CI green; the
resonance table present in module docs.

## 4. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-u4b-report.md, per-phase figures +
pre-draw fields (difficulty M, task-class NUMERIC — frame
orthonormalization and degeneracy thresholds are numeric
decisions). Open, do NOT merge. Final message: PR number +
report path + ≤10-line summary.

# LIB-PYBUNDLE spec — the bindings-parity tail: G4/G6/G7/G9 + riders (binding)

Mandate: continue closing the north-star audit's remaining gaps
where the document layer already has the vocabulary. The 2026-08-10
substrate survey established the Node enum ALREADY carries
`Fillet { target, radius, selection }`, `Split { target, tool }`,
`Transform { input, translation, rotation_axis, rotation_angle }`,
`Pattern { input, count, kind }`, `Declare { pairs }`, and
`Boolean { ..., declare }` — so G4 (fillet), G6 (split), G7
(transform) and G9 (multi-loop profiles) are BINDING work. This
unit is the first PYBUNDLE slice: **G4 + G6 + G7 + G9 + the
banked riders**. OUT (own units later): G5 declared-contact (its
Python story should land together with the R3 refusal-menu wiring
— one coherent detect/declare surface), G8 pattern+structural-param
(blocked on the kernel pattern×boolean payload gap the survey
recorded — bind `Pattern` only if the evaluate path actually
serves the heatsink shape TODAY; measure, report, do not force),
G11 tessellation/STL (rides with G5 or its own S unit).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim — foreground builds one at a
time, build slots, no parking (kill your own stale waiter first),
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments state
the INVARIANT.

## 1. Design ground (binding)

LIBRARY-DESIGN §L3/§L4; the audit page at head;
crates/editor-core/src/node.rs (the five variants above and their
Expr slots); names/StableName (Fillet.selection is Vec<StableName>
— U7's name doors are the selection vocabulary);
crates/pncad-py house patterns (PYG1/PYG23A). Scenes:
diefillet/diecomposed (G4), tiltedcut/cutaway (G6),
crosslap_exploded (G7), plate/rocker/az (G9), diepips
(G1+G7+group-boolean — measure what its row needs).

## 2. The binding design (settled; deviations numbered + reported)

1. **`Node.fillet(target, radius, selection)`** — radius Length →
   Expr literal; selection a non-empty list of StableName strings
   (the U7 selector vocabulary as it crosses the document layer;
   NO new selector machinery — the names the audit scenes already
   use). Refusals from the kernel through existing tags.
2. **`Node.split(target, tool)`** and **`Node.transform(input,
   translation, rotation_axis, rotation_angle)`** — translation
   (Length,Length,Length), axis dimensionless floats, angle
   Angle; all → the variants' Expr slots exactly as extrude/
   revolve do.
3. **`Node.profile` grows multi-loop**: accept a single loop OR a
   list of ClosedLoops (ProfileProgram.loops is already a Vec) —
   the audit's G9. Stub via @overload; validation stays kernel-
   side (nesting/containment refusals at evaluate, untouched).
4. **Riders (from ordinals 19/20/22, adjudicated in-log)**:
   (a) `DocParam.__eq__`/`__hash__` (Rust PartialEq exists —
   mirror it; document the semantics); (b) `SketchPlane`
   accessors `origin`/`u`/`v`/`normal` + bit-exact `__eq__` (the
   Doc.bit_eq precedent) IF the Rust surface gains the same
   accessors additively — one vocabulary, else report;
   (c) `Node.boolean` gains `declare=` (the existing Node field,
   currently hardcoded None in the binding — G5's DATA door only;
   the detect/declare protocol surface stays out).
5. Nothing re-implemented; every argument crosses immediately;
   typed refusals at the call site or evaluate.

## 3. Fence

OUT: G5's detect/declare protocol surface (find_flush_candidates
etc. — future unit with R3), G8 unless measured-servable, G11,
sweep/tube (U4 territory), Expr-bearing arguments beyond what
plate_param already exercises, kernel/editor-core changes except
measured-additive accessor twins under §2.4b, schema/persist, CI
structure. Anything missing: REPORT, never build. ASM-1 may land
schema v5 mid-unit — re-merge promptly, report collisions.

## 4. Deliverables

1. §2's bindings + stubs + ty fixtures (legal chains + illegal
   rows per new door).
2. **Audit flips, honest, counts re-derived from the table**:
   expected flips — diefillet (7, G4), tiltedcut (11, G6; its
   secondary "also circles" is G1-closed), cutaway (31, G6+G7),
   crosslap_exploded YES*→YES (29, G7), plate (2, G9),
   rocker (6, G9), az (27, G9), diecomposed (9, G4+diepips
   chain — measure; if group-boolean blocks, re-partition
   honestly), diepips (8 — measure the placement/group need
   against Transform; flip only what executes). Every flip
   against the scene's exact oracle; YES* semantics per the
   established vocabulary; absence assertions flip; G4/G6/G7/G9
   → Closed gaps as earned.
3. Guide: one fillet-by-name block and one multi-loop (plate)
   block, both languages' parity stated (Rust blocks exist).
4. plate_param residue check: with G9 closed, state on the audit
   page what NOW blocks plate_param-from-Python (the Expr-bearing
   profile steps door — G1's recorded residue), keeping the
   register pointer accurate.
5. Numbered findings per the demos' binding purpose.

## 5. Acceptance

Python suite green (state delta); cargo test -p pncad-py -p
pncad; cold clippy CI scope both lanes; hosted CI green; zero new
[[test]] binaries; stub + ty rows green; audit arithmetic
script-re-derived.

## 6. PR discipline

One PR (split into two only if the diff exceeds ~2.5k lines —
then G4/G6 first, G7/G9+riders second, each with its own review).
Report ≤150 lines to ~/.local/share/cad-work/lib-pybundle-report.md
with per-phase figures. Open, do NOT merge. Final message: PR
number + report path + ≤10-line summary. Forks: report, smallest
faithful reading, flag.

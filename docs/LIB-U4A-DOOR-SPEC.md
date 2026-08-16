# LIB-U4A-DOOR spec — the chain→curve composition door in geom-curves (binding)

Mandate: the ratified LQ3(b) (LIBRARY-DESIGN §L7, #362; M8
kernel-side concurrence recorded on that thread; their #369
long-turn unit covers the complementary single-curve half): a
geom-curves door composing an ordered chain of C¹-compatible
legs (line/arc/nurbs segments) into ONE NurbsCurve3 — the exact
join §10.4's sweep consumer wants, and the honest discharge site
for the banked SWEEP_FRONTIER. SCOPE IS THE DOOR ONLY: no
editor-core wiring, no wire_sweep changes, no path-authoring
vocabulary (that is U4a-proper, sequenced after RESPELL). This
unit touches crates/geom-curves (+ additive geom-core helpers if
measured necessary — report first).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding
(local-scripts/with-build-slot.sh, foreground one-at-a-time,
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge, checks
STARTED, cold clippy CI scope, k-lint discipline, invariant
comments).

## 1. Deliverables

1. **The door**: `compose_chain(segments: &[...]) ->
   Result<NurbsCurve3<f64>, ComposeError>` (name/signature per
   the crate's conventions — measured; segments as exact curve
   pieces, e.g. the existing arc/line→NURBS exact conversions).
   The join is EXACT: C⁰ by construction (shared endpoints
   verified typed, not value-matched — take endpoints by
   reference/shared data per the authored-once doctrine),
   C¹ verified at each seam by the crate's own tangent
   predicates; refusals typed per seam (`ComposeError` naming
   the seam index and the failed property). Knot-vector merge
   per the crate's KnotAlgebra machinery (reuse, do not
   re-implement).
2. **Exactness contract stated and pinned**: composing exact
   arcs yields the exact rational representation (no sampling,
   no fitting — this door is the anti-interpolate); a
   differential row proves compose(quarter-arc, quarter-arc)
   equals the exact half-arc's curve data where the
   representation admits it, and states the knot-insertion
   normalization where it does not.
3. **The s_duct oracle as acceptance evidence** (READ-ONLY use
   of the demo): rebuild s_duct's 17-point interpolated path's
   INTENT as compose(two exact quarter arcs + lines) in this
   unit's OWN tests and measure the divergence from the
   interpolation (the P2 finding, quantified — evidence for the
   future U4a-proper migration; the demo itself is untouched).
4. **Findings**: anything the future wire_sweep discharge or
   U4a-proper vocabulary will need that the door's shape makes
   awkward — numbered, for the design record.

## 2. Fence

OUT: wire_sweep / SWEEP_FRONTIER changes (the discharge is its
own later unit, coordinated with the kernel program),
editor-core, schema, path-authoring vocabulary, demos/corpus
edits, Python, CI structure. Anything missing: REPORT.

## 3. Acceptance

cargo test -p geom-curves green (delta stated); the exactness
differential rows; refusal rows per seam property; cold clippy
CI scope; zero new [[test]] binaries; hosted CI green.

## 4. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-u4a-door-report.md, per-phase
figures + pre-draw fields (difficulty M-L, task-class NUMERIC).
Open, do NOT merge. Final message: PR number + report path +
≤10-line summary.

# LIB-PYSEL spec — audit gap G13: the selector surface from Python (binding)

Mandate: close G13 (minted at #376, re-scoped at its fix pass —
the ordinal-28 ruling): Rust's selector surface (U7 structural +
SEL1 geometric) narrows materialized sets fine
(lib_sel1_geoselect.rs:507-560 narrows diecomposed's own two
filters name-for-name); Python has NO selector surface, so
fillet selections from Python are authorable only via the
opaque-by-contract name text (forbidden as API) or full-set
operations. This unit binds the selector vocabulary; diecomposed
flips YES*→YES as the acceptance scene.

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT).

## 1. Design ground (binding)

LIBRARY-DESIGN §L3/§L4; SELECT-DESIGN (the ratified selector
doc) + the SEL1/SEL2 rows in docs/MODEL-AB-LOG.md for what
landed; crates/pncad's curated selector surface (Selector,
NamePat, SegTag, GeomPred exact/decided split, select_where —
census what pncad::document/prelude actually re-export TODAY and
report any curation gap rather than widening silently);
crates/pncad-py house patterns; the PYBUNDLE-landed all_*
materializers (your input sets); demos/tour/src/diecomposed.rs +
lib_sel1_geoselect.rs (the acceptance filters).

## 2. The binding design (settled; deviations numbered + reported)

1. **`select_where(materialized_set, predicate)`** and the
   selector/predicate value vocabulary cross with prelude-parity
   names, verb-for-verb with the Rust surface — GeomPred's
   exact/decided split preserved as TYPED structure (the trilean
   discipline crosses; no boolean flattening; a Tied/undecided
   outcome refuses exactly as Rust's does).
2. **Nothing re-implemented**: predicates are DATA constructed in
   Python, evaluated in Rust; no Python-side geometry, no
   isinstance ladders; typed refusals through the tags machinery.
3. **The result feeds the existing sinks**: what select_where
   yields plugs into Node.fillet's selection (and any other
   StableName sink) without touching name text.
4. Quantities per §L4 where predicates carry dimensioned
   arguments (lengths/angles as Length/Angle; kinds as bound
   enums — bind the smallest faithful forms of CurveKind/
   SurfaceKind sets the predicates need; SurfaceKind is already
   curated).
5. OUT (fence): SEL2's detect/declare protocol
   (find_flush_candidates/declare — that is G5's slice with R3),
   the reserved convexity atom (GS-Q2), any NEW predicate atom,
   any kernel/editor-core change (curation-additive in pncad
   only if measured necessary — report first), CI structure,
   schema.

## 3. Deliverables

1. §2's bindings + stubs + ty fixtures (legal chain incl. the
   diecomposed filters; illegal rows: a decided predicate where
   exact is required if the Rust types split them, wrong
   dimensions, etc. — mirror what compiles).
2. **Audit**: diecomposed YES*→YES with the scene's own two
   filters executed from Python against its exact oracle
   (0.952915... — the Rust row's value), fillet selections built
   via select_where, ZERO name-text parsing; G13 → Closed gaps;
   counts script-re-derived. The impossibility/absence rows
   flip honestly.
3. **Guide**: one select-then-fillet Python block (executed),
   mirroring the Rust selector section.
4. plate_param residue line and the G5/G12/G14 pointers stay
   accurate; numbered findings per the demos' purpose.

## 4. Acceptance

Python suite green (state delta); cargo test -p pncad-py -p
pncad; cold clippy CI scope both lanes; hosted CI green; zero
new [[test]] binaries; stub + ty green; audit arithmetic
re-derived from the table.

## 5. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-pysel-report.md with per-phase
figures. Open, do NOT merge. Final message: PR number + report
path + ≤10-line summary. Forks: report, smallest faithful
reading, flag.

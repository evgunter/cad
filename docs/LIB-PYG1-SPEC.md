# LIB-PYG1 spec — audit gap G1: arcs and circles in profiles from Python (binding)

Mandate: close the north-star audit's G1 (docs/guide/
north-star-audit.md — 7 stops blocked; "the single biggest
blocker") by binding the PATHS typestate lattice to Python per
LIBRARY-DESIGN §L4. The residual register (docs/LIB-LOG.md,
category B) names this the opening unit of the bindings-parity
program. NOTE ON NAMES: this is the AUDIT's G1; docs/LIB-G1-SPEC.md
is the unrelated Rust vocabulary-growth unit — do not confuse them.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes.
Slot rules: `local-scripts/with-build-slot.sh -- cargo ...`;
`--express SECS` for ≤10-min rows; long rows default mutex,
BLOCKING foreground waits (timeout 590000, re-issue after killing
your own previous waiter; setsid+poll past the cap); NEVER park —
run every build/battery row as a synchronous FOREGROUND Bash call,
one at a time, reading each result before the next; NEVER arm
waiters, monitors, or background chains for your own builds/tests;
when the build-slot queue is busy, a BLOCKING foreground wait is
the correct state. Commit AND push per coherent chunk. NO
Co-Authored-By trailer in lane commits; never name any model; if a
model mention lands in a PUSHED commit, STOP and report. Merge
origin/main immediately before opening the PR, re-merge whenever
main moves while it is open; after any push confirm checks STARTED
(`gh pr checks`). Cold clippy both lanes before opening: `cargo
clean -p <touched-crates>` then CI's exact clippy invocations
(ci.yml ~425 and ~612). If the k-lint gate fires, do NOT change
geometry to silence it — report to the orchestrator. Comments
state the INVARIANT, not the history.

## 1. Design ground (read first, binding)

- docs/LIBRARY-DESIGN.md §L3 (Python speaks the document layer;
  authoring sugar emits recipe data; ONE lowering, two host
  languages) and §L4 (the type story: stub lattice static under
  `ty`; runtime checks at the Rust boundary ONCE; typed
  quantities; CI-checked stubs).
- docs/PATHS-DESIGN.md §2 (the lattice, binders/directors, legs,
  fillet, Start closure), §2a (G1 vocabulary: circle, arc_via,
  arc_center, far-end .to(anchor), toward), §2b (arc-carrier
  fillet modes at_on/to_on and the squared-radius rule).
- The Rust surface you are mirroring: `pncad::prelude::{Open,
  PartialPath, Start, circle, PathError, ...}` (crates/profile/
  src/path.rs; program recording in path/program.rs).
- docs/guide/north-star-audit.md — the measurement this unit
  moves.

## 2. The binding design (settled; deviations numbered + reported)

1. **Distinct runtime classes per lattice state**, exposing ONLY
   that state's legal continuations (an off-lattice call is an
   AttributeError because the method does not exist — no
   isinstance ladders, no runtime state flags): `PathOpen`,
   `PathPoint` (plain), `PathDirectedPoint` (leg end, incoming
   tangent intrinsic), `PathAngle`, `PathDirected`. If the Rust
   typestate exposes an asymmetric method set between Directed
   flavors, split the class rather than widen a method set —
   mirror what COMPILES in Rust, exactly. `Start` is a first-class
   token; closing verbs return a `ClosedLoop` value.
2. **Prelude-parity names**: export `Open`, `Start`, `circle`
   under the same names as the Rust prelude; verbs verb-for-verb
   (`at`, `angle`, `toward`, `tangent`, `to`, `line`, `line_to`,
   `arc_to`, `arc_via`, `arc_center`, `tangent_arc_to`, `fillet`,
   `at_on`, `to_on`, `turn`, — the full CURRENT Rust vocabulary,
   nothing invented, nothing dropped except §3's fence). One
   semantics, two host languages; the guide will show the same
   loop in both.
3. **Arguments are typed quantities** per §L4 and the existing
   `Node.polygon` convention: points as `(Length, Length)`
   tuples, lengths/radii `Length`, angles `Angle`, `toward(dx,
   dy)` dimensionless floats, bulge a float. Winding crosses as
   the bound form of the Rust enum (bind it if absent, smallest
   faithful form).
4. **The Python layer re-implements NOTHING**: each method
   crosses into the same Rust machinery immediately, so geometry
   refusals (junction check, NoCornerForFillet, tangent-line
   close, ...) fire AT THE CALL SITE as the SAME typed errors
   (PathError through the established tags machinery — typed
   payloads, never strings). No pre-checks in Python, no
   re-verification, no parallel lowering.
5. **Terminal**: `Node.profile(loop, elevation=...)` builds the
   document ProfileProgram node from the loop's RECORDED program
   (record-as-you-lower already exists — find and reuse the
   existing Step→document-program seam; do NOT hand-write a
   parallel conversion). Plane story identical to `Node.polygon`
   (xy + elevation; planes are G3, not yours). Exactly ONE loop
   (multi-loop is G9). `Node.polygon` remains untouched.

## 3. Fence

OUT of scope: loft/sweep/tube (audit G2), non-xy planes (G3),
multi-loop profiles (G9), rigid placement (G7), pattern (G8),
NURBS legs (register category D — unbuilt in Rust too),
Expr/param-bearing profile steps from Python (record as the named
follow-up that, with G9, completes plate_param-from-Python), any
kernel or editor-core change (this is a binding unit: crates/
pncad-py + at most additive pncad curation; if a missing door
blocks you, REPORT it — never build it, never work around it
silently), CI structure (see §4 ty disposition), renders.

## 4. Deliverables

1. The lattice classes + Start + circle + ClosedLoop +
   Node.profile in crates/pncad-py, per §2.
2. `.pyi` stubs for the whole lattice — each stub class declares
   ONLY its state's legal continuations; name-for-name check
   green under BOTH layouts.
3. Illegality rows: runtime absence probes for PATHS §2's
   canonical illegal states (double director; `.tangent()` on a
   plain point; leading `.fillet`; leg from a half-bound tip;
   `close()` does not exist) — the Python analog of the Rust
   E0599 probes.
4. **ty disposition, MEASURED**: if the hosted python-suite
   environment can run `ty` (or the venv tooling U9S built can,
   cheaply — dependency policy: ≥2-week-old release), add a
   stub-level static check: guide snippets typecheck, a small
   deliberate-illegal set FAILS typecheck (the compile-fail
   analog). If the environment cannot support it honestly, say so
   in the report and record the follow-up — do not fake it.
5. **Audit flips, honest**: bracket (1), vase (3), sheave (4),
   bossplate (12) become executed YES rows in
   test_north_star.py, each rebuilding the scene from Python
   through the new surface and asserting the SAME exact oracle
   the Rust scene asserts. Rows where G1 was primary but
   secondary blockers remain re-partition to their
   next-most-fundamental gap (rocker, diepips — recount so the
   gap ids still partition the NO rows exactly). The audit page
   updates: G1 → Closed gaps table, authorable count 7→11,
   gap-list arithmetic re-stated; absence assertions flip
   (test_the_named_gaps_are_still_gaps).
6. **Guide**: the profile-authoring section gains the Python
   mirror of the Rust PATHS blocks (executed from Markdown by
   test_guide — same loop, both languages, oracle asserted).
7. **bracket.py upgraded to the REAL bracket** (the filleted
   §L3 journey — demo-purpose memory: natural usage, awkwardness
   recorded as findings, never hidden). Byte-identity is NOT owed
   (new authoring); oracle equality IS.

## 5. Acceptance

- Python suite green (count grows; state the delta), incl. new
  north-star rows, stub check, illegality rows, guide blocks.
- cargo test -p pncad-py -p pncad green; batteries on touched
  crates; cold clippy both lanes; hosted CI green.
- Zero new [[test]] binaries; lint drift-check green.
- Every awkwardness met while authoring the four scenes recorded
  as a numbered library finding in the report.

## 6. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-pyg1-report.md` with per-phase
token/wall figures. Open the PR, do NOT merge. Final message: PR
number + report path only. Forks: report, take the smallest
faithful reading, flag for adjudication.

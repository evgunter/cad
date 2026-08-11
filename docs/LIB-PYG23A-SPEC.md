# LIB-PYG23A spec — audit G3 (non-xy sketch planes) + G2's loft half (binding)

Mandate: close the north-star audit's G3 entirely and G2's LOFT
portion from Python. The substrate survey (2026-08-10) established:
`Node::Loft` exists in the document layer since M5 PR 10 (node.rs
~:300) with evaluation (`wire_loft`) and program-anchored naming —
its binding is mechanical; `SketchPlane` has only `xy()` and
`from_frame(origin, u, v)` — G3 needs two additive Rust
constructors plus the binding. SWEEP/TUBE ARE EXPLICITLY OUT (see
§3 — kernel frontier banked by ruling; U4/LQ3 ratified-open;
schema-v5 collision with ASM-1).

## 0. Discipline (absolute)

Identical to docs/LIB-PYG1-SPEC.md §0, verbatim — foreground
builds one at a time, build slots, no parking (kill your own stale
waiter before re-issuing), commit+push per chunk, NO
Co-Authored-By, no model names (STOP-and-report a pushed leak),
merge origin/main before opening + re-merge as it moves, confirm
checks STARTED, cold clippy at CI's exact scope both lanes,
k-lint discipline, comments state the INVARIANT.

## 1. Design ground (read first, binding)

docs/LIBRARY-DESIGN.md §L3/§L4; docs/guide/north-star-audit.md;
crates/editor-core/src/node.rs (`Loft { profiles, v_degree }`),
eval/wire.rs::wire_loft, tests/corpus/loft_prism.rs (the document
twin you are making authorable); crates/profile/src/lib.rs
(SketchPlane); crates/pncad-py/src/py/doc.rs (the binding
patterns; polygon's elevation→from_frame seam);
demos/tour/src/skinned.rs (loft_prism/nonuniform_loft) and
letterforms.rs (the silhouette family). PYG1's landed patterns
(typed quantities, tags, stubs+ty fixtures) are the house style.

## 2. The binding design (settled; deviations numbered + reported)

1. **SketchPlane crosses to Python as a value.** Additive Rust
   constructors first: `SketchPlane::yz()` and `SketchPlane::zx()`
   beside `xy()` (profile crate, canonical frames, doc'd with the
   same orientation conventions the letterforms captions use);
   Python binds `xy/yz/zx` plus the general
   `from_frame(origin, u, v)` (origin as (Length,Length,Length),
   u/v as float 3-vectors — same faithful contract as Rust,
   including its documented unchecked-rigidity convention, stated
   in the stub docstring; NO Python-side orthogonality predicate).
2. **`Node.polygon` and `Node.profile` gain `plane=`**, mutually
   exclusive with `elevation=` (both → TypeError at the boundary;
   elevation stays as the xy sugar it is today). The lowering
   reuses the existing from_frame seam — one spelling of plane
   construction, no duplicate.
3. **`Node.loft(profiles, v_degree)`**: profiles a non-empty list
   of NodeIds, v_degree an int crossing as `Expr::count` (the
   corpus twin's exact form). Refusals are the kernel's typed
   LoftError family through the existing tags (tags for skin/loft
   already exist). No placement argument — per the document
   design, placement rides each section profile's own plane.
4. **Nothing re-implemented**: every construction crosses
   immediately; refusals fire at the call site or at evaluate
   exactly as the Rust surface has them.
5. Existing surfaces untouched: `Node.polygon` semantics,
   `Node.profile` from PYG1, extrude/revolve/boolean.

## 3. Fence

OUT, with the reasons recorded (state them verbatim in the PR):
- **Sweep**: `wire_sweep` unconditionally refuses
  (`SWEEP_FRONTIER`, eval/wire.rs — the path-composition lane
  banked past M6 by the PR 10 MAJ ruling). Binding a
  door that always refuses flips no audit row; un-banking is
  kernel-side and not this program's call.
- **Tube**: no `Node::Tube` exists; adding a node kind is a
  SCHEMA-VERSION break (the v3 precedent was exactly Loft/Sweep
  landing) and ASM-1's in-flight unit owns the next bump (v5,
  docs/ASM-1-SPEC.md §D-6). The tube/sweep/3-D-path tail is a
  DESIGN CONVERSATION (U4's measured spec, LQ3 ratified-open),
  logged in LIB-LOG — not smuggled in here.
- G4 fillet node binding, G7/G8/G9, Expr-bearing profile steps,
  arbitrary-pose/mirror doors (U4/P6/P7), any change to loft
  evaluation or naming semantics, CI structure.
- Kernel/editor-core changes beyond: the two SketchPlane
  constructors (profile crate, additive) and — only if measured
  necessary for the loft binding — additive curation in pncad.
  Anything else missing: REPORT, never build.

## 4. Deliverables

1. §2's bindings + stubs (the .pyi grows the plane vocabulary and
   loft; ty fixtures extend: at least one legal plane/loft chain
   and one illegal line, e.g. elevation+plane together).
2. **Audit flips, honest**: silhouette (22), silhouette3 (23),
   the three silhouette shadows (24–26), loft_prism (13),
   nonuniform_loft (14) become executed YES rows in
   test_north_star.py against the scenes' exact oracles (the
   shadows share bodies with their parents — flip what is
   honestly reproducible and state row-sharing exactly as the
   Rust scenes do). `az` (27) re-partitions its primary gap
   G3→G9. Rows 15–19 (s_duct, twisted ducts, tube_along_arc)
   STAY G2 — update their "missing door" text to name the real
   blockers (SWEEP_FRONTIER bank; no Tube node; U4/LQ3). Gap
   list, counts, and the partition re-stated; absence assertions
   flip. G3 → Closed gaps; G2 stays open with its remaining
   stops recounted.
3. **nonuniform_loft's point**: its scene is the non-uniform
   section-params read-back. Measure whether the U5-curated
   read-back (loft_parameters/section_params) is reachable from
   Python cheaply; if yes, assert it in the row; if not, the row
   asserts the volume oracle and the docstring names the
   read-back residue honestly (the PYG1 m3 precedent).
4. **Guide**: the profile/plane section gains the Python mirror
   (a yz sketch + extrude, and a three-section loft — executed
   from Markdown); the Rust side gains nothing new (its blocks
   already exist).
5. Findings: every awkwardness numbered in the report (the
   demos' binding purpose), esp. anything the plane vocabulary
   makes clumsy that the tour spells naturally.

## 5. Acceptance

- Python suite green (state the delta); cargo test -p pncad-py,
  -p pncad doctests, -p profile (the two new constructors get
  Rust rows: to_world round-trips pinning orientation); cold
  clippy CI scope both lanes; hosted CI green; zero new [[test]]
  binaries; stub name-for-name + ty rows green.
- The audit page's arithmetic re-verified from the table (the
  PYG1 tally-error lesson: derive counts FROM the rows).

## 6. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-pyg23a-report.md`, per-phase
figures. Open, do NOT merge. Final message: PR number + report
path + ≤10-line summary. Forks: report, smallest faithful
reading, flag.

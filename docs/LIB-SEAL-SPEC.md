# LIB-SEAL spec — ProfileLoop seals: private fields + read accessors (ruled by Evan in-chat 2026-08-16; binding)

Mandate: execute the sealing ruling recorded in docs/LIB-LOG.md
(2026-08-16): `ProfileLoop`/`ProfileVertex` fields go PRIVATE with
read accessors, so the PATHS-channel funnel becomes the only
COMPILABLE route to a loop at every crate boundary — a downstream
struct literal stops compiling; the types stay nameable and
readable. This closes the residue RETTAIL flagged (public fields
mean a struct literal constructs a loop wherever the type is
nameable) and settles #431's open question. Read first:
`crates/profile/src/lib.rs:151-290` (the two types + `RawLoop`),
`crates/profile/src/validate.rs:878-911` (`ValidatedLoop` — the
in-repo precedent shape for private-fields-plus-accessors),
`crates/editor-core/src/persist/wire.rs:1-16` (the cannot-mint
argument this unit re-proves), docs/LIB-LOG.md's RETTAIL entry.

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT).

## 1. Deliverables, in dependency order

1. **The seal.** `ProfileLoop<T>` (`vertices`, `tangent_joints`)
   and `ProfileVertex<T>` (`pos`, `bulge`) fields go private.
   Read accessors mirror the `ValidatedLoop` precedent exactly:
   `ProfileLoop::vertices() -> &[ProfileVertex<T>]`,
   `ProfileLoop::tangent_joints() -> &[usize]`;
   `ProfileVertex::pos() -> Point2<T>`, `::bulge() -> T` (Copy —
   by value). No mutable accessors, no iterators beyond what the
   slices give for free. `reversed()` stays.
2. **The construction doors.** `RawLoop` remains the ONE raw loop
   door (`new`, `polygon`); extend it with a tangent-joints
   spelling (e.g. `with_tangent_joints(self, Vec<usize>) -> Self`
   — your choice of shape, but it must live ON `RawLoop`, so
   pncad's existing omission of the trait keeps excluding it; the
   7 test sites that today assign the field directly need it).
   Add `ProfileVertex::new(pos, bulge)`. State in its doc what
   the seal does and does not claim: vertex VALUES stay mintable
   everywhere (plain data, like `Point2`; privacy here buys
   representation freedom, not mint-prevention) — the funnel
   claim is about LOOPS, and it holds because `RawLoop` is not on
   the presented surface.
3. **The migration.** Every out-of-crate struct literal moves to
   the doors — census (2026-08-16): ~452 literal sites, ~93% test
   fixtures in `crates/sweep/tests` (325), `mesh` (39+1), `stl`
   (21), `step-export` (20), `step-import` (7); production sites
   `crates/sweep/src/loft.rs:225-234` and
   `crates/editor-core/src/eval/anchor.rs:238-244` (both
   scalar-lift rebuilds — migrate to the doors; if a dedicated
   scalar-map door reads cleaner, propose it in the report, do
   not silently add surface); `tools/k-lint/tests/litmus.rs:73`;
   `demos/tour/src/lily.rs:682-686`. Mechanical fixture rewrites
   are bit-identical by construction — the pinned suites must not
   move. **lily is special**: its literal is the #433 named-gap
   site (exactly-collinear vertices the lattice refuses and
   validate accepts). It migrates to the `RawLoop` spelling with
   the gap comment PRESERVED at the site — the raw spelling is
   the honest recording of the vocabulary gap (demo-purpose:
   awkwardness recorded, never hidden), and it must keep stating
   the invariant, not this unit's history.
4. **The cannot-mint proof (serde).** The ruling requires it
   recorded, not assumed: (a) grep-level — the profile crate is
   serde-free by policy (`crates/profile/src/path/program.rs:54`)
   and neither type derives any serde trait; pin this with a
   source-scan row in the profile suite (no `serde` attribute on
   the two types, no serde dependency in profile's Cargo.toml);
   (b) argument-level — the wire.rs §1-16 statement (the stored
   form is the PROGRAM, replayed; deserialization rebuilds
   programs only and can never mint a `ProfileLoop`) goes in the
   PR body as the unit's proof, verified against head, citing
   `crates/editor-core/src/program.rs:14`.
5. **Closure rows.** (a) A compile_fail doctest (or trybuild-free
   equivalent — the repo has no ui harness; a `compile_fail`
   fenced doctest is the existing idiom) pinning E0451: an
   out-of-crate `ProfileLoop { .. }` literal does not compile.
   (b) An accessor-completeness row exercising every accessor
   against a door-built loop. (c) The workspace compiling with
   zero out-of-crate field access IS the completeness proof for
   the read surface — say exactly that in the row's doc comment.
   (d) Extend the pncad façade absence guard
   (`crates/pncad/tests/all.rs:657`) with the struct-literal
   pattern and update its doc at :650-655 — its declared blind
   spot is exactly what this unit closes.
6. **Doc rewrites** at the four residue sites, each restated as
   the now-true invariant: `crates/profile/src/lib.rs:165-167`
   (the plain-data convention doc), `:225-229` (the RawLoop
   residue paragraph), `crates/pncad/src/profile.rs:29-37`,
   `crates/pncad/tests/all.rs:650-655`. Add the honest boundary
   sentence where the seal is documented: privacy seals at the
   CRATE boundary; `crates/profile`'s own internals stay on the
   sealed-verbs discipline.

## 2. Fence

- NO lattice/verb/state changes — the OnArc design conversation
  is live and separately ruled; nothing here touches
  `path.rs`/`path/` semantics.
- NO sealing of `Profile`, `ClosedLoop`, `ValidatedSegment` —
  their public fields are recorded adjacents (register note),
  out of this unit's fence.
- NO schema/persistence changes (census: zero impact; there is
  no wire form to preserve).
- NO pncad-py changes (zero references to either type).
- NO new public surface beyond §1 items 1-2; no new crates.
- Pinned fixtures stay bit-identical; if any pinned row moves,
  STOP and report — that is evidence, not an adjustment.

## 3. Acceptance

1. Hosted matrix green (the only gate); python-suite untouched
   and green.
2. Fields private, accessors per §1.1; E0451 compile_fail row
   red-under-revert (state the executed falsification in the
   report: make a field pub, watch the row fail to fail).
3. All census sites migrated; `git grep` shows zero out-of-crate
   `ProfileLoop {` / `ProfileVertex {` literals outside doc
   prose.
4. The serde proof recorded per §1.4; the façade guard extended
   per §1.5d.
5. lily's #433 gap comment survives verbatim-in-substance at the
   migrated site.
6. Report ≤150 lines: deviations enumerated, door-shape choices
   stated with reasoning, the fixture bit-identity claim stated
   as executed (which suites, which command).

## 4. PR discipline

One PR, branch `lib/seal`. PR body: the sanitized change story,
the cannot-mint proof (§1.4b), the honest crate-boundary
sentence, closure rows named. Note in the body that this settles
#431's open question (do not close #431 yourself — the
orchestrator handles issue disposition). Merge-main-before-open;
re-merge on movement; checks STARTED before handoff.

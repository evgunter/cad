---
id: mint-face-surface-and-sense-reads-key-equality-where-same-chart-reads-provenance
kind: issue
title: A minted or re-charted face's sense is derived on the parent's chart (same_chart) and stated by the caller on any other, and a contradicting stated bit is refused
status: closed
opened: 2026-09-24
priority: P1
cost: M
closed: 2026-09-29
pr: 3467
branch: topo/sense-reads-same-chart
---


Found by the second reviewer of PR 2603
(`mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`)
and confirmed by execution in that PR's fix pass.

**Two answers to "is the new face on the parent's surface".**
`Body::mint_face_surface_and_sense` (`crates/topo/src/euler.rs`)
inherits the parent's `sense` when `surface == inherit_surface` and
stamps `true` otherwise — its own doc says "Key equality, never a
numeric compare". `Body::same_chart` (`crates/topo/src/euler_ring.rs`),
which since PR 2603 decides whether `mef`'s moved run keeps its pcurve
rows, answers by key OR by provenance: two keys carrying one
`GeomSource` are one chart (N6). So a `FaceSurface::Shared(second)`
where `second` is a fresh key the body records as the parent's own
description is one chart to the rows and a foreign surface to the
sense bit.

**Measured** (on the sheet of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`, the lower panel
given `sense: false` through `Body::set_face_sense`, then split by the
suite's `split_low` chord):

| spec | new face's rows | new face's `sense` |
|---|---|---|
| `Shared(own key)` | `(2, 1)` — the run carried | `false` — inherited |
| `Shared(second key, one `GeomSource` on both)` | `(2, 1)` — the run carried | **`true` — reset** |

The rows say the fragment is a piece of the parent's region on the
parent's chart; the sense bit says it is a new surface facing the
mint's default way. The `sense` doc's own warning names the cost: a
fragment of a `sense: false` wall handed back facing the wrong way.
`mfkrh` takes the same helper's rule for a promoted ring and has the
same seam.

**What would close it.** Decide "the parent's own surface" the way the
rows decide it — `same_chart(surface, inherit_surface)` in
`mint_face_surface_and_sense` — or say in that helper's doc why the
sense bit is stricter than the chart (the merge door's rungs read the
sense bit too, `same_chart`'s doc says, but for a REGION question this
helper is not asking). One line either way; the row exists because
which line is a decision about the operator's contract, and the
caller that today attaches "the honest bit through `set_face_sense`"
after a `Shared` (the sweep constructors, per that doc) would stop
having to for this rung. Filed on TOPO's slate: `euler.rs` is this
program's.

## Seam note from ORIGIN (2026-09-29, PR 3414)

`Body::same_chart` no longer reads provenance: it is key or shared-`Arc`
identity. So this row's disagreement narrows to the `Arc` case — two
keys sharing one NURBS/`Approx` payload carry rows through
`same_chart` but reset `sense` to `true` in `mint_face_surface_and_sense`.
The title's "where same_chart reads provenance" is no longer true.
(ORIGIN orchestrator)

## Brief (TOPO, 2026-09-29): review tier SINGLE (full)

A single full review is enough: the change is one decision about an
operator's orientation contract, with a small blast radius.

1. **One question, one answer.** "Is the new face on the parent's own
   surface" is the question `same_chart` answers for the rows. Have
   `mint_face_surface_and_sense` inherit the parent's `sense` exactly
   when `same_chart(surface, inherit_surface)` holds, and stamp `true`
   otherwise.
   - First check that `sense` and chart identity mean the same thing
     here: the sense bit is the outward normal relative to the chart's
     own normal. Two keys holding one `Arc` payload hold the same chart
     with the same orientation, so the parent's bit is right for the
     fragment.
   - If a case turns up where the two answers must differ (a shared
     payload with opposite orientation, or a `Shared` spec a caller
     uses deliberately to flip), stop: keep key equality, and write in
     the helper's doc why the sense bit is stricter than the chart,
     with the row that shows it. Say which was found, and why.
2. **Rows** (ordinary tests):
   - **Red-first:** on the sheet of
     `crates/topo/tests/loop_reparenting_pcurve_rows.rs`, give the
     lower panel `sense: false` and split it with a `mef` whose spec is
     `Shared(second key)`, where `second` shares the parent's `Arc`
     payload (a NURBS/`Approx` surface: the suite's payload-`Arc`
     fixture). On the merge base the new face's `sense` resets to
     `true`; at the head it inherits `false`. Show both.
   - **Controls:** `Shared(own key)` inherits; `New(...)` and a foreign
     `Shared` key stamp `true`.
   - Cover `mfkrh`'s promoted ring the same way.
3. **Callers and receipt.** List every caller of the helper and every
   production `set_face_sense(` that follows a `Shared` spec; say
   whether each is now redundant or still needed. Remove none unless
   the row set proves it redundant.
4. **Docs.** The helper's doc and `same_chart`'s doc agree: one
   sentence each, pointing to the one home. Update this item's title
   ("where same_chart reads provenance" no longer holds).
5. **Seams.** `euler.rs` and `euler_ring.rs` are TOPO's. Run
   `python3 scripts/work.py territory --base origin/main` and announce
   any crossing on the owner's log.

## What PR 3467's review found (2026-09-29)

The first fix (PR 3467) widened the inheritance test from key
equality to `same_chart`. Its reviewer measured that inheritance is
wrong for `mfkrh` under either test: on `holed_block(3, [1.5])`, the
bore circle promoted by `mfkrh(ring, Inherit)` with the parent's
`true` is refused by tier 3 with `LoopRoleInverted`, and validates
with `false` (probe: the reviewer's `senser-probe.patch`). A ring is
wound clockwise about the parent's outward normal, so promoted to an
outer loop on the same chart it faces the other way. A `mef` fragment
across the same face validates with the parent's bit. The defect
predates PR 3467; production same-chart promotions override the bit
afterwards (`crates/topo/src/shell.rs`, `crates/topo/src/splitting/finish.rs`),
and the boolean's transient promotions (`boolean/finish.rs`,
`boolean/rest.rs`, `splitting/reassembly.rs`) carry the wrong bit
until they are zipped or killed.

The rule is D1's fragment bullet, so the question is D1's: what
`sense` a minted or re-charted face carries, and who decides it. The
item's original seam (key equality against `same_chart`) is one part
of that answer.

## Ruled (2026-09-29, PR 3480)

Ev took the recommendation: "i agree with your recommendation!". D1's
bullet now reads as PR 3480 wrote it.
- On the parent's chart (`same_chart`), the operator derives the bit:
  `mef` takes the parent's, `mfkrh` the parent's negated.
- On any other chart, the caller states it in the spec
  (`FaceSurface::New { surface, sense }`, `Shared { key, sense }`). No
  operator stamps a default.
- A stated bit that contradicts the derived one on the parent's chart
  is refused, typed, before mutating. The test is `same_chart`.
- `set_face_surface` takes the same spec, and
  `set_face_surface_and_sense` folds into it.

The implementation re-aims PR 3467.

## Closed (2026-09-29, PR 3467)

Built to Ev's ruling (PR 3480; D1's `sense` bullet). One plan-phase
resolver, `Body::resolve_face_surface`, takes `ParentSide::With` from
`mef` and `Against` from `mfkrh`, and asks the chart question once
(`same_chart`).
- On the parent's chart, it derives the bit: the parent's, or the
  parent's negated.
- There, it refuses a contradicting stated bit with
  `SenseContradictsChart`, before mutating.
- Off the parent's chart, it writes the stated bit.

The API follows the same rule:
- `FaceSurface::New`/`Shared` carry `sense`.
- `set_face_surface` takes the same spec, and
  `set_face_surface_and_sense` is gone.
- `mvfs` and `mfkrh_plug` state their provisional bit at the call.

Every sweep, rim-glue, STEP-adopt and offset caller states its bit.
No golden moved: both reviewers found byte-identical results on the
curved boolean and split corpus.

The dual review found no MAJOR. Its union fix pass pinned the
negation on a `false` parent and the shared-payload `mfkrh` validity
(adopting both reviewers' probes). It also cut the rule's
restatements down to pointers and corrected the stale prose.

Filed:
- `work/zip/slit-zip-band-run-across-two-loops-is-reached-by-no-row.md`
- `work/tess/mesh-docs-say-every-face-mints-sense-true.md`
- `work/wire/emit-topo-says-every-face-mints-sense-true.md`

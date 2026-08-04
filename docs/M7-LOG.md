# M7 log — orchestrator record

Concurrent-orchestrator arrangement (Evan, 2026-08-04): this log
belongs to the M7 orchestrator (session cad-implement-m7); the M6
orchestrator's live record is docs/M6-LOG.md (read, don't touch).
Protocol: memories/concurrent-orchestrators.md; briefing:
~/.local/share/cad-work/handoff-prompt-m7.md.

## Session start (2026-08-04)

Checklist done: origin/main merged; monitors installed from this
checkout and armed (away-channel with
CAD_SIGNOFF_WATCHLIST=…/signoff-watchlist-m7.txt, disk watchdog,
hourly check-in); CPU canary 1.03s (healthy); disk 96G free;
cargo-slots.txt verified — slot 2 = cad-implement-m7, free; slot 1
= cad-implement-m6 (ev/ci-test-collapse, now PR #179).

**Plan + spec PR (#180)**: docs/M7-PLAN.md assembled from #169 +
D7 + #161 §2 (nothing newly proposed — self-merges per the
standing rule) plus the binding docs/M7-1-SPEC.md and this log.
Feasibility verified before speccing: topo's Euler-operator
vocabulary and geometry-attachment doors are public (step-export's
own test corpus builds bodies through them from outside topo), so
`crates/step-import` needs no kernel edits; hosted CI's
`--workspace`/closure scoping picks up a new member with zero CI
edits; the fixtures' declared uncertainty is 1e-9 and units are
metres (`.expect` volumes are mm³ — ×1e-9).

**A/B, block M7-1**: difficulty for M7-1 (import crate skeleton +
own-corpus round-trip) logged **L** BEFORE the draw — new crate
with a Part-21 parser, an Euler-op assembly algorithm over the
full entity subset (incl. composed_die's 89 faces, 42 reversed),
D7 adoption, and a six-row acceptance suite. Draw (after the
difficulty commit): byte 177 → **(fable, opus)** — M7-1 = FABLE;
opus remainder owed to the next A/B-eligible dispatch (expected:
M7-2, the FreeCAD-authored foreign corpus). Reviewer blinded as
always; row recorded AT MERGE.

**M7-1 DISPATCHED (2026-08-04)**: #180 (plan + binding spec +
this log) self-merged on green; implementer launched on lane
~/.local/share/cad-work/m7-import, branch ev/m7-import (cargo
slot 2 claimed in cargo-slots.txt); PR to be HELD for blinded
adversarial review per standing process. Fence in both the spec
(§0) and the prompt; report lands at
~/.local/share/cad-work/m7-1-report.md.

**Evan's five notes on #180 (comment, 2026-08-04), dispositions:**
(1) parser hand-rolling is necessity not preference — the F6
spike (references/notes/step-spike-report.md) found no usable
Rust STEP semantic layer; spec §Leg A corrected, and ruststep's
working *syntactic* layer + truck-stepio's `in::Table` noted as
precedented dev-dependency oracles. (2) Mäntylä notes
(references/notes/mantyla-ch9..15) are to be read BEFORE the
scans — relayed to the implementer with the main-checkout path
(references/ is git-ignored and absent from lane clones).
(3) reversed faces: the corpus's `.F.` faces are deliberate
S10/S11 output, not a bug — no healing now; reaffirmed to the
implementer. (4) adoption machinery should be reusable for GUI
remedies — refusals carry structured data, recorded in the plan's
contract section and relayed. (5) wild licensed STEP files inside
the subset as a late demo corpus — plan unit 4, deferrable. All
three docs amended; implementer messaged mid-flight (no
acceptance-row changes).

**M7-1 implementation COMPLETE (2026-08-04): PR #183 open, all
six acceptance rows reported green (8/8 nextest), ONE numbered
deviation, blinded adversarial review DISPATCHED.** Headline
measurement: the first re-export is byte-identical to the
COMMITTED fixture for **all 14/14** solid fixtures (row 2 only
required the second export to fix-point the first). Deviation 1:
five sidecars' EXPECT_EDGES record OCC's post-import
normalisation (pole edges, seam splits), not the kernel census —
resolved by asserting the kernel census quoted from the sidecars'
own comments; no fixture/sidecar touched; sidecar KERNEL_* fields
suggested as a design conversation. Architecture: hand-rolled
Part-21 parser (~350 lines, zero new deps; step-export enters as
dev-dependency oracle only); rotation-system Euler assembly
(σ(u)=next(mate(u)) fan orbits, mev/mef/mekr + ring_move/kfmrh
genus, strut+kemr hole-planting, strut+kev anchor rotation) with
a loop-cycle verification pass; file-order fixed-point discipline
(Shell::faces + Cycle::first); D7 adoption ladder
Intersection→TangentIntersection / Seam→MappedCurve with
structured (candidate, refusal) errors per Evan's remedy
directive; then mint_pcurves. Notable discovered facts: the
tangent gate accepts the full circular-trimline class (its
refusal text still names the M5 line-only class — stale, banked);
kev's unconditional Cycle::first re-anchor is the public
loop-rotation door (deserves a topo pin someday); STEP cannot
carry solid grouping (kiss_assembly imports 2 solids / 2 shells,
matching its sidecar). Impl ~441k tokens, ~1.9h wall. Review
assigned attacks: byte-identity provenance (anti-laundering),
deviation-1 adjudication, adoption-ladder corruption probes,
rotation-system stress (genus, permuted-order files), volume
tolerance teeth, same_sense flip fidelity, refusal coverage,
ε_in. Slot 2 → m7-import-review lane.

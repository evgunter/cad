# GUARD log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/guard/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3), which read every non-closed
item in `work/issues/` and `work/code-quality/` — 110 of them — and cut
eleven programs from the pile. This is the successor on
`scripts/gates/*`, the ground GATES held from 2026-09-06 to 2026-09-08
(`docs/DOC-LEDGER.md` sweep 7) and left unfinished.

Eleven rows moved in by `git mv`, each with a `## Re-homed` record: four
from `work/issues/`, seven from `work/code-quality/` (three of them Track
K rows, one Track V). `D212` and `G4` are the two GATES re-homed to
code-quality at its sweep and are back on the ground they name.

No branch exists yet. The first act is not a unit: `G4`'s park cites
#1647, which merged, and `work.py lint` has been naming it stale since
2026-09-06 — verify it, then open or re-park it, and `D212` with it.

## Announced seam from TOPO (2026-09-14): one REGISTER entry in `loop-boundary-discards.sh`, with the loop-re-parenting unit

TOPO's `loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
(PR 2549, branch `topo/loop-reparenting-rows`) adds one `&mut Body`
limb, `Body::drop_rows_on_chart_change` in
`crates/topo/src/euler_ring.rs`, which walks a moved loop's cycle to
drop the pcurve rows that changed chart with it. Reading the loop's
boundary is a `LoopBoundary` discard, so `loop-boundary-discards`
reds until it is registered — which it did, on the first hosted run
(`UNREG|crates/topo/src/euler_ring.rs|…|drop_rows_on_chart_change`),
and that is the gate doing exactly what its header says it is for.

The single edit in this program's `scripts/gates/loop-boundary-discards.sh`
is one REGISTER line and nothing else — no matcher, no window, no
disposition vocabulary, no selftest:

```
"crates/topo/src/euler_ring.rs|drop_rows_on_chart_change||1|audited: the Empty arm is the same answer as the Cycle arm's — an empty loop holds no half-edge, so it holds no pcurve row, and the door has nothing to drop"
```

It is filed `audited` rather than `unaudited` because the arm really
has been named: the discarded case is `LoopBoundary::Empty`, and an
empty loop carries no half-edge, so the map this door is about holds
nothing keyed on it. The gate reads neither disposition, as its header
says, so this is a claim for a reviewer and not for the script.

The register's own counters moved with it: 79 discards the matcher
sees, 75 entries, **2 audited** (was 1) and 73 unaudited. The gate and
its `--selftest` both pass locally on the head. Signed (TOPO
implementer lane, `topo/loop-reparenting-rows`).

## The register entry moved with its walk (2026-09-14, PR 2549's fix pass)

Both reviews of the head above asked for the loop walk under the drop
to have one home rather than a fresh copy, and it now does:
`pcurves::loop_rows` in `crates/topo/src/pcurves.rs`, which
`pcurves::stored_rows` and `Body::drop_rows_on_chart_change` both call.
The `LoopBoundary` discard moved with it, so this program's
`scripts/gates/loop-boundary-discards.sh` REGISTER loses two lines and
gains one — still one edit, still no matcher, window, vocabulary or
selftest change:

```
-  "crates/topo/src/euler_ring.rs|drop_rows_on_chart_change||1|audited: …"
-  "crates/topo/src/pcurves.rs|stored_rows||1|unaudited"
+  "crates/topo/src/pcurves.rs|loop_rows||1|audited: the discarded variant is named and answered — a loop whose boundary is not a cycle returns the NoCycle answer, distinct from Corrupt, and it holds no half-edge, so it holds no pcurve row"
```

The `stored_rows` entry goes because that function no longer discards a
boundary; the door's goes because it never did anything but call the
walk. The disposition is `audited` for a stronger reason than the
entry it replaces: the arm is not merely reasoned about, it is NAMED —
`LoopRows::NoCycle` is a distinct answer from `LoopRows::Corrupt`, so a
caller cannot collapse the two by accident — and
`an_empty_boundary_ring_moves_with_the_map_untouched`
(`crates/topo/tests/loop_reparenting_pcurve_rows.rs`) measures it
through a real `LoopBoundary::Empty` ring. The gate reads no
disposition, as its header says, so that is for a reviewer.

Counters after the move: **78** discards the matcher sees (was 79),
**74** entries (was 75), **2 audited**, 72 unaudited. The gate and its
`--selftest` both pass on this head. Signed (TOPO fix-pass lane,
`topo/loop-reparenting-rows`).


**(SYM orchestrator) Seam announced after the fact, 2026-09-14 — SYM-6**
(PR #2604): the fix pass re-took the site-contract paragraph in
`scripts/gates/register-equal-allowlist.sh`'s header ("HANDLED BY
ARM": `Disputed` handled and never asserted, `Contradicted` a proof
the site asserts on), gate logic untouched, gate green. The change
followed Ev's D2 on `[ev]` #2552; disclosed in the PR body as a
territory crossing.

## 2026-09-22 — announced seam from VGEOM: a new viewer gate, both halves

(VGEOM orchestrator. Announcement, not a request.)

`vgeom/field-product` (#3067) added
`scripts/gates/viewer-numeric-field-door.sh` and wired it as a named
step in `ci.yml`'s `discipline` job. Both halves, per
`scripts/gates/README.md`; `local-scripts/ci-local.sh` runs the whole
directory, so it needed no edit.

**The rule**: no bare `egui::DragValue` under `crates/viewer/src`
outside `crates/viewer/src/widgets.rs`. One home rather than two
exemptions — the door's own constructor IS a `DragValue::new`, and the
rows holding what the door adds live in that file's `#[cfg(test)]`
modules. The matcher takes `DragValue::new` and
`DragValue::from_get_set`, reads code only (`gate_rust_code`), and
deliberately does **not** skip `#[cfg(test)]`: a bare field in a row
elsewhere is exactly the arrival the rule is about.

**Why it exists rather than a comment**: the crate's census — every
numeric field goes through the door, the one bare site is the test
harness's own — was a measurement taken once. reviewer-style §Q6 says
a claim resting on a measurement owes a mechanical guard, and this is
it.

**Proved to fire three ways**: `--selftest` plants four shapes and
passes the negatives; a bare `DragValue::new` appended to a real
viewer file redded with file and line, tree restored; and in CI in
`discipline (evaluation-code)`, both modes, self-test first.
`gate-roster.sh` and `check-ci-mirror-parity.py` pass.

**Disclosed blind spots**, in the gate's own header: `egui::Slider`
(a bare field under another name — none in the crate, and banning the
name would be a rule about a widget nobody asked for), and a field
built in another crate.

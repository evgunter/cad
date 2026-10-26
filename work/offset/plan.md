# OFFSET — the plan

the offset lane: the carriers it cannot hold and the folds it calls bounds

Opened 2026-09-20 by SHELL's priority-seam cut (`work/README.md`,
Track size). Picked up 2026-10-24.

## Where it sits against INTENT

INTENT builds D10 in the editor layer (`crates/editor-core`, the
façade, the viewer), and the refactor hold it sets is on units that
touch dependency, placement or intent. Nothing on this slate does: it
is `topo`'s offset doors and rigid map and `geom-brep`'s offset
carriers and meters. `topo::transform_rigid` is the map a D10
placement calls, and INTENT's stage 3 D kept it; nothing in D10's
retirement list (`docs/DESIGN.md`, State of the build) names it, the
offset doors, or the `MappedSource` declarations the axial door
re-authors. So no row here is made irrelevant by INTENT.

The one row that WILL be read differently later is the P0: a new
carrier kind is a new case for stage 4's canonical carrier forms and
the symbolic tier (COINCIDE), so its designers are told to weigh that.
The two P2 `classify` rows are the overlap with HONE's
`coaxiality-of-two-revolution-carriers-is-tested-five-ways` (one pose
reading for coaxiality), not with INTENT; whoever takes them reads
that row first.

## This sitting (P0 and P1)

| pri | item | cost | how |
|---|---|---|---|
| P0 | `offset-lane-has-no-conic-carrier` | H, design | designer pair, then an `[ev]` PR: it moves C5 R1's ratified "outside the conic inventory by decision" |
| P1 | `doors-still-read-the-whole-body-for-tier1` | H, design | designer pair first (a tier-1 restricted to a scope without a second validator; SHELF's `offset-doors-take-solids-and-a-per-chart-rule` reshapes the same doors) |
| P1 | `topo-transform-rewritten-is-a-hashset-the-crate-doc-argues-against` | E | implementer, with the P4 `transform-corrupt-renders-an-orphan-entry-as-a-missing-reference` riding along (the same set's refusal) |

`offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` closed
at pick-up as overtaken (its row says by what).

## The conic build (Ev's ruling on PR 5652)

| unit | pri | cost | depends on |
|---|---|---|---|
| `closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them` (limb 2 for every carrier) | P0 | H | — |
| `the-plane-cone-arc-is-minted-exactly-and-the-axial-door-reads-the-moved-section` | P0 | H | unit 1 |
| `a-classified-pairs-section-states-the-bounds-that-retire-the-tube` | P0 | M | unit 2; lands last and alone (Ev's reservation) |

`offset-lane-has-no-conic-carrier` closes when they land. Review: unit 1
and unit 2 are broad (every closed-form edge's certificate; a new
section path every caller reads), so each draws a dual.

## The tail (P2–P4)

Left for a successor unless this sitting's context makes a row cheap.
`work.py status --program offset` is the list.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The `rewritten`
set is a mechanical change: orchestrator's read. The P0's build, once
ruled, is a new carrier kind every reader takes, so it draws a dual.

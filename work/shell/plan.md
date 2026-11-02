# SHELL — shell, offset and transform (plan)

Live state is `work/shell/log.md`'s tail and the item files beside
this plan, never this file. Branch prefix **`shell/`** (unit branches
`shell/<unit>-<slug>`); away-channel tag `(SHELL orchestrator)`.
Sibling tracks sharing this ground by design: CLEAR (the clearance
engine), OFFSET (the offset lane's carriers), SHELF (the follow-on).

## Charter

Finish the shell and offset verbs to the semantics Ev ruled, on the
everyday shapes a user authors: every operand the kernel can build
shells or refuses typed, and every refusal says what to do next.

## Territory

`crates/topo/src/{shell,replace_face,transform,offset_together,offset_axial}.rs`,
`crates/geom-brep/src/{offset,offset_meters}.rs`,
`crates/sweep/tests/verbs_shell*.rs`, `crates/editor-core/src/clearance.rs`
— shared with the sibling tracks above. Not this program's:
`offset_fit.rs`, `crates/verbs` (its shell `VerbRecord` arm must agree
with `ShellNaming`), `editor-core`'s recipe doors.

## Unit order

The landed units are in `log.md`: SHELL-1, 2 and 5–10 earlier; the
2026-10-06 cut's seven units (PRs 4111, 4112, 4115, 4117, 4163, 4151,
4191) on 2026-10-06/07. SHELL-3 and SHELL-4 are CLEAR's. The slate
after that cut, as units:

8. **Tilted planar walls** — `shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel`
   (P1, M): the planar gate reads only antiparallel pairs, so two
   walls meeting at an angle across less than `2t` shell silently.
   A silent wrong body; first in line.
9. **The lofted oblique corner** — `shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam`
   (P1, H): decided 2026-10-08 (the item's `## Decided`): the
   per-chart door derives edges by section and corners by crossing,
   carrying ISO's `nurbs-iso-derive-line-rim-arm-refuses-an-interior-row`
   first. Lofts then stop at the first wall:
   `a-fitted-wall-has-no-section-with-a-moved-cap` is decided
   2026-10-08 (a fitted face's section is its fit's; P2, after unit 9),
   and behind it the wall–wall seams
   (`a-wall-seam-between-two-fits-has-no-section`, P2;
   `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`, P3).
   The one-door merge `offset-doors-are-one-door-with-a-held-distance`
   (P3, H) follows.
10. **The face door is at rest** — `replace-face-offset-answers-for-the-complement-of-an-inside-out-body`
    (P2, M) carrying `shell-operand-shape-arms-behind-the-at-rest-gate`
    (P3, E): decided 2026-10-08 (the item's `## Decided`): the
    doors stay construction steps, the premise is corrected and
    pinned; the arms the `shell` gate made unreachable go.
11. **The sealed arm's two refusals** — `shell-of-a-cone-tip-refuses-at-the-nappe-decision`
    and `shell-of-a-tangent-dome-refuses-at-the-axial-corner` (P2,
    M each). Unit 7's cone tip, tangent dome and the lift's cone arm
    all wait here.
12. **A band between two boundaries** — `shell-open-band-wrapping-between-two-boundaries`
    (P2, M), unit 7's residue.

The P3 re-anchor rows (`reanchor-does-not-extend-…`,
`reanchor-reads-a-closed-spline-carrier-…`,
`replace-faces-offset-drops-rows-…`,
`nurbs-lane-absence-has-three-spellings-…`) and the per-chart door's
`per-chart-door-transports-a-torus-rim-…` follow; the two unpriced
rows are priced before they are dispatched.

then, as units:

13. **A fitted wall's section is its fit's** — `a-fitted-wall-has-no-section-with-a-moved-cap`
    (P2, H): decided 2026-10-08 (the item's `## Decided`). C5 routes
    `(Plane, Approx)` over `approx.fit()`, the edge stores the `Approx`
    key, and nothing is composed into its bound. Lofts then stop at the
    wall–wall seams. Dual review.
14. **The per-chart door's inverted body** — `the-per-chart-door-adopts-an-inverted-body`
    (P2, M): unit 10's `## Decided` already rules that the doors are
    construction steps, finished only through `AtRestBody::validate`.
    What is owed is the check that every caller adopting the door's
    result passes that gate, and pins showing the frustum and tube
    moves refuse there. Single review.

After 13, re-measure and price `a-wall-seam-between-two-fits-has-no-section`
(P2, H) before dispatching it.

then, as units:

15. **A moved fit's corners** — `a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section`
    (P2, H): seed the plane's root on a derived spline section, or root
    the fit along a held edge, so a fitted face bounded by planes moves.
    Lands after 13 (both in `replace_face.rs`). Dual review.
16. **The tilted read's three gaps** — `tilted-read-accepts-a-zero-touch-on-any-vertex-sharing-pair`
    (E), `tilted-read-skips-edge-adjacent-pairs-that-cross-away-from-their-edge`
    (M) and `tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball`
    (M), all P3 and all in `moved_walls_cross` (`shell.rs`). The skipped
    edge-adjacent pair is a possible silent crossing, so the unit is
    M-tier: rule-1 draw byte 178 (mod 3 = 1), sequential.

The wall seam re-measured (its `## Measured`, 2026-10-09): the loft's
seams refuse at the iso-row guard and at `Approx × Nurbs`, behind a fit
budget the default ε misses. It needs a designer pair before it is
priced, after 15.

The wall seam went to a designer pair (fork-log row 106). Ev ruled on PR 4515: one complete NURBS × NURBS arm, a `Section` handle with `branch_at` and `all()`. The designed final state is in the item's `## Designed` and `## Decided`. It cuts into:

17. **Narrow the iso-row arm** — `the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row` (P2), plus the iso-row half of the wall-seam `## Designed`. The arm fires only for a self-shared image or a neighbour that holds the move, and reads the image's own row. M tier: rule-1 byte 37 (mod 3 = 1), sequential.
18. **The general simultaneous door** — `shell-moves-every-chart-of-a-solid-through-one-simultaneous-door` (P2, H). Every chart of a solid moves at once; edges are sections of moved surfaces, and corners are their roots. Dual review. It runs beside 17 (both touch `replace_face.rs`; each merges main often).
19. **The NURBS × NURBS arm** — `a-wall-seam-between-two-fits-has-no-section` (P2, H), as ruled. After 18.
20. **The saddle fit's reach** — `a-saddle-walls-offset-fit-stalls-short-of-the-default-eps` (P3, measure first).
21. **The plane × NURBS door takes `branch_at`** — `the-plane-nurbs-door-selects-its-branch-by-nearest-sample` (P2, M), the ruling's second half (PR 4515: "in the arm's unit or the next"). A `Section`-shaped handle over the plane × NURBS lane; `plane_nurbs_ssi` becomes `all()`, and `plane_wall_section` calls `branch_at(mid)`. M tier: rule-1 byte 32 (mod 3 = 2), sequential. It runs beside 20 (different files).

Unit 19 merged (PR 4597, 2026-10-11). Its follow-ups are filed: the boundary pass's Side/Corner regions and its cost (SSIEDGE), `all()`'s small-loop budget and the ℝ⁴ walk's duplication (SSIEDGE), the M8-4 mint with a non-plane pair and the loft top rim's certificate (ISO).

Curved wall clearance (geom-brep README, Open) is the gate before a shelled loft is sound at rest. It is priced after 19.

Units 8 and 10 run in parallel (different files); unit 9's measure
runs beside them.

## Adjacent, not taken

- `work/seat/shell-doors-take-tolerance-beside-tol` — SEAT's.
- WALKS' `producer-closing-mint-is-a-convention-with-thirteen-copies`
  — the cross-producer half of the laundering class.

## Exit shape

The verbs' README states the ruled semantics and every item above has
landed; the walk convention applies.

## SHELF's slate, merged in 2026-11-02

SHELF merged into SHELL on 2026-11-02 (`docs/doc-ledger/shelf-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/shell/`.

Opened 2026-10-06 by SHELL's priority-seam cut (`work/README.md`,
Track size). SHELL keeps the rows on the goal — authoring arbitrary
geometry through the UI — and this track takes the architecture and
hygiene behind them. Nothing dispatched.

### Order

1. `offset-doors-take-solids-and-a-per-chart-rule` — ruled (Ev, PR
   3310), so pure execution. Its step 1 (one grouping home) has
   landed as `topo::chart_groups::ChartGroups`; re-base the remaining
   steps on it. Coordinate with OFFSET's
   `doors-still-read-the-whole-body-for-tier1`, which touches the same
   doors. The whole-body mint in `replace_faces_offset` (item 3 of the
   closed `shell-offset-three-followups`) rides it.
2. `transform.rs` as one sitting: `check-rigid-squares-a-column-…`
   (E) and `a-rigid-map-admitted-on-the-metre-band-…` (design: lever
   the admission by extent, or re-orthonormalize at mint) share
   `check_rigid`; `transform-rigid-recertifies-images-enclosed-apart`
   is a designer pair first (three viable candidates), weighed with
   TOPO's parked `a-boxed-rotation-refuses-not-rigid-at-every-placer`.
3. `replace_face.rs` rows: `replace-face-transports-a-nurbs-edge-as-a-ruling`
   (pairs with SHELL's lofted-wall-seam row if that lane has room),
   `cap-rim-refusal-order-follows-the-arena`,
   `replace-face-reads-an-approx-iso-as-u-fixed`.
4. The E hygiene rows batch: `r2-drum-reverted-…`,
   `shell-rekey-and-scope-walks-…`, `shelled-result-does-not-name-…`.

`partial-cone-frustum-three-quarter-turn-…` stays parked on OFFSET's
conic-carrier fork.
`shell-corpus-documents-held-out-of-the-registry` asks the registry's
question (TCOST's ground; the Dual64 row's per-document exemption or
a dual shell door) — announce the seam when it runs.

## OFFSET's slate, merged in 2026-11-02

OFFSET merged into SHELL on 2026-11-02 (`docs/doc-ledger/offset-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/shell/`.

the offset lane: the carriers it cannot hold and the folds it calls bounds

Opened 2026-09-20 by SHELL's priority-seam cut (`work/README.md`,
Track size). Picked up 2026-10-24.

### Where it sits against INTENT

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

### This sitting (P0 and P1)

| pri | item | cost | how |
|---|---|---|---|
| P0 | `offset-lane-has-no-conic-carrier` | H, design | designer pair, then an `[ev]` PR: it moves C5 R1's ratified "outside the conic inventory by decision" |
| P1 | `doors-still-read-the-whole-body-for-tier1` | H, design | designer pair first (a tier-1 restricted to a scope without a second validator; SHELF's `offset-doors-take-solids-and-a-per-chart-rule` reshapes the same doors) |
| P1 | `topo-transform-rewritten-is-a-hashset-the-crate-doc-argues-against` | E | implementer, with the P4 `transform-corrupt-renders-an-orphan-entry-as-a-missing-reference` riding along (the same set's refusal) |

`offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` closed
at pick-up as overtaken (its row says by what).

### The conic build (Ev's ruling on PR 5652)

| unit | pri | cost | depends on |
|---|---|---|---|
| `closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them` (limb 2 for every carrier) | P0 | H | — |
| `the-plane-cone-arc-is-minted-exactly-and-the-axial-door-reads-the-moved-section` | P0 | H | unit 1 |
| `a-classified-pairs-section-states-the-bounds-that-retire-the-tube` | P0 | M | unit 2; lands last and alone (Ev's reservation) |

`offset-lane-has-no-conic-carrier` closes when they land. Review: unit 1
and unit 2 are broad (every closed-form edge's certificate; a new
section path every caller reads), so each draws a dual.

### The tail (P2–P4)

Left for a successor unless this sitting's context makes a row cheap.
`work.py status --program offset` is the list.

### Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The `rewritten`
set is a mechanical change: orchestrator's read. The P0's build, once
ruled, is a new carrier kind every reader takes, so it draws a dual.

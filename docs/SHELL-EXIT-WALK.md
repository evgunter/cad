# SHELL exit walk

**Program:** SHELL, the shell, offset and transform verbs (`work/shell/`).
It opened 2026-09-03, ahead of VERBS' exit, on Ev's direction. The
2026-10-06 priority-seam cut moved the rows behind its goal slate to
SHELF, and the curved clearance pair (SHELL-3, SHELL-4) to CLEAR.
**Charter and criteria:** `work/shell/plan.md`. **Narrative:**
`work/shell/log.md`.

The criteria are the plan's `## Exit shape`, quoted verbatim and taken
one clause at a time:

> The verbs' README states the ruled semantics and every item above has
> landed; the walk convention applies.

## The units

| unit | item | PR | review |
| --- | --- | --- | --- |
| SHELL-1, 2, 5–10 | the `ShellNaming` birth channel, the `Approx` transform door, the shell/offset follow-ups | before the cut (`log.md`) | per their rows |
| 1 | `shell-of-a-pole-touching-sphere-refuses-mapped-source` | 4111 | single |
| 2 | the doors take an `AtRestBody`: `shell-answers-for-the-complement-of-an-inside-out-operand`, `shell-launders-a-stale-operand-row` | 4112 | single |
| 3 | `shell-clearance-footprint-reads-vertices-not-arcs`, `shell-walls-antiparallel-decides-a-cosine-on-the-metre-band` | 4115 | single |
| 4 | `shell-refuses-every-lofted-body-at-a-wall-seam-carrier` | 4117 | single |
| 5 | `shell-refusals-short-of-the-shape-guard`, `replace-face-refusals-open-with-a-stage-prefix-and-name-keys`, `replace-face-zero-span-reanchor-reads-as-a-kernel-defect`, `offset-doors-small-doc-and-message-drift` | 4163 | single |
| 6 | `shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow`, `per-chart-door-transports-a-torus-rim-the-section-would-mint` | 4151 | DR-91 |
| 7 | `shell-open-refuses-a-curved-designated-face` (designer pair, no `[ev]`) | 4191 | single |
| 8 | `shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel` | 4311 | single |
| 9 | `shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam` | 4351 | DR-115 |
| 10 | `replace-face-offset-answers-for-the-complement-of-an-inside-out-body` (with `shell-operand-shape-arms-behind-the-at-rest-gate`) | 4315 | single |
| 11 | `shell-of-a-cone-tip-refuses-at-the-nappe-decision`, `shell-of-a-tangent-dome-refuses-at-the-axial-corner` | 4356 | single |
| 12 | `shell-open-band-wrapping-between-two-boundaries` | 4391 | single |
| 13 | `a-fitted-wall-has-no-section-with-a-moved-cap` | 4404 | DR-126 |
| 14 | `the-per-chart-door-adopts-an-inverted-body` | 4403 | single |
| 15 | `a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section` | 4472 | DR-138 |
| 16 | the tilted read's three gaps (`tilted-read-…` ×3) | 4467 | DR-133 |
| 17 | `the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row` | 4525 | DR-145 |
| 18 | `shell-moves-every-chart-of-a-solid-through-one-simultaneous-door` | 4524 | DR-148 |
| 19 | `a-wall-seam-between-two-fits-has-no-section` (Ev's ruling, PR 4515: the NURBS × NURBS `Section`) | 4597 | concurrent dual, row `PR-4597` |
| 20 | `a-saddle-walls-offset-fit-stalls-short-of-the-default-eps` | 4672 | sequential, row `PR-4672` |
| 21 | `the-plane-nurbs-door-selects-its-branch-by-nearest-sample` (PR 4515's second half) | 4677 | sequential, row `PR-4677` |

## The criteria

> The verbs' README states the ruled semantics

**Met once this PR's README edits land; four were missing or stale and
are fixed here.** "The verbs' README" is the shell and offset design
page: `crates/geom-brep/README.md`'s OFFSET-DESIGN clauses (O1–O6),
which `crates/topo/README.md`'s "Shell and offset surgery" row points
to. Each of Ev's shell/offset rulings was checked against that text and
against the code:

| ruling | stated | in the code |
| --- | --- | --- |
| a hollow body thickens every boundary: the outer shell inward, each void outward, `k + 1` solids | **was only implied**, and O4 still said "a two-shell solid": O4 now states it | `shell.rs` (step 3, `move_shells_to_new_solid`) |
| a chart is body-wide (PR 3412): each solid's faces grouped by chart | **was not named**: O4 now says it | `ChartGroups::within` per solid |
| the doors are construction steps, finished only through `AtRestBody::validate` (unit 10's `## Decided`) | yes, topo README's surgery row | `shell`/`shell_open` take `AtRestBody` |
| the cone nappe's home (`mint-offset-ignores-cone-mirror-nappe`) | yes, O1 (`offset_nappe::face_nappe`) | yes |
| PR 4515: NURBS × NURBS is one complete `Section` (`branch_at`, `all()`); the plane × NURBS door takes `branch_at`, and a region holding the seed is the answer | yes, O4 and the section lane (C3/C5) | `ssi/pair.rs`, `ssi/plane.rs` |
| #1737, ruled B: the clearance engine's body-level half moves into `topo` behind the interval scalar for the curved gate | **was stale**: the Open item routed it to ERROR-DESIGN; it now states the ruling and CLEAR's two rows | not built; the gate is planar only, which the Open item and the shell module docs say |

O4's refusal list also lacked the two planar clearance refusals
(`WallClearance`, `OffsetsCross`); it now names them and points at the
curved Open item.

Three other rulings in the log are not shell semantics: Ev on PR 3352
(certification recourse), on `[ev]` PR 3402 (`Decided { sign, margin }`)
and D1 (PR 3480, the sense stated at the mint). They are stated in
`docs/DESIGN.md`. Checking D1 turned up one divergence in shell's own
code: the rim glue still mints its face with `FaceSurface::Inherit` and
sets the sense afterwards. It is filed on SHELF as
`the-rim-glue-sets-its-face-sense-after-minting-it` (P3).

> and every item above has landed;

**Met for every numbered unit; not met for six P3 rows the plan named
as follow-ons. They move to SHELF in this PR, ids kept.** Every item
the plan dispatched as a unit (1–21) is closed by the PR above. The
plan also named, without numbering, rows that "follow" the units, and
six of those are still open:
- the re-anchor pair `reanchor-does-not-extend-a-spline-carrier-the-wall-would-carry`
  and `reanchor-reads-a-closed-spline-carrier-from-the-seed-side-only`;
- `replace-faces-offset-drops-rows-at-a-vertex-the-move-leaves-in-place` (unpriced);
- `nurbs-lane-absence-has-three-spellings-across-the-doors`;
- the smooth seam between two fits, `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates` (H);
- the one-door merge, `offset-doors-are-one-door-with-a-held-distance` (H).

`per-chart-door-transports-a-torus-rim-the-section-would-mint`, the
seventh such row, is closed. WALKS'
`producer-closing-mint-is-a-convention-with-thirteen-copies` was listed
as adjacent and not taken; it stays WALKS'.

All six are P3. None is a silent wrong body: each is a typed refusal
where an answer exists, a missing pin, or a door shape. Their owner
going forward is SHELF, whose charter is "the rows behind SHELL's goal
slate" on these same files.

> the walk convention applies.

This walk is it. The rest of the slate moves too (below), so nothing
open is left in `work/shell/` when the directory is swept.

## The residue

Every live row moves to SHELF in this PR (`git mv`, ids kept; SHELF's
log records the hand-over): the six above and nine more filed during
the units, plus the rim-glue row this walk filed. All are P3.
- `a-move-past-a-fitted-solids-reach-refuses-op-not-the-collapse`
- `axial-corner-nearly-tangent-refusal-has-no-row`
- `axial-corner-solves-no-circle-circle-pair`
- `clearance-footprint-reads-an-arc-as-its-whole-carrier-ball`
- `opening-a-tangent-dome-ties-at-the-lift`
- `per-chart-door-refuses-a-cone-window-reaching-its-apex`
- `shell-refuses-a-finished-body-wearing-one-chart-both-ways`
- `offset-derive-net-folds-drop-a-poisoned-control-point` (E)
- `shell-glue-precondition-skips-the-hosts-existing-rings` (E)

`main-reds-on-the-impl-census-since-plane-nurbs` (P0, filed on this
slate by another program at 14:23Z) is closed in this PR. Main fixed it
at `be03d2357d`. The census missed `ssi/plane.rs` because unit 21's own
CI did not select test-utils.

Rows other programs own and this program's units filed stay where they
were filed: SSIEDGE (the NURBS × NURBS boundary pass's regions and
cost, `all()`'s small-loop budget and call-order dependence, the ℝ⁴
walk's duplication of r3), ISO (the loft's top-rim certificate, the
M8-4 mint with a non-plane pair), ENCL (the two stop rules of the
certify-and-refine loops), CLEAR (SHELL-3, SHELL-4: the curved
wall-clearance window, `shell-curved-wall-clearance-window`, parked on
the consumer's layering question).

**What a shelled lofted body does today**, for the record: the twisted
loft's walls fit at the default ε and its four wall–wall seams derive,
and the shell stops at the top rim's at-rest plane × fit certificate
(ISO's row). A thin curved neck still shells silently wrong until
CLEAR's curved gate lands; the shell module docs and the geom-brep
README's Open list say so.

## After ratification

The sweep deletes `work/shell/` and strips the `refs:` entries that
name its closed rows, then records a
`docs/doc-ledger/shell-leaves-the-tracker.md` note with the
recoverable SHA.

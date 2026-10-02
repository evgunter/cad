# SHOW — log

## 2026-10-02 — opened

Ev asked in chat for a check of the demos against recent work, then
for this program, with this session as its orchestrator. The audit
(three read-only lanes plus a full tour run on `6cacd42dd`, exit 0)
found four fudges whose gaps are closed (klein's two-elbow loop, the
heat sink's 1/16 embedment, the teapot lid's vent, the letterforms'
1/16 decoupling), two unblocked plans (lanceolate blades; gallery
documents), retired walls (klein 5 and 7, lily 13), and seven
undemoed landings. Ev's answers in chat: (a) and (b) all in, the
partly-unblocked items the orchestrator's call; (c) all in, folded
into existing cells where possible, the snowman its own cell.

Orchestrator calls, logged:
- **Partly-unblocked items deferred** to their owners' existing rows
  (plan, "Not in scope").
- **The twisted loft goes into `lofts`, not over `twisted_duct`**:
  `twisted_duct` already lost its montage cell to `twisted_tube`, and
  `twisted_tube`'s roll is a placement roll, a different fact from a
  correspondence twist. `lofts` is the cell about what a loft reads
  from its author.
- **The bored section goes into `projectbox`** (its bosses become
  bored), **the engraving into `tiltedcut`** (the elliptical face
  first), **joined-rim fillets into the heat sink**, **gauges into
  `bench`**, **keyhole creases into `rocker`**; the helix and the
  split-by-name are fold candidates taken last.
- **Claimed into SHOW**: `klein-scene-should-adopt-the-one-body-loop-sweep`
  (from issues/), its rider from lib/, and both heat-sink rows from
  issues/. `heatsink-union-lives-in-the-demo` closed on arrival (the
  union has been in-document since #1344's first half);
  `heatsink-placedunion-base-union-unfinished` re-scoped to its
  remaining follow-up, the flush declared fins.
- **Filed on CLEAVE**:
  `boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` —
  three demo joins pin `CurvedEdgeUnsupported` and no row scheduled
  lifting it.
- **Stale demo prose fixed in the opening PR** (Ev: "fix the out of
  date stuff"): the citations the audit found rotted in `impeller.rs`,
  `lily.rs`, `teapot.rs` and `demos/README.md`.

## 2026-10-02 — wave 1 dispatched

Three Opus implementer lanes, each in its own worktree, from the
opening branch (work/show/ is not on main yet): `snowman-cell`
(`show/snowman-cell`), `klein-scene-should-adopt-the-one-body-loop-sweep`
with its rider (`show/klein-one-body-loop`),
`heatsink-placedunion-base-union-unfinished` (`show/heatsink-flush-fins`).
Common brief at a lane-private path; it points at
`docs/prompts/implementer-discipline.md` by path. Review tier for all
three: single Opus review, style lane plus the unit's claims (each
moves an oracle and a frame; none is architectural).

## 2026-10-02 — snowman in review; the render door

`snowman-cell` is PR 3787, green, in single Opus review. Its lane found
that agents cannot dispatch `render.yml` (`403 Resource not accessible
by integration`), so the frames are unrendered. Ev approved a commit-tag
trigger in chat: `render-on-a-commit-tag` is dispatched (single Opus
review; crosses into CIW's workflows, announced there). Scene PRs hold
until their frames are looked at, through that door once it lands.

## 2026-10-02 — snowman reviewed (MERGE WITH FIXES)

Single Opus review of PR 3787. The oracles were re-derived independently: five
configurations, slice quadrature, agreement within 2.5e-15. No MAJOR. The fix pass
is with the lane:
- the off-axis narration overclaims wall 7: an x shift, a z shift and a spin meet
  three different doors;
- the torus check is missing its axis and centre;
- the census and the exactness claim are not pinned;
- the README carries a measured number;
- the contact-free routing has a third copy;
- the filed TQUERY issue's refs, and `naming.seam_edges` as the public route it
  should address.

Class findings with no home of their own, recorded here:
- `demos/tour/src/main.rs`'s crate doc says every scene is generic over the run
  scalar; about 20 scene modules are f64-only (`grep -L '<S:'`). Pre-existing;
  correct the doc in the next SHOW unit that touches `main.rs`.
- `strut/seed-finder-home-reads-only-the-y-station` cites
  `no-public-rim-arc-selector`, which has no file in `work/`. Pre-existing and
  STRUT's; noted on STRUT's log.
- "Exclude co-surface seams" has three spellings: snowman `waist`, `bodies.rs`
  `bud_rim`, and `rim_select.rs` `Seeds::TwoSided`. The TQUERY issue owns the
  sweep.

## 2026-10-02 — snowman fix pass in; heat sink in review; wave 2 begins

- **`snowman-cell` (PR 3787):** the fix pass is in, head `ec09704f9`, green.
  - Every review finding is addressed.
  - The off-axis poses are pinned by a test.
  - The z-shift refusal is `SectionNotPolar` on the lane's build, which disagrees with the reviewer's reading. The PR body says so.
  - It holds only on its frames.
- **`heatsink-placedunion-base-union-unfinished` (PR 3793):** green, in single Opus review.
  - The flush declared fins refuse at the count edit.
  - The post-union fillet refuses on the rectangular fin-foot rings.
  - Both are pinned as walls.
  - The base is now rounded in-document, before the union.
- **Wave 2 opens:** `projectbox-section-cuts-through-bores` is dispatched.

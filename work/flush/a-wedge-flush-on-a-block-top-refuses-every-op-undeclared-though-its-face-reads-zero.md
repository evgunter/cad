---
id: a-wedge-flush-on-a-block-top-refuses-every-op-undeclared-though-its-face-reads-zero
kind: issue
title: A wedge flush on a block's top, its face tilted 1.2 to 2 eps, refuses every op undeclared though the face stands off the block's plane by a Zero margin; the declared spelling builds, and INTENT F retires it
status: closed
opened: 2026-10-11
priority: P1
cost: M
branch: flush/wedge-flush-reads-zero
pr: 4709
closed: 2026-10-11
---


Found by `flush/readers-closes` re-measuring
`a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`
without its declaration (main `19b704d92f`, release build).

## Repro

The fixture of `crates/topo/tests/door_backstop_settled_residue.rs`:
a `3 × 4.5 × 1` block, and a parallelepiped cornered on its top at
`(0.5, 0.2, 1)` by a 5° wedge angle. The parallelepiped's face in
the block's top plane is tilted about the wedge's edge by
`θ = k · band.zero()`, `k ∈ {±1.2, ±2}`. It either stands on the block
(height 1) or is sunk 0.5 into it. The test declares the pair
(`Rest` standing, `Continuation` sunk). Run the same poses through the
plain `topo::union` / `intersect` / `subtract` instead.

## Measured

- **Main, undeclared**: every op refuses, in both operand orders, at
  every pose and at ε 1e-9, 1e-6 and 1e-12 alike (36 runs per ε row).
  Each refusal is `Escalated { decision: Coincidence(Sectors,
  Settles(Settling { class, .. })) }`, offering `Rest` (standing) or
  `Continuation` (sunk), from `boolean/recl.rs` `require_same` →
  `unglued_tilt`. The predicate is `bool_sector_coplanar`, and its
  margin is 1.046e-10 (±1.2ε) or 1.743e-10 (±2ε) against the zero band
  ±1e-9, at ε 1e-9.
- **Main, declared** (the test as written): it builds every op within
  the gap of the box arithmetic. The exceptions are the crossings the
  test allows, which refuse `ResultVolumeImplausible`. Green at all
  three ε.
- **INTENT F's WIP head** (`intent/s4-f-declared-pairs-retire` at
  `68555105c`), plain booleans: all 24 runs refuse "how two corners
  of the two solids overlap where they meet is undecided: margin
  1.743e-10 lies within the zero band (±1e-9). Recourse: move the
  parts … or tighten the tolerance". F deletes
  `door_backstop_settled_residue.rs` and its declared spelling. Once F
  lands, these poses refuse every op and no test pins them.

## Why it is a finding

The face's largest standoff from the block's plane is
`θ · sin φ · |e_b| = 2e-9 · sin 5° ≈ 1.74e-10` at 2ε, which is the
reported margin: Zero. D10 ("Booleans") glues what its verdicts decide
Zero. The refusal comes from the carrier reading before it:
`carrier_eq_reading` on an undeclared pair reads the two planes over
an `ExtentBall` about the corner of radius `s1.span()` (the
`ReadOn::FirstBounds` reach), not over the faces. The tilt, levered
at that radius, decides `Distinct`, so `unglued_tilt` refuses although
every bound reads Zero. The declared spelling reads the carriers over
the door's consumed extent of both faces, which is why it builds.

So on main a declaration does move this body: it turns a refusal into
a build. INTENT F's premise that a declaration moves no body does not
hold for this fixture, and F deletes the fixture.

## Not measured

- Whether reading an undeclared pair's carriers over the two faces'
  extent (what the declared door consumes) is the fix, or whether the
  span lever is right and the bounds' Zero should be read as the
  sliver band instead. *Answered below: neither; the glue door already
  reads the faces, in band.*
- Which other planar near-flush poses `unglued_tilt` refuses this way.
  *Answered: none. The review's instrumented runs (1920, over 160
  poses × 6 ops at two ε rows) never reach `unglued_tilt`; every
  `Coincidence(Sectors)` refusal on these poses is `vtxfac`'s coplanar
  lump. This row's first diagnosis, and FLUSH's earlier F seam note on
  `work/intent/log.md`, named `recl` wrongly.*

The pre-PR-3866 history of this fixture is in the closed row
`a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`,
and its volume-backstop residue in
`work/reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`.

## Measured after F (`flush/wedge-flush-reads-zero`)

On main `121741a2b` (F merged), release build, the fixture's poses
through the plain booleans: both orders, every op, `k ∈ {±1.2, ±2}`,
standing and sunk. **All 48 runs per ε row refuse, at ε 1e-9, 1e-6
and 1e-12 alike** (144 runs), each `Escalated { decision:
Coincidence(Sectors, ..) }` quoting the bounds' margin `|θ|·sin 5°`
(1.046e-10 at 1.2ε, 1.743e-10 at 2ε, ε 1e-9). Same outcome as the
declared spelling's undeclared twin before F.

The row's premise misses one reading: **the glue door
(`glue::decided_declarations` → `carrier_pair_relation`) reads the pair
in the sliver band at every pose**: `bool_plane_parallel` 3.25ε at
`k = ±1.2`, 5.42ε at `k = ±2` (band `(ε, 10ε)`), at every ε row. The
pair's consumed extent is both faces (`carrier_pair::pair_extent`, the
ball enclosing them), and the block's top reaches ~2.3 m from the
wedge's hinge, so the block's far corners stand off the wedge's plane by
2.76ε (k = ±1.2) or 4.60ε (k = ±2), exactly: in band in exact arithmetic too, not a lever artefact. The
wedge's face stands off the block's plane by a Zero margin, but the
block's face stands off the wedge's by an in-band one.

## The reading

- **The refusal is right under the ratified design.** topo README C2:
  "conformality is decided by the carrier ladder's margins over the
  consumed extent — Zero is one carrier, in band escalates unless a
  declaration bridges it"; D10 (Booleans) "refuses what falls in the
  sliver band", and retires the declaration that bridged it; DESIGN.md
  D1: "Booleans on independently modeled nearly-touching bodies fail
  loudly rather than guess — the design thesis". The declared spelling
  built only because a `Rest`/continuation declaration bridged the
  in-band residue (C4: "bridged only where that displacement's upper
  bound is in band at every consumed point"). F's premise that a
  declaration moves no body fails here because this bridge was real, and
  D10 retires it on purpose. The other reading, that a standing union
  keeps only the block's description and so its result holds no in-band
  pair, is what those clauses rule out: the pair is in band over the
  extent its verdict is consumed on, and the boolean does not guess.
- **The refusal site is `vtxfac.rs`'s coplanar lump**, not `recl.rs`.
  The lump levers the sector's tilt at `s.span()` (`bool_sector_coplanar`),
  reads it in band, admits the class the pair's senses make it, and
  refuses on the bounds' margin (`steeper`). `recl.rs`
  `require_same` → `unglued_tilt` quotes the same predicate and margin
  but admits no class (`DeclarationRead::Moot`); the pin tells them
  apart by the lump's `Settles`.
- **The corner's reason is true, and its offer executes.** With θ fixed
  at 2e-9: just under the offer the glue door's margin would make
  (≈5.4e-10), the glue door reads the pair Distinct and every op in both
  orders still refuses at the corner; just under the corner's offer
  (1.74e-11) the ops build sound (tier 3′ clean) or refuse
  `CrossingInsertion` (`NotTransverse`) with an offer of their own,
  1.5e-12. So the corner names the margin a smaller tolerance has to
  clear, and the glue door's reading does not.
- **The span lever stands in for the sector's region, not the faces'**,
  and over-reaches a thin convex sector by ~`1/sin 5°`. It moves the
  route, not the outcome: the review's 160-pose grid never saw it flip
  one. Shrinking only `vtxfac`'s tilt lever to the sector would land
  the pair on the plane ladder's `Escalated` arm that `vtxfac.rs` calls
  "in band, unreachable here", so it is not a free change either.

## Pinned

- `crates/topo/tests/wedge_flush_on_a_block_top.rs`, on
  `test_support::wedge_on_a_block_top`: every pose, both orders, every
  op refuses `Coincidence(Sectors, Settles(_))` at `bool_sector_coplanar`
  quoting `|θ|·sin 5°`, and the glue door reads the pair's tilt in band
  (`PlaneRung::Parallel`), at every ε row.
- `boolean/offer_rows.rs`: `thin_wedge_in_band_over_its_faces` executes
  the corner's offer on a pose the glue door reads in band (T1, passes
  at 1.57e-11), beside `thin_wedge_on_a_block`, whose glue door reads
  Distinct. `the_in_band_wedge_is_in_band_at_the_glue_door` holds that
  difference, and runs the child row `wedge_in_band_every_op` just under
  the glue door's would-be offer: every op still refuses
  `Coincidence(Sectors)` there, so CI goes red if that offer turns true.
- `crates/topo/tests/wedge_flush_review_probes.rs`: the review's
  ignored battery (the PR poses with exact standoffs, and a 160-pose grid
  over opening, wedge length, block and tilt), with its run command.

No kernel change. INTENT's
`an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence`
is the same structure (glue door in band, the corner reports its own
reading) but not one fix (evidence added to that row).

## Closed (2026-10-11, PR 4709)

The refusal is correct, and it is pinned. The pair stands in the sliver band over its consumed extent: the block's corners are 2.76ε / 4.60ε off the wedge's plane. D10 refuses what falls in the sliver band, and DESIGN.md's thesis is that nearly-touching bodies fail loudly rather than guess. So there is no kernel change and no design fork. Every pose refuses `Coincidence(Sectors)` at `vtxfac`'s coplanar lump (`bool_sector_coplanar`), with a true offer. The glue door's offer would be false, and a test pins that. The PR had a single FULL review, APPROVE-WITH-FIXES, and every fix was taken. INTENT's in-band glue-verdict row carries the sibling family's facts.

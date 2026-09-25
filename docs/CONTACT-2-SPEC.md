# CONTACT-2 — the Planar join lane measures a conic against its section plane

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-2.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/2-axis-lap`. Row carried:
`work/contact/axis-coincident-lap-trips-the-planar-join-invariant` — read it
in full. Its line citation and its `work/bool/…` pointer are stale; the
frontier row it names is now `work/reach/slab-cut-cylinder-refuses-sector-side`.

## The orchestrator's reading (a survey, not a trace — reproduce first)

- The invariant is raised in `chord_join.rs` `between_edge_in_plane`, in
  its `JoinLane::Planar` arm, when the edge's carrier is a `Circle` or an
  `Ellipse`. It is reached only through `skip_adjacent_chord`, from
  `ChordJoiner::join`'s two adjacent-chord guards.
- The lane is chosen per germ face pair in `boolean/join.rs`: plane×plane
  → `Planar`. The operand gate (`reduce.rs` `gate_operand_pairs` /
  `gate_operand_edges`) has admitted `Circle`/`Ellipse` edges since
  M5 PR 9. **So the arm's premise — "the operand gate promises every
  carrier planar" — is stale.** A cylinder's cap disk is a planar face
  with conic edges, and it can be one side of a plane×plane germ.
- The likely path: the extruded circle is two semicircles with vertices
  at the ±x poles, on y = 0. The y = 0 box face meets the z = 4 cap
  (plane×plane) along the diameter between those poles. The two chord
  endpoints are therefore adjacent on the cap's loop, with one semicircle
  between them, and that semicircle reaches the Planar arm. The right
  answer there is "mint the chord": the arc bellies off the section
  plane. The Split arm gets that answer from `ctx.plane`; the Planar lane
  carries no plane.

**Step 1 is to confirm or correct this.** Reproduce the three poses in the
row through `topo::subtract`. Also run the full-length flat, which the row
says builds, and find out why it does not reach the same arm.

## Settled design

**The false half is the gate's promise, so the fix is in the lane, not
the gate.** A plane×plane germ has a section plane: the partner face's
plane. The Planar lane decides a conic edge's side against that plane,
the way the Split arm already does against `ctx.plane`. Share the one
midpoint (or belly) test between the two arms; do not write a second
copy. Delete the invariant arm and its "unreachable" prose, and correct
every comment still promising the gate keeps carriers planar
(`rg 'every carrier planar|carrier planar'`, and sweep the whole family
of that premise).

If step 1 shows a different path, stop and report before you write the
fix. That settles which half is wrong, and the design above assumes it
is the gate's promise.

**What the pose then does is whatever the kernel does, reported
honestly.** It may build. It may instead refuse
`CurvedSectorSideUnsupported` like its off-axis siblings (the REACH
frontier). Either is acceptable. Another invariant, or a wrong body, is
not. If it builds, the body must certify at rest and have the right
volume: analytically, the cylinder's volume less the half-disc cross
section times the 1 of rod length the box covers. Check it with the
tree's own mass properties. Also run `admesh` on the export if the tour
tooling makes that cheap.

## Rows

- Each of the row's three poses through the public door, pinned at its
  actual outcome.
- The axis pose with the box's `y` range flipped (`[-1, 0]`).
- A pose whose section chord meets a single ellipse arc adjacently, so
  that an `Ellipse` carrier takes the same arm: for example, a cylinder
  cut by an oblique cap. If no such fixture is reachable through a
  public door, say so.
- A unit row in `chord_join.rs` for the Planar arm deciding a circle
  edge both ways (it bellies off the plane / it lies in the plane).

## Territory — announce the seam

`chord_join.rs` is claimed by REACH and TANG. `boolean/join.rs` is
ZIP's. None of them is CONTACT's path. Run
`python3 scripts/work.py territory --base origin/main` and name every
program it prints in the PR body. The fix is local to this lane, so
nothing waits on them.

## Review

A single FULL review (claims plus style). The change is small, but minting
or skipping a chord wrongly produces a wrong body.

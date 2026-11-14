# TESSLANE — the plan

The slate is the item files beside this plan
(`python3 scripts/work.py status --program tesslane`); live state is
the tail of `log.md`.

## Charter

A body that builds, passes the tiers and measures is drawn, or refused
typed with a reason that names what is not built yet; and each lane
meshes as coarsely as its own worst-triangle theorem allows
(`crates/mesh/README.md`, `[mesh.sizing]`). The standing goal this
serves is authoring arbitrary geometry through the UI: a body the
viewer cannot draw is where that goal most visibly breaks.

## The two halves of the slate

- **Reach.** Faces no lane meshes: a (chart, boundary) pair the
  trimmed lane has no arm for (a torus or plane face bounded by a
  spiric, a cone face cut by a conic, a sphere face bounded by a
  spline), a ring on a curved face, a sphere face around or near a
  chart pole, a thin arc-bounded planar face.
- **Sizing.** The density each lattice emits and what proves it: the
  one-count grid axes, the arms that size at `UNPROVED_HEADROOM` for
  want of a theorem, split rims, the NURBS lattice's share, rows and
  cells, and one δ for a whole body.

A new arm lands with its theorem or with its headroom filed, so a reach
unit reads the sizing rows of the lane it extends before it is specced.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` `[tolerance.chordal]` and `crates/mesh/README.md`
  `[mesh.sizing]`.
- D9's byte-identical replay; D2's typed refusals; structural rungs
  only, never an inferred one.
- Ev's ruling on PR 5913 (DESIGN `[topology]`): every face chart and
  edge carrier is C¹ strictly inside its cell, and no reader carries a
  crease arm. A row whose fix would teach a lane more about creases
  waits on the construction that removes the crease instead.

## Process

Review tiers and the model draws are `memories/orchestration-model.md`'s
and `docs/MODEL-COMPARE-PROTOCOL.md`'s; each unit's tier and reason are
logged at dispatch. A row filed weeks ago is measured before it is
built against (`memories/refusal-text-is-not-cause.md`).

No exit criteria are set: the track is handed back when its P0 and P1
rows are done.

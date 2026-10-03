# MSOLVE-15 — A mate side is a base with an offset, and the offsets carry the roll (spec)

Unit of the `msolve` program. Item: `work/msolve/MSOLVE-15.md`.

It builds two of Ev's rulings together:
- **PLACE's design-fork row 53** (2026-10-03): `MateFrame { base: Part | Face, offset: Placement }`. The offset is any rigid motion, written in the base's frame. Which offsets a mate admits is its contact class's to say. ASSEMBLY.md A3 and A11 (5) state this.
- **MSOLVE's `[ev]` PR 3681** (2026-10-03, "the new plan makes sense"):
  - the sides' offsets carry the roll, and there is no turn field;
  - `MatePrimitive` names only the residual subgroup: `FrameCoincidence`, `Coaxial { roll: Free | Pinned }` and `PlanarRest`;
  - `PlanarRest`'s standoff retires;
  - the rider, `MatePrimitive::Clocking`, `table_gap`, `TableLacks`, `mate_clocking_redundant` and `Lever::Roll` are deleted.

  ASSEMBLY.md A3 and A11 (1) state this.

It closes `a-face-frame-cannot-turn-its-roll` and `a-clocking-rider-is-levered-unreduced` (the rider goes). It also closes `mate-primitive-unit-variants-load-from-a-null-payload` if no unit variant is left with a second spelling; check `FrameCoincidence` and `PlanarRest`.

Read all three rows, row 53's fork-log entry, and #3681's body in full. PLACE's row `a-part-resting-on-a-gauge-cannot-follow-a-part-edit` carries a note that MSOLVE builds the frame shape. Its crate case and its tour gauge stay with PLACE.

- **Track:** a structural change to the mate datum, its wire and its solve.
- **Review:** dual. The unit changes a public type, the save format of every document holding a mate, and the solve's table.
- **Dispatch:** after MSOLVE-14 merges, since both rewrite `mate/`.

## What the tree says now

- `MateFrame` is `Authored(AuthoredFrame { origin, axis, reference })`, or `FromFace`: the side's own head face, naming no face (#3888).
- `Alignment` holds two frames, a `MatePrimitive` (`FrameCoincidence`, `Coaxial`, `PlanarRest { offset }`, `Clocking`), an `AxisSense` and `clocking: Option<f64>`.
- The coincidence row decides the rider against zero over the lever (`mate_clocking_redundant`, `Lever::Roll`), and refuses a nonzero rider as `Contradictory`.
- A face frame's roll is its carrier's `u_ref`, and nothing can turn it.

## What the unit builds

**1. `MateFrame { base: FrameBase, offset: Placement }`**, with `FrameBase = Part | Face`.
- A `Part` base is the side's part frame. Today's `AuthoredFrame` becomes a `Part` base with the offset that takes the part frame to `(origin, axis, reference)`, a single literal rigid step.
- A `Face` base is the side's head face (#3888), and its offset is written in that face's frame.
- The offset is the `Placement` type gauges and transforms already use: parametric steps of `Expr`s, or literal proper matrices held to A6.
- The side's frame is `base ∘ offset`, at the evaluation's scalar (MSOLVE-14).
- The wire is externally tagged, and every inner struct is `deny_unknown_fields`.
- There is no backward compatibility (Ev, #3123), so the tracked documents holding a mate regenerate through the repo's tooling.

**2. `MatePrimitive = FrameCoincidence | Coaxial { roll: Free | Pinned } | PlanarRest`.** No primitive carries a number.
- `FrameCoincidence`: the two resolved frames coincide, and the residual is `Trivial`.
- `Coaxial { roll: Pinned }`: the residual is prismatic. The roll is the angle between the two sides' resolved references.
- `Coaxial { roll: Free }`: cylindrical, which is today's coaxial with no rider.
- `PlanarRest`: planar. A standoff is a translation in an offset, and under `Rest` the gate refutes one, as it does a set-back side.
- `Alignment::clocking` is deleted, and so are `MatePrimitive::Clocking`, `table_gap`, `MateFault::TableLacks`, `MateToolError::TableRefused`, `mate_clocking_redundant`, `Lever::Roll`, and the self-contradiction shape of `Contradictory` (`held == added`) with its clause in `CONTRADICTORY_RECOURSE`.
- The table then levers nothing, and `admit_mate` forms no lever for it.
- `authored_lengths` and its readers read the offsets' translations. Check: the lever's `‖origin‖` terms include an offset's translation once the side is resolved.

**3. The roll.**
- A mate the tool authors is two `Face` bases with empty offsets. A turn is a rotation about the base's axis, composed onto one side's offset.
- Convention: the second pick's side. Say which side in the tool's doc and in the Python verb.
- Python and the tool get a "turn" verb that composes `Rigid { axis: (0,0,1), angle }` onto that offset. It is sugar, not stored state.
- If row 53's build gives an offset's steps slot addresses, as an instance offset has (`SlotId::rigid`), then turning a committed mate is a `SetParam`/`SetExpression` on that angle. Measure whether it does. If it does not, give a mate side's offset steps slot addresses. If that needs a door outside the fence, file it and say so.

**4. ±0.** Add a row: a vertical wall with normal z = `+0`, and the same wall at `−0`, mated by a `Face` base with the same offset. The solved pose is identical. This is Ev's principle that ±0 never matters; #2468 already meets it at the basis.

**5. The viewer.**
- The mate tool authors face frames with empty offsets.
- It loses `TableRefused`, and its form offers `Coaxial`'s roll as free or pinned.
- A turn control (±90° steps and an angle field) composes onto the offset.
- The story suite's `authored_from_world` fallback goes. The second sail is face frames plus a quarter turn.
- A tour or story row whose verdict moves re-baselines, with the reason given in the PR.

## Acceptance

- **A1:** The windmill. Sail B is a `FrameCoincidence` of `sail_b.bottom` with `hub.back_wall`, opposed, with b's offset turned π/2. The blade lands crossed. Shortening the hub keeps it crossed.
- **A2:** Every former rider row re-baselines, each listed. A nonzero rider used to refuse; the equivalent offset turn now solves.
- **A3:** `Coaxial { roll: Pinned }` with turned offsets solves prismatic at the offsets' angle. `Coaxial { roll: Free }` is cylindrical.
- **A4:** The wire round-trips every arm, and a stray key on any arm refuses. The tracked corpus regenerates through tooling.
- **A5:** The ±0 row (§4).
- **A6:** The items close, and `work.py lint` is clean.

## Constraints, binding

- **Discipline:** `docs/prompts/implementer-discipline.md` in full. Hosted CI is the verification of record. Merge-only.
- **Fence:**
  - `crates/editor-core/src/mate.rs` and `mate/` (all of it);
  - `eval/` (the side frame's resolution and the memo key's mate rows);
  - `persist` (the wire, via the tooling);
  - `crates/viewer/src/matetool.rs` and its pane and form;
  - `pncad-py` (the alignment payload, tags, `.pyi` and census);
  - the tracked corpus (regenerated);
  - ASSEMBLY.md A3 / A11 (1) / A11 (5), wherever the built shape differs from the sentence;
  - tests and the items.

  `geom-core` and `topo` are outside the fence.
- **Stop clause:** STOP and draft the PR if any of these holds:
  - a mate verdict moves that no rider, standoff or roll row explains;
  - the offset's slot addressing needs a door outside the fence;
  - `Placement` cannot be read at a `Face` base's frame without a change to `Placement` itself.

## Review (dual)

The claims to falsify:

- **C1:** One relative pose has the spellings the ruling admits, and no other: offsets on either side, with no turn field.
- **C2:** Every deleted arm has no remaining producer, and its Python crossing is consistent.
- **C3:** The roll is measured from the base's zero (`u_ref`, or the part's reference), and survives a part edit.
- **C4:** The ±0 row holds.
- **C5:** No verdict moves beyond the rows the PR names.

---
id: LIB-PYNAMES
kind: unit
title: the five role-name builders in Python, answering the name text the fillet and shell doors take
status: review
branch: lib/pynames
opened: 2026-09-09
refs: [pncad-py-has-no-door-that-mints-a-revolves-role-names]
pr: 2252
---


`pncad.band`, `band_pi`, `band_rim`, `meridian_vertex` and `carried`
— the Python halves of `pncad::select`'s five builders, each
answering the name TEXT `Node.fillet`'s frozen selection and
`Node.shell`'s open list already take.

## Delivered

- **The seat: module level, `pncad.band(...)`, spelled exactly as
  Rust spells it.** The extension is one flat module — every free
  door in the façade crosses flat (`evaluate`, `product`, `assemble`,
  `circle`) and there is no `pncad.select` submodule to mirror the
  Rust path with. The identical spelling is also what the binding
  census's rule 1 accounts, which is why the five rows leave
  `NOT_BOUND` outright rather than moving to `BOUND_AS`. Home:
  `crates/pncad-py/src/py/select.rs`, the module that mirrors
  `pncad::select`.
- **`meridian_vertex(end, node, vertex)` takes the bound `MeridianEnd`
  mirror**, not a word: the mirror already exists for `SegPat.side`,
  and a closed vocabulary crossing as a class is what `Datum`/`SegTag`
  do. `MeridianEnd::to_kernel` was factored out of `SideArg::to_kernel`
  so the two callers share one mapping — the shape `SplitHalf` already
  had.
- **`carried(node, inner)` takes the inner name TEXT** and refuses
  non-name text at the boundary (`ValueError`), through the same
  `name_from_text` every door that reads a name uses.
- **No conversion sites existed.** The Python suite and the guide
  hand-write no name text today: they select off an evaluation. The
  grep that says so is in the PR body; the doors' positive form is
  `crates/pncad-py/tests/test_role_names.py`, which authors a shell
  and a fillet before anything is evaluated.
- **Prose that had gone false was fixed, not left**: `Node.fillet`'s
  "there is no name-building vocabulary in Python", the two "never
  composed" claims on `Node.chamfer`/`Node.shell`, the guide's copy of
  one, and `test_north_star.py`'s reason for keeping `StableName` on
  the named-gaps list. The invariant that survives is "a name is
  never READ or assembled"; "there is no way to write one" is gone.
- **A guide step**: GUIDE.md §2, "Naming a role before the body
  exists", executed by `tests/test_guide.py` like every other block.
- Census: the five rows and the `B-NAME-BUILDERS` charter are gone,
  with the closure recorded where the file records closures.
- ty fixtures: five off-lattice rows (a name where a node belongs, a
  length where an index belongs, a `CapEnd` where a `MeridianEnd`
  belongs) and the legal authoring beside them. Two of the legal
  fixture's own variables were shadowing the new `carried` door and
  were renamed.

### Deviations

- The unit's scope named the guide only as a conversion site; a guide
  STEP was added anyway, because the doors are user-facing and the
  section that teaches selections said the opposite of what is now
  true.
- The two volume oracles in `test_role_names.py` are asserted to a
  relative `1e-12` rather than exactly: both closed forms carry `pi`
  and sum its rounding in a different order than the kernel
  accumulates in (the shell's differed in its last bit). Every
  dimension in the scenes is dyadic, and no decimal is transcribed
  from a run — the fillet's oracle is Pappus over the removed corner
  section.

### Not this unit

The outer-loop limit is inherited unchanged and stays open as
`the-role-name-builders-reach-only-the-outer-profile-loop`: `seg` and
`vertex` index the outer loop on both sides of the boundary, and the
Python doors say so in their docstrings and in
`test_role_names.py`'s module docstring.

---
id: subtract-of-a-hollow-operand-files-the-island-under-one-solid
kind: issue
title: subtract(A, hollow B strictly inside A) files B's cavity as a second Outer shell of A's solid instead of a solid of its own
status: closed
opened: 2026-09-08
priority: P0
cost: H
closed: 2026-10-03
pr: 3891
---


Measured by the SHELL-5 lane (PR #2159, 2026-09-08) and placed here by
the SHELL orchestrator: `topo::subtract(box 6³, shell(box 2³ at
(2,2,2), 0.25))` — B a hollow body strictly inside A — returns `Ok`
with ONE solid and THREE shells: A's outer (`Outer`, volume 216), B's
outer (`Void`, −8) and B's cavity (`Outer`, +3.375), all under one
solid; tier 3 green; total volume 211.375, which is the correct
number. The material inside B's cavity is a connected component of the
result's material disconnected from A's wall, so it is a solid of its
own, not a shell of A's solid. The containment fallback hands
`insert_void` the whole of B — the door's docs say "positively
oriented single-solid closed body" and its graft attaches every shell
under the destination solid — so after the door's reversal B's cavity
lands as an OUTWARD-facing shell of A's solid. `shell` on a hollow
operand meets the same one-solid state transiently and re-homes each
void twin with `Body::move_shells_to_new_solid` (SHELL-5); the boolean
fallback could re-home the same way, paired off B's own shell roles.
Tier 3 does not catch the shape (see TOPO's
`tier-3-does-not-check-shell-roles-per-solid`), which is why it
validates. Signed (SHELL orchestrator).

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## 2026-09-24 — the shape is valid at rest (ATREST-7)

The sentence above, "Tier 3 does not catch the shape (see TOPO's
`tier-3-does-not-check-shell-roles-per-solid`), which is why it
validates", no longer gives the reason. Tier 3 now reads the nesting
(check 10, `ValidationError::ShellWinding`) and admits this body on the
merits: inside the island the shells wind `+1 − 1 + 1 = 1`, so the
island is material and the solid's shells bound winding 0 or 1
everywhere (`crates/topo/tests/shell_winding.rs`,
`an_island_inside_a_void_of_its_own_solid_certifies`). What this row
asks for is the GROUPING — the island filed as a solid of its own —
which is the boolean's output convention to pursue, not an at-rest
invalidity.

## Ruled (Ev, PR 3901, 2026-10-03)

A solid is one piece of material. Its `Outer` shell and the `Void`
shells of the cavities in its material together bound that material
and nothing else. A cavity belongs to the piece whose material
surrounds it. Pieces that only touch, at a corner, along an edge, or
across a face that contact records hold apart, are distinct solids. A
body is any number of solids. Booleans, `shell` and `split` take
bodies, return bodies, and sort their results into solids, so every
output is an operand.

A product operand refuses in the editor, naming the explicit
cross-instance union. Ev's words: "if an explicit fuse could be added
to make the boolean op work then refuse". Ev also asked that "nearest
enclosing" be replaced ("nearest how?"); the text above defines a
cavity's owner by the material around it instead.

**What this changes for PR 3891.** Its rule, that a piece inside a
cavity is its own solid while pieces side by side stay together, does
not land. The rework sorts every `Outer` into its own solid:
- **The reader:** #3891's nesting reader becomes the shared sort,
  reusing check 10's witness loop and quad-lane roles.
- **Refusals:** a piece whose owner cannot be read refuses typed.
- **The rest of the build:**
  - check 10 tightens to one `Outer` per solid;
  - the single-solid gates on boolean, `shell` and `split` go;
  - `graft_disjoint_all_onto_keyed` goes;
  - `Connectedness` counts solids;
  - `wire_boolean` refuses a product operand.
- **The review's fix list** (m1–m3, N1, Q1, Q4, Q5, Q7) carries over.
- **Owed with the change:** `rows-do-not-cross-a-boolean-remap` stays
  fenced, because products refuse.

## Closed (FUSE, PR 3891, 2026-10-03)

Built to Ev's ruling (PR 3901): a solid is one piece of material.
- **The sort.** `crates/topo/src/pieces.rs` sorts every boolean, split
  and shell result into one `Outer` per solid, with each `Void` under
  the piece whose material surrounds it. It shares check 10's witness
  loop. It refuses typed (`PieceSortError`) where ownership cannot be
  read, and refuses an `Outer` nested in an `Outer` as overlapping
  material.
- **Check 10** requires exactly one `Outer` per solid.
- **Gates.** Booleans, split and shell take bodies, and
  `graft_disjoint_all_onto_keyed` is gone.
- **The editor** refuses a product operand (`ProductOperand`, with the
  explicit union as the recourse) wherever material is fused or
  reshaped. The part count `NodeValue::parts` is carried through
  instantiate, transform, pattern and part. A face-frame datum only
  reads a face and is admitted.
- **Rows:** `crates/topo/tests/hollow_island.rs`, and the docm6 fence
  rows.
- **Review:** dual (kernel; editor and baselines). One MAJOR: the
  product fence leaked through a nested sub-assembly, a `Transform` and
  `PlacedUnion`. It was fixed, and the reviewer re-checked it by
  execution.
- **Residue:**
  - `connectedness-counts-outer-shells-where-it-could-count-solids` (P3);
  - `one-home-for-where-a-shell-stands` (P2);
  - restfront's `tier-3-admits-two-solids-of-one-body-whose-material-overlaps`
    (P1);
  - the STEP `BREP_WITH_VOIDS` writer on export's
    `step-export-refuses-every-hollow-body`.

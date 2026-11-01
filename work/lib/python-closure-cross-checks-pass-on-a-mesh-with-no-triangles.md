---
id: python-closure-cross-checks-pass-on-a-mesh-with-no-triangles
kind: issue
title: The guide's and test_mesh.py's caller-written closure checks pass on a mesh with no triangles
status: open
priority: P2
cost: E
opened: 2026-09-22
---



Found by TESS-4 (PR 3094) as a sibling instance of the defect that unit
fixed in Rust. One class, two sites, both LIB's — `docs/guide/meshing.md`
by TESS's `keep_out`, and `crates/pncad-py/tests/test_mesh.py` by
`work.py territory`. TESS-4 did not edit either.

**The sites.**

- `crates/pncad-py/tests/test_mesh.py`, `unmatched_half_edges` — folds
  the directed edges of `mesh.triangles` into a dict and returns the
  unmatched ones. Its docstring: *"Empty means watertight AND
  consistently oriented."* Over zero triangles the dict is empty and it
  returns `[]`, so the three `TestWatertight` rows that assert
  `== []` would pass on a mesh of nothing. The helper is imported by
  `test_north_star.py`, so the reading travels.
- `docs/guide/meshing.md`, `is_closed` — the same fold, returning
  `all(n == 1 and seen.get(e[::-1]) == 1 for e, n in seen.items())`.
  `all` over an empty dict is `True`, and the page's assertion is
  `assert is_closed(mesh), "watertight at every budget"`.

**The rule they now contradict.** As of TESS-4, `mesh::validate::check_mesh`
refuses a mesh with no triangles by name (`MeshError::NoTriangles`), on
the stated contract that the value is **the mesh of a solid** and not
merely a closed 2-manifold: every closure condition is universal over
edges, so a mesh of nothing satisfies all of them by having none. Both
sites state the old reading in prose — "empty means watertight", "every
directed triangle edge has exactly one opposite twin" — and both are the
*reason given* for `check_mesh` being unbound in Python:
`test_mesh.py`'s module docstring argues that a caller-written
divergence-theorem sum and closure fold are the more honest cross-check.
That argument is good, and it is why this matters: these two functions
are what a reader is taught to write, so the vacuity is being taught.

**Reach.** The producer is real from Python: `Body.tessellate` goes
through `mesh::tessellate`, which answers `Ok` with a mesh of nothing
for a body with no faces and has no face-count guard (TESS has the
open question — `work/tess/should-tessellate-refuse-a-body-with-no-faces.md`).
Neither site can be reached by the shapes it actually meshes (a box, a
cylinder, a washer), so this is a teaching defect and a latent one, not
a live failure.

**The cheap fix, if LIB wants one**: both folds gain a non-emptiness
clause — `is_closed` returns `bool(seen) and all(...)`, and
`unmatched_half_edges` grows a sentence saying an empty result means
watertight *only if there were triangles* (or the rows assert a count
beside it — `mesh.triangle_count` is already bound and `test_mesh.py`
asserts it in five other rows). The docstring prose is the part that
matters more than the code.

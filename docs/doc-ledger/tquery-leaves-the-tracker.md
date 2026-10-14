# TQUERY leaves the tracker — 2026-10-02

TQUERY covered the split and query doors that refuse geometry the kernel
can build. TOPO's priority-seam cut opened it on 2026-09-20; it closed
on 2026-10-02 after one sitting. **No exit walk is owed.** Its plan set
no `## Exit criteria`, and `work/README.md` says that "a plan that
set no criteria leaves a walk nothing to check, and its program closes
without one". `work/tquery/` (`program.md`, `plan.md`, `log.md` and
every closed row) is recoverable at **`50b3434895`**.

## What closed

**Both P0 rows:**
- `split-refuses-cylindrical-feature-box` (PR 3768). Both refusals had
  one cause: a non-unit `SplitPlane.normal` that the tilted
  plane × cylinder section read as unit. `SplitPlane.normal` is a
  `UnitVec3`, and the boolean decides its germ-plane normals at the
  read.
- `rim-of-refuses-extruded-multi-arc-rims` (PR 3773). `rim_of` reads
  one rim from structure, by surface keys and shared vertices, not by a
  bit compare of carriers. That is Ev's ruling on PR 3767, which
  replaced the `GeomSource` repair ruled on PR 3156: that repair had
  nothing to read on kernel-direct bodies.

**Three rulings Ev made at this program's `[ev]` PRs:**
- PR 3763 — one fieldless kind mirror per geometry enum, in `geom`;
  built in PR 3777.
- PR 3767 — the structural `rim_of`; built in PR 3773.
- PR 3813 — a split's copies of one cut vertex share its point; built
  in PR 3856.

Their designer rows are `docs/DESIGN-FORK-LOG.md` rows 41, 42 and 45.

**The rest:**
- the riders on `rim_of` (a dangling curve key, and the
  `NotOneRim` prose), in PR 3773;
- `split_edge`'s key-retention row (PR 3761);
- split's tier-2 self-check, which also closes HONE's
  `split-hands-out-a-body-without-running-tier-3` (PR 3797);
- the unit-normal row: `SplitPlane` in PR 3768, `slab_extent` in
  PR 3809;
- the `vertex_pairs` orientation fix and the `Solid` doc (PR 3820);
- the `edge_sides` door (PR 3803).

## Residue, re-homed by `git mv` with ids kept

- `split-edge-cannot-carry-a-fitted-or-general-pcurve-row` → PCERT.
  It is parked on PCERT's own PR 3759.
- `adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam` → WIRE.
  SHOW filed it on TQUERY's ground the day TQUERY closed. Its kernel
  read has landed (`edge_sides`); what remains is a selection atom.

References to the deleted rows elsewhere now name the PR that closed
each one: 3856, 3773 and 3803.

## Findings this program filed elsewhere

These are on their owners' slates and are not residue:
- WIRE and CONTACT: the P1 pair saying that contact records are dropped
  by every op but the boolean.
- CLEAVE: a band-made pinch now passes the door silently, which makes
  split's ON verdict the guard (P1).
- CONTACT: the operand-labelled `ContactRecords` used as the at-rest
  currency.
- BOXES: cone and torus extents that read their axis as unit by prose.
- CIW: the change filter misses source-reading and tour consumers.
- EXCH: a STEP round trip parts shared points.
- `work/issues/`: f64 bit compares of stored geometry.

## Band

**6500–6599** was claimed at the opening (`docs/MODEL-AB-LOG.md`) and
never drew an ordinal. TQUERY ran no duals: single full or style
reviews, and the orchestrator's read for mechanical rows.

## Classes the sitting measured

These are worth carrying past the log they were written in:
- **A precondition held as prose was the live cause of a P0.** It was
  "unit, unchecked" on `SplitPlane.normal`. The witness ruling's
  remaining prose sites deserve the sweep it asks for.
- **Every fix in the first wave re-minted what it closed, or left a
  twin beside it, and the reviewer, not the author, caught each one.**
  The instances were `chord_join::SectionPlane`, a second a/an helper
  and a second z-poled seed helper. This is the style lane's §1 trap,
  measured again.
- **A shared identity is only as good as the least careful mutator.**
  The offset doors re-minted every vertex in scope and solved twins
  separately, so they had to learn to move per shared point. Any op
  that rebinds vertices must reason per point, not per vertex.
- **A structural identity moves the guard to the producer.** Once a
  touch is held as a shared point, the census cannot catch an op that
  declares structure it only decided within the band. That guard sits
  at the producer's ON verdict (CLEAVE's band-pinch row).

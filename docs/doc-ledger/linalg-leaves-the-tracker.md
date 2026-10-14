# LINALG leaves the tracker — 2026-10-01

LINALG covered geom-core's vectors, frames and interval conventions,
where an answer that is loose at `f64` is wrong at `Interval`. It
opened 2026-09-20, in PROPS's priority-seam cut, and closed 2026-10-01
with an empty slate. Its plan set no exit criteria, so under
`work/README.md`'s closing rule no exit walk is owed.

**What landed:**
- **#2468** `Vec3::orthonormal_basis`. It crosses the normal with a
  world axis chosen by a comparison, with no sign transfer (Ev's
  option 1 on #1944). It rode in DECIDE-3…7 and SYM-9.
- **#3687** D9's NaN qualifier at linalg's home, `Mat3`'s product
  pinned, and the interval backend's zero signs chosen by rule.
- **#3686** `torus_meridian_orient` and `sector_shape` take the
  decided door, and `k_stats::decide_negative` is added.
- **#3711** The sphere-pole branch pick, measured to be a sound hull.
  No refusal was added.
- **#3710** Array doors, `norm_inf`, the frame-escalation recourse, and
  svd's folds.
- **#3725** The test suites adopt those doors.
- **#3727** `Certification::sqrt`, with the private outward roots
  retired into it.

**Fixed on the way, on other programs' ground:**
- **#3636** REACH's backstop rows, red at the 1e-6 and 1e-12 rows.
- **#3661** `ci-filter.py` blanked SEEDS on TIER=all, so a diff that
  touched `.config/nextest.toml` skipped its own slow set.

**Two surface questions settled by a converged designer pair,** both
"no door":
- `Point3` gets no order (the rule is in `linalg.rs`'s "Deliberate
  omissions").
- There is no f64 literal constructor below `pncad::authoring`.

The third question, the camera's 4×4, moved to
`work/vgeom/the-viewer-camera-spells-4x4-arithmetic-with-no-mat4-to-lower-to.md`
with its converged design. It was the program's only residue.

The blinding bytes for both designer pairs are on
`analysis/design-fork/linalg-point-surface` and
`analysis/design-fork/linalg-mat4`. Neither fork went to Ev.

Band 9400–9499 was claimed in `docs/MODEL-AB-LOG.md` and never drew an
ordinal.

Recover with `git show 43333da8a:work/linalg/<file>`, using `plan.md`,
`log.md`, `program.md` or an item id.

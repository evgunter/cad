# CONTACT-9 — do the boolean's side codes read a levered Zero as a verdict?

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-9.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/9-side-codes`, from `main`. The row is
`work/contact/boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on`:
read it. Also read the "Why it is exact" and S1 sections of CONTACT-7's
spec (`git log --all -- docs/CONTACT-7-SPEC.md` finds it) and CONTACT-7's
closing note in `work/contact/CONTACT-7.md`, for the class this unit
checks.

## The class

A reading `x·L` of a unit direction times a lever length has the right
sign whenever it is definite, but the lever moves where Zero falls. It
is harmless where a Zero abandons something (a candidate dropped, a
ray skipped). It is unsound where a Zero is a VERDICT: "this chord
lies in the face's plane", which the machinery then treats as ON. The
census's touch analysis had five defects of this class and CONTACT-7
replaced it with metric readings. Both of that unit's designers
pointed, independently and without checking, at the same shape here:

- `crates/topo/src/boolean/sectors.rs`, `side_code` /
  `enters_material(dir, normal, arm)`;
- `crates/topo/src/boolean/vtxfac.rs` (`classify_vertex_on_face`);
- the splitting lane's twin (`crates/topo/src/splitting/neighborhood.rs`).

Each reads a chord's side of a face plane as `dir·n × arm`, with `arm`
the shorter chord, and a Zero (Tangent) becomes ON, then is
reclassified. A bisector's code has no vertex behind it. The boolean's
`oriented_plane_eq` coplanarity gate may backstop it.

## The work, in order

**1. Trace before changing anything.** For each Zero of each side code:
- follow it to its consumer;
- say whether it is a verdict (it decides ON, and something is built
  or classified from that) or a skip;
- say what, if anything, backstops it (`oriented_plane_eq`, a later
  exact check, a refusal).

Name each site with `file:line`.

**2. Try to make it fail.** Where a Zero is a verdict with no backstop,
build a body pair through the public doors: a face tilted about a chord
so that the chord's far end leaves the plane by many times the band
while the levered reading is Zero. Look for the analogue of CONTACT-1's
obtuse-sector witness. Check the boolean's result against ground truth
(volume, tier 3, point membership).
- If you cannot make it fail and a backstop is shown, stop and report:
  that is a valid outcome, and the row closes on your argument.

**3. Fix only what fails.** The fix is metric: a real point's signed
distance from the plane, as CONTACT-7's `mod metric` does. It is not a
retuned lever. Share CONTACT-7's reader if it fits; do not respell it.
Rows pin every wrong verdict found. Show that the full `topo` and
`sweep` suites keep their answers, or name each one that moves.

## Rows

- One row per site pinning the trace's conclusion: either the
  backstop, exercised, or the fixed verdict.
- A witness row for every failure found.

## Discipline

- Your own clone and `CARGO_TARGET_DIR=/home/user/contact-9-target`,
  with `CARGO_INCREMENTAL=0`.
- `with-build-slot.sh` for every cargo command.
- `df` first.
- No doc-gate script.
- No process listings. Kill only PIDs you recorded; to yield the build
  slot, stop queuing and let your current job finish.
- Never run git commands in `/home/user/cad`.
- Push the branch only; no PR.

Before any hand-back that changes code, run:
- `topo` + `sweep` at three eps;
- ALL of `editor-core`;
- `test-utils`;
- the Python suite (maturin wheel, `unittest discover` under
  `crates/pncad-py/tests`);
- clippy, `cargo fmt --all --check`, gates, lint.

Hand back after step 2 if nothing fails, or after step 3 with the fix
pushed.

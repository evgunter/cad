# CENSUS — one vocabulary, spelled by hand in several places (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`census/`**. Away-channel tag `(CENSUS orchestrator)`.
A/B ordinal band **CENSUS = 3900–3999**.

## Charter

The repository's answer to "is this list complete?" is an instrument: a
census test or a gate that enumerates the real population and compares.
Where one exists it works. Where one does not, the list is hand-written
and drifts, and where one DOES exist but scans the wrong population, the
drift is invisible *and* certified — which is worse.

Every row here is one of those three shapes. They are one program
because the fix has one shape too: **find the one place the population
is declared, make every other site read it, and leave an instrument that
fails when a spelling is added.** A lane that has done one of these rows
can do the next.

The trap this program must not spring is the standing one in its purest
form — **the fix mints a fresh instance of the defect it closes**, and
naming that in your own PR body does not prevent it: a census added by
hand is a new hand-written list. An
instrument that enumerates by reading source text is a reader, and the
reader needs a guard of its own — which is the lesson GUARD's
`gate-roster-and-probe-census-have-no-reader-guards` is paying for on
the other side of the fence.

## Territory — none, and why

This program claims **no paths**. Its subject crosses every crate by
construction: the class is "a vocabulary with several spellings", and
the spellings are in `editor-core`, `viewer`, `topo`, `profile`,
`geom-core`, `geom-brep`, `sweep` and `pncad-py` at once. Each row draws
its fence in the PR that lands it and announces it to the owners; the
`keep_out` in `program.md` names the seven that are already known.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `inert-deny-unknown-fields-on-unit-enums` | **M** | Workspace sweep of 73 sites, but each decided mechanically by unit-vs-struct | `crates/editor-core/src/names/role.rs` + ~17 `editor-core/src` files, `crates/viewer/src/prefs.rs`, editor-core wire tests |
| `hand-listed-debug-censuses-in-geom-core-geom-and-topo` | **M** | Pattern already proven by #2093, but 9 impls in 3 crates and the PartialEq half needs a scope call | `crates/geom-core/src/spline/knots.rs`, `crates/geom-core/src/spline/hull.rs`, `crates/geom/src/curves/nurbs.rs` (incl. `nurbs_curve!`), `crates/geom/src/surfaces/nurbs.rs` — `Debug` impls and the sibling `PartialEq` impls |
| `S113` | **M** | Multi-file; member (d) needs a real invariance re-derivation of the retry ladder. | `crates/geom-core/src/ring_interval.rs`, `crates/topo/src/chart_region.rs`, `crates/topo/src/splitting/containment.rs`, `demos/README.md` (+ S64/S67/S74/S89/S98 members) |
| `S133` | **M** | profile half discharged; remaining sweep+disposition rides staffed lanes, scope needs judgement. | `crates/topo/src/chord_join.rs`, `crates/profile/src/path/{path.rs,family.rs,program.rs}` |
| `S57` | **M** | Five known sites, but widening a crate-scoped guard to a concept needs a new instrument. | `crates/editor-core/src/names/emit_topo.rs`, `crates/sweep/src/blend/{build.rs,battery.rs}`, `crates/topo/src/face_normal.rs`, the anti-re-fork guard in `scripts/gates/*`; unswept `crates/mesh/src/walk.rs`, `crates/step-export/src/` |
| `prose-census-cannot-see-a-bypassed-prose-renderer` | **H** | Instrument rework plus triage of 453 unmeasured sites; verdict key is a design choice | `crates/pncad-py/src/prose_census.rs` (`census()` scan set, `declaration_verdict`), plus sites it reds: `crates/viewer/src/session/refuse.rs`, `crates/editor-core/src/edit.rs`, `crates/pncad-py/src/py/`, `crates/test-utils/` |
| `the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused` | **H** | Two decisions owed, owner undecided, 23+ sites over seven crates and three programs. | `crates/pncad-py/src/prose_census.rs`, `crates/editor-core/src/{expr.rs,node.rs,mate.rs,edit.rs}`, `crates/viewer/src/{session/refuse.rs,tools.rs,sketch.rs,pane/properties.rs}`, 23 `label()`/`name()` sites across `geom-brep`, `sweep`, `topo`, `profile`, `geom-core` |

## Order

`inert-deny-unknown-fields-on-unit-enums` first: 73 sites decided
mechanically by unit-vs-struct, no design call, and it establishes the
lane's habit of leaving the instrument behind.
`hand-listed-debug-censuses-…` next — the pattern is already proven by
PR 2093, so the row is a repetition with a scope call on the `PartialEq`
half.

Then `S113` and `S133`, which are prose and duplication counts that
several staffed lanes already ride.

The two prose-census rows go last and go together:
`prose-census-cannot-see-a-bypassed-prose-renderer` fixes the
instrument's scan set, and
`the-prose-word-for-a-kind-has-four-spellings-…` is the population that
instrument would then see. Landing the second first means censusing four
spellings by hand into a census that cannot see one of them.

`S57` is the seam row: its call sites are landed here and its
anti-re-fork guard is `scripts/gates/*`, so the guard half is **filed on
GUARD**, never landed from here.

## Review posture

One style review per unit, plus a correctness arm on any unit that
changes what an instrument SCANS — a census that stops seeing a
population fails silently and no test catches it.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.

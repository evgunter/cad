# COMB — the roll-ups and the sweeps that go last (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`comb/`**. Away-channel tag `(COMB orchestrator)`.
A/B ordinal band **COMB = 4300–4399**.

## Charter

Two kinds of row that share one property: **no fence can hold them.**

- **The sweeps.** `L1`–`L5`, `S36`, `S37`, `S38`, `D79` — comb-and-rename
  over ~230 suites, comment trimming over every crate's `src`, the
  milestone-coded rustdoc in four kernel crates, this program's own
  audit of its struck rows and its one-sided dispositions. Each is
  document-wide or workspace-wide, operates on files whose fate earlier
  rows have not settled, and was written from the start as
  *not takeable while a track is open on those files*.
- **The roll-ups.** `S35` (62 live sub-rows), `S11`, `S19`, `S43`. These
  are not units and must never be dispatched as units. **A roll-up is
  PARTITIONED**: a member whose ground a live program owns is filed
  there as its own row and struck here; what is left when the tracks
  empty is what this program actually carries, and `L3` is the name of
  that remainder.

## Territory — none, by construction

This program claims **no paths** and cannot. `L2`'s population is every
`crates/*/src/**/*.rs`; `L1`'s is every suite; `S35`'s spans eight
crates. Claiming ground for them would put this program in conflict with
every other program in the tree simultaneously, which is precisely the
fact that made them "last, deliberately".

**The gate on dispatch is therefore not a fence but a question**: does
any live program have an open unit on the files this row would touch? If
yes, the row is not takeable. That question is asked at dispatch, not
written down here, because the answer changes weekly.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S43` | **M** | Rule ratified as D2 addendum; leftover is one crate's conversion with per-site proofs. | 5 idioms, 4 discharged; residue = `crates/mesh/src/{tessellate.rs,walk.rs,chords.rs,curved.rs,planar.rs,trimmed.rs}` (idiom 2), plus S14-owned `docs/DESIGN.md` + `crates/topo/tests/review_m1_pr5_internal.rs` |
| `S19` | **H** | ~260 refusal sites over 5 crates; downstream of S43's addendum; placed as D36/D48/D366. | 3 table rows + 5 postmortem rows: `crates/mesh/src/*.rs` (`MissingEntity`, 49 sites), `crates/geom-brep/src/pcurve_cache.rs` (22 sites), `crates/topo/src/validate.rs` (59 variants), `crates/pncad-py/src/tags.rs`, `crates/editor-core/src/{edit.rs,program.rs}` |
| `S11` | **H** | Roll-up over 8 crates; several rows still await an unasked delete-or-keep call. | ~14 table rows + 12 sort rows + 4 still-tabled: `crates/geom-brep/src/{props/quad.rs,pcurve.rs,ssi.rs}`, `crates/geom-core/src/{spline/hull.rs,linalg/vec.rs}`, `crates/geom/src/curves/boxes.rs`, `crates/topo/src/boolean/mod.rs`, `crates/editor-core/src/{eval/wire.rs,resolve/vdiff.rs,mate.rs,mate/{coset,solve}.rs}`, `crates/step-import/src/recognize.rs`, `crates/profile/src/validate.rs`, `crates/sweep/src/blend/open/planar.rs` |
| `S35` | **H** | Sixty-two independent findings spanning every track; partitioned out row by row. | roll-up of 63 table rows (1 struck closed 2026-09-04, 62 live) over `crates/editor-core/src/{meta,persist,mate,names,resolve,eval}/**`, `crates/topo/src/{validate.rs,euler*.rs,boolean/,splitting/,null.rs,body.rs}`, `crates/geom-brep/src/{ssi/,certify.rs,pcurve_cache.rs,blend/,props/}`, `crates/geom/src/curves/**`, `crates/geom-core/src/{linalg,spline}/**`, `crates/step-import/src/**`, `crates/profile/src/**`, `crates/mesh/src/trimmed.rs`, `crates/bvh/src/lib.rs` |
| `L3` | **H** | Residue of a 60+ row table spanning every track's territory; scope unknown until tracks empty. | whatever S35 rows no track claimed: `crates/{editor-core,topo,geom-brep,step-import,profile,geom,mesh,bvh}/src/**` |
| `S36` | **H** | Comb-and-fixup ~230 suites plus lift `review_*` out of src | `crates/topo/tests/review_m*`, `crates/topo/src/review_m1_pr*.rs`, `crates/step-import/src/cr_r1_probes.rs`, `crates/*/tests/*.rs`, `crates/*/src` stale `docs/M*-LOG` refs |
| `L1` | **H** | Per-suite comb-then-rename over hundreds of files; collides with every fence | `crates/*/tests/*.rs` (~230 suites), `crates/topo/src/review_m*`, `crates/step-import/src/cr_r1_probes.rs` |
| `S37` | **H** | Cross-cutting rustdoc/comment sweep over four kernel crates; collides with every track | `crates/topo/`, `crates/editor-core/`, `crates/geom-brep/`, `crates/geom-core/` rustdoc (~1115 lines); 473 `//` comments workspace-wide |
| `S38` | **H** | Judgement-heavy prose trim across many crates; every track's files | `crates/pncad/src/{closure.rs,select.rs}`, `crates/mesh/src/{lib.rs,walk.rs}`, `crates/profile/src/fillet_select.rs`, `crates/geom/src` (`speed_lower_bound`), `crates/topo/src/boolean/ops.rs`, `crates/geom-brep/src/props/quad.rs`, `crates/sweep/src/blend/surgery.rs`, `crates/geom-brep/src/{planar.rs,curved.rs}` |
| `L2` | **H** | Workspace-wide sweep colliding with every track; ordering rule 1 holds it. | workspace-wide comments: `crates/*/src/**/*.rs` (S38 population plus S37's 473 milestone-coded `//`) |
| `D79` | **M** | One file, but what to cut is pure judgement; waits behind L2's sweep | `demos/tour/src/lily.rs` (118-line `//!` header, 40% comment body) |
| `L4` | **H** | Register-wide re-audit of one-sided gate reasoning; collides with every track | no code fence — the register itself: `work/code-quality/*.md` dispositions (and their cited sites when re-opened) |
| `L5` | **H** | Program-wide walk of every struck row on every track; no fence, citations re-derived | `work/*/logs/*`, `work/code-quality/*.md` (new rides-along files), `docs/DOC-LEDGER.md` |

## Order

Partition before sweep, and within that, **decide before you delete and
delete before you polish** — the ordering these rows were written under,
and the reason every one of them says it goes last: they operate on
files whose fate earlier rows have not settled.

1. **Partition the four roll-ups.** `S43` first — four of its five
   idioms are already discharged and the residue is one crate's
   conversion. Then `S19`, `S11`, `S35`. Each partition PR files rows on
   live programs and strikes the members it filed; none of them fixes
   anything. This is most of the program's actual output.
2. **`L3`** is by definition what step 1 leaves, so it is not readable
   until step 1 finishes.
3. **The comb pass** — `S36` then `L1`, per-suite review before
   comb-and-rename, which is the order both rows already state.
4. **The comment passes** — `S37`, `S38`, `L2`, `D79`. Last, because
   every earlier deletion changes what there is to comment on.
5. **`L4` and `L5`** are audits of this program's own record and can run
   at any point; they produce findings and rides-along files, never edits
   to another program's live item.

## Review posture

Infra-and-prose for the sweeps: one style review per unit, no A/B row.
The partition PRs take a second reader whose only question is **was a
member filed or was it quietly dropped** — a struck row with no new home
is the exact loss `L5` exists to walk back.
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

# Sweep 9 — 2026-09-08: EVAL leaves the tracker

Sweep SHA: `9c515cb150e8f396c6f197c2354650e8e676e11d` — the ratification
commit immediately before the deletion (on the closing PR's branch,
reachable from `main` through that PR's merge commit), so every path
below is recoverable at
`git show 9c515cb150e8f396c6f197c2354650e8e676e11d:work/eval/<FILE>`,
`git show 9c515cb150e8f396c6f197c2354650e8e676e11d:docs/EVAL-EXIT-WALK.md`
and `git show 9c515cb150e8f396c6f197c2354650e8e676e11d:docs/EVAL-<N>-SPEC.md`
for the seven specs listed below.

EVAL — the evaluation seat (`crates/editor-core/src/eval/*`, the verb
seat, the names emitters, `topo::query`/`flush`), the ground SEAT held
and Track V never staffed — opened 2026-09-06 in the tracker-wide cut
and closed 2026-09-08 on Ev's ratification of `docs/EVAL-EXIT-WALK.md`
(PR #2201, "lgtm!", merged `8d34121c7`). Per the sweep-5 rule the
program's directory leaves whole: `program.md`, `plan.md`, `log.md` and
every item file, every one `status: closed` except the five re-homed
below. Eleven E units under the CIW/CHROME posture (one style review
each, a correctness arm on units 2, 6, 7, 8, 9), no A/B rows (band
3000–3099 claimed and unused), two `[ev]` rulings (PRs 2137, 2138).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `eval` | EVAL — the evaluation seat: wiring, the verb seat and the names emitters | 2026-09-08 | this row and the exit-walk row below; the units' PRs (2139, 2153, 2160, 2165, 2168, 2173, 2176, 2186, 2190, 2194, 2195); design at `docs/DESIGN.md` Band 1 (the content key's inputs, amended by PR 2201), `crates/profile/README.md` (V6, the validated lift), `crates/verbs/README.md` (the `Verb` convention) and the tag declarations at `eval/mod.rs`'s `mod tag` (format 7) |

What the walk records: eleven units MET (three with recorded honesty),
the two rulings answered (placers are shape-preserving over
`Instances`; a node's log is every decision made on its behalf), the
deferred row carried; ten honesty rows, among them the session
rate-limit interruption, three red pushes from skipped pre-push
checks, three spec premises the lanes refuted, and the ground
reverting to no owner at close (Ev's ratification took the walk's
recommendation).

### Residue re-homed before the deletion

Moved by `git mv` with ids kept (ownership is the directory):

| item | to |
| --- | --- |
| `D360` | `work/topo/` — a standing sweep rule over `topo`'s refusal enums |
| `map-affine-retires-into-affine3-try-map` (parked) | `work/props/` — beside the PROPS door it is parked on |
| `two-verb-seats-do-not-compose` (deferred), `frame-f64-placement-is-re-evaluated-per-profile`, `wire-expected-phrases-spell-family-words-as-literals` | `work/issues/` — the evaluation seat has no live program after EVAL; with `profile-embed-lift-has-two-homes-anchor-and-loft` and `placement-lifts-its-affine-by-hand-beside-affine3-map` already there, the successor's opening slate |

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `EVAL-EXIT-WALK.md` | 2026-09-08 | PR #2201, "lgtm!" | this row; the residue table above; the amended DESIGN Band 1 sentence |

### Per-unit specs, unit merged

Seven specs were not deleted at their units' merges and leave at this
sweep, recoverable at the sweep SHA; the other four left per-merge
and are ledgered here at the head that last carried each:

- `EVAL-1-SPEC.md` — EVAL-1, the affine lift's one home (#2139)
- `EVAL-2-SPEC.md` — EVAL-2, the tag vocabularies declared once (#2153)
- `EVAL-3-SPEC.md` — EVAL-3, `emit_blend` cites the kernel's arguments (#2160)
- `EVAL-4-SPEC.md` — EVAL-4, `D367`'s accept funnel (#2165)
- `EVAL-5-SPEC.md` — EVAL-5, the two `Verb` types' convention (#2168)
- `EVAL-6-SPEC.md` — EVAL-6, the placers over `Instances` (#2173; the spec rode `[ev]` PR 2137)
- `EVAL-7-SPEC.md` — EVAL-7, the node bracket (#2176; the spec rode `[ev]` PR 2138)
- `EVAL-8-SPEC.md` — EVAL-8, validate once under the pinned lift (#2186); recoverable at `git show d285b8301cc13f31ea60193f72e7de5a07a8bcf3:docs/EVAL-8-SPEC.md`
- `EVAL-9-SPEC.md` — EVAL-9, the slot nominal joins the content key, format 7 (#2190); recoverable at `git show 2011634817c3417feb72e0285ad4475856edbb24:docs/EVAL-9-SPEC.md`
- `EVAL-10-SPEC.md` — EVAL-10, one nominal environment (#2194); recoverable at `git show cf1ad66e0570ffeef1db6f1aa747341a21f77b1f:docs/EVAL-10-SPEC.md`
- `EVAL-11-SPEC.md` — EVAL-11, `node_value_kind` through the placer chain (#2195); recoverable at `git show 0e59a45d374dfa56300925e1ce8b9e08738829ca:docs/EVAL-11-SPEC.md`

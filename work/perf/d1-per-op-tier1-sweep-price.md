---
id: d1-per-op-tier1-sweep-price
kind: ruling
title: D1's per-op whole-body tier-1 postcondition - keep it, narrow it to a declared-delta check, or make it once-per-door
status: open
opened: 2026-09-10
needs_ev: true
---

## The question

D1's ratified clause runs a whole-body tier-1 validation after every
successful Euler operator under `cfg(debug_assertions)`
(`crates/topo/src/euler.rs:62` states it; `assert_euler_postcondition`'s
tier-1 arm at `:2405`, 26 call sites), and the attach setters
`set_face_surface` / `set_edge_curve` re-certify the whole body the
same way (`crates/topo/src/attach.rs:93,332`). Ev asked that any
"stop doing this" that removes work a ratified clause asks for be an
`[ev]` PR before it lands. This ruling carries the price; the PR asks
the question.

## The price, measured

PERF developer lane, `perf/explore-dev`, CI's dev/test profile, only the
tier-1 arm switched (the arena-delta check at `euler.rs:2400` stays):

| binary | on | tier-1 arm off | share |
|---|---:|---:|---:|
| editor-core | 10.85 s | 7.47 s | **31 %** |
| sweep | 10.63 | 10.16 | 4.4 % |
| topo | 3.86 | 3.55 | 7.9 % |
| mesh, geom-brep | — | — | nil |
| six binaries | 76.2 | 71.9 | **5.7 %** |

Benches (`benches/`, release, surgery-only rows): 6.5× on
`kernel/build/extrude`, 5.2× on `kernel/boolean/two_bricks`. Both
numbers are true; the suite number is what a developer waits.

Ev's own binary pays it too: the workspace's `[profile.release]` keeps
`debug-assertions = true` (`Cargo.toml`, "PRE-PUBLISH: this comes back
OUT"), and the PERF GUI lane measured that at 3–4.5× on evaluation
(`die` 82 → 28 ms, `die_composed_tour` 88 → 20 ms, `corner_table` 9.0 →
2.9 ms) and 1.1–1.2× on tessellation — most of it this clause. On the
documents whose edit lags it is not the wait (the index build is,
`index-rebuilds-every-root-on-every-edit`); on node-heavy documents it
is the whole of the wait.

The kernel-API seat pays it in the shipped release binary (PERF kernel
lane, `perf/explore-kernel`, release both ways, 4 vCPU): `die` 78–82 ms
with assertions on vs 35–37 ms off — tier 1 fires 1470 times for 37 ms,
46 % of the rebuild; `die_composed_tour` 90–95 vs 26 ms (1704 firings,
65 %); every corpus row 2.2–5.5×; `demos/wild` 226 → 54 ms (72 % is
tier 1); `demos/tour` 9.5 → 7.0 s. Per node that is ~17 operator
sweeps, where one door-level sweep would do.

CI wall is unaffected in practice: test jobs run a prebuilt nextest
archive at 46–74 s per shard, so this is 2–4 s per shard against an
11-minute compile critical path.

## The options

1. **Keep it** — the soundness pin per Euler op, at 5.7 % of suite
   execution, 31 % of editor-core, and 3–4.5× on Ev's release
   evaluation. The honest floor.
2. **Narrow it to a declared-delta check** — each op already declares
   an `ArenaDelta`; keep that and the local incidence checks the op can
   state about the entities it touched, drop the whole-body
   re-derivation. Loses whole-body tier 1 per op; keeps it at every
   public door via `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`.
3. **Once per public door, not per Euler op** — the sweep moves from
   the 26 operator sites to the door that composes them (an extrude,
   a boolean), so a build of n ops pays one sweep. Keeps whole-body
   tier 1 at every observable boundary; a bug inside a composition is
   caught one door later rather than one op later, with the op
   sequence in the provenance to localize it.
4. **Keep the clause, drop it from `[profile.release]`** — the dev/CI
   lanes keep every assertion, Ev's binary runs at release cost. The
   stanza's own rationale ("the profile that would meet real parts is
   the one profile checking nothing") argues against this alone.

## Recommendation

Option 3. It removes the ops × N term that every seat pays — the
kernel-API user in release, the developer in editor-core's suite — and
keeps whole-body tier 1 at every observable boundary, with the failing
op recovered on failure by replaying the door's operator sequence with
per-op checks on. Option 2 is the only one that weakens what is
checked; option 4 alone leaves dev and CI paying the same term and is
argued against by the stanza's own rationale. Option 1 is the honest
floor if the surgery scope is judged not worth its clarity cost.

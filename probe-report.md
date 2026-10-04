# Probe: topo kill rows vs tier 3 (kef / kef_minting / kev / kemr)

Base: `0edbbecb` (main). This was measurement only; the probe was never committed and is removed from the tree.

## Probe
- A temporary `crates/topo/src/kill_probe.rs` hooks `kev`, `kef` and `kef_minting` (`euler_kill.rs`) and `kemr` (`euler_ring.rs`).
  - It takes the faces before the call: the face of `he`'s loop and of its mate's loop, plus, for `kev`, every face at both endpoints.
  - It measures them after the door's `Ok` (on `Err` the body is untouched).
  - It only measures faces whose rows are complete (`stored_rows(..).complete()`) on a periodic, non-plane chart.
  - Tier 3 is `validate_pcurves` restricted to that one face. The comparison runs `mint_pcurves_of(clone, &[face])` and diffs each row: Debug-equal, or ends off by k·period.
  - It is active only under `KILL_PROBE_OUT`. It appends one line per (call, face), written in a single `write`. This replaces the brief's thread-local dump at exit: nextest runs one process per test, and the main thread's TLS destructors do not run at `exit`.
- **Outermost door only.** A thread-local depth counter is incremented on entry to each hooked door and decremented by `Drop`, so it also unwinds on the `?` paths. The probe records only at depth 0 and logs `NESTED` otherwise. **0 `NESTED` lines in both runs.**
  - `kef_minting` reaches the private `kef_with`, not `kef`.
  - `kev` runs `kev_plan`/`kev_execute`.
  - No hooked door calls another.
  - Producers that call these doors (`merge_faces.rs:2519/2680/2776/2799`, `revolve/full.rs`, `boolean/rest.rs`, `blend/surgery.rs`) are callers, not doors, so the state measured is the state right after the kill, as the brief asks.
- Only `Body<f64>` is measured (`Any` downcast). Not measured: 1,387 calls in sweep (`blend_dual_tangent`, `Dual`) and 1,494 in topo (`*interval*` tests).
- Each joint of the stored chain is also tagged A (dying loop, i.e. the remnant) or B (surviving loop), plus whether it enters an old `first`. The before-kill tier-3 verdict of both faces is recorded too.

## Table: complete periodic faces after the door (rows = class; cells = tier-3 result)

sweep `ci` profile (f64):

| door | (i) byte-equal | (ii) whole-loop shift | (iii) mixed | pass refuses |
|---|---|---|---|---|
| kef | 13 clean | 0 | 8 `LoopDiscontinuity` | 0 |
| kef_minting | 6,277 clean | 168 clean (k=−1: 136, +1: 32) | 1,633 loud: 1,419 introduced (face clean before) + 214 inherited (B loud before) | 1, tier-3 **clean** |
| kev | 592 clean | 0 | 0 | 0 |
| kemr | 73 clean | 0 | 0 | 0 |

`cargo test -p topo` (f64):

| door | (i) | (ii) | (iii) | pass refuses |
|---|---|---|---|---|
| kef | 12 clean | 0 | 1 loud (`RowInterval,Certify,LD×2`; A face planted loud) | 4, all loud (`LoopDiscontinuity`/`LoopNotClosed`) |
| kef_minting | 1 clean | 0 | 1 loud (same planted fixture) | 0 |
| kev | 1 clean | 0 | 0 | 0 |
| kemr | 10 clean | 0 | 0 | 0 |

Every (iii) is tier-3-loud with `LoopDiscontinuity`; no other finding appears on a face that was clean before the kill. 1 `kef_minting` face that was loud before ends byte-equal and clean (`pi_seam_and_kiss…::a_bowl_or_a_split_ball…`). For comparison, the earlier measurement had 168 shifts and 1,589 mixed. This run has 168 shifts and 1,633 mixed, over 8,079 `kef_minting` faces, against the ~4,146 calls it saw; I did not reconcile the totals.

**The claim is confirmed.**
- `kef` sets the surviving loop's `first := d = next(m)` unconditionally (`euler_kill.rs:1625`).
- So the merged cycle is `d…c` (the B lift) then `b…a` (the remnant, from the A lift).
- Where the two lifts' offsets differ, the stored chain jumps by a whole period at the junction `c→b`, mid-loop.
- On a non-wrapping loop it jumps back at the closure `a→d`.
- Located jumps on the 1,427 introduced (iii) faces (sweep, `kef` + `kef_minting`):
  - 1,319 at the lift junction `c→b`;
  - 93 at an old closure joint (see Wrapping);
  - 8 torus faces where the jump is a whole period in **v** (gy = −τ);
  - 7 at the junction on a sub-band-radius cylinder (`contact_edge_must_carry*`), where the lever is in-band.

## Mixed but tier-3-clean
**0 seen** in either run.
- Neither candidate explanation produces a clean (iii).
  - A zero junction gap, where both lifts are at the same k, shows up as (i) or (ii). The 168 (ii) are exactly that: both lifts agree, and only the pass, walking from the new anchor, lands one period over.
  - Pole branch choice: 4 faces with a whole-period jump at a pole joint (zero lever) are (i) byte-equal and clean. The pass makes the same free choice there.

## Pass refusals afterwards
- **sweep, 1 case, tier-3 clean.** `snowman::the_snowman_builds_under_every_boolean`, `kef_minting` → sphere cap face `5v3`, `mint_pcurves_of` → `LoopNotClosed`.
  - The loop is: rim arc, a meridian **slit to the pole** (both halves of one edge, u=π), back down, rim arc.
  - The stored rows (lifted from the old anchor) are continuous, wrap once, and validate clean.
  - Walking from the anchor `kef` re-set, the pass runs arc u∈[−2π,−π], then up to the pole at u=−π. At the pole joint `pin_branch` skips the shift (`ku = 0`, `pcurves.rs:3626-3630`), so the down-half keeps its base branch at u=+π: a free 2π jump with zero lever.
  - It ends at u=2π, so the net span is 4π. `loop_closes` (`pcurves.rs:1782`) allows only ±1 period → `LoopNotClosed`.
  - The pass's verdict therefore depends on the anchor, and it refuses a row set tier 3 accepts. (Walk traced with a temporary `eprintln!` in `walk_cycle`.)
- **topo, 4 cases.** All in `row_walk_proofs::{row_walks_on_a_few_torn_bodies, row_walks_on_seeds_that_divert_the_sheet}`. The B face was incomplete before the kill and the fixture's rows are planted with non-period gaps (gx=±1.2). The pass refuses `LoopDiscontinuity`/`LoopNotClosed`, and tier 3 is loud with the same findings. These are expected.

## Wrapping loops (winding ±1) through kev / kemr / kef
- **kef_minting (sweep):** 1,368 faces carry a wrapping loop: 919 (i) clean, 448 (iii) loud, plus the 1 snowman refusal.
  - **93 faces: the unconditional re-anchor leaves the old closure joint mid-loop.** The only whole-period jump enters an old `first`: 68 at B's (`BBF`), 25 at A's (`AAF`). There is no jump at the new closure.
  - All 93 are tier-3-loud (`LoopDiscontinuity`).
  - The other wrapping (iii) faces carry the wrap at the lift junction `c→b`.
- **kef (keys-only, sweep):** 14 wrapping faces: 9 clean, 5 loud (junction).
- **kev:** 0 faces with a wrapping loop seen (sweep 592, topo 1, all wrap=0), so the re-anchor at `euler_kill.rs:518-522` was not exercised on one.
- **kemr:** 3 faces with two wrapping loops each (sweep), all (i) and clean; 0 old-closure jumps.

## The three prior findings
1. **"Pole-skip margin bug (azimuth-free joint, zero lever)": CONFIRMED, real.**
   - **(a1) zero lever.** The skip at `pcurves.rs:3630` (`Ok(Sign::Zero) | Err(_) => Ok(T::zero())`) also drops the whole-period bookkeeping. A half after a pole can stay a period off the chain, continuity cannot see it, and closure does, so the pass refuses depending on the anchor.
     - Seen live once: the snowman case above.
     - Recipe: run that test. After its `kef_minting` on the cap, `validate_pcurves` is clean and `mint_pcurves_of(&mut body.clone(), &[cap])` returns `LoopNotClosed`.
     - Synthetic: a sphere cap whose loop is two rim arcs plus a meridian slit to a pole vertex, anchored on the rim arc whose derived base sits a period below the slit's.
   - **(a2) in-band lever.** The gate meters the arm itself (m/rad) as a length (`Margin::of(joint_arm.magnitude())`, `pcurves.rs:3624-3628`). Continuity then meters arm·|gap| (`:3667-3671`).
     - So for lever ∈ (ε/|gap|, Kε] the shift is skipped and continuity refuses, although the shifted branch is exact.
     - Reproduced with a temporary unit test calling `pin_branch`: cone, α=0.5, joint at v. v∈{5e-10, 1e-9, 2e-9} gives `Escalated`; v∈{5e-9, 1e-8, 2e-8} gives `Discontinuity`; v≥3e-8 shifts and fits; v≤1e-10 fits.
     - It errs toward refusing and never certifies wrongly. The code comment at `:3600-3620` accepts it ("the skip defers; the margins decide").
     - Live: 0 hits. All 21,566 live skips in sweep had arm < 1e-12 or = 0, and all fit. In topo, 5 of 5 live skips had arm = 0 and fit; the other 9 skips were the unit test itself.
2. **"Old-closure-joint re-anchor risk, 432 faces at kef": PARTLY CONFIRMED.**
   - 432 reproduces exactly as "`kef_minting`, wrapping loop, one mid-loop jump" in the first run.
   - Joint tagging splits it: ~93 faces are the old closure joint left mid-loop by `euler_kill.rs:1625`. The rest are the lift-junction jump, which is the main claim.
   - All are tier-3-loud, so neither is silent.
3. **"Probe artifact (kemr wrapper)": CONFIRMED as an artifact, not a kernel defect.**
   - With outermost-only hooking (0 nested), `kemr` shows 83 faces, all byte-equal and clean, and no wrapping-loop jump.
   - Nothing calls `kemr` from inside another hooked door. Its in-kernel callers are producers (`merge_faces.rs:2799`, `shell.rs:1991`).

## Other off-question defects
- None beyond (a1)/(a2).
- Note: tier 3 accepts any u gap at a zero-lever joint (`pcurves.rs:4233`, arm at `prev.y`), and the pass's closure counts it. That asymmetry is what makes the snowman row set tier-3-clean but un-re-mintable.

## Commands and wall time (4 vCPU, `CARGO_TARGET_DIR=/home/user/probe-target`)
- `cargo nextest run -p sweep --profile ci --no-run`: 1m28s for the first build.
- `KILL_PROBE_OUT=…/sweep.tsv cargo nextest run -p sweep --profile ci`: 2,040 passed, 122 skipped, 73 s. Run 3 times as the probe was refined; the tables are from the final run.
- `KILL_PROBE_OUT=…/topo.tsv cargo test -p topo`: lib 1,367 passed + `tests/all` 897 + 14 + 10, 0 failed, 76 s.
- `cargo test -p topo --lib probe_in_band_lever_needing_one_period -- --nocapture`: the (a2) harness.
- `WALK_DUMP=1 KILL_PROBE_OUT=… cargo nextest run -p sweep --profile ci -E 'test(=snowman::the_snowman_builds_under_every_boolean)' --no-capture`: the (a1) trace.

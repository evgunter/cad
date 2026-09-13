---
id: ladder-rim-phase-may-retire-a-new-split-key
kind: unit
title: blend: the ladder rim phase can push a fresh split key as a retirement
status: closed
closed: 2026-09-13
opened: 2026-09-05
branch: blend/8-ladder-split-key
pr: 2505
---

## Finding

In `crates/sweep/src/blend/surgery.rs`, `rim_phase` step (2) splits each
rim vertex's meridian and names the piece still touching the rim vertex
the UPPER remnant:

```rust
let upper = if touches_v(body, m) { m } else { created.new_edge };
rec.meridian_splits.push((created.vertex, m));
let lower = if upper == m { created.new_edge } else { m };
rec.meridian_remnants.push((lower, m));
remnants.push((v, upper, m));
```

and step (6) retires the upper remnant with `rec.dead.edges.push(mr)`
where `mr` is that `upper`. When `split_edge` hands the source key `m` to
the LOWER piece (the parent keeps the half whose `he_plus` starts at the
far end), `upper == created.new_edge`, and a FRESH key is pushed to
`dead.edges`. `Retired` is documented as source keys; the totality walk's
direction (b) — "every retirement names a SOURCE key" —
(`test_support::assert_naming_totality`) would fail on it, and the
document layer's `emit_blend` would build a retired-set entry that no
row can ever match.

## Status

MEASURED, and the branch is LIVE. The census instrumented every ladder
carve the `sweep` suite runs — **153 carves over 25 rows** (beside 251
annulus carves over 100 rows) — and finds both orientations shipped:
`upper == created.new_edge` and a fresh key is pushed on the carves where
the rim vertex sits at the END of the meridian's stored direction.
Fourteen shipped rows reach the fresh-key push — among them
`ring_clearance_forms::the_bosss_dome_rim_carves_inside_its_hosts_circular_boundary`
and `fillet_h4_concave_rim::the_boss_carves_a_concave_ladder_band_and_adds_the_cap_fill`
— and none of them runs `assert_naming_totality`, which is why no row
went red. The item's earlier reading ("no shipped ladder fixture reaches
the orientation") was the unmeasured guess it announced itself to be.

**The DOOR does not decide the orientation** — the census's first reading
of its own data ("a revolve seam runs pole-to-rim, a boolean pip seam
rim-to-pole") is false, and the review measured it false. What decides it
is which end of the seam's own STORED direction the rim vertex sits at,
and each door mints both: a boolean pip cut into a slab's underside runs
pole-to-rim while one cut into its top runs rim-to-pole — the one door
in this tree that shows both orientations; the `slab ∪ ball` boss's
surviving union piece runs pole-to-rim like every revolve or union boss
that reaches the ladder (the review's delta caught this sentence saying
rim-to-pole against the row it cites) — and the extruded two-arc rims
are fresh-key through the extrude door the census never
named. The rows that pin those are
`review_ladder_split_key_r1_probes::r1_a_pip_and_a_boss_on_the_slabs_underside_carve_naming_total`
and `review_ladder_split_key_r2_probes`' six.

The census's own scope, as run: `-p sweep --test all --features
test-support`. The interval lane's pipped die
(`m6_surgery_interval::certified::interval_one_pip_composed_die_is_bracketed`)
was run once behind `--features interval`; `demos/tour`'s `bud.rs` and
`diechamfer.rs` carves are outside it and were not run. `docs/BLEND-8-SPEC.md`'s
Phase 1 door (b) — a profile authored in the other traversal order — is
UNREACHABLE: `profile` canonicalizes traversal, so the reversed profile
mints the same meridian direction
(`review_ladder_split_key_r2_probes::r2_a_profile_authored_in_reverse_mints_the_same_meridian_direction`).

The ruled band's `split_rim` in the same crate guards the same shape with
`if near == source` and a fragment-provenance read, which is the fix
shape; the ladder path was deliberately NOT changed in FILLET-H7 (PR
1897) because every existing carve's dump is held bit-identical there.
The dump IS bit-identical under the fix — the records the fix moves are
naming rows, which no geometry reads.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `blend/surgery.rs`'s ladder rim phase; owes a fixture before the branch is touched.

## Landed (PR 2505)

`sweep::blend::surgery::split_fragment` + `retire_fragment` are the one
home of the band surgery's split provenance and of "only a SOURCE key is
a retirement"; the ladder rim phase's steps (2) and (6) and the ruled
band's `split_rim` call them, and `blend_surgery` ends with a debug
postcondition over both `Retired` arenas.

- **The rule**: not the door but which end of the seam's stored
  direction carries the rim vertex. `split_edge` keeps the parent key for
  the `[t0, t]` child, so the rim-side piece is the source key when
  `he_plus` starts at the rim vertex and a minted key when it ends there.
- **Counts, measured**: at the true merge base `9739568afa` the suite
  takes 640 `blend_surgery` calls and at the head 648 (the head adds
  three suites); excluding this unit's own suites, 630 records at both
  SHAs with **31 moved**, over 16 rows, every difference confined to
  `dead.edges` losing minted keys. The bit-dump is byte-identical over
  all 14 corpus files, taken in separate target directories.
- **The mutant** (the old `rec.dead.edges.push(mr)` restored) reds **21
  rows**, not one: twenty at the postcondition, which fires before
  `assert_naming_totality` direction (b) can, and
  `review_d2_adv_probes::d2_no_input_reaches_a_panic` reporting the same
  panic out of its fuzz door. Swapping `split_fragment`'s near/far
  naming reds 45 rows at the carve itself.
- **The annulus** keeps a second spelling of the split half; measured
  clean (no birth row anywhere names a minted key, over 1378 rows) and
  filed as `annulus-rim-phase-keeps-a-second-spelling-of-the-split-provenance`
  with the shared-wall refresh measurement behind it.
- Two residues elsewhere: `topo`'s `split-edges-key-retention-direction-is-pinned-by-no-row`
  and `work/issues/emit-blend-cannot-observe-a-retirement-naming-a-minted-key`.

## Closed (2026-09-13, PR 2505)

The split's provenance has one home (`split_fragment`, asking the live
body which piece kept the source key) and the surgery's retirements
are guarded by a debug postcondition over edges and vertices. The
premise was wrong the useful way: the retirement was live on 14 shipped
rows, not unmeasured, and the fix makes them naming-total with every
carve bit-identical. The door → direction rule first committed here was
withdrawn by measurement — what decides the orientation is which end of
the seam's stored direction the rim vertex sits at (346 ladder meridian
splits and 112 cap-rim splits over 153 carves on 25 rows). Residues on the
slate from this unit: the annulus's second spelling of the split half
(measured, filed with both fix shapes); `topo`'s retention direction
pinned by no row (TOPO's slate); `emit_blend` unable to observe a
minted-key retirement (`work/issues/`, for EVAL).

---
id: annulus-rim-phase-keeps-a-second-spelling-of-the-split-provenance
kind: issue
title: blend: the annulus rim phase keeps a second spelling of the split's provenance
status: open
opened: 2026-09-13
---


## Finding

BLEND unit 8 hoisted the band surgery's split provenance into one home,
`sweep::blend::surgery::split_fragment` (the split, the near/far naming,
the original-source lookup and the stale-row retain) with
`retire_fragment` beside it. The ladder rim phase and the ruled band's
`split_rim` call both. `rim_phase_annulus` calls only `retire_fragment`:
its split keeps a second spelling.

- the local `split` closure in `rim_phase_annulus` (`crates/sweep/src/blend/surgery.rs`,
  just above the mate-seam loop) is `split_fragment`'s body in other
  words — `seam_split_param`, `split_edge`, then `edge_touches` to name
  the rim-side and far pieces;
- the stale-fragment `retain` is hand-kept at the two call sites (the
  mate-seam loop and the `HostFoot::Seam` arm), where `split_fragment`
  does it as part of the split;
- the fragment rows (`meridian_remnants.push((mate_feet[ix].2, c.mate_seam))`
  and `((*far_side, *seam))`, both in the birth-data block) are named
  after the PLAN's seam key, with no original-source lookup — the one
  thing `split_fragment`'s doc says the shared home exists for.

## What is measured, and what is not

Measured over every carve the `sweep` suite runs (1378 rows), with a
temporary assertion at the end of `blend_surgery` over the SOURCE half
of every birth row (`meridian_remnants`, `slits`, `meridian_splits`,
`rim_trims`, `blends`, `trims`, `arcs`, `corners`, `feet`, `rim_feet`,
`bands`):

- **no birth row anywhere names a key the call minted** — 0 firings. The
  annulus's plan-key spelling is behaviourally correct today, and the
  duplication is drift, not a live defect.
- the refresh arm IS exercised: instrumenting the
  `refresh_annulus_seams` call site, a shared-wall composition moves the
  live seam key off the plan's **37 times** (27 mate, 10 host), and in
  every one of those the live key is a key the call MINTED. So the rows'
  correctness rests entirely on their being named after the plan key
  rather than the key that was split, which is stated in one comment at
  the retirement site and enforced by nothing.
- `test_support::assert_naming_totality` does not check that a
  `meridian_remnants` parent is a source key (it checks the piece, in
  directions (d) and (e)), and `assert_dead_arena`'s analogue is the
  shell's. The kernel-side check that does exist after unit 8 covers
  `Retired` only, not the birth rows' parents.

## Why unit 8 did not route it

`docs/BLEND-8-SPEC.md`'s out-of-scope clause reserves the annulus phase
("its seam splits have their own provenance rows … do not fix it here").
Routing the annulus's split through `split_fragment` also changes what
those rows say: the helper looks the source up off the LIVE key's
existing row, and where a previous band recorded the piece in `slits`
rather than in `meridian_remnants` the lookup finds nothing and falls
back to the live key — which the measurement above shows can be a minted
one. The plan-key spelling is therefore not merely an alternative way of
writing the same thing, and swapping it needs the fixture that
distinguishes them.

## What would close this

Either (a) route the annulus's split through `split_fragment` with a
fixture that exercises the shared-wall refresh arm and pins which key
each row names, or (b) extend the `blend_surgery` postcondition to the
birth rows' SOURCE halves, so the plan-key discipline is enforced rather
than commented, and keep the two spellings with that guard behind them.
(b) is the cheaper half and is what the measurement above was written as.

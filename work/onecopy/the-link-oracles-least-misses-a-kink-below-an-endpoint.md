---
id: the-link-oracles-least-misses-a-kink-below-an-endpoint
kind: issue
title: The link oracle's segment least refines only about its best sample and misses a kink
status: closed
opened: 2026-11-10
priority: P2
cost: E
branch: onecopy/link-oracle-least
closed: 2026-11-10
---


## The finding

`topo link::oracle_rows::offset_and_lowest_on_meet_the_written_down_distance`
is red on main (`69435f09a7`) at a random seed. Replay:

```
CAD_FUZZ_SEED=0xd75256da3a450b2e CAD_FUZZ_EFFORT=1 cargo nextest run -p topo offset_and_lowest_on_meet
```

It reports `saddle: lowest on (−0.52144, −0.62840, −0.25708)–(−0.53091,
0.56129, −0.84407) reads Ok(−0.5309488803624018), not
−0.5309134354014744`.

**The kernel is right; the oracle is wrong.** A dense scan of the
written-down `sd_saddle` (2·10⁵ samples) puts the segment's least at
`t ≈ 0.4666`, at `−0.5309487`. That is where three distances meet: the
top face's `|z|` and the two walls' distances. The value
`−0.5309134` the oracle gives is the far end's.

The oracle's `least` (`crates/topo/src/link/oracle_rows.rs:150`) takes
the single best of 2000 samples and refines about it by a ternary
search. A signed distance is a max or min of several distances, so its
least can sit at a kink where the samples on either side read about
1e-4 above it, and so above an endpoint's value. The endpoint then wins
the sample, and the refinement never visits the kink.

## Closed

`least` refines about every local least of the sample, and takes the
least of those. With it:
- the seed passes;
- 40 random seeds at `CAD_FUZZ_EFFORT=8` (8× the default draws each)
  pass.

So no kernel reading below a true least is hidden behind the old
oracle at those seeds.

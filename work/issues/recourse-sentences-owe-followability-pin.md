---
id: recourse-sentences-owe-followability-pin
kind: issue
title: Recourse sentences owe a followability pin: the dead-recourse class recurred in consecutive units
status: open
opened: 2026-08-30
github: 1278
refs: [1222, 1267]
---

## From GitHub issue 1278

opened 2026-08-30, 0 comments.

Class finding from the S-BLEND program's first two review cycles, filed so it has a durable home.

**The class.** A refusal's recourse sentence endorses an action ("request the rim whole", "use a radius below R") that, when followed, re-refuses — sometimes with the *identical variant*. Both instances were caught only by reviewers executing the recourse, because the suite's pin in each case asserted something weaker than followability:

1. [PR #1222](https://github.com/evgunter/cad/pull/1222) (BLEND-1): `FILLET3_SEAM_VERTEX_RECOURSE` promised a carve that concave seam-split rims then refuse — the suite contained *both halves* of the contradiction and never composed them. Fixed by conditioning the sentence and adding a composed pin (one arc's recourse + the whole-rim answer asserted together, both material sides).
2. [PR #1267](https://github.com/evgunter/cad/pull/1267) (BLEND-7): `FilletEnclosesLegCarrier`'s recourse named a bound below which five sampled radii re-refuse with the same variant (first-hit leg instead of min over enclosing legs, and the class bound instead of the existence bound) — the pin asserted class-absence at a reduced radius, not that the reduced radius *builds*. Being fixed in that PR's pass with a buildability pin.

**The rule the class suggests:** a recourse constant is a claim about a *second* request, so its pin must EXECUTE that second request and assert the promised outcome — a composed row, not a vocabulary or class-absence check. "The sentence is rendered" and "the old class is gone" are both insufficient, demonstrated twice.

**The sweep owed:** the shared recourse constants in `sweep::fillet` (`FILLET3_*_RECOURSE` family) and `profile` (`FILLET_*_RECOURSE`, junction recourses) — for each, either a pin that follows the recourse to its promised outcome exists, or the gap is recorded at the site. Not claimed by any unit yet; whoever takes it should check the two fixed instances above for the pin shape to copy.

## Home

`work/issues/` — the sweep it owes runs over `sweep::fillet` and `profile`'s recourse constants, both S-BLEND-era ground, and S-BLEND is closed.

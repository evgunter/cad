---
id: tour-scenes-gate-and-measure-one-body-through-two-doors
kind: issue
title: tour scenes gate a body and measure it through two doors, the spelling the certificate door exists to replace
status: open
opened: 2026-09-24
---


Found by ATREST-3's sweep for *a door whose doc promises one
derivation and whose body takes two* (a per-function grep for a
tier-3 door and a measurement door called in one body). The library
doors it hit are either fixed by that unit (`step-import`'s `gate3`)
or already take the gate's certificate and continue it
(`pncad-py`'s `validate_geometric_measured`). What is left are tour
scene functions that gate one body with `validate_geometric` (or
`classify_shells`) and measure the SAME body with `mass_properties`
in a separate call — the spelling
`work/perf/gate-then-measure-pays-two-quadratures` (closed, #2440)
fixed in the tour's main walk (`demos/tour/src/main.rs`, `run_body`)
and not in the scenes:

- `demos/tour/src/diefillet.rs`, `stops` — `composed` is measured and
  then validated (`mass_properties`, then `validate_geometric`);
- `demos/tour/src/skinned.rs`, `stops` — `twisted_tube` is validated
  and then measured;
- `demos/tour/src/impeller.rs`, `stops` — each solid of the result is
  measured and then validated through `body_of(ev, r.solid)`;
- `demos/tour/src/teapot.rs`, `torusvessel.rs`, `ring.rs`,
  `tubewall.rs`, `letterforms.rs` — the same pair of calls in one
  scene function (not read site by site).

**Not measured**: which of these bodies carry a quadrature face, so
whether each pair costs two certified quadratures or two closed forms.
On a closed-form body the second call is cheap and this is only the
spelling. Either way the scenes are the tour's evidence of how a user
writes gate-then-measure, and they write it the two-door way; whether
that is friction to remove (a scene helper that continues the
certificate, as `run_body` does) or a deliberate demonstration of the
separate doors is the owner's call.

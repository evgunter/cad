---
id: a-torus-arm-level-row-charges-the-escalation-as-its-datums-rounding
kind: issue
title: A torus-arm level row charges the band's escalation as its datum's rounding
status: open
opened: 2026-11-20
priority: P2
cost: E
---


Found by the blind-spot sweep of `oneround/rounded-pairs` (value/error
pairs read by hand beside `geom_core::running::Rounded`), second pass
(`.abs() ± slack`).

`geom_brep::pair_section::torus_arms::parallel_torus_arm` (closure
`clears`) reads a level row's datum `d` as
`delta = d.abs() + slack + T::from_f64(band.escalate())`. Its comment
says the escalation covers the datum's rounding. That is a tolerance used
as a rounding bound: the datum's rounding is neither computed nor
bounded by anything that ties it to `band.escalate()`, so at a datum
whose rounding passes the escalation (a far pair at a fine ε),
`delta` undercounts. At a coarse ε it overcounts by orders of magnitude,
and the neck it feeds (`√(2·shape·δ)`) widens accordingly.

Either the datum carries its own running bound (`Rounded`, read through
`magnitude()`), or it is re-posed so that no charge is owed (the
`[simple.conditioned]` route, `crates/geom-core/README.md` "Float
rounding"). Either way the escalation leaves the sum.

P2: it decides only how wide the neck is read, and a band's escalation is
well past a near-origin datum's rounding.

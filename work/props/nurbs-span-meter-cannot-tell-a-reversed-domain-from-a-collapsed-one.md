---
id: nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one
kind: issue
title: nurbs_span_meter's d1 - d0 goes negative on a reversed domain, now indistinguishable from a collapsed span
status: open
opened: 2026-09-20
---


## What

`geom_brep::certify`'s NURBS span arm meters the carrier's own knot
domain: `net_length = Margin::metered(T::from_f64(d1 - d0), meter)`,
gated to be definitely positive under `nurbs_span_meter`. On a carrier
whose domain is stored REVERSED (`d1 < d0`) the comparand is definitely
NEGATIVE, and the gate refuses it — correctly, in that nothing should
be certified off it, but with a diagnostic that says the span
COLLAPSED.

Pre-existing: the arm refused a definite non-positive sign before PR
2928 too, and minted the same `MarginDiag::Invalid`. What 2928 changed
is that the refusal is now recorded on the escalation log under
`nurbs_span_meter`, which makes the conflation visible where it was
previously only returned — and a reader of the log now has two
different faults arriving under one name.

## Why it matters

A collapsed span is a geometry problem the user can act on (the
two-tolerance recourse applies). A reversed domain is a malformed
carrier — an invariant the mint side should have refused, with a
different repair. Reporting the second as the first sends the user to
the wrong lever, and `certify`'s own `IntervalNotForward` arm one
statement below shows the crate already distinguishes a backwards span
where it can see one.

## Shape

Check the domain's orientation before metering it and refuse a reversed
one with its own arm, so the gate sees only the question it is for.

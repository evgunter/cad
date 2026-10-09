---
id: nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one
kind: issue
title: nurbs_span_meter's d1 - d0 goes negative on a reversed domain, now indistinguishable from a collapsed span
status: closed
opened: 2026-09-20
priority: P0
cost: E
closed: 2026-10-09
pr: 4435
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

## The meter is now a decision of its own (ENCL)

ENCL's `work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`
routes the gate as its own decision: a definite non-positive verdict is
`CertifyError::SpanMeterCollapsed { verdict }` under
`CertCheck::ParamSpanMeter`, ending in the meter's lever ("move the
geometry so this spline edge turns through less"), and no longer an
`Invalid` escalation under `ParamSpan`. It does not split the domain
question: a reversed domain still reads `Refused::Negative` there, beside
a floor that turns negative. The split this row names is still this
row's, and lands before the meter, in `certify.rs`'s NURBS span arm.


## Closed (2026-10-09, PR 4435): the premise was false

A reversed domain cannot be minted, so there is no conflation for a new
arm to split. `KnotVector::clamped` refuses non-finite knots, decreasing
pairs, and end runs that are not exactly `degree + 1` long. So
`knots[p] < knots[p + 1] ≤ knots[len − 1 − p]`, and every domain is
strictly forward. Every other constructor goes through `clamped`, and
the fields are private.

The finding came from a false sentence in `SpanMeterCollapsed`'s doc
("a knot domain stored reversed reads below zero too"). PR 4435 corrects
that sentence and states the invariant on `KnotVector::domain` and at
the certify arm. It adds `a_reversed_or_collapsed_domain_is_not_mintable`,
which covers reversed END runs; the existing tests reach only interior
steps back. Behaviour is unchanged.

Review tier: the orchestrator's read, down from the STYLE review set at
dispatch. With no new arm, what remains is a doc correction and one
test.

---
id: a-chords-locus-certificate-is-not-its-section-components-cut-to-its-span
kind: issue
title: A chord's locus certificate is derived again at its join door: the carried component's SsiCertificate is taken on another net than the one the chord reads
status: open
opened: 2026-11-20
priority: P1
cost: M
design: true
refs: [a-certificate-is-read-by-its-support]
---


Found by `a-certificate-is-read-by-its-support` (branch
`sect/certificate-by-support`, 2026-11-20), whose item 2 asked first
whether a carried section component's certificate covers a chord's own
carrier. It does not, so that item was left out of the unit.

## What is the same and what is not

- **The band and the limbs are the chord's.** The section table carries
  every pair at the operation's one band (`section_table::carried`,
  `held.band`), and the march certifies each branch with
  `certify::Limbs::All` against the pose's operands as given.
- **The net is not.** A closed loop is certified on
  `FittedR3::net` = `periodic.base_opening()` (`ssi.rs`). A chord on it
  (`chord_join::carried_chord_spec`, the `fitted_loop` arm) stores the
  periodic carrier over the chord's span, and every edge certificate
  reads `certify::ReadSpec::of` → `reading_of` →
  `PeriodicNurbs3::clamped_view((a, b))`, a knot-inserted net with its
  own rounded bits; only a span equal to the base domain returns
  `base_opening` itself. A seated component's span
  (`PairSection::seat`: `(at, at + turn)`) can straddle the base
  opening's end, where only a shifted copy covers it.
- An open component's chord is `chord_join::cut_arc` → `split_at`, a
  re-cut net; a chord leaving backward is that net reversed
  (`reversed_column`) on the negated interval, and the partner
  operand's copy of a chord can run the other way, which is why ball ∪
  rod still takes 12 rung-3 calls on 8 chords, not 8.
- `edge_nurbs::analytic_rung3` cuts the carrier again for the tube
  (`edge_piece`).

So the component's limb-2 sup and tube bound the base-opening net, and
the chord's door reads another net. The same holds for a split edge or
a split pcurve row (`pcurves::site` "each child's certificate IS the
parent's"): a sub-span is a theorem about the parent's certificate, not
the parent's support, so each derives again today.

## What is open

Which of these a chord's (and a split child's) locus certificate rests
on, neither chosen:

1. **Charge the cut.** Keep the nets as they are, and restrict the
   component's certificate by a bound on `|view(t) − base(t)|` over the
   chord's span (the knot insertion's rounding, and for a run-back chord
   the reversal's, which is exact): new certification arithmetic, small.
2. **Read the parent's net.** An edge on a carried component reads the
   component's own net over its interval, not a re-cut one, so the
   restriction is exact (designer B's lean on PR 7182); this changes
   what every reader of such an edge evaluates (C4: "the net past the
   edge's ends is not the edge's"), and a seated chord that straddles
   the base opening's end still needs the shifted copy.

What either would remove: ball ∪ rod's 12 rung-3 derivations per
operation (counted; their share of the operation's 1.7 s at the default
ε was not measured), each over an arc the section already certified.

## 2026-11-21: re-banded P1

A blocker of `a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build`
(P1), which is parked on it: the 12 rung-3 derivations this row would
remove are about 29 % of ball ∪ rod (−0.55 to −0.6 s at opt-level 2;
that row's 2026-11-21 section). Its design question is still open; weigh
it per `docs/prompts/designer.md` before a lane builds it. (SECT orchestrator)

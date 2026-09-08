---
id: k-lint-rule-1-prose-assumes-every-in-band-site-refuses
kind: issue
title: k-lint: rule 1's prose says an indeterminate sample means the kernel refused typed; a folding site records one and does not refuse
status: open
opened: 2026-09-08
---


Found by unit 7's falsification review while folding PR 1220's roster
change into the K baseline (`docs/K-REPORT.md`'s M11 addendum).

**Line numbers below are at `c39a904e` (2026-09-08).** The first
version of this file cited `curved.rs:1747` and `:1594-1751`, which
were already stale when written — they named the branch's merge base
while MESH-12 had moved the file on main. Re-cite against a named tip,
not against "the current tree".

**The claim.** `tools/k-lint/src/lib.rs:16` — *"a recorded
`indeterminate` is a margin INSIDE the ambiguity band `(ε, Kε)`
(maximal fragility; **the kernel refused typed**)"* — and
`tools/k-lint/src/lib.rs:391` — *"a margin **the run could not decide
at all**"*. Both halves of rule 1's description assert the same thing:
that a recorded in-band sample is a decision the run failed to make.

**The counterexample, in shipped code.**
`crates/geom-brep/src/props/curved.rs:1799`
(`sphere_meridian_span_levels`, `:1775-1803`) decides
`props_meridian_pole` and then

```rust
Ok(Sign::Positive | Sign::Zero) | Err(_) => levels.push(extreme),
Ok(Sign::Negative) => {}
```

— the indeterminate arm FOLDS. The decide records through the funnel
like any other, so the sample reaches the sweep and rule 1 would flag
it, but nothing was refused and the answer is not in doubt: the two
fold choices differ by ~band²/2 in latitude, and the continuity
argument is in the fn doc, in PR 1220's body, in
`docs/predicate-dimension-audit.md`'s row and now in K-REPORT.

**What is NOT the fix.** A name-shaped exemption in the rule set is a
threshold adjusted to restore a number, and it would blind rule 1 to
every other predicate's landing on the same corpus. Rule 1 must keep
gating this name. The fix is the two doc lines: say that an in-band
sample is a margin the run classified as indeterminate, and that
whether that was a REFUSAL is the deciding site's disposition — with
the pointer to K-REPORT's M11 addendum, where the two-step trigger
protocol (check the dimension, then check the disposition) is written.

**Why it is filed rather than fixed here.** `tools/k-lint/*` was held
by METER unit 3's fix pass for the life of unit 7's branch. The
measured population is in the addendum; nothing about the gate's
behaviour changes either way — this is a legibility defect in the
sentence a reader reaches for when the row goes red, which is exactly
when a wrong sentence costs the most.

**Measured, so the item is not speculative.** A sweep at `c39a904e`
records 7 940 `props_meridian_pole` samples per ε row and **zero**
in-band ones, so rule 1 is not firing on this name today. The defect is
the sentence a reader reaches for on the day it does.

Refs: `docs/K-REPORT.md` (M11 addendum, 2026-09-08),
`crates/geom-brep/src/props/curved.rs:1708-1803`, PR 1220.

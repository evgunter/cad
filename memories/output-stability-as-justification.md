---
name: output-stability-as-justification
description: Byte/bit-preservation may choose among equivalent implementations; it may never justify keeping code
metadata:
  type: convention
---

Output stability may decide *how* to write a thing — emission order,
arithmetic association, which of two correct spellings. It may never
justify **keeping** a thing: not a second implementation, not a dead
lane, not a worse shape.

**The positive license (Evan, 2026-08-29, ruling on S-CERT Q2 and
its follow-up): bit-preservation is not the bar.** A bit change
≪ ε is always acceptable when it buys cleaner code — and that is
*sufficient, not necessary*: even a flipped classification is fine
when the flip is semantically correct and the resulting code is
cleaner. The obligation that survives is semantic: a structural
exactness a gate relies on (a guaranteed bit-zero, a bitwise
cross-lane identity) may move only with the gate re-derived so the
classification stays correct — never by re-baselining a wrong
answer into place.

If changing code would move committed bytes, say what the bytes are and
whether they are regenerable. Usually they are a golden, and regenerating
a golden is a chore, not a contract.

**Not this rule** — three uses of the same vocabulary that are
load-bearing and must survive it:

- the D2/D9 determinism contract itself (bit-identical replay,
  byte-identical export);
- math-equivalence annotations (`powi(2)` is bit-identical to `a*a`);
- regression scoping ("bit-identical to before this pass existed"),
  which bounds a change's blast radius and is the *right* use.

**The tell:** a comment saying code is *kept*, *retained*, or *not
subsumed* because its output would otherwise change.

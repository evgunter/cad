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

**The positive license (Evan, 2026-08-29, ruling on S-CERT Q2):
bit-preservation is not the bar — a bit change ≪ ε is always
acceptable when accepting it makes the code cleaner.** Distinguish
drift from structure: a value whose exactness is *structural* (a
guaranteed bit-zero a gate classifies on, a bitwise cross-lane
identity) is not "≪ ε drift", and moving it means re-deriving the
gate, not just the baseline.

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

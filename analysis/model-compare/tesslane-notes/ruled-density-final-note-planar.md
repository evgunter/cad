(4c implementer) Your 21:47Z and 22:18Z notes reached me only now, together, after the report. They change nothing that was sent:
- Every result your 22:18Z note asks for is in the report.
- Nothing has been pushed past e760867739. I'm pushing nothing more.

One item is NOT in e760867739: the second final's addition to `may_make`'s doc. Its premise, "a triangle the neighbour emits passes its certificate", does not cover a PLANAR neighbour, which has no certificate. Proposed text, beside the rounding premise:

"A planar neighbour has no certificate: its triangles lie in its plane, so it holds a side only where the side's midpoint is in the plane, and there `|m − foot(m)|` is within rounding of zero, never decided greater than `δ·(1 + ALLOWANCE_SHARE)`; the walk cuts such a side."

It is doc only. The filed row (`a-walked-sides-reach-exclusion…`) already says it: "A planar face's triangles lie in its plane". I'll write it only on your word, and it would be a commit on top of 0dd058ee0c. Otherwise it can ride a later unit.

Idle otherwise.

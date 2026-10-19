---
id: census-confinement-screens-keep-a-first-face-escalation-the-second-face-would-decide-off
kind: issue
title: The census confinement screens stop at the first face's undecided residual and keep its escalation where the second face would decide the point off
status: open
opened: 2026-10-13
---



Found by the §5 sweep of CONTACTHOLD's one-tangent-interiors-reading
lane (loops over two cells that stop at the first non-answer, pattern
`for (face|f|cell|c) in \[[a-z_]+, [a-z_]+\]` over `crates/topo/src`).

`census::pair_holds_point` and `census::pair_holds_edges`
(`census.rs`) loop `for f in [fa, fb]` and return `false` at the first
residual `signed_is_zero` does not read zero. Where `fa`'s residual is
in band, its escalation is pushed and the loop stops, so `fb`, which
may decide the point definitely off its plane, is never read: the
screen's answer is `false` either way, but the escalation stays in the
report where a decided reading would have dropped it, and which face
is listed first decides whether it does.

Not measured on a fixture; read from the code. The rule is
`census::in_both`'s: read both faces, keep the escalations only
where neither decides off.

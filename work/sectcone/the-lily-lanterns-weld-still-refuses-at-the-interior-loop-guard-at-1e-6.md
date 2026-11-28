---
id: the-lily-lanterns-weld-still-refuses-at-the-interior-loop-guard-at-1e-6
kind: issue
title: Since PR 7395 retired lily wall 2, the tour's lantern weld builds at the default ε but still refuses CurvedPairUnsupported at the interior-loop guard at ε 1e-6
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Filed by GREEN's U21 lane (2026-11-24), from GREEN's P0
`main-red-two-editor-core-slow-rows-no-row-holds-at-f2815a23`, whose
fourth item this is.

## Finding

`demos/tour` `eps_regression::tour_runs_green_at_eps_1e_6` is red on main.
It panics at `tests/eps_regression.rs` (`demo-tour <outdir>` exits 101 at
eps `1e-6`), and inside the run, at `src/lily.rs` (wall 2's
`pncad::topo::union(lant, arch, tol).expect(...)`):

`the lantern welds onto the arch: CurvedPairUnsupported { op: Some(Union), site: InteriorLoopGuard, operand: A, face: FaceKey(5v1), kind: Cone, other_face: FaceKey(3v1), other_kind: Torus }`

The nightly's k-lint (release-default) job shows it, 138240634517 on
`6a680cb845`. The default-ε and 1e-12 rows pass.

## Bisect (hosted CI, first parent, ε 1e-6)

Probe run 45382969404. The row passes at `1883da3554`, `94d1b121b9` and
`b4ffffec5b`. It fails from **`f5744c8c20` (PR 7395,
`the-cone-torus-pair-door`)** onward: `c853a860fb`, `aeb9879e68`,
`1cf86c9e96`, `62abb9cab0`, `401ffb83b5`, `25084f4fd6`, `17e78d9c90` and
`6a680cb845`.

## Reading

Before PR 7395, lily wall 2 pinned exactly this refusal at every ε.
PR 7395 retired the wall, kept live: "the interior-loop guard reads the
neck cone against the arch's torus wall through the section value's
cone × torus pose, marched, so the union builds". That holds at the
default ε and at 1e-12, but not at 1e-6, where the guard still meets the
pair unsupported. The PR's gate walks the tour at the default ε only.

What is owed is SECTCONE's judgement. Either the cone × torus pose should
also serve the guard at 1e-6, which is a kernel fix, or the wall stays
pinned at 1e-6 with its reason. In that case re-pin it the way the tour
retires a wall at one ε and not another. Not held; the row is red on main
at 1e-6 until then.

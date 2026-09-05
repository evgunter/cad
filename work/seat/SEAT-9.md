---
id: SEAT-9
kind: unit
title: the shell arm on Verb, and ε stops travelling as an f64 (Ev's ruling (i), ZST-only)
status: closed
opened: 2026-09-05
branch: seat/shell-arm
refs: [shell-doors-take-tolerance-beside-tol, 1904, LIB-G17]
pr: 1995
closed: 2026-09-05
---



Spec: `docs/SEAT-9-SPEC.md` (deleted at merge per `docs/DOC-LEDGER.md`).
Two rulings executed as one unit: VERB-SEAT-DESIGN §2 V4 for the shell
(`Verb::Shell { thickness, open }`, `VerbRecord::Shell(ShellNaming)`,
the enabler `LIB-G17`'s `Node::Shell` waits on), and Ev's ruling (i) on
`[ev]` PR 1904: the shell doors' raw `tolerance: f64` is dropped, the
fit target is ε_precision, and the tolerance travels as the `Tol` ZST
through every signature between the door and the one site that
classifies the fit residual — never an `f64`. `FIT_TOL` constants
retire; the NURBS fit cost at ε is measured and reported, not gated.
Crosses SHELL/PROPS territory (`topo/src/shell.rs`, `replace_face.rs`,
the offset fit): `scripts/work.py territory --base main` receipt in the
PR body. Closes `shell-doors-take-tolerance-beside-tol`. Block SEAT-B3
slot 3 — the block's last; its merge closes SEAT-B3.

## Closed

PR 1995 (2026-09-05). Delivered as specified with one argued
deviation (`Arity::Shell` for "arity One"); the dual found zero code
defects in the unit, weighed the fit-engine boundary the frozen head
had drawn, and the fix pass carried the `Tol` witness into
`geom-brep`'s production doors (the one `eps()` read now sits beside
the classification, in `offset_fit.rs`). Residues filed as their own
items at the fix: `work/issues/approx-surface-tolerance-is-now-always-
the-runs-eps.md` (geom/TRANSFORM — the one production caller of a
numeric-target routine is the transform lane's re-certification
against the stored value), `work/issues/offset-fit-at-tight-eps-
refuses-every-curved-nurbs-chart.md` (PROPS/S-CERT — the engine's
budget vs tight ε, and the unmeasured mint-path cost at default ε),
`work/lib/lib-g17-is-parked-on-a-fired-trigger.md` (LIB). Record:
`work/seat/log.md` "SEAT-9 MERGED"; MODEL-AB-LOG row SEAT9 (ordinal
1011, sample #147). Block SEAT-B3 closed here.

---
id: SHELL-10
kind: unit
title: the simultaneous doors walk only their scope — construction, the pcurve pass and the closure check narrowed to the solids a move set names
status: closed
opened: 2026-09-08
branch: shell/10-scoped-walks
refs: [shell-doors-still-walk-the-whole-body, SHELL-8, SHELL-9]
pr: 2229
closed: 2026-09-08
---



SHELL-8 scoped what the two simultaneous offset doors MOVE to the
solids a move set names, and disclosed three whole-body walks left
around the scoped solve: `Scope::of_solids` walks every shell, face,
loop and half-edge of the body and refuses `Corrupt` on a solid the
moves do not name; `mint_pcurves` clears and re-derives every row of
the whole clone; `validate_closed` reads the whole clone. SHELL-9's
reviewers counted the cost: `shell_open` on an N-solid body pays N
cavity-door mints, k lift mints and one closing mint, all whole-body.
The unit narrows the three walks to the scope — a scope built from the
named solids alone (or shared, as `shell` already does through
`re_scope`), a pcurve pass over the scope's faces, a closure check over
the scope's shells — with the cache-row differential SHELL-9 built as
the evidence that nothing outside the scope changes and nothing inside
it changes either. Closes `shell-doors-still-walk-the-whole-body`.
Spec `docs/SHELL-10-SPEC.md`. Pre-draw difficulty **S–M**, task class
**STRUCTURAL** — logged AFTER block SHELL-B3's byte was drawn (slot 1;
the byte was drawn at slot 0's cut, so this row's covariate is
contaminated the way every non-first slot's is, and is disclosed as
such).

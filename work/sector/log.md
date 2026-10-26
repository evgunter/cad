# SECTOR — the log

## 2026-10-09 — opened

Cut out of CONTACT by CONTACT's orchestrator when it closed (Ev, in
chat 2026-09-29: close CONTACT and move all remaining rows to 2-3
successor programs). CONTACT measured 99.5 budget points against 30;
its units CONTACT-10, -11 and -12 merged (PRs 4363, 4368, 4372) and
CONTACT-13 parked on the D10 hold. Every row here moved by `git mv`
with its id, body and history unchanged. Rows dispatchable: 15, for 28.5 points.
— (CONTACT orchestrator)

- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)
  (Landed on CONTACT's log as CONTACT closed; carried here by the CONTACT orchestrator, since `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid` reads `LeverEscalation`.)

- 2026-10-26 — (tracker sweep) Closed three rows:
  - `unclaimed-half-edge-read-as-a-minus-half-in-contact` (fixed): no `he_plus` read is left in `solid_contain`, and `face_edges` reads through `Edge::claim`.
  - `a-thin-fin-voids-boolean-refuses-tier-3-prime-with-an-edge-face-pierce` (settled by FLUSH's PR 5534): the refusal was the old crown's own self-pierce, and the corrected `posed_fin_crown` builds.
  - `torus-walk-steps-over-null-edges-where-the-cone-refuses` (fixed by `6f920b62`): `region::boundary`'s one `Scaffolding` rule now serves the torus and ruled trims alike.

  Re-read notes on `degenerate-torus-operand-…` (the declare menu is gone; "the solid itself is fine" stands) and on `pierce-germ-direction-within-…` (the pierce half is on main; `sectors::within`'s per-bound lever is still owed). The plan's Order note is corrected: `booleans-glue-on-zero` closed, so the `contact_verify` half is no longer held. 24 points in 12 rows.
- 2026-10-26 — (easy-fixes lane) Closed `a-cube-sunk-flush-into-a-block-refuses-ray-exhausted-on-every-order`: the witness fuses in every order, pinned in `wire_legal_union_refusals`.

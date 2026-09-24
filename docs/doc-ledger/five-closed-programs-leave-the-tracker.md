# Five closed programs leave the tracker — 2026-09-03

**The rule the later sweeps run on** (Ev, 2026-09-03): `work/` tracks work
still to be done, so a closed program's directory leaves the tracker whole
— `program.md`, `plan.md` and `log.md` — not just the narrative pair sweeps
1 and 3 took. `work/README.md` carries it.

Fifteen files, five programs, all `status: closed` with no live items.

Sweep SHA `f955ddc75cda454a268f9214d2a753ae1a9bbd0f`.

    git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/<program>/<FILE>

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `blend` | S-BLEND — fillet/chamfer completion | 2026-08-31 | `docs/S-BLEND-EXIT-WALK.md` (criteria quoted verbatim from the plan) |
| `gauth` | GAUTH — part authoring in the GUI | 2026-08-31 | Ev's in-chat ruling that no exit walk is needed |
| `gui` | GUI v1 | 2026-08-28 | `docs/GUI-EXIT-WALK.md` (paraphrases the plan's criteria) |
| `pcurve` | PCURVE — edge-description unification | 2026-08-29 | `docs/PCURVE-EXIT-WALK.md` (criterion rows quoted verbatim) |
| `qa` | S-QA — gates that lie | 2026-08-31 | `docs/S-QA-EXIT-WALK.md` (criteria quoted verbatim) |

Those five walks left `docs/` with `exit-walks-and-design-docs-leave-docs`.

**Two of the five leave content that is only in git**, which is why this is
named rather than left to be found: `gauth` had no exit walk, so the
closing entry of `work/gauth/log.md` was its done-state of record; and
`GUI-EXIT-WALK.md` paraphrases `GUI-PLAN.md`'s criteria rather than quoting
them, so the criteria text is recoverable only at the SHA above. GAUTH's
A/B ordinals survive in `docs/MODEL-AB-LOG.md` and its residue on its own
issues.

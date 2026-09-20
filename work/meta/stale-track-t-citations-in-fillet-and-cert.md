---
id: stale-track-t-citations-in-fillet-and-cert
kind: issue
title: Three citations to Track T's rows in fillet's and cert's slates are stale
status: closed
opened: 2026-09-04
closed: 2026-09-20
---


## Finding

Track T's rows moved and closed; three sentences in two other programs' slates
still describe the old arrangement. None is this program's to edit — the
one-file-one-item rule makes a cross-program edit a merge conflict by design —
so they are recorded here for their owners.

- `work/fillet/plan.md:71` and `work/fillet/program.md:12` (the `keep_out`
  line) both say Track T's rows `D320`–`D325` land as riders on the fillet
  units that touch their files. `D320`, `D321`, `D323` and `D324` have landed
  on their own Track T branches instead (PRs #1782 and the T-2 PR), so the
  sentence now over-counts by four; what remains true of it is `D322` and
  `D325`, which are held precisely because two fillet lanes are live in
  `blend/surgery.rs`. **FILLET's to re-aim.**
- `work/cert/plan.md:238` — *"Track T's `D320` follows what `D240` mints
  (filed, not taken)"*. `D240` minted `NurbsSurface::map_scalar`, `D320` has
  since been taken and closed on it, so the sentence is spent. **CERT's to
  strike.**

Raised by lane T-1 (code-quality, Track T) during its closing citation sweep
and routed here rather than edited, per `docs/prompts/implementer-discipline.md`
§6. The citations are accurate as of 2026-09-04; re-derive before acting.

## FILLET's half (2026-09-05, PR 1964)

`work/fillet/program.md`'s `keep_out` clause naming Track T's rows is deleted:
`D322`, `D325` and `D326` are all closed, so nothing it kept out is live.
`work/fillet/plan.md:71` is left as written — the exit walk quotes it verbatim.

## CERT's half (2026-09-06, sweep 7)

`work/cert/plan.md` was deleted whole at S-CERT's closing sweep
(`docs/DOC-LEDGER.md`, sweep 7) and the spent sentence went with it,
recoverable at the sweep SHA. Nothing remains for CERT to strike.

## Was

`unrowed` — raised by lane T-1 (code-quality, Track T).

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/meta/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Closed (2026-09-20)

Both halves were discharged in this row's own body and the tree has
since removed what was left of either. FILLET's half landed on
2026-09-05 (PR 1964); CERT's half went with `work/cert/plan.md` at
sweep 7. The one residue the body records — `work/fillet/plan.md:71`,
*"left as written, the exit walk quotes it verbatim"* — is moot:
neither `work/fillet/` nor `work/cert/` is in the tree, both having
gone with their programs' closing sweeps and being recoverable at the
SHAs `docs/DOC-LEDGER.md` names.

Nothing was left to do, which is the finding: the row stayed open for
sixteen days after its last live half was discharged, because closing
it was nobody's dispatched unit. `work/README.md`'s new "The tracker
is not comprehensive" clause is the answer to that shape.

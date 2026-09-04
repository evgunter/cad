---
id: stale-track-t-citations-in-fillet-and-cert
kind: issue
title: Three citations to Track T's rows in fillet's and cert's slates are stale
status: open
opened: 2026-09-04
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

## Was

`unrowed` — raised by lane T-1 (code-quality, Track T).

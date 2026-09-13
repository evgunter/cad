---
id: tool-versions-outside-the-env-block-have-no-source-of-truth
kind: issue
title: FreeCAD, the 3.12 interpreter and the demo render venv are pinned as literals nothing reconciles
status: open
opened: 2026-09-11
refs: [session-start-hook-restates-ci-pins, pinned-version-named-in-present-tense-prose]
---

Found by the whole-tree arm of the pin sweep on PR `ciw/pin-residue`, and
filed rather than fixed there because the fix is a DESIGN decision — where
each of these three values is declared — and not a tense or a reader.

**The blind spot it sits in is a stated one.** `scripts/ci-pin.py`'s header
prescribes the sweep for this class: *"A sweep for this class starts from the
`env:` block — each pinned name and each pinned value — and asks where else in
the tree they appear."* Every value below is a tool version CI installs and
NONE of them is in that block, so the prescribed sweep cannot see any of them.

Three populations, one question each, all inside CIW's territory:

1. **FreeCAD 1.1.2** — 9 lines in `.github/workflows/ci.yml` (the STEP-oracle
   job: cache key, step name, four URL/filename interpolations), 10 in
   `.github/workflows/render.yml` (the same install, plus the provenance line
   the render lane writes into its own output), 1 in `demos/README.md`. The
   two workflows install it independently and nothing compares them; the
   checksum step verifies the ARTIFACT against its published SHA256, which
   proves the download is the 1.1.2 someone asked for and says nothing about
   whether the two lanes asked for the same version. A render lane and a STEP
   oracle on different FreeCADs is the failure, and it is silent.

2. **The 3.12 interpreter** — `.github/workflows/ci.yml:4030` and
   `.github/workflows/nightly.yml:648` each set `python-version: "3.12"`;
   `.claude/hooks/session-start.sh`'s `PY=/usr/bin/python3.12` is a third
   copy (a PATH, so it cannot read a pin as it stands); `ruff.toml:35`
   asserts the fact in prose — *"the hosted python-suite job pins 3.12"* —
   to justify `target-version = "py311"`, which is the present-tense shape
   `pinned-version-named-in-present-tense-prose` is about, in a file no
   program's `paths` claims.

3. **The demo render venv** — `'numpy==2.2.6' 'matplotlib==3.10.3'` at six
   sites in three files: `demos/render.sh:107,110`,
   `demos/render-wild.sh:44,47` and `.claude/hooks/session-start.sh:351,354`,
   whose comment names the first two as the source of truth without reading
   them. A hook that warms the venv to the wrong pins costs a rebuild; the
   two render scripts disagreeing with each other would cost a frame.

**What it needs first is the decision, not a reader.** Either the `env:`
block widens to carry these (and `ci-pin.py` already answers any name in it,
so `render.yml` and the hook read them the way `nightly.yml` reads
`NEXTEST_VERSION`), or each population gets a declared home of its own — a
`demos/render-requirements.txt` both render scripts and the hook install from
is the cheap shape for (3), and it is a different answer from (1)'s. Taking
one of those decisions per population is the unit; `.claude/` is out of every
hosted gate's reach either way, which is what
`session-start-hook-restates-ci-pins` settled by READING rather than checking.

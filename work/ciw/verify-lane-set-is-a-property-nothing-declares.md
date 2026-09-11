---
id: verify-lane-set-is-a-property-nothing-declares
kind: issue
title: render-hosted.sh's --verify lane set is a property render.yml states only in prose
status: open
opened: 2026-09-11
---


`local-scripts/render-hosted.sh`'s `VERIFY_LANES` names the lanes
`--verify` can round-trip: the ones that are byte-reproducible off-box,
so a pulled file may be compared to the committed one at all. It is
`uv mc wild` today, and it is hand-written.

`scripts/check-render-lane-parity.py` holds every other lane fact in
that file against `.github/workflows/render.yml` — the roster, the
artifact names, the committed directories, the poll's job regex — and
it cannot hold this one. render.yml states the property in prose, per
lane, in its own header ("wild — matplotlib Agg … pinned, no GL
anywhere in the path. Byte-reproducible on any box"; "UV — renderer-free
and text, so byte-reproducible on any box like the wild lane"; "The MC
density sheet joined later on exactly the same terms … same producer,
same format, same reproducibility") and declares it nowhere a reader can
key on. The workflow's `if:` conditions select lanes; nothing in it says
which lanes are reproducible.

**"Nothing" is too strong, and the correction is the interesting half.**
`local-scripts/ci-local.sh:552-558` (`uv_sheet_drift`) re-renders the uv
and mc sheets on a developer's box and `git diff --exit-code`s
`demos/renders-uv/ demos/renders-mc/` — a row that MECHANICALLY exercises
byte-reproducibility for two of the three lanes `VERIFY_LANES` names,
every time the local gate runs a code-tier change set. What it does not
do is compare itself to `VERIFY_LANES`, or cover `wild`, or run hosted;
and its own comment says why it pairs those two lanes (one tour run
feeds both), not that it is holding a property list to anything. So the
accurate statement is: the property is exercised for uv and mc and
asserted nowhere, and `VERIFY_LANES` is held to nothing at all.

**Why it was left rather than guarded**, written at the site in the same
words: a lane missing from the list is a lane `--verify` silently does
not prove, and `--verify` prints how many files it checked, so the drop
is visible in the output rather than asserted; a lane wrongly added reds
on the first GL-stack difference, loudly and in the right direction.
That is a bounded cost either way, unlike the roster's — a missing lane
there made `--lane mc` a hard refusal and `--lane all` a silent four of
six.

**What closing it looks like**, if it is worth closing: give render.yml
a declared per-lane property the reader can key on — a `reproducible:`
line beside each `rebaseline-lane` step's `lane:`/`dir:` pair, or a
`lanes-reproducible:` list beside the `lanes` input — and add it as a
fifth claim to `check-render-lane-parity.py`, which already reads both
of those shapes. The cost is a key in render.yml that nothing else
reads, which is why it was not taken in passing.

Site: `local-scripts/render-hosted.sh` (`VERIFY_LANES`, with this
reasoning at the site) and `scripts/check-render-lane-parity.py`'s
"WHAT IT DOES NOT PROVE".

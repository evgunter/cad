---
id: python-lint-row-is-locally-unverifiable-on-this-image
kind: issue
title: the container ships ruff 0.15.8 against a 0.16.1 pin, so the python-lint row skips for every lane
status: open
opened: 2026-09-11
---

Placed by the orchestrator from CIW unit 4's style review
(implementer-discipline §6: the lane reports, the party with the whole
board writes the file). The subject spans `scripts/check-python-lint.py`
and `.claude/hooks/session-start.sh`, both CIW's.

## Measured

On the agent container, 2026-09-11:

```
$ ruff --version
ruff 0.15.8
$ grep RUFF_VERSION .github/workflows/ci.yml
  RUFF_VERSION: "0.16.1"
$ python3 scripts/check-python-lint.py
check-python-lint: SKIPPED — `ruff` is ruff 0.15.8 and
.github/workflows/ci.yml pins 0.16.1. …
```

`.claude/hooks/session-start.sh` installs `cargo-nextest`, `maturin` and
`ty` from the `ci.yml` pins. **It does not install `ruff` at all**, so
the version every lane gets is whatever the image happens to ship, and
the image is two minors behind.

## Why it is a row and not a nuisance

`check-python-lint.py`'s header designs the two behaviours deliberately:
hard failure on the gate of record, a loud skip elsewhere, because
"rule sets and default behaviour move between ruff releases". That is
the right design and it is not what is wrong here.

What is wrong is that **the skip is the state for every lane on this
image, permanently, and nothing says so.** The message reads as a local
misconfiguration a developer could fix — *"Install the pinned version,
or set `$RUFF` to it"* — when in fact no lane can, because nothing
provisions it. So a lane runs its local checks, sees one benign-looking
skip among green rows, pushes, and learns from hosted CI that the row it
could not run is red.

**That is not hypothetical**: CIW unit 4's first hosted run
(`34557183882`) was RED on `B905`/`RUF007` over a `zip(hits, hits[1:])`,
on exactly this row, for exactly this reason. The lane recorded it
honestly; it cost a round trip.

It is also the shape this program has spent three slates on — **a guard
that reports the same thing whether it ran or not** — one level out. The
difference from `nightly-demotions-have-never-run` and
`opt-level-selftest-runs-nowhere` is that here the row DOES run on the
gate of record, so nothing reaches `main` broken. What is lost is the
local half, silently, for everyone.

## Shapes

1. **The hook installs `ruff` from the pin**, like the other three. It
   already has the machinery, and unit 7 gave it a refusal path for a
   failed pin read. Cheapest, and it makes the local row real.
2. **Say so where a lane reads it.** The skip message tells the reader to
   install the pinned version; it could say that on this image nothing
   does, and name the row as one hosted CI will still run. Cheap, and it
   converts a silent gap into a known one without provisioning anything.
3. **Nothing**, on the argument that the gate of record catches it. That
   is the status quo and it has already cost one red run; recorded so the
   next instance is not re-derived.

Shapes 1 and 2 are not exclusive and 1 does not make 2 pointless — a
container that fails to install still wants the message to be true.

## Fence

`scripts/check-*.py` and `.claude/hooks/*` are both CIW's
(`work/ciw/program.md`'s `paths`, the latter as of unit 7). Whether the
DISCLOSURE half belongs in `docs/prompts/implementer-discipline.md` is
META's call — `work/meta/program.md` cedes §2 to CIW, and §2 is about
what a run gates, which is arguably exactly this.

## Not verified

Whether other pinned tools are in the same state (the image's `python3`
is 3.11 against a 3.12 pin, which the hook works around by naming
`/usr/bin/python3.12` rather than by installing). A sweep of "what the
image ships vs what `ci.yml` pins" has not been taken and is the thing
that would say whether `ruff` is one instance or a class.

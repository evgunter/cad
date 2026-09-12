---
id: local-half-restates-ci-pins-as-literals
kind: issue
title: the local half restates ci.yml's tool pins as literals in five places and nothing reconciles them
status: closed
opened: 2026-09-04
refs: [nightly-pin-reading-idiom-four-copies, ruff-pin-read-shares-the-first-match-shape]
branch: ciw/pin-reconciler
pr: 2070
closed: 2026-09-07
---

`ci.yml`'s workflow-level `env:` block is this repo's single source of
truth for every pinned tool version. `nightly-pin-reading-idiom-four-copies`
gave the copies that are READ PROGRAMMATICALLY one anchored reader
(`scripts/ci-pin.py`). This item is the other population, which that unit
did not touch and its sweep did not look for: **versions hand-restated as
literals, where nothing compares them to the pin.**

Sites, all confirmed against `origin/main` at `c11f47c5`:

- `local-scripts/ci-local.sh:38-40` — the prereq note: *"cargo-nextest
  (test rows; pinned 0.9.140 to match hosted —
  `cargo install cargo-nextest --locked --version 0.9.140` or the prebuilt
  from `https://get.nexte.st/0.9.140/linux`)"*. Three occurrences of the
  version on three lines.
- `local-scripts/ci-local.sh:588-589` — `nextest_check()`'s failure text,
  which is **executable**, not prose: it tells a developer, at the moment
  the row refuses, to install exactly `0.9.140`.
- `local-scripts/ci-local.sh:594` — *"measured against the pinned
  0.9.140"*, in the `--no-fail-fast` argument.
- `local-scripts/gate.sh:37` — *"sccache v0.16.0 is at
  `~/.local/bin/sccache`"*, guidance agents act on.

**What breaks.** The day `NEXTEST_VERSION` is bumped in `ci.yml`, every
line above still names the old one. The hosted half moves; the local
half — the lane reached for *"before a merge that would be expensive to
get wrong"*, per `docs/prompts/implementer-discipline.md` — goes on
telling a developer to install a version hosted no longer runs, and
`nextest_check()` passes them on a binary that is now the wrong one. It
fails the way this whole class fails: quietly, with no red anywhere, at
the moment someone is trusting the local gate most.

**Not a straight substitution, and that is the interesting part.**
`ci-local.sh:588-589` may want a literal ON PURPOSE — it is a
copy-pasteable command handed to a human whose box has no cargo-nextest,
and a fix that turns it into a `$(scripts/ci-pin.py NEXTEST_VERSION)` is
handing them a command that is one more thing to get wrong, in a script
that at that moment is telling them their tooling is broken. So the
shape a fix wants is probably not "read the pin at every site" but "one
check that reconciles the literals against `ci.yml` and reds when they
drift", leaving the text a human sees as text. That check has a natural
home: `scripts/check-ci-mirror-parity.py` already reads both halves, and
its `MIRROR_EXEMPT` mechanism is already about declared asymmetries
between them.

**Weigh before doing it**: a reconciler needs to know which literals are
pins and which are coincidence — `0.98.4+` on `ci-local.sh:38` is an
admesh floor, not a pin, and sits in the same sentence. An
enumeration-by-hand of the sites is the thing this repo keeps learning
not to write, so derive the population (every `x.y.z` in the local half,
checked against the set of values `ci.yml` pins) or state plainly why
the roster is written out.

**Also in this class, and named so the next sweep does not re-find it:**
`.claude/hooks/session-start.sh:104,123,124` restates `NEXTEST_VERSION`,
`MATURIN_VERSION` and `TY_VERSION` as literals for the agent-container
provisioner. Hosted CI deletes `.claude/` at checkout by design, so it is
outside the parity checker's reach and a fix there is a separate
argument; it drifts the same way.

## How this was found, which is the point

The `nightly-pin-reading-idiom-four-copies` sweep looked for **readers**
— `sed -n 's/^ *NAME:`, `head -1`, `.group(1)`, and every `_VERSION`
reference outside `ci.yml`. It found the readers. It never asked where a
pin's VALUE had been retyped, so a bare `0.9.140` in prose or in an
`echo` — no `NAME:` key, no `_VERSION` token, nothing for either arm to
match — was invisible to it, and all five sites above sat inside files
the sweep had already opened.

That unit's own `MIRROR_EXEMPT` sentence then asserted the local half
"HAS no pin to read", which is false, and the review caught it. A unit
about a pinned version restated where nothing checks it shipped a
sentence restating an unchecked claim about exactly those pins. The
sentence is corrected; this item is the residue it was hiding.

## Disposition (PR 2070)

Closed by a RECONCILER, not by substitution, which is the shape this item
argued for. `scripts/check-ci-mirror-parity.py` gained claim 11: it derives
`ci.yml`'s pins through `scripts/ci-pin.py`'s new `read_pins` (the whole
workflow-level `env:` block, sharing `read_pin`'s anchoring), derives every
`x.y.z` under `local-scripts/` by walking the tree, and reds when a literal
names no pin the block sets — with a second, name-anchored arm for the case a
literal drifts onto a DIFFERENT pin's value. `nextest_check()`'s text stays a
literal a human can paste, as this item asked.

The roster the item warned against is not written: what IS hand-written is
`PIN_FREE`, the declared NON-pins, so a new version literal in that tree is an
error until someone says what it is. Its first entry is the trap this item
named — `0.98.4+` on `ci-local.sh:38` is an admesh floor, declared as one.

The three `.claude/hooks/session-start.sh` lines this item carried are NOT
closed here: hosted CI deletes `.claude/` at checkout, so a claim about them
would pass hosted and red only locally. They are filed on their own, with the
argument a fix there has to settle, as
`work/ciw/session-start-hook-restates-ci-pins`.

### Fix pass (both reviews, 2026-09-06)

Two escapes were found in the reconciler and both were this item's own subject.
`NEXTEST_VERSION=0.16.0` — the pin's own key glued to another pin's value —
passed both arms, as did `# Nextest 0.16.0`; arm B now derives every
underscore-separated part of a pin's key, matches case-insensitively, and
treats `_` as a boundary. The population is `git ls-files -- local-scripts`
rather than a directory walk, so a developer's own `*.local.*` file (which this
repo's `.gitignore` invites) cannot red their gate over a file hosted never
sees. `PIN_FREE` gained an inversion guard and now excuses arm B as well as
arm A, so a declared literal beside a tool name has a declaration path.

The blind-spot list at `PIN_FREE` no longer states a COUNT: a review planted
four shapes it did not contain and one it described wrongly (`v1.2.3-rc1` is
not invisible to the literal regex — the `1.2.3` inside it matches). Three of
the four are now fixed rather than listed.

## Closed 2026-09-07

PR 2070. `scripts/check-ci-mirror-parity.py`'s claim 11 reconciles every
version literal in the tracked files under `local-scripts/` against the pins
`ci.yml` declares today — two arms, one on the value and one on the tool the
line names — and the literals a human reads stay literals, which is what this
item asked for over a substitution. The population is `git ls-files`, not a
roster; `PIN_FREE` is the inversion (it declares the three literals that are
NOT pins), so an undeclared literal is an error and a declaration whose
literal disappears is an error too.

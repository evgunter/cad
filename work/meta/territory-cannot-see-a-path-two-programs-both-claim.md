---
id: territory-cannot-see-a-path-two-programs-both-claim
kind: issue
title: scripts/work.py territory is silent on a path two open programs both claim — it flags only paths the branch's own program does not claim
status: open
opened: 2026-09-04
---


Found by FIX's `transform-rigid-refuses-described-nurbs` lane
(PR 1742) while running the check that was supposed to catch exactly
the collision it missed. Filed here rather than on FIX's or SHELL's
slate: the tool serves every program.

## The measurement

`scripts/work.py territory --base main` reads a branch's prefix and
its diff and names every path **another** program owns. The
implementation asks whether a changed path falls outside the branch's
own program's `paths` globs — so a path the branch's program **does**
claim is never reported, whatever else claims it too.

At that lane's merge base, `crates/topo/src/transform.rs` appeared in
the `paths` list of BOTH `work/fix/program.md` and
`work/shell/program.md`. SHELL opened 2026-09-03, after FIX's charter
had read the file as unowned. The lane ran `territory`, which was
silent on that path — the one fence that mattered — and reported the
others correctly. The collision surfaced only because the orchestrator
noticed SHELL's `paths` while merging main for an unrelated reason.

## Why this is the failure that matters

`work/README.md` says territory "warns; it does not block", and that
is the right posture. But the warning it does not give is precisely
the contested case: a path exactly one program claims is a clean
handoff a lane can announce, while a path two programs claim is a
live conflict neither orchestrator may know about. The check is
strongest on the easy case and blind on the hard one.

Nothing detects the double claim today. Lint enforces that every glob
matches at least one tracked path; it does not ask whether two
programs' globs match the same path.

## The two candidate fixes, not decided here

1. **A lint rule**: two open programs' `paths` globs matching one
   tracked path is an error, or a warning printed by `status`. This
   catches the state at rest, in CI, for every program at once, and it
   fires on the day the second program opens rather than on the day a
   lane happens to run `territory`. It needs a decision about
   deliberate overlaps — whether any exist, and whether `keep_out`
   prose is the sanctioned way to record one.
2. **A `territory` change**: report a changed path claimed by another
   program EVEN IF the branch's own program also claims it, with
   wording that distinguishes "X owns this" from "X also claims this".
   Narrower, and it only warns the lane that happens to touch the path.

They are not exclusive and (1) is the one that would have caught this
instance early. FIX has since dropped `transform.rs` from its glob and
recorded the crossing in `keep_out`, so the instance is closed; the
blind spot is not.

## Home

`work/issues/` — `scripts/work.py` is tracker tooling in no open
program's territory, and the finding is about the tool rather than
about either program that collided in it.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/meta/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Measured tree-wide — CORRECTED (2026-09-05)

**The first version of this section, landed in PR #1899, got two of its
three counts wrong** (17 pairs for 16, "four" recorded overlaps for
three, "eleven" tests-family pairs for nine) and marked DOCM/MSOLVE as
a recorded overlap when it is only recorded on one side. Corrected
below by re-deriving on merged main rather than by patching the
numbers. The mechanised version of this pass is what item (1) builds,
which is the point: a hand-run census miscounts, and this one did.

The instance this issue was filed on is closed; the state at rest is
not. A `fnmatch` pass of every open program's `paths` over
`git ls-files` finds **16 program pairs sharing at least one tracked
path**, none reported by anything today.

**Recorded on BOTH sides — the tracker working as intended (3):**

| pair | paths | where it is written |
|---|---|---|
| `chrome`+`view` | 44 | both `keep_out`s; CHROME lands first, `crates/viewer/src/*` |
| `bool`+`curved` | 37 | both `keep_out`s; the prose fence on `boolean/*`, `splitting/*` |
| `cert`+`props` | 1 | both `keep_out`s; `geom-core/src/k_stats.rs` at S-CERT's exit |

**Recorded on ONE side or neither (13):**

| pair | paths | shape |
|---|---|---|
| `exch`+`tcost` | 177 | `crates/*/tests/*` against a crate owner |
| `mesh`+`tcost` | 56 | same |
| `chrome`+`tcost` | 50 | same |
| `bool`+`tcost` | 42 | same |
| `m10`+`tcost` | 32 | same |
| `lib`+`tcost` | 30 | same |
| `fillet`+`tcost` | 11 | same |
| `seat`+`tcost` | 6 | same |
| `shell`+`tcost` | 3 | same |
| `docm`+`msolve` | 3 | **one side only** — see below |
| `bool`+`fillet` | 2 | `profile/src/{fillet_select,path/arc_fillet}.rs` |
| `cert`+`m10` | 1 | `geom-core/src/dual.rs` |
| `docm`+`lib` | 1 | `docs/RECIPE-DOORS-DESIGN.md` |

**`docm`+`msolve` is the finding the correction turned up.** MSOLVE's
`keep_out` names DOCM at length — "the two globs overlap until DOCM
cedes them or declines to, announced on its orchestrator PR and NOT
assumed" — and **DOCM's `keep_out` does not name MSOLVE at all**. The
overlap on `mate.rs` and `mate/*` is therefore recorded by the program
that arrived second and invisible from the side that was there first,
which is precisely the asymmetry that makes a one-sided record worth
nothing. It is not a defect in either program: MSOLVE opened on
2026-09-04 and did exactly what the rule asks. It is what a lint would
have asked DOCM for on the same day.

**This settles the open decision inside shape (1).** Deliberate
overlaps plainly exist, so a rule erroring on any shared path would be
unusable the day it landed. The rule instead: **an overlap is an error
unless BOTH programs' `keep_out` name the other.** That passes the
three above, and fires on thirteen — nine of them the `*/tests/*`
family, four of them one-sided or unrecorded pairs that nobody has
looked at, DOCM/MSOLVE among them.

The `*/tests/*` family is the one that wants its own answer rather than
nine `keep_out` lines: S-TCOST's territory is every crate's tests by
design, and code-quality Track W already states the seam in prose ("a
track that owns a crate's `src/` does **not** otherwise own its
`tests/`"). Whether the lint learns that seam or nine programs write it
down is the one thing left to decide.

Method, so it is re-derivable and so the next run can be diffed against
this one: `fnmatch.fnmatchcase` of each open `program.md`'s `paths`
globs against `git ls-files`, counting paths matched by more than one
program, then asking for each pair whether each program's `keep_out`
text names the other. Same matcher `work.py` uses. Measured at
`a2bcab785`.

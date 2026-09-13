---
id: apt-update-fails-on-the-runner-image-google-chrome-repo
kind: issue
title: apt-get update fails repo-wide on the runner image's google-chrome list, and it reds four steps in two workflows
status: closed
opened: 2026-09-09
branch: ciw/apt-preamble
pr: 2277
refs: [apt-preamble-bypass-is-unguarded, nightly-rows-cannot-be-dispatched-by-a-lane]
closed: 2026-09-10
---

Found by CIW unit 4's fix pass (PR 2263) when two consecutive runs went
red at steps that unit did not touch, and confirmed **not** to be that
branch's doing.

## What fails, exactly

`apt-get update` on `ubuntu-latest` returns non-zero because a
third-party list **the runner image ships**, not one this repo adds,
serves an index that does not match its own Release file:

```
E: Failed to fetch https://dl.google.com/linux/chrome-stable/deb/dists/stable/main/binary-amd64/Packages.gz  Hash Sum mismatch
   Last modification reported: Wed, 09 Sep 2026 09:41:12 +0000
   Release file created at:    Wed, 09 Sep 2026 17:16:59 +0000
E: Some index files failed to download. They have been ignored, or old ones used instead.
```

Note apt's own second line: the indexes it needs **were** fetched and it
carried on. The failure is the exit status, not a missing package.
Every package these steps install (`xvfb`, `libgl1`, `libglx-mesa0`,
`libgl1-mesa-dri`, `libglu1-mesa`, `mesa-vulkan-drivers`, `vulkan-tools`,
`python3-venv`, `m4`) comes from Ubuntu's own archive. The broken list is
irrelevant to all of them.

## The four sites, and how each one dies

- `.github/workflows/ci.yml:1884` — `install a software Vulkan adapter
  (lavapipe)`, in the `fmt` job. `apt-get update` is its own line under
  `bash -e`, so the step dies there and
  `viewer app-feature rows (chrome + gpu pipeline smoke)` fails after it
  with no adapter.
- `.github/workflows/ci.yml:3777` — `sudo apt-get update -qq && … m4`,
  in `python-suite`. Same shape, `&&` chain.
- `.github/workflows/render.yml:813` and `:1108` — the render lanes'
  `venv deps` and `renderer system deps (software GL + Xvfb + venv)`.
  These already carry a **three-attempt retry loop**, and it does not
  help: the loop re-fetches the same inconsistent index seconds apart,
  and its own error text — *"This is an upstream/mirror problem, not a
  repo one — re-run the lane"* — is right about the cause and expensive
  as advice: two of this lane's runs, twenty minutes apart, hit it
  identically, and a third an hour later did not.

## Not one branch's problem — measured

| run | branch | carries unit 4's row? | apt steps |
| --- | --- | --- | --- |
| 34383262029 | `lib/loops` | no | red |
| 34383996624 | `ciw/unreachable-roots` | yes | red |
| 34385213731 | `lib/loops` | no | red |
| 34386092369 | `ciw/unreachable-roots` | yes | red |

`lib/loops` does not contain the new wasm row at all — its `fmt` job has
no such step — and fails at exactly the same three steps. The last green
run on either branch predates 17:29Z; every run after it is red the same
way. **`main` is being blocked by this, not one PR.**

## It cleared, and that is recorded here rather than quietly dropped

Run `34388201501` at 18:23Z is green: the mirror recovered on its own,
about 55 minutes after the first red. **So a re-run did eventually work,
and the sentence above about the workflow's advice is too strong** — it
is corrected here rather than left standing, because this file's whole
subject is a red run that means something other than what it says.

That does not close the item. The window swallowed **four runs across
two branches**, the in-step retry (three attempts over ~15 s) covered
none of it, and nothing distinguishes "wait 55 minutes" from "this
mirror is broken for the day" while it is happening. An outage that is
indistinguishable from a real red, for an hour, on every branch at once,
is the thing to fix — not the fact that it ended.

## Why this is worth a file rather than a re-run

The retry loop was written for a transient mirror hiccup and this is a
longer one: a hash-sum mismatch on a third-party list persists until the
publisher fixes its own index, which here took the better part of an
hour. What the loop buys is 15 seconds. Three things follow, and the
third is the one that matters:

1. The gate is **red for a reason no diff can fix**, so every lane on
   the repo either waits or learns to read a red run as "probably not
   me" — which is the habit that makes a real red get waved through.
2. The failing list is **not needed by any package any of these steps
   install**. `apt-get update` is being asked to succeed at more than
   the job requires.
3. `ci.yml:1884` and `:3777` have **no retry at all**, so they are
   strictly worse off than the render lanes, which at least warn.

## Shapes, not decided here

The cheap and honest one is to stop asking apt about lists no step
needs — dropping `/etc/apt/sources.list.d/google-chrome*` (an image
artefact) before `apt-get update`, or scoping the update to the Ubuntu
sources. A second is to let `update` fail and let the **install** be the
verdict, since the install is what the step actually needs; that trades
one failure mode for a slower one when a package genuinely is missing.
Either way it belongs in one place: four sites in two workflows spelling
the same preamble four ways is its own smell, and the two `ci.yml` sites
have no retry while the two `render.yml` sites have one.

**Whichever shape wins, it must be verified against a red run**, not
against a green one — a change that makes `apt-get update` pass on a
healthy mirror proves nothing about the case this file is about.

## Not taken by unit 4

That unit is a wasm32 row. This is four steps in two workflows, and its
fix wants a lane that can watch a real failure. Filed rather than
drive-by-fixed, and the unit's own row is green at STEP level in both
red runs above (54.36 s and 54.89 s).


## Disposition (PR 2277)

The first shape, with the hazard closed rather than named. All five apt
call sites in `.github/workflows/` — the four above plus
`nightly.yml`'s `install admesh`, which the sweep found and this file did
not name — go through `scripts/apt-install.sh`. It sets every source
list under `/etc/apt/sources.list.d/` whose URIs are not on an Ubuntu
archive host aside, runs `update` and `install` behind the render lanes'
`timeout` and three-attempt retry, and **restores every file it moved on
any exit**. The scrub therefore lasts one apt transaction, not the job:
a later step that legitimately needs a third-party list finds it where
the image left it. A package only a foreign list carries still fails, and
the error names the lists it set aside so the cause is readable.

**Verified against a constructed red, not a healthy mirror.** The script's
`--selftest` — run by `ci.yml`'s tier-blind `mirror` job on every PR at
every tier — builds two `file://` repositories, one publishing a Release
file whose stated SHA256 for `Packages.gz` is not that file's, and
asserts that an unnarrowed `apt-get update` over the pair exits non-zero
with `Hash Sum mismatch` while the same update through the script exits
0, that the retained repository's package is still visible afterwards,
and that every set-aside file is back. The same condition was planted in
a real `/etc/apt/sources.list.d/` as a `google-chrome.list` beside four
genuine third-party lists: today's preamble exits 100, the script exits
0 and installs.

Hosted run 34417694481 is green, and green at STEP level where it
matters: `apt preamble selftest` succeeded (it RAN — the `mirror` job
carries no `if:`), `install a software Vulkan adapter (lavapipe)` and the
`viewer app-feature rows (chrome + gpu pipeline smoke)` that depends on
its adapter both succeeded, and both render lanes' installs succeeded.
Two steps are NOT covered by it: `oracle-certify`'s `m4 for the gmp
build`, whose job is gated on `run_interval_oracle` and did not run, and
`nightly.yml`'s `install admesh`, which no PR run executes.

**Fix pass (review of PR 2277).** One MAJOR, found by both reviewer lanes
and reproduced: an unguarded `mktemp -d` made the `mv` destination `/$name`,
so an absent `TMPDIR` or a full disk moved the image's source lists to the
filesystem root, restored none of them, and exited 0 while logging "restored
on exit" — the silently-disarmed condition the design exists to prevent,
reached by a path it had not considered. `hold_path` is now the only producer
of a destination and refuses an unusable holding directory, so that case
degrades to NOT NARROWING, loudly. Three more: the narrowed update passes
`APT::Get::List-Cleanup=0`, without which restoring a set-aside list restored
the file and not its cached indexes (measured: docker 2 -> 0 -> 2); a package
apt cannot find fails at once with apt's own status instead of being retried
three times and annotated as a mirror outage; and comments are stripped before
URIs are read. Every caller now `exec`s the script, so a cancel or a
`timeout-minutes` expiry reaches the process holding the restore trap, and
every caller sits at `timeout-minutes: 8` — the bound the script's header
derives, below which the inner `timeout` cannot buy a retry.

**And the same shape survived one more round, in the file's other half.**
The guard went into `set_aside_foreign_sources`; `selftest_main`'s own
`SELFTEST_TMP="$(mktemp -d)"` was left unguarded in the same diff, so an
unusable `TMPDIR` re-rooted every fixture path at `/`. Severity is not the
point — it is the harness, not the transaction — but the CONSEQUENCE was
worse than the litter it looked like: the battery writes a stub `apt-get`
to `$t/bin/apt-get`, which with an empty prefix is `/bin/apt-get`, and a
verification run on the shared box **overwrote the real `apt-get` binary**
(restored by re-extracting it from `apt_2.8.3_amd64.deb`). The lesson is in
the header now, as the invariant rather than as a note about one call: *no
path built from a possibly-empty variable is ever a `mv`, `mkdir` or
redirection target* — an empty prefix does not fail, it silently re-roots
the operation at `/`. The harness has no degraded mode worth having, so it
refuses to run instead.

The selftest grew from five rows to 31, each written against a named mutation:
14 injected failures, 14 killed, including the eleven the review found
surviving the first battery. Run 34423995419 is green with `apt preamble
selftest` and all four executable install rows green at STEP level.

Residue, two files: nothing stops a new step spelling its own preamble
inline again (`work/ciw/apt-preamble-bypass-is-unguarded.md`), and a lane
token gets 403 on `workflow_dispatch`, so the nightly row landed
unverified (`work/ciw/nightly-rows-cannot-be-dispatched-by-a-lane.md`).

## Closed 2026-09-10

PR 2277. One preamble, `scripts/apt-install.sh`, at all five sites (the
sweep found a fifth the item missed: `nightly.yml`'s `install admesh`).
It sets aside every `sources.list.d` entry whose URIs are not on an
Ubuntu archive host, runs `update` + `install`, and restores every file
on any exit — so the narrowing lasts one apt transaction rather than the
job, which is the hazard the shape had to close rather than merely
declare.

The constructed failure is the deliverable as much as the fix: the
mirror had recovered, so the outage was **rebuilt** — a repository whose
`Release` states a SHA256 its `Packages.gz` does not have, reproducing
the same two `E:` lines and exit 100 the runner served. That fixture is
now `--selftest`, sited in `ci.yml`'s tier-blind `mirror` job, so it
re-runs on every PR at every tier instead of being a claim in a merged
PR body. That siting is why a script beat a composite action.

`-o APT::Get::List-Cleanup=0` is passed in production, because without
it the narrowed `update` deletes the cached indexes of every set-aside
repo — measured, docker's candidate falling back to the installed
version — which made "one transaction" false. The selftest asserts the
option two ways rather than setting it.

### What the reviews found, and it was not small

**A MAJOR that was worse than the outage it fixed.** With `mktemp -d`
failing (full disk, unwritable `TMPDIR`) `HELD_DIR` was empty, nothing
aborted — the script has no `set -e` — and the move loop re-rooted every
foreign list at `/`, exit 0, log claiming restoration. Fixed by making
`hold_path` the only producer of a `mv` destination.

**The same bug then survived forty lines away**, in `selftest_main`'s own
`mktemp -d`, and the orchestrator hit it while verifying the fix: the
battery writes its stub `apt-get` to `$t/bin/apt-get`, so an empty `$t`
made that **`/bin/apt-get`** and replaced this box's real binary with a
shell stub. Restored from the `.deb` and verified. **The defect's true
shape was never "litters `/`" — it was "destroys an unrelated
executable"**, and nobody reasoned that far, including at grading.

The header now states the invariant rather than the incident: *no path
built from a possibly-empty variable is ever a `mv`, `mkdir` or
redirection target*. The first guard was written as "guard the mktemp
that bit us", which is exactly why its twin survived in the same diff.

Also fixed: `exec` at all five sites, because the runner signals the step
shell and not the script, so the trap never fired on a cancel; a missing
package no longer retries three times and blames the mirror; a
commented-out foreign URI no longer makes an Ubuntu file foreign; and
`venv deps`' 6-minute bound, which could never admit a retry, so its
inner `timeout` was decoration.

Residues, each its own file: `apt-preamble-bypass-is-unguarded` (nothing
stops a sixth site being written inline) and
`nightly-rows-cannot-be-dispatched-by-a-lane`.

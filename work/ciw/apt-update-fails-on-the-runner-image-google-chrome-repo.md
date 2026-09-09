---
id: apt-update-fails-on-the-runner-image-google-chrome-repo
kind: issue
title: apt-get update fails repo-wide on the runner image's google-chrome list, and it reds four steps in two workflows
status: review
opened: 2026-09-09
branch: ciw/apt-preamble
pr: 2277
refs: [apt-preamble-bypass-is-unguarded, nightly-rows-cannot-be-dispatched-by-a-lane]
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

Residue, two files: nothing stops a new step spelling its own preamble
inline again (`work/ciw/apt-preamble-bypass-is-unguarded.md`), and a lane
token gets 403 on `workflow_dispatch`, so the nightly row landed
unverified (`work/ciw/nightly-rows-cannot-be-dispatched-by-a-lane.md`).

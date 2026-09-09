---
id: apt-update-fails-on-the-runner-image-google-chrome-repo
kind: issue
title: apt-get update fails repo-wide on the runner image's google-chrome list, and it reds four steps in two workflows
status: open
opened: 2026-09-09
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
  repo one — re-run the lane"* — is advice that does not work while the
  index stays inconsistent. Two of this lane's runs, twenty minutes
  apart, hit it identically.

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

## Why this is worth a file rather than a re-run

The retry loop was written for a transient mirror hiccup and this is not
one: a hash-sum mismatch on a third-party list persists until the
publisher fixes its own index, which can be hours. What the loop buys is
15 seconds. Three things follow, and the third is the one that matters:

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


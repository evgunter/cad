---
id: hosted-renderer-announces-itself-preview-only
kind: issue
title: hosted-render-guard — the canonical renderer announces itself as PREVIEW ONLY, do NOT commit what this pass draws
status: open
opened: 2026-08-19
github: 630
refs: [626]
---

## From GitHub issue 630

Opened 2026-08-19; 0 comments.

## The symptom

`ci.yml` and `render.yml` set `CAD_RENDER_LOCAL_OVERRIDE: i-accept-local-render-drift` at their render steps. So every hosted render job — the **canonical** renderer, the one whose frames get committed — prints this into its log:

```
[render.sh] LOCAL RENDER OVERRIDE in effect — this pass is PREVIEW ONLY.
[render.sh]   Frames it publishes carry THIS box's renderer/GL stack.
[render.sh]   The committed tree is refreshed by CI, which re-baselines
[render.sh]   every lane on a push — do NOT commit what this pass draws.
```

Every line of that is false in the hosted case, and the last one instructs the reader against exactly what the job is about to do. It is misleading precisely when someone is reading a render log to diagnose something — which is the only time anyone reads one.

## Why the current design is right, and what is actually wrong

`demos/hosted-render-guard.sh:21-30` is explicit and, I think, correct:

> **THE RULE IS STRUCTURAL, NOT SNIFFED.** CI does not get an exemption for being CI: `render.yml`, `ci.yml` and `local-scripts/ci-local.sh` each set this variable in the file, at the step that renders. There is no `GITHUB_ACTIONS` check here on purpose — a sniffed exemption is invisible at the call site and grows silently (every new runner, every act-like local emulator), whereas an env line in the workflow is reviewable where the render is requested.

That reasoning holds and should not be weakened — sniffing `$GITHUB_ACTIONS` would trade a reviewable declaration for an invisible one. The defect is narrower: **there is one sentence for two genuinely different acceptors**, so the message has to be written for one of them, and it was written for the local developer.

## Proposed (Ev, 2026-08-19)

> *"idk a clean way to check that it's running in hosted CI. i guess we could let the override var also allow the value 'i-am-ci' or something"*

Accept a **second sentence** meaning "this pass IS the canonical renderer", e.g.:

```sh
CAD_RENDER_LOCAL_OVERRIDE_SENTENCE='i-accept-local-render-drift'
CAD_RENDER_HOSTED_SENTENCE='i-am-the-hosted-renderer'
```

`require_hosted_render()` accepts either, and prints the message that matches. The workflows switch to the hosted sentence; `local-scripts/ci-local.sh` keeps the local one, since it genuinely is a local pass. Refusal behaviour for any other value is unchanged.

This keeps every property the header argues for — still a sentence nobody types by accident, still declared in the file at the step that renders, still no environment sniffing, still reviewable where the render is requested. It only stops one declaration from having to serve two meanings.

## Settled: the variable name is the implementer's call

`CAD_RENDER_LOCAL_OVERRIDE` reads oddly once its value can say "I am not local". **Ev, 2026-08-19: *"renaming the misleading local override var is fine but so is leaving it alone."*** So either is sanctioned and this is not worth re-litigating:

- **Rename** — update the three workflow files plus the refusal text at `hosted-render-guard.sh:82` and `:85`. Cleaner, slightly more churn.
- **Leave it** — add a line to the header saying why the name outlived its adjective, so the next reader does not file this again.

Pick one, do it in the same change as the second sentence, and do not spend review time on the choice. This is a log-message bug, not a correctness one.

---

Filed from the SMELL-SCAN wave-1b lane; found while diagnosing a red on #626, not by the scan.

## Home

`work/issues/`: `demos/hosted-render-guard.sh` plus the three workflow files are S-QA's ground and S-QA is closed.

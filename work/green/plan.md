# GREEN — the plan

Main green and the per-PR gate fast. Opened 2026-10-28 by a
priority-seam cut across the infra group (`work/green/log.md`).

## State

No orchestrator holds the track (`status: ready`). Its P0 and P1 rows
are closed:

- **Latency.** PR CI's median is 14.3 min over the 116 runs after
  #6019 and #6039, and the nightly alarm reads 14.7 min. The closure
  tier is at median 14.6 / p90 19.4. The all tier is at 24.1, inside the
  25-min deep-kernel bar.
- **Main reds.** Every red named on this slate through nightly 77 is
  greened: re-blessed with its cause, fixed at its site, or pinned with
  its owner's open row cited. `scripts/gates/pin-cites-an-open-row.sh`
  holds those citations to open rows.

## The slate

| pri | item | cost |
|---|---|---|
| P3 | `nightly-all-features-build-hung-once-with-a-test-binary-running` | E |

## How the track runs when picked up

Each morning, read that night's `triage/nightly-<date>` PR and green
what it left red, under `program.md`'s two rules: a red is greened,
not fixed, here; and GREEN acts on the triage, not beside it. Wait for
the triage table before touching any red.

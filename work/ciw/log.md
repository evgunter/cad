# CIW log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/ciw/plan.md`. A/B band 1500–1599
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CIW section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `main-latently-red-at-tier-all` from `work/issues/`
- `render-lanes-red-at-missing-merge-ref` from `work/issues/`
- `retire-render-automatic-matplotlib-fallback` from `work/issues/`
- `hosted-renderer-announces-itself-preview-only` from `work/issues/`
- `nightly-pin-reading-idiom-four-copies` from `work/issues/`
- `mirror-parity-never-compares-flags` from `work/issues/`
- `python-suite-zero-test-guard-three-copies` from `work/issues/`
- `committed-conflict-markers-reach-main` from `work/issues/`
- `bounds-tripwire-blind-to-named-alias` from `work/issues/`
- `cache-rendered-cells-on-input-hash` from `work/issues/`
- `d107-release-profile-job-lives-in-nightly` from `work/issues/`
- `rustdoc-gate-disagrees-with-workspace-doc` from `work/issues/`
- `rustdoc-gate-private-intra-doc-links` from `work/issues/`
- `doc-gate-two-unread-axes` from `work/issues/`
- `sccache-trial-verdict-to-read` from `work/issues/`
- `geom-brep-test-unused-edgedescription-import` from `work/verbs/`
- `perf-history-cannot-identify-its-host` from `work/perf/`
- `facade-guards-defer-to-rustdoc-json` from `work/lib/`

LIB's clippy-row item (`the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row`)
landed on main the same day and stays closed in `work/lib/`.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## The opening re-read (2026-09-04)

The slate was built on 2026-09-03 out of items filed between 2026-08-09
and 2026-09-03. Before dispatching any of it, the orchestrator checked
all eighteen against the tree rather than against their own bodies. Six
moved. Ev then ruled on three of them in chat the same day.

**The finding that moved most of them.** `evgunter/cad` went **public**
on 2026-09-03 (`5cc16e81`…`483212ef`). Standard-runner minutes are free
and the runner is 4 vCPU / 16 GB, up from 2 / 7. That kills the premise
of `docs/CI-MINUTES-2026-08.md` — *"the Actions allowance was being
consumed faster than the work justified"* — and with it the stated cost
argument behind F3, this month's demotions to the nightly, and at least
two declines on this slate. Ev directed that CIW open the re-costing as
a unit rather than assume the answer:
`work/ciw/f3-recosting-on-a-public-repo`. Same day, before the
visibility change, the account's Actions spending limit denied job
starts for two and a half hours — the old regime's last data point, and
already closed in `work/issues/`.

**What the tree said about the slate.** Three items were closed as not
live. `main-latently-red-at-tier-all` was the plan's FIRST unit and has
nothing to fix: the pyo3 half was repaired at `5859c8c6` (its own
comment said so), and the viewer bin/lib doc collision turns out to be
a **cargo** diagnostic rather than a rustdoc one, so `-D warnings`
never reaches it — `scripts/doc-gate.sh --pr --scope '--workspace'` is
green on this tree and `cargo doc --bins -p viewer --all-features`
exits 0 with the warning printed. `rustdoc-gate-disagrees-with-workspace-doc`
was answered by running both sides: `SweepStrategy::Idealized` is
`#[cfg(feature = "sweep-testing")]`, the gate documents at
`--all-features` and resolves it, a plain `cargo doc` at default
features does not (exit 101 on both prose sites) — a feature selection,
not a misconfiguration. `sccache-trial-verdict-to-read`'s carrier, PR
1648, had merged.

Two more closed on Ev's call (2026-09-04):
`committed-conflict-markers-reach-main`, because a committed marker is
self-limiting — obvious, repairable later, nothing compounds on it, so
it is a poor subject for an absence detector; and
`python-suite-zero-test-guard-three-copies`, never observed and needing
a developer tool's contract plus a parity seam moved. The orchestrator
recorded the counter-evidence on the first before closing it.

Two were re-homed for being outside this program's fence, and one new
item filed to S-TCOST for the same reason:
`bounds-tripwire-blind-to-named-alias` (the tripwire is now
`scripts/gates/bounds-allowlist.sh`, whose ratified header argues
against the ask as KNOWN GAP 3, with a fixture pinning the gate to pass
on exactly those uses), `d107-release-profile-job-lives-in-nightly`
(the whole fix is an edit to a Track P finding), and
`rust-cache-never-restores-across-branches` (PR 1648's finding (d) —
five of seven build jobs restored nothing; caches are S-TCOST's knob).

`cache-rendered-cells-on-input-hash` is parked rather than dropped: its
staleness-window argument never rested on minutes and survives, but PR
1648 measured the Actions cache budget evicting a ~205 MB entry inside
the hour, and a render-cells cache would both miss and crowd out the
build lanes' entries.

**Two items got sharper rather than weaker.**
`nightly-pin-reading-idiom-four-copies` has a confirmed instance now —
`c5263958`, "the gated-suite re-take's pin-read step had unbalanced
quotes and never ran", the same idiom, found by a person reading a log.
`perf-history-cannot-identify-its-host` is now urgent rather than
tidy: the runner class changed on 2026-09-03, so a step change of
unknown size runs through all three histories at that date and the
`environment` block cannot name it.
`geom-brep-test-unused-edgedescription-import` grew from one unused
import to four, in files it does not name (measured, not assumed).

**Filed new, beyond the two above.**
`nightly-demotions-have-never-run`: TCOST-C1/C2/C3 moved three jobs
into `nightly.yml` on 2026-09-03, and none has executed — the last
completed nightly (run 33741400551) predates all three merges and its
job list does not contain them; the only run since was a cancelled
dispatch. `c5263958` is the class already firing once. Ev, 2026-09-04:
read tonight's scheduled run rather than forcing a dispatch.

Eighteen items to ten units plus one unscheduled reading. No branch
exists yet and no unit is cut. The first dispatch claims its ordinal
from the band and records it in `docs/MODEL-AB-LOG.md` — though on Ev's
direction (2026-09-04) this program runs **no A/B protocol at all**:
one subagent style review per unit, and a second reviewer for
correctness only where a unit earns it, named in its PR with the
reason.

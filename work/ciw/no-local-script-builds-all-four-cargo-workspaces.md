---
id: no-local-script-builds-all-four-cargo-workspaces
kind: issue
title: The repo has four Cargo workspaces plus tools/tess-meter and no local script builds them all, so a signature change sweeps crates/ and reaches hosted CI red from demos/
status: open
opened: 2026-09-06
---


Reported by MSOLVE-3's implementer lane (PR 2081), outside its fence;
filed by the MSOLVE orchestrator. CIW's by shape.

The root workspace, `benches/`, `demos/tour/` and `demos/wild/` are
four Cargo workspaces, and `tools/tess-meter` a fifth root; hosted CI
builds them all, and no local script does. A lane changing a public
signature (MSOLVE-3's `Frame::rotate_then_translate` gaining a band
and a `Result`) swept `crates/`, went green locally, and reached hosted
CI red on `demos/tour/src/assembly.rs`. `local-scripts/ci-local.sh`
is the natural home for a "every root" build, or a `--all-roots`
mode on `test-fast.sh`; until then every signature sweep is a grep
over `crates/ benches/ demos/ tools/`, and the discipline doc could
say so.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **E** — loop `cargo` over five roots in an
existing script plus a discipline sentence; fix is stated. The class is
a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Re-homed to CIW (2026-09-12, by the CITE orchestrator)

**CIW's by territory and by the row's own words.** The fix lands in
`local-scripts/ci-local.sh` or a `--all-roots` mode on
`test-fast.sh`, and `local-scripts/*` is CIW's `paths`. CITE's
`keep_out` already said so — *the two local-script rows are announced
there and CIW may simply take them* — and this is that announcement made
as a move, since CITE never started the row and holds no work on it.

The other of the two, `build-slot-banner-leaks-the-holders-command-line`,
was closed by CITE rather than moved: Ev asked for the script fix
directly (2026-09-11, *"no memory, just script fix"*) and it is landed
and verified. So `local-scripts/with-build-slot.sh` changed under CIW's
fence with CIW's knowledge, which is recorded here because the next CIW
lane to open that file should know why its banner is one field shorter.

One thing worth re-deriving before staffing this: the row says four
workspaces plus `tools/tess-meter`, and `docs/prompts/implementer-discipline.md`
§2 warns in terms that apply to the row itself — *"`--workspace` is not
every cargo root… **do not carry a count in your head**, this bullet's
included: `scripts/doc-gate.sh --print-roots` derives the list, and a
root has landed before with every prose count in the repo left saying
the old number."* Take the list from the script, not from this row.

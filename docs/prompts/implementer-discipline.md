# Implementer discipline — standing lane obligations

**Read this in full before you start.** It is binding on every implementer
lane, alongside the unit's own spec or brief.

---

## 1. Output discipline

≤~150 lines per tool call. Chunked reads. Skeleton-first writes, then fill.
Final report ≤150 lines.

## 2. Verification

**Hosted CI is the verification of record.** Push and let it run. It runs on
hardware not shared with any other lane and its result is a durable artifact.

**The per-PR gate is sized for latency, and the nightly holds the rest.**
`.github/workflows/ci.yml` runs the default-eps suite minus the slow set (the
`ci` profile in `.config/nextest.toml`), the slow tests of the crates your diff
touches, the 1e-6 and 1e-12 rows of the eps-sensitive crates your diff touches,
lint (fmt, clippy `--all-features`, `scripts/gates/`), and the rows whose own
subject you touched (viewer, python, demos, tools, mesh budget, topo release,
step import, interval). `.github/workflows/nightly.yml` runs everything else:
the whole suite at every eps row, the k-lint rows, the render lanes, rustdoc,
wasm32. The `change filter` job's log prints what this run selected, and
`gate ok` is the one check to read.

- **A green PR is not a green nightly.** If your change could plausibly move
  a slow test, another eps row, or a demo, run that locally
  (`cargo nextest run -p <crate>` runs the slow set too;
  `CAD_TOLERANCE_EPS=1e-12` for a row) — or accept that the nightly may name
  you. **A red nightly is a red main**: whoever reads it first fixes it or
  files it.
- **A test that costs ≥ 1 s goes in the slow set** unless it has caught
  something the fast set would miss. Add it to `.config/nextest.toml`'s `ci`
  filter in the PR that adds the test.

**Draft PRs do not run the gate at all.** Mark the PR ready for review when you
want it gated; undrafting triggers a full run on the same head.

**Run builds or tests locally only when it is genuinely faster for
development**: a tight edit-compile loop on one failing test, reproducing a
specific failure before you can fix it, or a case where a CI round trip would
cost more than the fix itself. That is an iteration tool, not verification — it
does not replace the CI result and it is not what you report green on. If CI
cannot run at all, say so explicitly rather than substituting a local run
silently.

When you do run locally:

- **Prefer foreground, one at a time**, reading each result before the next.
  Backgrounding a build or test is not forbidden, but treat it as risky:
  harness bugs mean the completion notification often does not arrive, so a
  backgrounded row can finish with nothing waking you.
- **Never end your turn with background work still active.** That is the case
  where a lost notification costs you everything — nobody is waiting, nothing
  wakes, and the lane stalls completely rather than failing visibly. Finish or
  abandon the background row first.
  **A hosted CI wait is the same case, not an exception** — three lanes in one
  day parked on "the CI watcher will wake me" and had to be nudged awake.
  "Push and let CI run" means CI is the verification of record, not that you
  may sleep on it: poll the run's jobs API in the foreground (an until-loop
  inside one call) until it concludes, then report in the same turn.
- When the build queue is busy, a blocking foreground wait is the correct state
  — re-issue a timed-out call rather than parking.
- **Use your own `CARGO_TARGET_DIR`, never one shared with another lane.** A
  shared target directory clobbers across git worktrees and will serve you
  another lane's binary — observed twice in one wave, once reporting a test
  count from sources that were not yours, once behind a green claim over ten
  broken assertions. Confirm a `Compiling <crate>` line appears before trusting
  any run.
- **Keep that target directory OUTSIDE the worktree** (`/home/user/<lane>-target`,
  not `.lane-target/` inside the checkout): the repo's `.gitignore` covers
  `/target` and a few named roots, not an arbitrary in-tree name, and a lane
  that `git add -A`'d its build directory pushed 114 files of incremental
  artefacts into its branch history — unfixable under merge-only rules except
  by abandoning the branch and re-landing the diff (CERT-M2, 2026-09-02). Read
  `git status` before every `git add`; never add with `-A` unattended.
- **Never kill processes by pattern on a shared box.** `pkill -f cargo` or
  `pkill -f nextest` matches every lane's build, not yours. Kill only a PID
  you have attributed to yourself — read `/proc/<pid>/environ` for your own
  `CARGO_TARGET_DIR`, which is the other reason that directory has to be
  yours alone.
- **Prefix every file you write to a shared scratchpad with your lane's name.**
- **`--workspace` is not every cargo root, and the roots outside it are
  not covered uniformly.** `Cargo.toml` `exclude`s `benches`, `demos`,
  `tools` and `interval-transcendentals`, so
  `cargo clippy --workspace --all-targets` — the natural check before a
  push — compiles nothing under them. **Do not carry a count in your
  head**, this bullet's included: `scripts/doc-gate.sh --print-roots`
  derives the list, and a root has landed before with every prose count
  in the repo left saying the old number. Two of those roots,
  `demos/tour` and `demos/wild`, are ordinary consumers of the public
  API, so a signature change breaks them the way it breaks a user: two
  lanes in one hour changed a return type, re-spelled every caller
  `--workspace` could see, and learned from CI that `demos/tour/tests/`
  was still red. The PR gate lints and tests those two and the `tools/*` roots only when
  the diff touches them; the nightly takes them every night. So when you
  change a public signature, run
  `(cd demos/tour && cargo clippy --all-targets -- -D warnings)` and the
  same in `demos/wild` before you push.
- **A build is not a test.** `cargo build` cannot see a broken
  `assert!(msg.contains(…))`. A lane that rewrote text asserted anywhere and ran
  only builds has verified nothing about it.

**Write assertions a bug could break.** Name the runtime value that would make
one false; where there is none — a predicate over things fixed at compile time,
or one its neighbours already subsume — it is documentation, and deleting it is
the repair.

## 3. Baselines, demos, and the status quo

**No baseline is a target to preserve.** A lint threshold, a committed render, a
golden file, a test expectation — each exists to report what the kernel actually
does. When one moves, the only question is whether the new behaviour is
correct. "How do I get the old number back" is never the question, and a change
whose justification is that output stayed identical has not been justified at
all (`memories/output-stability-as-justification.md`).

**k-lint** (nightly). If it fires, do **not** change geometry to silence it. A fired
lint is distribution evidence: re-derive the baseline per the K-REPORT runbook,
or escalate to the orchestrator.

**Demos.** The tour and the wild corpus render what the kernel produces through
the public API, from an outside consumer's seat — they are evidence, not
decoration. **Write them the way a real user would**: the natural spelling of
the task through the public doors, to the greatest extent possible. A demo that
reaches past the API, hand-builds what a door should produce, or leans on a
private path stops being evidence about the library and becomes evidence about
itself — and it stops showing the friction a user would actually hit. A frame
that changed is telling you the kernel changed. Never adjust
a scene, tolerance, or camera to restore a frame. Decide whether the new output
is right: if it is wrong, fix the kernel; if it is right, re-baseline and say in
the PR what moved and why.

## 4. Comment style

Comments state the **invariant**, not the history. No retired-type archaeology,
no unit tags, no milestone or PR archaeology. An argument about how the code
used to work belongs in the PR description, which is where this repo documents
the logic of a change.

## 5. Sweeps

If your unit fixes an instance of a class, say what pattern you swept with and
**what that pattern could not match** — then try to check that blind spot,
with a second pass shaped at the gap you just named, and report that one too.
A sweep whose blind spot is unstated is an unverified claim, not a negative
result; one stated and never looked into is where the next instance tends to
be. Where a gap genuinely cannot be searched, say that and say why. Note also
that a sweep is accurate as of your merge base, not your merge: a long-running
lane owes a re-sweep before it lands.

**Assume it is a class.** The trigger above is your own judgement that the
defect has siblings, and that judgement is where this rule misses. Before you
write the scope sentence, grep for the **shape** — not the symbol — and put
**the hit list and its disposition** in the PR description, one line per hit:
fixed, or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt.

Scope sentences read as completeness even when the claim above them does not
share their scope. One euler-operator header asserts the universal — *"a
mutation phase announces a failed lookup rather than discarding it, at every
write"* — while its evidence is *"these modules"*; the same diff left three
silent discards in a sibling file, ten lines below two `unreachable!`
conversions it had just added.

## 6. Filing what you find outside your fence

A sweep that works turns up defects that are not yours. **File them, on the
slate of the program whose ground they land on, in the same PR that found
them** — no permission, no routing through anyone. `work/README.md` settles
it: *"a finding goes straight onto the slate of the program whose ground it
lands on"* (`:117`), and *"a lane does not need the owner's permission to put
a finding where it belongs"* (`:146`). `work/issues/` is the last resort it
has always been, for a finding with no obvious owner.

`python3 scripts/work.py territory --files -` says who owns a path. Grep that
program's directory first: if a row already covers your finding, add your
evidence to it rather than opening a second — one file per item, so a
duplicate costs someone a merge conflict.

**Filing is not optional, and a PR body is not a slate.** *"Disclosing a
residue is therefore not scheduling it — give it its own file at the moment
you disclose it"* holds on every slate, not just your own. A finding left in
a PR body is gone once the owning program closes and its directory is
deleted.

Say in your report which rows you filed and where.

## 7. Citations

**Cite by name; line numbers rot.** A number may ride along beside the
name and is allowed to go stale; a bare `file.rs:NNN` is not a citation.

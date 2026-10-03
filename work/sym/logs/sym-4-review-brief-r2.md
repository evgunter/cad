# SYM-4 review brief — r2 (the cost of a form)

You are reviewer **r2** of the v6 dual for unit SYM-4 of program
SYM (the E12 symbolic identity tier, `geom_core::sym`), PR #2565 on
`evgunter/cad`, **frozen head `1c98847fbd7feefd6414397fd598444ab85011e6`**. Your review is one of two
run CONCURRENTLY on this head by two reviewers who must not see each
other's work.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`
(binding; your report names which of its questions you exercised and
carries `sure`/`likely`/`unsure` on every finding); `docs/SYM-4-SPEC.md`
(the binding spec); the item
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive.md` (its
`## The profile (SYM-1)` is the before, `## The change (SYM-4)` the
after); the PR body (`mcp__github__pull_request_read` `get`; load
with ToolSearch `select:mcp__github__pull_request_read,mcp__github__actions_list,mcp__github__get_job_logs`);
`crates/geom-core/src/sym/form.rs` and `sym/rational.rs` whole (the
diff and the files); the walk-ledger digest row the unit added;
`sym.rs`'s `# Node ids are CONTENT HASHES (D9)`, `# Freezing`, `# Cost`;
`memories/output-stability-as-justification.md`,
`memories/perf-measurement-lane.md`,
`memories/review-and-dependency-policy.md`.

## Your lane (v6 item 5 isolation — READ side)

- Worktree `/home/user/lanes/sym-4-r2`, checked out at the frozen
  head on branch `sym/4-review-r2` for any probe rows. Every
  command: `cd /home/user/lanes/sym-4-r2 && …` with
  **`CARGO_TARGET_DIR=/home/user/sym-4-r2-target` on the same
  line**. Wait until `/home/user/sym-4-r2-target/SEEDED` exists
  before any cargo command (foreground `ls` poll). Private scratch
  `/home/user/sym-4-r2-tmp/`.
- **Until your report is delivered you must not fetch, check out, or
  read the other reviewer's branch (`sym/4-review-*` other than yours),
  worktree, target, scratch or CI artifacts, nor the implementer's
  worktree (`/home/user/lanes/sym-4`) or scratch.** The PR, the
  branch `sym/4-form-cost` and your own tree are what you read. Any
  accidental glimpse is DISCLOSED in your report.
- Pushing early and often is mandatory (a remote container can be
  reclaimed); push your probe branch as you go.
- Foreground rule: no background waiters for your own builds; anything
  over 600 s runs `setsid`-detached with a recorded PID and is polled
  in the foreground; never end your turn with a detached job running.
  Four cores shared with two other lanes: one crate at a time,
  `--test-threads=1` for anything you time, callgrind one replay per
  run.
- Hosted run **34825535838** on the frozen head: read its job list yourself
  (`actions_list`, `list_workflow_jobs`) — twelve `test (…)`, five
  `k-lint (gate, …)`, on THIS head SHA — before conditioning anything
  on it.

## Reviews here include end-to-end exercise

Reading the diff is not enough: build and run the tier through its
public doors on the documents (the M10-3 slab, the plate at the
nominal, and at least one document the unit did not measure — the
annulus, the bracket, the pad, the link via the M10-10 evidence
fixtures) and report what the exercise showed. Local runs are unique
signal only: your own probes, mutants, callgrind re-takes,
merge-base differentials; the existing suites ride the hosted gate
and are not re-run here.

## The claims to falsify (the dispatcher's hypotheses, not findings)

1. **Every decision is bit-identical.** `Poly`'s `BTreeMap<Mono, Rat>`
   became a sorted `Vec<(Mono, Rat)>` in the map's iteration order
   (`Mono`'s `Ord`), so every `digest` — hence every atom key, every
   freeze, every discharge — is unchanged. Held by the M10-8/9/10 pins,
   the tier-off byte rows, the accounting goldens and the new
   walk-ledger digest row. Your instruments: a merge-base differential
   (build the merge base in a second target if you can afford it, or
   read the digest row's captured corpus and its capture method) and
   your own MUTANTS — swap two terms' order in `insert`, or break the
   merge's tie case in `add`, and show the digest row red; if it does
   not red, that is a MAJOR. Read `insert`, `add`, `neg`, `mul`,
   `as_constant`, `degree` for a case the vector handles differently
   from the map: an equal monomial arriving twice, a zero coefficient
   sum, an empty polynomial, the constant term's position.
2. **The gcd skip is sound on every `Rat` shape.** `from_parts` skips
   the gcd when `den.is_one()`; a non-dyadic `den` still reduces; the
   canonical form (odd `num`, `exp2` carrying the twos) is unchanged;
   `add`'s alignment and `mul`'s "products by one" shortcut give
   identical `Rat`s. Write the row that would catch a non-canonical
   `Rat` escaping (two equal rationals with different representations
   hashing differently — that breaks D9 and the atom keys) and run it.
3. **The counts are identical and the shares moved as claimed.** Every
   count in the profile (forms, atoms, frozen by cause, decisions by
   outcome, the ring's promotions) identical before → after on the
   slab and the plate; the storage class's instruction share on the
   slab down by what the PR says (before 57 %), the ring's `from_parts`
   share on the plate down. Re-take the callgrind rows on your box
   (instruction counts are contention-proof) and read the same order
   and shares within a few points; a class misfiled between "storage"
   and "the walk" moves the ranking and is a finding.
4. **The hosted before/after is what the PR says**: the editor-core
   interval shards' `run archived tests` wall on the PR run against
   the nearest base PR run (no test matrix runs on `main` push), the
   chamber row's cpu-s; and the local chamber drive wall. Resolve
   every run id the PR cites — status AND head SHA — before quoting
   it.
5. **Nothing moved that the spec fenced**: no dial, budget, rule,
   `COEFF_BITS`, driver or `real.rs`; a term bound that became exact
   where it was an upper bound would move `frozen` and is out.
6. **The record**: `# Cost` re-stated with the new shares and its
   instrument named; `form.rs`'s header says what a `Poly` is now and
   why the order is the map's; the item's ask 3 answered for the two
   levers with what remains stated by number.

## Style lane emphasis

Q1: does the sorted-vector polynomial mint a second small-map
abstraction beside `IdMap`/the memo maps (the spec's sweep obligation
names them as the walk's, not this unit's — but say what you see)?
Q3: can the digest row fail on anything but a representation change
(a captured corpus that is re-captured at head passes trivially)?
Q6: every disclosed deviation — is it an improvement or does it owe a
schedule? Q7: the shape of `mul`'s accumulation (binary-search insert
vs sort-and-merge) — is it what you would have done, and does the PR
body's measurement justify it? Q8: read `form.rs` whole.

## The dispatcher's own exposure

Everything above is the dispatcher's belief about the tree; check it
before building on it, and report any correction as a finding.
Reviewers correcting the dispatcher is the lane working.

## Report (≤150 lines, to the orchestrator; do not post on the PR)

Verdict (`MERGEABLE` / `MERGEABLE-AFTER-FIXES` / `NOT MERGEABLE`) with
MAJOR/MINOR/NOTE findings on the claims, each with `file:line` and a
confidence, and for each finding whether it was DEMONSTRATED BY
EXECUTION (a red probe, a mutant gone red, a measured wrong value) or
by inspection; a `## Style` section per the style lane; the CODE QUALITY REPORT
with the fixed rubric — counts of MAJOR / MINOR / NOTE findings; spec
deviations counted as reported vs SILENT (a silent one is a deviation
the PR body does not disclose; count them separately); and three
ratings 1–5 with one line of evidence each: idiom/structure, test
quality (do the tests pin the real contract?), doc/comment honesty; the
questions exercised; the gate verification (run id, head SHA, job
census); what you ran locally with its numbers beside the PR's; the
e2e exercise and what it showed about scope and ergonomics; any
glimpse of another lane's material (disclose). Push your probe branch
if you wrote rows and name the commit.

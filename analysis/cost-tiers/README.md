# Cost tiers: how much work E, M and H items take

A one-off measurement of the tracker's cost tiers (`work/README.md`,
Vocabularies → cost; Track size), taken 2026-10-27 over main at
`97ce6f9562`. Charts: https://claude.ai/artifact/EeczR6uwkLdu2yMyGM4Pjx.

## Method

- **PRs**: every first-parent "Merge pull request #N from evgunter/<branch>"
  on main since 2026-09-20 (2662 PRs). Per PR: non-merge commits in
  `m^1..m^2`, `git diff --numstat m^1 m` split into code (`.rs .py .pyi .sh
  .yml .toml`), prose (`.md` outside `work/`), work files (`work/`, without
  `STATUS.md`) and data (goldens, fixtures, svg, csv, ...); created/merged
  times from the GitHub REST PR list.
- **Item closures**: an item file whose header goes to `status: closed` in a
  PR's diff (2427 closures, 2173 priced). Moved items are matched by id.
- **Join** item → implementing PR: the `pr:` header (1166), then `branch:`
  (181), then the closing PR when it is not a `claude/*` tracker branch
  (630). "Worked" items are those whose implementing PR changes code; closures
  as moot, duplicate or fixed elsewhere mostly drop out there.
- **Bundling**: `bundle_size` is the number of closed priced items that share
  an implementing PR. The primary sample is items alone in their PR, opened on
  or after 2026-09-27 (E 141, M 361, H 67). `summary.txt` also has all solo
  items, D included, and all post-redefinition items with PR metrics divided
  equally across the bundle.
- **Active time**: the sum of gaps of at most T minutes between the branch's
  commits, the PR's opening and its merge; a longer gap is a pause.
  T = 30, 60, 90, 120, 240. Work before the first commit, review lanes on
  other branches and the orchestrator's own time are not visible.
- **Follow-ups**: `followup_points` sums the tracker weights of items whose
  `parent:` or `refs:` names the item.
- **Review**: log entries (`work/*/log.md`, last version of each, deleted
  programs included) that cite exactly one PR are scanned for review verdicts,
  fix passes and MAJOR/MINOR tokens; `docs/dual-review-log/rows` give finding
  counts for dual-reviewed PRs. Coverage is thin (most PRs match no
  attributable entry), so these columns are indicative only.

## Results (solo items, opened on or after 2026-09-27)

| proxy | E median | M median | H median | M:E | H:E |
|---|---|---|---|---|---|
| active min, T=90 | 63 | 108 | 190 | 1.7 | 3.0 |
| active min, commits only, T=90 | 34 | 83 | 172 | 2.5 | 5.1 |
| commits | 3 | 6 | 9 | 2.0 | 3.0 |
| code lines changed | 247 | 762 | 1786 | 3.1 | 7.2 |
| files changed | 9 | 15 | 26 | 1.7 | 2.9 |
| PR open to merge, h | 1.4 | 3.9 | 7.0 | 2.8 | 4.9 |
| follow-up points (mean) | 0.46 | 1.33 | 5.22 | 2.9 | 11.3 |

H:E of median active time by pause threshold: T30 2.2, T60 2.9, T90 3.0,
T120 2.8, T240 4.0.

PRs carrying only E items: active ≈ 60 min + 19 min per item (least
squares, 187 PRs). 65% of post-redefinition E items share their PR with
other items. Net of the 60-minute fixed cost, a solo H is about 6.8
marginal E and a solo M about 2.5.

19% of H items (13 of 67) leave 10 or more points of follow-up rows;
no E item does.

## Reading

- Per item, H is about 3× E in time and about 7× in code, not 10×.
  A suggested set of weights is E 1, M 3, H 8.
- The tail of H is follow-up work, so a free numeric `points:` override (with
  a reason) suits a row that seeds a program better than a fourth letter.
- A solo E is mostly the fixed cost of a dispatch. A chore tier near zero
  (0.25 to 0.5), dispatched only in bundles, matches how E rows already
  ship.

## Confounds

Labels are the cost at close and may be mis-priced; a re-priced row is
flagged in `items.csv` (`repriced`). Bundled PRs carry mixed tiers, which
is why the primary sample is solo items. Active time misses reading time
before the first commit, which weighs most on E. Designer passes for
`design: true` rows land in their own PRs and are not added to the row.

## Files

- `collect.py` writes `prs.json` (git only). `sessions.py` counts
  `Claude-Session` trailers per PR (almost always one, so unused).
- GitHub PR pages were fetched with
  `gh api "repos/evgunter/cad/pulls?state=closed&per_page=100&page=$i&sort=created&direction=desc"`
  into `ghp_$i.json`; program logs were recovered with `git show` into `logs/`.
- `analyze.py` writes `items.csv` and `prs.csv`; `summarize.py` prints
  `summary.txt`; `export_dash.py` writes the charts' dataset.
- `items.csv` is the per-item table (2173 closed priced items, 1977 joined to
  a PR).

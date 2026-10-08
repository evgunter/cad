IN PROGRESS

# Review — PR #4317 at frozen head 1deb0892

Interim. Done so far:
- Case table (claim 1) checked exhaustively in a model: 0 arm changes for every
  arc with ends in different entries.
- `arc_holders` (claim 3, walk positions) and `holds_whole` (claim 3, cuts with
  ties): old and new readings agree in a model.
- `pinch_runs_battery` and `four_pairs_battery` (not run by the PR): 0 lines moved.

Still running: the remaining batteries and the mutants.

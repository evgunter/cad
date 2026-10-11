---
# The full readouts that close a counting window of the dual-review log's
# foot (docs/DUAL-REVIEW-PROTOCOL.md rule 9). A full readout happens only
# when Ev requests one; once Ev has ruled on it, its line is added here.
# `triggers` lists each full readout's triggering DR number (the last row it
# read), oldest first; `first` is the number of the first one listed
# (readouts 1 and 2 predate rule 1's arms and close neither window). A
# trigger in `units_restart` restarts the count of M-tier units; one in
# `misses_restart` restarts the M-tier misses. Readout 3 (DR-104, PR 4283)
# restarted the misses only: Ev ruled the next full readout "at 20 (as
# now) or 2 more m tier misses".
first: 3
triggers: [104, 144, 165]
units_restart: [144, 165]
misses_restart: [104, 144, 165]
---

The full readouts that close a window of the dual-review log's foot. The
foot itself is computed by `scripts/dual-review-log.py`.

---
# The full readouts that close a counting window of the dual-review log's
# foot (docs/DUAL-REVIEW-PROTOCOL.md rule 9). `triggers` lists each full
# readout's triggering DR number, oldest first; `first` is the number of the
# first one listed (readouts 1 and 2 predate rule 1's arms and close
# neither window). A trigger in `units_restart` restarts the count of
# M-tier units toward twenty; one in `misses_restart` restarts the M-tier
# misses toward two. Readout 3 (DR-104, PR 4283) restarted the misses
# only: Ev ruled the next full readout "at 20 (as now) or 2 more m tier
# misses". The orchestrator recording Ev's ruling on a full readout adds it.
first: 3
triggers: [104, 144]
units_restart: [144]
misses_restart: [104, 144]
---

The full readouts that close a window of the dual-review log's foot. The
foot itself is computed by `scripts/dual-review-log.py`.

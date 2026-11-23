# The verdict-read lane's gate harness (`embed/verdict-read-measure`, not for merge)

`embed/verdict-read` plus one log-and-pass hook in `AtRestBody::validate`:
with `CAD_GATE_LOG=<dir>` set, every finish asks `topo::clearance::embedded`
of each solid on the facts the finish just kept, at the gate's budgets
(depth 40, 65 536 pairs), and appends `GATE <test> <scalar> <verdict> <seconds>`
to `<dir>/<pid>.log`. It never refuses.

The "before" reading is the same hook on `embed/remeter`'s head, asking
`embedded(&Side { body, faces }, ..)`.

Run: `CAD_GATE_LOG=/tmp/gate cargo nextest run -p sweep --features test-support --no-fail-fast -E '<tests>'`.

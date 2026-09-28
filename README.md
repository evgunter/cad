# DR-4 and DR-6: the pair artifacts

The inputs and outputs behind rows DR-4 (ATREST-3, PR #3191) and DR-6
(ATREST-9, PR #3204) of `docs/DUAL-REVIEW-LOG.md` on main: the brief
template, each lane's brief, each review's report, the coder's
attribution-stripped inputs (`coder-A.md`, `coder-B.md`), the
/dev/urandom coding byte, and the orchestrator's working notes
(`meta.md`, and for DR-6 the drafted row).

Kept here, not on main, per `docs/DUAL-REVIEW-PROTOCOL.md` item 10:
analysis material lives under `analysis/dual-review/`, and an
orchestrator with a dual in flight should not read it.

**The hashes.** `sha256-as-dispatched.txt` holds the sha256 of each
file as it was handed to the lanes. That is the record the log's
"identical briefs … (sha256 stored)" rests on. Before publishing, the
local home directory in paths was replaced by `~`, which changes the
bytes. `sha256-as-published.txt` hashes the files as they are here. To
compare the two briefs of a pair, diff them: they differ only in the
lane paths.

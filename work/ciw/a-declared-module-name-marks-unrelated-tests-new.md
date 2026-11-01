---
id: a-declared-module-name-marks-unrelated-tests-new
kind: issue
title: Declaring any module named like one on a test's path marks that test new in the slow-set and fast-set-time tooling
status: open
opened: 2026-10-19
---


Found while landing PR 5401 (`encl/kernels-own-type`).

`added_tokens` (`scripts/check-fast-set-times.py` ~101) emits a bare
`pkg:mod:<name>` for every module an added line declares, with no path.
`is_new` (`scripts/slow_set.py` ~129) counts a test as new when **any**
module on its path is in that set. So declaring `mod census` or
`mod tests` anywhere in a package marks every test of that package under
any module of that name as new. That puts unrelated slow tests in the
PR's run, and holds unrelated fast tests to the 0.5 s new-test bar.

PR 5401 declared `mod census` in `crates/topo/src/policy_lane.rs`, which
matched other `census` modules in `topo`, and renamed it to
`origin_census` to work around this.

The leaf rule (`{pkg}:{leaf}`) has the same weakness: an added identifier
anywhere in the package that equals a test's leaf name marks that test as
new.

**Repair shape.** Key the module token by its path: the file's module path
plus the declared name. Then `is_new` matches it only as a prefix of the
test's own path.

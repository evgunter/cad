---
id: gated-marker-omits-sibling-helper-imports
kind: issue
title: nothing checks a marker's path set against its own suite's helper imports
status: closed
opened: 2026-09-03
closed: 2026-09-11
---


`scripts/gates/gated-suite-paths.sh` (via `ci-filter.py --gated-check`)
proves every path a marker NAMES exists. Nothing proves the converse —
that everything the suite DEPENDS on is named — and there is one
dependency the tree makes mechanically checkable: the suite's own
sibling helper module.

**The hole.** A marker's own file is an implicit member of its path set
(`crates/test-utils/src/lib.rs`, `gated_to!`'s docs). A sibling helper
module is NOT, and a suite that writes `use crate::common;` takes its
fixtures, its bodies and often its tolerance from that directory. A pull
request editing `crates/profile/tests/common/mod.rs` seeds the `profile`
package — so the crate's tests are in scope — and then the filter SKIPS
every gated suite whose set omits `crates/profile/tests/common/`, on the
one diff most likely to have broken them. Nothing reds; the notice line
reads exactly like a correct skip.

**The census.** TCOST-9 swept all 54 markers for a `use crate::<h>` /
`use super::<h>` whose module directory was absent from the marker's set,
and found **ten**: `editor-core`'s `gui1_pick_r2`, `m10_1_r2_probes`,
`m10_3_r1_probes_interval`, `m4_pr6_floats` and `review_gui1_r1`
(`tests/fixture/`), `r1_m10_1_probes` (`tests/corpus/`),
`mesh8r2_probes` (`tests/common/`), and `profile`'s
`canonical_invariance`, `path_property` and `review_s2`
(`tests/common/`). Seven of the ten were TCOST-1's, three were TCOST-9's
own — which is the point: **every author made the same omission, twice
over, under a review whose stated bar was the path set.** All ten are
widened in TCOST-9's PR; the class is what remains.

**The fix this wants.** The check is mechanical and needs no toolchain,
which is why it belongs beside the one that is already there. For a
marked `crates/<c>/tests/<suite>.rs`, read the head of every
`use crate::<h>` / `use super::<h>`, resolve `<h>` through that crate's
`tests/all.rs` `#[path]`/`mod` pairs the way `_all_rs_modules` already
does, and require the resolved file — or its directory — to be in the
marker's set. `--gated-check` owns the marker vocabulary already, so the
new arm is a few lines there and one more assertion in
`gated-suite-paths.sh`'s selftest (plant a suite importing a helper the
marker does not name; the gate must red).

**What the census pattern cannot match**, stated so the negative result
is honest: a helper reached through a re-export rather than a `use` of
the module head; a `#[path]`-mounted helper whose `mod` name differs
from its directory and which `tests/all.rs` does not declare; a fixture
loaded at run time from a data file rather than imported; and the whole
`src/` marker shape, where `use crate::<m>` names a crate SOURCE module
and the question is the ordinary path-set judgement rather than this
one. The sweep was over the 47 `tests/` markers; the 7 `src/` markers
were checked by hand and each already names the modules it imports.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Closed (2026-09-11): the arm is in `--gated-check`, and it found three more

Fixed as the item asked: `--gated-check` now reads every marked
`crates/<c>/tests/<suite>.rs` for `use crate::<h>` / `use super::<h>`,
resolves `<h>` through that crate's `tests/all.rs`, and requires the
resolved file to be covered by the marker's path set — by the SAME match
`GatedSuite.selected_by` makes, so the check cannot accept a spelling the
filter would not honour.

**One thing the item's fix sketch got wrong, and it was load-bearing.**
The sketch says to resolve `<h>` "the way `_all_rs_modules` already
does". That reader records `#[path = "..."] mod x;` PAIRS — it exists
because a suite's test-id prefix is not derivable from its filename — and
the helper trees are mounted by a bare `mod common;` with no attribute at
all. Built on `_all_rs_modules` the check resolves NOTHING and passes
every tree, silently and in green: the exact failure shape this row is
about, one level up. So `_tests_sibling_files` is a second reader beside
it, taking the `#[path]` pairs plus the bare `mod x;` declarations
resolved by Rust's own rule (`x/mod.rs`, then `x.rs`). `_all_rs_modules`
is untouched, so no term derivation moved.

**It found three offenders on `main`, and they are not the ten.**

- `crates/sweep/tests/review_chamfer_r1_probes.rs`
- `crates/sweep/tests/review_verbs_rim_lever_probes.rs`
- `crates/sweep/tests/verbs_rim_r1_probes.rs`

All three import `use crate::common;` and named no part of
`crates/sweep/tests/common/`. All three markers were written on
**2026-09-09** (`8ee8cf1c`) — six days AFTER this row's census swept all
54 markers and widened the ten it found. So this is not a gap in that
census: it is the class regrowing at the predicted rate, under the
review this row said would not catch it, which is the argument for the
mechanical check rather than another sweep. Eleventh, twelfth and
thirteenth instances. Fixed in the same commit by naming the directory.

**The planted failure, because a check nobody has seen fail is not a
check.** Two cases in `scripts/gates/gated-suite-paths.sh`'s selftest:
`plant_helper_import_unnamed` (the gate must red, and the fixture mounts
its helper with a BARE `mod common;` — an attributed one would pass
against a reader that could not resolve the real shape) and the near miss
this directory's convention owes, `plant_helper_import_named` (the same
import with the directory named; the gate must pass). The near miss also
carries `use crate::{common, common as alias};`, which is the brace form
the head reader splits on top-level commas only.

**What it still cannot see**, restated from this row's own honest list
and unchanged: a helper reached through a re-export rather than a `use`
of the module head; a `#[path]`-mounted helper whose `mod` name differs
from its directory and which `tests/all.rs` does not declare; a fixture
loaded at run time from a data file; and the whole `src/` marker shape,
where the question is the ordinary path-set judgement. The check is
deliberately silent on a head it cannot resolve — over-matching would
demand a path the suite does not depend on, and that reds a correct tree.

Cross-fence: `scripts/gates/gated-suite-paths.sh` is code-quality Track
K's and was edited here for the two planters, announced in the PR. The
shell gate calls `--gated-check` and decides nothing itself, so the rule
lives in S-TCOST's file where it belongs.

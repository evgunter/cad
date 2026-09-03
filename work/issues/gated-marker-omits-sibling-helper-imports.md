---
id: gated-marker-omits-sibling-helper-imports
kind: issue
title: nothing checks a marker's path set against its own suite's helper imports
status: open
opened: 2026-09-03
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

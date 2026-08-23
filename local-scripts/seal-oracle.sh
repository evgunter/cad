#!/usr/bin/env bash
# local-scripts/seal-oracle.sh — C10's compile oracle: can the K recorder
# be written anywhere but `geom-core`?
#
#   local-scripts/seal-oracle.sh [path-to-checkout]
#
# Defaults to the checkout this script lives in, so it runs with no
# arguments. Needs a toolchain matching the workspace's rust-version and
# a crates.io index (each probe is a scratch crate with its own
# [workspace], resolved independently of this one).
#
# WHY IT EXISTS. #801 answered §D row C10 "no — the recorder cannot
# leave the crate", and a negative result is only as good as the probe
# that failed. The first version of this oracle asked ONE direction and
# its answer was written as universal; #801's adversarial review broke
# that by holding the TYPE in a crate above `geom-core` and keeping every
# kernel-trait impl behind, which builds clean. The claim survived in a
# smaller, truer form. The script now asks BOTH directions, and every
# probe is RUN — an earlier revision described probe D instead of running
# it, which is the same defect one level down.
#
# WHAT IT ESTABLISHES. `impl Decide for Probe` is the recorder, and it is
# pinned from both sides:
#   * BELOW `geom-core`, because `Decide: SpanLocate` and `SpanLocate` is
#     sealed by a `pub(crate)` module whose impl list is the scalar set;
#   * ABOVE it, because naming `Decide` means depending on `geom-core`,
#     which would have to depend back.
# What is NOT pinned is the `Probe` TYPE. Conflating the two is the
# mistake this script exists to stop anyone repeating.

set -u
CAD=${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}
TOOLCHAIN=${TOOLCHAIN:-$(sed -n 's/^rust-version[^"]*"\([^"]*\)".*/\1/p' "$CAD/Cargo.toml" | head -1)}
TOOLCHAIN=${TOOLCHAIN:-stable}
echo "checkout: $CAD"
echo "toolchain: +$TOOLCHAIN"
echo

echo "############ DOWNSTREAM: can a crate BELOW geom-core define a type that decides?"
echo "############ EXPECTED: every probe fails to compile."
D=$(mktemp -d); mkdir -p "$D/src"
cat > "$D/Cargo.toml" <<TOML
[package]
name = "probe-oracle"
version = "0.0.0"
edition = "2024"
[dependencies]
geom-core = { path = "$CAD/crates/geom-core", features = ["probe"] }
[workspace]
TOML
probe() { echo "--- $1"; cat > "$D/src/lib.rs"; (cd "$D" && cargo "+$TOOLCHAIN" build 2>&1 | grep -E "^error" | sed 's/^/    /' | head -3); }
probe "A: impl Decide for a foreign type" <<'RS'
use geom_core::predicate::{Band, Decide, Indeterminate, Sign};
pub struct P(pub f64);
impl Decide for P {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> { self.0.sign_within(band) }
}
RS
probe "B: impl the sealed supertrait" <<'RS'
use geom_core::spline::{KnotVector, SpanLocate, SpanSet};
pub struct P(pub f64);
impl SpanLocate for P {
    fn locate_spans(self, k: &KnotVector) -> SpanSet { self.0.locate_spans(k) }
    fn enclosure_hull(self, _o: Self) -> Self { P(f64::NAN) }
}
RS
probe "C: name the sealing trait" <<'RS'
pub struct P(pub f64);
impl geom_core::spline::locate::sealed::Sealed for P {}
RS
rm -rf "$D"

echo
echo "############ UPSTREAM: can a crate ABOVE geom-core hold the recorder?"
echo "--- D: the TYPE moves up; every kernel-trait impl stays in geom-core"
echo "    EXPECTED: BUILDS CLEAN, suite green. A local trait on a foreign type is"
echo "    legal, so this needs no unseal and no new public API. It relocates the"
echo "    newtype and its core::ops and leaves impl Decide for Probe where it was."
U=$(mktemp -d)
git -C "$CAD" archive HEAD | tar -x -C "$U"
mkdir -p "$U/crates/adv-probe/src"
printf '%s\n' '[package]' 'name = "adv-probe"' 'version = "0.0.0"' 'edition = "2024"' \
  > "$U/crates/adv-probe/Cargo.toml"
{
  echo '#[derive(Clone, Copy, Debug)]'
  echo 'pub struct Probe(pub f64);'
  for OP in Add:add:+ Sub:sub:- Mul:mul:'*' Div:div:/; do
    T=${OP%%:*}; R=${OP#*:}; M=${R%%:*}; SY=${R#*:}
    echo "impl core::ops::$T for Probe { type Output = Self; fn $M(self, r: Self) -> Self { Self(self.0 $SY r.0) } }"
  done
  echo 'impl core::ops::Neg for Probe { type Output = Self; fn neg(self) -> Self { Self(-self.0) } }'
} > "$U/crates/adv-probe/src/lib.rs"
python3 - "$U" <<'PY'
import difflib, sys
u = sys.argv[1]
p = u + '/Cargo.toml'; s = open(p).read()
s = s.replace('    "crates/bvh",', '    "crates/adv-probe",\n    "crates/bvh",')
open(p, 'w').write(s)
p = u + '/crates/geom-core/Cargo.toml'; s = open(p).read()
s = s.replace('probe = []', 'probe = ["dep:adv-probe"]')
s = s.replace('libm = { workspace = true }',
              'libm = { workspace = true }\nadv-probe = { path = "../adv-probe", optional = true }', 1)
open(p, 'w').write(s)
p = u + '/crates/geom-core/src/k_stats.rs'; before = open(p).read()
start = before.index('#[derive(Clone, Copy, Debug)]\n#[cfg(feature = "probe")]\npub struct Probe(pub f64);')
end = before.index('#[cfg(feature = "probe")]\nimpl Real for Probe {')
after = before[:start] + '#[cfg(feature = "probe")]\npub use adv_probe::Probe;\n\n' + before[end:]
open(p, 'w').write(after)
d = list(difflib.unified_diff(before.splitlines(), after.splitlines(), n=0))
rm = sum(1 for l in d if l.startswith('-') and not l.startswith('---'))
add = sum(1 for l in d if l.startswith('+') and not l.startswith('+++'))
print("    k_stats.rs: -%d/+%d lines (newtype + core::ops out, one re-export in)" % (rm, add))
PY
(cd "$U" && CARGO_TARGET_DIR="${ORACLE_TARGET:-$U/target}" cargo "+$TOOLCHAIN" test -p geom-core --features probe 2>&1 \
   | grep -E "^error|^test result" | sed 's/^/    /' | head -8)
rm -rf "$U"

echo "--- E: the IMPL moves up too — the upstream crate must name Decide,"
echo "    so it must depend on geom-core, which depends back."
echo "    EXPECTED: cargo refuses. This fires at resolution, before any compile."
W=$(mktemp -d); mkdir -p "$W/src"
cat > "$W/Cargo.toml" <<TOML
[package]
name = "adv-probe-oracle"
version = "0.0.0"
edition = "2024"
[dependencies]
geom-core = { path = "$CAD/crates/geom-core", features = ["probe"] }
adv-probe-oracle = { path = "." }
[workspace]
TOML
echo 'pub struct Probe(pub f64);' > "$W/src/lib.rs"
(cd "$W" && cargo "+$TOOLCHAIN" metadata --format-version 1 >/dev/null 2>"$W/err"; sed 's/^/    /' "$W/err" | grep -E "error|cyclic" | head -4)
rm -rf "$W"

echo
echo "Both directions pin the IMPL and neither pins the TYPE. That, and not"
echo "\"Probe cannot be defined outside geom-core\", is what C10 turns on."

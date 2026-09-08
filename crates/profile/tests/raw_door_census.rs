//! The raw door's census: what a shipped build can and cannot mint.
//!
//! [`profile::RawLoop`] is a fixture door behind
//! `#[cfg(any(test, feature = "test-support"))]`, and the claim that
//! buys is a claim about a build nobody here runs: the one a downstream
//! crate makes with `cargo build [--release]`, where no dev-dependency
//! edge exists to turn the feature on. Two rows stand under it:
//!
//! 1. [`no_production_source_writes_a_vertex_table`] — the census of
//!    the tree as it stands. Every writer the emission layer and
//!    [`profile::ProfileLoop::map`] do not account for is inside a
//!    test region or a test target.
//! 2. [`the_door_carries_its_gate`] — the door itself, so an edit that
//!    widens the gate fails here rather than silently restoring the
//!    minting tier.
//!
//! Two more rows stand outside this file. `crates/pncad`'s
//! `every_profile_layer_root_export_is_carried_or_listed` reads the
//! façade's side: `RawLoop` is not in this layer's shipped root export
//! set at all any more, which is why that guard's interior list is
//! empty. And `scripts/gates/test-features-dev-only.sh` reads the
//! manifests: no non-dev edge in the repository turns `test-support`
//! on.
//!
//! **What none of them is: a compiler's answer.** The enforcement is a
//! compile error in a build that has no dev-dependency edge, and these
//! rows read source and manifests rather than running that build. The
//! counterfactual was MEASURED once, by hand, and the measurement is in
//! the unit's PR with both arms verbatim; it is not a row here because
//! the suite runs from a `cargo nextest` archive on a machine with no
//! toolchain and no registry, where a nested `cargo check` cannot run —
//! a row that answers "cargo was not available" is not the row it
//! claims to be. The honest permanent home for it is a gate with a
//! toolchain, which is a CI-surface change and not this unit's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use test_utils::source::{code_and_literals, rust_sources};

/// The repository root, from this crate's manifest directory.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root resolves from crates/profile")
}

/// The attributes attached to line `i`: the run of `#[…]` lines
/// immediately above it.
fn attributes_above(lines: &[&str], i: usize) -> String {
    let mut out = String::new();
    let mut k = i;
    while k > 0 && lines[k - 1].trim_start().starts_with("#[") {
        k -= 1;
        out.push_str(lines[k].trim());
    }
    out
}

/// Does this attribute run gate on a test cfg? `#[cfg(test)]` and
/// `#[cfg(any(test, feature = "test-support"))]` both count; the
/// question this census asks is only whether a shipped build compiles
/// what follows.
fn gates_on_test(attrs: &str) -> bool {
    attrs.contains("cfg(") && attrs.contains("test")
}

/// Where a file's in-file test region begins — the first `mod` BLOCK
/// carrying a test cfg — or `usize::MAX` if it has none.
///
/// Deliberately coarse: the region runs to the end of the file. Every
/// hit in this tree is in a trailing test module, and a rule that
/// OVER-counts test regions can only make this census miss a writer,
/// never invent one.
fn test_module_starts_at(view: &str) -> usize {
    let lines: Vec<&str> = view.lines().collect();
    lines
        .iter()
        .enumerate()
        .find_map(|(i, l)| {
            let t = l.trim_start();
            let is_mod = t.starts_with("mod ")
                || t.starts_with("pub mod ")
                || t.starts_with("pub(crate) mod ");
            (is_mod && !t.ends_with(';') && gates_on_test(&attributes_above(&lines, i)))
                .then_some(i)
        })
        .unwrap_or(usize::MAX)
}

/// The FILE modules a source declares behind a test cfg
/// (`#[cfg(any(test, feature = "test-support"))] pub mod test_support;`).
///
/// A file gated at its declaration is not production source at all, and
/// the gate is one file away from the code it covers — which is why
/// this is resolved rather than allow-listed. `sweep::test_support` is
/// the live instance; the resolution is general.
fn gated_module_files(file: &Path, view: &str, out: &mut Vec<PathBuf>) {
    let dir = match file.file_name().and_then(|n| n.to_str()) {
        Some("lib.rs" | "main.rs" | "mod.rs") => file.parent().map(Path::to_path_buf),
        _ => file.parent().map(|p| {
            p.join(
                file.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default(),
            )
        }),
    };
    let Some(dir) = dir else { return };
    let lines: Vec<&str> = view.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        let Some(rest) = t
            .strip_prefix("pub mod ")
            .or_else(|| t.strip_prefix("pub(crate) mod "))
            .or_else(|| t.strip_prefix("mod "))
        else {
            continue;
        };
        let Some(name) = rest.strip_suffix(';') else {
            continue;
        };
        if !gates_on_test(&attributes_above(&lines, i)) {
            continue;
        }
        out.push(dir.join(format!("{name}.rs")));
        out.push(dir.join(name));
    }
}

/// Does this tree hold any `.rs` file at all? Asked before the shared
/// walk, which treats an empty result as the caller's bug.
fn has_rust(dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|e| {
        let p = e.path();
        if p.is_dir() {
            has_rust(&p)
        } else {
            p.extension().is_some_and(|x| x == "rs")
        }
    })
}

fn is_writer(line: &str) -> bool {
    let squashed: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    [
        "RawLoop::new(",
        "RawLoop::polygon(",
        "ProfileLoop::new(",
        "ProfileLoop::polygon(",
        ".with_tangent_joints(",
    ]
    .iter()
    .any(|m| squashed.contains(m))
}

/// Every `src/` file in the repository, outside this crate.
fn production_sources(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for area in ["crates", "demos", "tools"] {
        let Ok(entries) = std::fs::read_dir(root.join(area)) else {
            continue;
        };
        for entry in entries.flatten() {
            if area == "crates" && entry.file_name() == "profile" {
                continue;
            }
            let src = entry.path().join("src");
            if src.is_dir() {
                out.extend(rust_sources(&src));
            }
        }
    }
    out.sort();
    out
}

/// **The tree as it stands**: no production source writes a vertex
/// table.
///
/// The scan is over `src/` only. `tests/`, `examples/` and `benches/`
/// are test targets — the door's whole population, and the reason it
/// still exists — so scanning them would be asking the wrong question.
///
/// The view is `code_and_literals`, so a comment showing the retired
/// spelling does not answer for a call. Literals are KEPT because the
/// module gates this scan resolves are spelled with one
/// (`feature = "test-support"`); the cost is that a needle written
/// inside a string would read as a writer, which is a false RED and
/// therefore the safe direction.
#[test]
fn no_production_source_writes_a_vertex_table() {
    let root = repo_root();
    let sources: Vec<(PathBuf, String)> = production_sources(&root)
        .into_iter()
        .map(|f| {
            let text = std::fs::read_to_string(&f).expect("a source file reads");
            (f, code_and_literals(&text))
        })
        .collect();
    assert!(
        sources.len() > 100,
        "the scanner found only {} source files — the tree's shape \
         changed and this census was about to pass vacuously",
        sources.len()
    );

    let mut gated = Vec::new();
    for (file, view) in &sources {
        gated_module_files(file, view, &mut gated);
    }

    let mut hits = Vec::new();
    for (file, view) in &sources {
        if gated.iter().any(|g| file.starts_with(g) || file == g) {
            continue;
        }
        let cut = test_module_starts_at(view);
        for (n, line) in view.lines().enumerate() {
            if n < cut && is_writer(line) {
                hits.push(format!(
                    "{}:{}",
                    file.strip_prefix(&root).unwrap_or(file).display(),
                    n + 1
                ));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "production source writes a vertex table by hand — author \
         through the `path` lattice, or, for a table that already \
         exists in another scalar, `ProfileLoop::map`:\n  {}",
        hits.join("\n  ")
    );
}

/// **The two arms are still the two arms** — the shape, read from the
/// source that declares it.
///
/// This row is deliberately the WEAKEST of the three, and says so: the
/// enforcement is the compiler. The trait item is declared at the
/// visibility its arm grants (`raw_door!(pub)` / `raw_door!(pub(crate))`),
/// so in a shipped build the item is crate-private and **any** `pub use`
/// of it, under any name, anywhere in the crate, is E0365 — measured,
/// with `pub use crate::RawLoop as LoopMint;` added to the carried
/// `path` module: red in the shut arm, green in the wide one.
///
/// What is left for a row is that nobody quietly replaces that shape
/// with a gated re-export again, which is the shape the reviews broke:
/// while the item was `pub` in a private module, that same one-line
/// re-export compiled and sixteen rows stayed green.
#[test]
fn the_door_is_declared_at_its_arms_visibility() {
    let text = std::fs::read_to_string(repo_root().join("crates/profile/src/lib.rs"))
        .expect("profile's lib.rs reads");
    let view = code_and_literals(&text);
    let wide = "#[cfg(any(test, feature = \"test-support\"))]\nraw_door!(pub);";
    let shut = "#[cfg(not(any(test, feature = \"test-support\")))]\nraw_door!(pub(crate));";
    assert!(
        view.contains(wide),
        "the door's OPEN arm is not `raw_door!(pub)` under the test gate \
         — if the spelling moved, move this row with it"
    );
    assert!(
        view.contains(shut),
        "the door's SHUT arm is not `raw_door!(pub(crate))` — a shut arm \
         that declares the trait `pub` puts the minting tier back on \
         every shipped build's surface, and a re-export of it would then \
         compile"
    );
    assert!(
        view.contains("$vis trait RawLoop<T: Real>: Sized {"),
        "the trait is no longer declared at the macro's visibility \
         parameter — a literal visibility on the item is the gated \
         re-export shape coming back under another name"
    );
    assert!(
        !view.contains("pub trait RawLoop"),
        "`RawLoop` is declared `pub` somewhere — a literal `pub trait` is \
         exported unconditionally, and every re-export of it then \
         compiles too"
    );
    assert!(
        !view.contains("use raw_loop::RawLoop"),
        "the door is reached through a re-export again; the arms are the \
         declaration itself now, and a re-export is what this row exists \
         to keep out"
    );
}

/// **A crate that names the door must be able to REACH it**, from its
/// own manifest and not from a sibling's.
///
/// Cargo unifies features across a build graph, so a crate whose test
/// targets name a gated item compiles green inside a workspace build
/// where some OTHER crate's dev-dependency happens to turn the feature
/// on — and reds the moment someone builds it alone. That is not a
/// hypothetical: at this branch's first head `crates/sweep`'s
/// `test-support` did not forward `profile/test-support` while
/// `sweep/src/test_support.rs` named `profile::RawLoop`, and
/// `cargo check -p verbs --all-targets` was red with E0603 and five
/// E0599s against a fully green workspace.
///
/// Two textual requirements, one per half of that failure:
///
/// 1. a crate whose TEST-VISIBLE sources name the door names
///    `test-support` in its manifest;
/// 2. a crate whose `src/` names the door and that declares its own
///    `test-support` feature FORWARDS `profile/test-support` — the
///    exact miss above.
///
/// Textual, so it cannot see a feature reached through a two-deep
/// forward chain that spells neither string. The instrument for that is
/// a per-member `cargo check`, which this unit ran across all 18
/// workspace members and all 4 excluded roots and which CI does not
/// run; the filed gate item
/// (`work/bool/raw-door-compile-proof-needs-a-gate.md`) is where a
/// permanent home for compile-shaped proof belongs.
#[test]
fn every_crate_that_names_the_door_reaches_it() {
    let root = repo_root();
    let mut roots = Vec::new();
    for area in ["crates", "demos", "tools"] {
        let Ok(entries) = std::fs::read_dir(root.join(area)) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.path().join("Cargo.toml").is_file() {
                roots.push(entry.path());
            }
        }
    }
    roots.sort();
    assert!(
        roots.len() > 15,
        "the scanner found only {} cargo roots — the tree's shape changed",
        roots.len()
    );

    let mut faults = Vec::new();
    for krate in &roots {
        let name = krate
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if name == "profile" {
            continue;
        }
        // Comment lines are dropped BEFORE the containment checks: a
        // manifest whose prose explains the forward would otherwise
        // answer for the forward itself, and this row measured that on
        // itself — with the forward deleted and the comment left, it
        // still passed.
        let manifest: String = std::fs::read_to_string(krate.join("Cargo.toml"))
            .expect("a manifest reads")
            .lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");

        let names_door = |dir: &Path| -> bool {
            // `rust_sources` PANICS on a walk that finds nothing (its
            // anti-vacuity rule), and `benches/` is legitimately absent
            // or empty here — so the emptiness is answered before the
            // shared walk is asked.
            dir.is_dir()
                && has_rust(dir)
                && rust_sources(dir).iter().any(|f| {
                    let text = std::fs::read_to_string(f).unwrap_or_default();
                    code_and_literals(&text).lines().any(is_writer)
                })
        };
        let in_tests = ["tests", "examples", "benches"]
            .iter()
            .any(|d| names_door(&krate.join(d)));
        let in_src = names_door(&krate.join("src"));

        if (in_tests || in_src) && !manifest.contains("test-support") {
            faults.push(format!(
                "{name}: names the raw door but its manifest never says \
                 `test-support` — the door is gated, and a build of this \
                 crate alone cannot reach it"
            ));
        }
        if in_src
            && manifest.contains("\ntest-support = [")
            && !manifest.contains("profile/test-support")
        {
            faults.push(format!(
                "{name}: `src/` names the raw door and this crate has its \
                 own `test-support` feature, but that feature does not \
                 forward `profile/test-support` — it compiles only where \
                 a sibling's dev-dependency unifies the feature in"
            ));
        }
    }
    assert!(
        faults.is_empty(),
        "a crate cannot reach the door it names:\n  {}",
        faults.join("\n  ")
    );
}

/// **The type's own public surface mints nothing** — the hole the
/// review's mutation C found, closed.
///
/// Gating the TRAIT gates the trait. It says nothing about an inherent
/// `pub fn from_table(vertices) -> Self` added to `impl ProfileLoop`
/// three lines below it, which is a fourth door wearing no gate at all
/// — and when that mutation was applied, every row in this file, the
/// façade's root-export census and `seal.rs` all stayed green.
///
/// So the type's public method set is PINNED. A method that reads is
/// welcome and costs one line here; a method that MINTS a loop from
/// data is the thing the Q1 ruling retired, and the line it costs is
/// where someone has to argue for it.
#[test]
fn the_types_public_surface_mints_nothing() {
    let text = std::fs::read_to_string(repo_root().join("crates/profile/src/lib.rs"))
        .expect("profile's lib.rs reads");
    let view = code_and_literals(&text);

    // The inherent impl blocks on the type, by their opening lines; the
    // trait impl inside the macro is not one of them.
    let mut found: Vec<String> = Vec::new();
    let mut inside = false;
    for line in view.lines() {
        if line.starts_with("impl ProfileLoop<") || line.starts_with("impl<T: Real> ProfileLoop<") {
            inside = true;
            continue;
        }
        if inside && line == "}" {
            inside = false;
            continue;
        }
        if inside && let Some(rest) = line.trim_start().strip_prefix("pub fn ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            found.push(name);
        }
    }
    found.sort();
    assert!(
        !found.is_empty(),
        "the scanner found no inherent methods on ProfileLoop — the impl \
         blocks were re-spelled and this row is reading nothing"
    );

    // READERS hand back what is already stored; `map` is the
    // materialization door (a loop that already exists, at another
    // scalar); `reversed` derives from an existing loop. None of them
    // takes a vertex table.
    let pinned = ["map", "reversed", "tangent_joints", "vertices"];
    assert_eq!(
        found, pinned,
        "the public surface of `ProfileLoop` moved.\n  \
         found:  {found:?}\n  pinned: {pinned:?}\n\
         A method that READS is welcome — add its name here. A method \
         that MINTS a loop from a vertex table is the authoring tier \
         coming back as an inherent method, where the raw door's gate \
         cannot reach it: that is a ruling, not an edit."
    );
}

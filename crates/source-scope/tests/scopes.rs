//! **The resolution rules of `source_scope::read`, one row per rule.**
//!
//! Each fixture calls `probe(x)` where the question is, and the row
//! reads what the argument `x` resolves to. That is the same reading a
//! guard gets for a door's argument. The rules are Rust's own, so each
//! row is a shape a lexical scope reader in this tree has got wrong or
//! could have.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use source_scope::{Arg, Bound, Callee, Source, read};

fn parsed(text: &str) -> Source {
    read(text).unwrap_or_else(|e| panic!("fixture does not parse: {e}\n{text}"))
}

/// What the argument of each `probe(…)` call resolves to, in order.
fn probes(text: &str) -> Vec<Option<Bound>> {
    parsed(text)
        .events
        .iter()
        .filter(|e| e.name == "probe")
        .map(|e| match e.args.as_deref() {
            Some([Arg::Ident { bound, .. }]) => bound.clone(),
            other => panic!("a probe takes one name: {other:?}"),
        })
        .collect()
}

fn param(fn_name: &str, index: usize) -> Option<Bound> {
    Some(Bound::Param {
        fn_name: fn_name.into(),
        index,
    })
}

fn local() -> Option<Bound> {
    Some(Bound::Local)
}

fn used(path: &[&str]) -> Option<Bound> {
    Some(Bound::Use {
        path: path.iter().map(|s| (*s).to_owned()).collect(),
        alias: None,
    })
}

#[test]
fn a_let_binds_from_the_statement_after_it_to_its_blocks_end() {
    let got = probes(
        "fn f(row: &'static str) {
            probe(row);
            let row = 1;
            probe(row);
            { let row = 2; }
            probe(row);
        }",
    );
    assert_eq!(got, vec![param("f", 0), local(), local()]);
}

#[test]
fn a_lets_own_initializer_sees_the_binding_before_it() {
    let got = probes("fn f(row: &'static str) { let row = probe(row); }");
    assert_eq!(got, vec![param("f", 0)]);
}

#[test]
fn a_tuple_or_struct_pattern_in_a_let_binds_every_name_in_it() {
    let got = probes(
        "fn f(a: &'static str, b: &'static str) {
            let (a, S { b, .. }) = todo();
            probe(a);
            probe(b);
        }",
    );
    assert_eq!(got, vec![local(), local()]);
}

#[test]
fn a_closures_parameters_bind_over_its_body_only() {
    let got = probes(
        "fn f(row: &'static str) {
            let g = |row| probe(row);
            probe(row);
        }",
    );
    assert_eq!(got, vec![local(), param("f", 0)]);
}

#[test]
fn a_match_arms_pattern_binds_over_its_guard_and_body_only() {
    let got = probes(
        "fn f(row: &'static str) {
            match x {
                Some(row) if probe(row) => probe(row),
                _ => probe(row),
            }
        }",
    );
    assert_eq!(got, vec![local(), local(), param("f", 0)]);
}

#[test]
fn an_if_lets_pattern_binds_over_its_then_branch_and_not_its_else() {
    let got = probes(
        "fn f(row: &'static str) {
            if let Some(row) = x { probe(row); } else { probe(row); }
            probe(row);
        }",
    );
    assert_eq!(got, vec![local(), param("f", 0), param("f", 0)]);
}

#[test]
fn a_let_chain_binds_each_pattern_over_what_follows_it() {
    let got = probes(
        "fn f(row: &'static str) {
            if probe(row) && let Some(row) = x && probe(row) { probe(row); }
        }",
    );
    assert_eq!(got, vec![param("f", 0), local(), local()]);
}

#[test]
fn a_while_lets_and_a_fors_patterns_bind_over_their_bodies() {
    let got = probes(
        "fn f(row: &'static str) {
            while let Some(row) = x { probe(row); }
            for row in xs { probe(row); }
            probe(row);
        }",
    );
    assert_eq!(got, vec![local(), local(), param("f", 0)]);
}

#[test]
fn a_blocks_items_bind_over_the_whole_block_even_before_them() {
    let src = parsed(
        "fn f() {
            door();
            fn door() {}
        }",
    );
    let door = src.events.iter().find(|e| e.name == "door").unwrap();
    assert_eq!(door.callee, Callee::Bare(Some(Bound::Item)));
}

#[test]
fn a_nested_fn_does_not_see_the_locals_of_the_fn_around_it() {
    let got = probes(
        "use k_stats::row;
        fn f() {
            let row = 1;
            fn g() { probe(row); }
        }",
    );
    assert_eq!(got, vec![used(&["k_stats", "row"])]);
}

#[test]
fn a_parameter_shadows_an_import_of_the_same_name() {
    let got = probes(
        "use k_stats::row;
        fn f(row: &'static str) { probe(row); }
        fn g() { probe(row); }",
    );
    assert_eq!(got, vec![param("f", 0), used(&["k_stats", "row"])]);
}

#[test]
fn a_mod_does_not_see_its_parents_items_but_through_an_import() {
    let got = probes(
        "use k_stats::row;
        mod inner { fn g() { probe(row); } }",
    );
    assert_eq!(got, vec![None]);
}

#[test]
fn use_super_glob_in_an_inline_mod_reads_the_parent_in_the_file() {
    let got = probes(
        "use k_stats::row;
        const HERE: &str = \"F1\";
        mod tests {
            use super::*;
            fn g() { probe(row); probe(HERE); }
        }",
    );
    assert_eq!(got, vec![used(&["k_stats", "row"]), Some(Bound::Item)]);
}

#[test]
fn any_other_glob_is_reported_as_the_paths_it_reads() {
    let got = probes("use k_stats::*; fn g() { probe(row); }");
    assert_eq!(
        got,
        vec![Some(Bound::Glob {
            paths: vec![vec!["k_stats".into(), "*".into()]],
            behind: None,
        })]
    );
}

#[test]
fn a_use_of_a_parents_import_is_followed_to_the_path_that_leaves_the_file() {
    let got = probes(
        "use crate::k_stats::row;
        mod tests {
            use super::row;
            fn g() { probe(row); }
            mod deeper { use super::row as r; fn h() { probe(r); } }
        }",
    );
    let r = Some(Bound::Use {
        path: vec!["crate".into(), "k_stats".into(), "row".into()],
        alias: Some("r".into()),
    });
    assert_eq!(got, vec![used(&["crate", "k_stats", "row"]), r]);
}

#[test]
fn a_use_into_a_child_module_is_followed_to_its_item_or_import() {
    let got = probes(
        "use self::m::row;
        use self::m::HERE;
        mod m { pub use k_stats::row; pub const HERE: &str = \"F1\"; }
        fn g() { probe(row); probe(HERE); }",
    );
    assert_eq!(got, vec![used(&["k_stats", "row"]), Some(Bound::Item)]);
}

#[test]
fn a_self_or_super_path_call_resolves_where_it_points() {
    let src = parsed(
        "use crate::k_stats::door;
        fn local() {}
        fn f() { self::door(); self::local(); }
        mod tests { fn t() { super::door(); super::super::door(); } }",
    );
    let resolved: Vec<Option<Bound>> = src
        .events
        .iter()
        .filter(|e| e.args.is_some() && matches!(e.callee, Callee::Path { .. }))
        .map(|e| e.bound().cloned())
        .collect();
    let door = used(&["crate", "k_stats", "door"]);
    assert_eq!(resolved, vec![door.clone(), Some(Bound::Item), door, None]);
}

#[test]
fn cfg_kept_declarations_of_one_name_are_ambiguous_not_the_last_one() {
    let got = probes(
        "#[cfg(test)] use a::row;
        #[cfg(not(test))] use b::row;
        fn g() { probe(row); }",
    );
    assert_eq!(
        got,
        vec![Some(Bound::Ambiguous(vec![
            used(&["a", "row"]).unwrap(),
            used(&["b", "row"]).unwrap(),
        ]))]
    );
}

#[test]
fn a_super_glob_of_a_super_glob_reaches_the_glob_that_leaves_the_file() {
    let got = probes(
        "use k_stats::*;
        mod a { use super::*; mod b { use super::*; fn g() { probe(row); } } }",
    );
    assert_eq!(
        got,
        vec![Some(Bound::Glob {
            paths: vec![vec!["k_stats".into(), "*".into()]],
            behind: None,
        })]
    );
}

#[test]
fn a_blocks_glob_leaves_the_outer_import_behind_it_not_hidden() {
    let got = probes(
        "use crate::k_stats::row;
        fn g() { use other::*; probe(row); }",
    );
    assert_eq!(
        got,
        vec![Some(Bound::Glob {
            paths: vec![vec!["other".into(), "*".into()]],
            behind: Some(Box::new(used(&["crate", "k_stats", "row"]).unwrap())),
        })]
    );
}

#[test]
fn an_events_reached_names_follow_its_imports() {
    let src = parsed("use mod_a::choose as pick; mod b { use super::pick; fn g() { pick(); } }");
    let ev = src.events.iter().find(|e| e.name == "pick").unwrap();
    assert_eq!(ev.reaches(), ["pick", "choose"]);
}

#[test]
fn a_methods_and_a_fields_names_bind_nothing() {
    let got = probes(
        "struct S { row: u8 }
        impl S { fn row(&self) {} }
        trait T { fn row(&self); }
        fn g() { probe(row); }",
    );
    assert_eq!(got, vec![None]);
}

#[test]
fn a_use_tree_expands_groups_self_and_aliases() {
    let src = parsed("pub use a::{b::{self, c as d}, e};");
    let leaves: Vec<(Vec<String>, Option<String>, bool)> = src
        .uses
        .iter()
        .map(|u| (u.path.clone(), u.alias.clone(), u.public))
        .collect();
    let s = |p: &[&str]| p.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    assert_eq!(
        leaves,
        vec![
            (s(&["a", "b"]), None, true),
            (s(&["a", "b", "c"]), Some("d".into()), true),
            (s(&["a", "e"]), None, true),
        ]
    );
}

#[test]
fn a_calls_callee_kind_and_arguments_are_read() {
    let src = parsed(
        "fn f(x: u8) {
            a::b::door(\"F1\", x, 1 + 1);
            <T as U>::door();
            x.door();
        }",
    );
    let doors: Vec<_> = src.events.iter().filter(|e| e.name == "door").collect();
    assert_eq!(
        doors[0].callee,
        Callee::Path {
            qualifier: Some("b".into()),
            resolved: None,
        }
    );
    assert_eq!(
        doors[0].args.as_deref().unwrap(),
        [
            Arg::Str("F1".into()),
            Arg::Ident {
                name: "x".into(),
                bound: param("f", 0)
            },
            Arg::Other,
        ]
    );
    assert_eq!(
        doors[1].callee,
        Callee::Path {
            qualifier: Some("T".into()),
            resolved: None,
        }
    );
    assert_eq!(doors[2].callee, Callee::Method);
}

#[test]
fn a_call_knows_its_enclosing_fn_and_a_fn_its_shape() {
    let src = parsed(
        "pub fn wrap(row: &'static str, rows: &[&'static str]) { door(row); }
        #[cfg(test)]
        mod tests { fn t() { door(\"F2\"); } }
        impl S { fn m(&self) {} }",
    );
    let calls: Vec<&str> = src
        .events
        .iter()
        .filter(|e| e.name == "door")
        .map(|e| src.fns[e.enclosing_fn.unwrap()].name.as_str())
        .collect();
    assert_eq!(calls, ["wrap", "t"]);
    let wrap = &src.fns[0];
    assert!(wrap.public && !wrap.in_impl && !wrap.in_test && !wrap.receiver);
    assert!(wrap.params[0].ty.static_str && !wrap.params[0].ty.list);
    assert!(!wrap.params[1].ty.static_str && wrap.params[1].ty.list);
    assert_eq!(wrap.params[1].ty.idents, ["static", "str"]);
    let t = src.fns.iter().find(|f| f.name == "t").unwrap();
    assert!(t.in_test);
    let m = src.fns.iter().find(|f| f.name == "m").unwrap();
    assert!(m.in_impl && m.receiver);
}

#[test]
fn an_unparsed_macros_identifiers_are_macro_events_not_dropped() {
    let src = parsed(
        "macro_rules! m { () => { door(\"F3\") }; }
        fn f() { weird!(door => \"F4\"); }",
    );
    let kinds: Vec<&Callee> = src
        .events
        .iter()
        .filter(|e| e.name == "door")
        .map(|e| &e.callee)
        .collect();
    assert_eq!(kinds, [&Callee::Macro, &Callee::Macro]);
}

#[test]
fn a_parseable_macros_arguments_are_read_as_expressions() {
    let got = probes("fn f(row: &'static str) { assert!(probe(row), \"{}\", 1); }");
    assert_eq!(got, vec![param("f", 0)]);
}

#[test]
fn a_file_that_does_not_parse_is_an_error_not_an_empty_reading() {
    assert!(read("fn f( {").is_err());
}

#[test]
fn an_import_beside_an_item_of_its_name_is_another_namespace_not_ambiguous() {
    let src = parsed("use std::fmt;\nfn fmt() {}\nfn g() { fmt(); fmt::write(); }");
    let got: Vec<Option<Bound>> = src
        .events
        .iter()
        .filter(|e| e.args.is_some())
        .map(|e| e.bound().cloned())
        .collect();
    assert_eq!(got, vec![Some(Bound::Item), used(&["std", "fmt", "write"])]);
}

#[test]
fn a_paths_head_is_read_in_the_block_around_it_first() {
    let src = parsed(
        "mod m { pub use x::door; }
        use other::m2;
        fn a() {
            use other::m;
            m::door();
            mod inner { pub use y::door; }
            inner::door();
            use self::m as m2;
            m2::door();
        }",
    );
    let got: Vec<Option<Bound>> = src
        .events
        .iter()
        .filter(|e| e.name == "door")
        .map(|e| e.bound().cloned())
        .collect();
    assert_eq!(
        got,
        vec![
            used(&["other", "m", "door"]),
            used(&["y", "door"]),
            used(&["x", "door"]),
        ]
    );
}

#[test]
fn an_import_past_the_files_own_module_is_kept_as_written() {
    let got = probes(
        "use super::row;\nmod t { use super::super::row as r; fn g() { probe(r); } }\nfn h() { probe(row); }",
    );
    assert_eq!(
        got,
        vec![
            Some(Bound::Use {
                path: vec!["super".into(), "super".into(), "row".into()],
                alias: Some("r".into()),
            }),
            used(&["super", "row"]),
        ]
    );
}

#[test]
fn an_import_named_after_its_own_head_does_not_name_itself() {
    // `use quantity::quantity;`: the import's head is the crate, not the
    // import; another path's head is the import.
    let src = parsed(
        "use quantity::quantity;\nfn g() { quantity(); quantity::f(); }\n\
         use other::m;\nfn h() { use m::m; m(); }",
    );
    let got: Vec<Option<Bound>> = src
        .events
        .iter()
        .filter(|e| e.args.is_some())
        .map(|e| e.bound().cloned())
        .collect();
    assert_eq!(
        got,
        vec![
            used(&["quantity", "quantity"]),
            used(&["quantity", "quantity", "f"]),
            used(&["other", "m", "m"]),
        ]
    );
}

#[test]
fn an_import_of_an_in_file_module_is_walked_on_wherever_it_stands() {
    let src = parsed(
        "mod a { pub use super::n as q; }
        mod n { pub use x::door; }
        mod m { pub use y::door; }
        use self::a::q::door as leaf;
        fn f() { a::q::door(); leaf(); }
        mod t { use super::*; use m::door as g; fn h() { m::door(); g(); } }",
    );
    let got: Vec<Option<Bound>> = src
        .events
        .iter()
        .filter(|e| e.args.is_some())
        .map(|e| e.bound().cloned())
        .collect();
    let x = used(&["x", "door"]);
    let y = used(&["y", "door"]);
    let leaf = Some(Bound::Use {
        path: vec!["x".into(), "door".into()],
        alias: Some("leaf".into()),
    });
    let g = Some(Bound::Use {
        path: vec!["y".into(), "door".into()],
        alias: Some("g".into()),
    });
    assert_eq!(got, vec![x, leaf, y, g]);
}

#[test]
fn two_in_file_globs_that_each_bring_a_paths_head_are_ambiguous() {
    // One rule for a bare name and a path's head: `m` brought in by two
    // globs, each a different module, is ambiguous either way.
    let src = parsed(
        "mod a { pub mod m { pub use x::door; } pub use x::row; }
        mod b { pub mod m { pub use y::door; } pub use y::row; }
        use self::a::*;
        use self::b::*;
        fn f() { m::door(); probe(row); }",
    );
    let door = src.events.iter().find(|e| e.name == "door").unwrap();
    assert_eq!(
        door.bound().cloned(),
        Some(Bound::Ambiguous(vec![
            used(&["x", "door"]).unwrap(),
            used(&["y", "door"]).unwrap(),
        ]))
    );
    assert_eq!(
        probes(
            "mod a { pub use x::row; }\nmod b { pub use y::row; }\nuse self::a::*;\nuse self::b::*;\nfn f() { probe(row); }"
        ),
        vec![Some(Bound::Ambiguous(vec![
            used(&["x", "row"]).unwrap(),
            used(&["y", "row"]).unwrap(),
        ]))]
    );
}

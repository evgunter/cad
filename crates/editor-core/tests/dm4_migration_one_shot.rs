//! **The stored documents are converted by one total map** (FORK-DM4
//! unit 1): each document this unit regenerated is the base's save of it
//! under the map, up to minted ids.
//!
//! The map:
//!
//! - `Boolean { op: Union | Intersect, a, b, declare }` is `Union` or
//!   `Intersect { members: Spelled([a, b]), declare }`, and
//!   `Boolean { op: Subtract, a, b, declare }` is
//!   `Subtract { from: a, tool: b, declare }`; a chain stays a chain;
//! - a union's member list is that list spelled (`{"Spelled": [...]}`);
//! - a name's carry segment is `From { read, of }`, keyed by the read the
//!   minting node took the entity in through: `FromA`'s is the minting
//!   node's first seat (a subtract's `from`, a union's or intersect's first
//!   member), `FromB`'s its second, `FromTarget`'s its target, and
//!   `FromMember { member, of }`'s the member read the node `member`
//!   defines.
//!
//! Persist has no migration path ("regenerate the corpus and move on",
//! `persist/mod.rs`): the documents are regenerated once from their
//! authoring, and this one-shot checks the total map against the base's
//! saves. Edit logs record no minted id, so the map is checked in step
//! with the regenerated file, edit for edit, each carry segment against
//! the seat of the node that minted it in the regenerated document.
//!
//! Run against a checkout of the base: `DM4_BASE_TREE=<that checkout>`
//! and `--run-ignored only`. Ignored because the base is not in this
//! tree; the PR states the run.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::{load, save};
use geom_core::Tol;
use serde_json::Value;

/// The files this unit regenerated.
const FILES: [&str; 3] = [
    "crates/editor-core/tests/corpus/die_tool.pncad",
    "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
    "crates/pncad/tests/plate_param.pncad",
];

#[test]
#[ignore = "one shot against a checkout of the base, named by DM4_BASE_TREE"]
fn every_regenerated_document_is_the_base_one_under_the_total_map() {
    let base = std::path::PathBuf::from(
        std::env::var("DM4_BASE_TREE").expect("DM4_BASE_TREE names a checkout of the base"),
    );
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in FILES {
        let read =
            |root: &std::path::Path| std::fs::read_to_string(root.join(file)).expect("reads");
        let (old_text, new_text) = (read(&base), read(&here));
        let (old, new) = (
            crate::wire::wire_body(&old_text),
            crate::wire::wire_body(&new_text),
        );
        // The regenerated document as this build replays it: its nodes
        // and variables by id, which an edit log's names cite.
        let replayed = load(&new_text, Tol::witness())
            .expect("the regenerated file loads")
            .doc;
        let table = crate::wire::wire_body(&save(&replayed, &[], Tol::witness()).expect("saves"));
        let mut walk = Walk {
            seats: Seats::of(&table),
            map: Bijection::default(),
        };
        walk.value(&old, &new, "$")
            .unwrap_or_else(|err| panic!("{file}: {err}"));
        println!("{file}: the base's save under the total map, up to ids");
    }
}

/// The regenerated document's nodes, by id: each one's seats, in seat
/// order, and each variable's defining node.
struct Seats {
    nodes: BTreeMap<String, Value>,
    defined_by: BTreeMap<String, String>,
}

impl Seats {
    fn of(table: &Value) -> Self {
        let nodes = table["snapshot"]["nodes"]
            .as_object()
            .expect("a node table")
            .iter()
            .map(|(id, node)| (id.clone(), node.clone()))
            .collect();
        let defined_by = table["snapshot"]["vars"]
            .as_object()
            .expect("a variable table")
            .iter()
            .filter_map(|(id, var)| {
                let node = var["def"].get("Output")?.get("node")?.as_str()?;
                Some((id.clone(), node.to_owned()))
            })
            .collect();
        Self { nodes, defined_by }
    }

    /// The read in seat `k` of node `id`: a subtract's `from` and `tool`,
    /// a union's or intersect's spelled members, a blend's or shell's
    /// target.
    fn seat(&self, id: &str, k: usize) -> Option<&str> {
        let node = self.nodes.get(id)?;
        let (tag, fields) = node.as_object()?.iter().next()?;
        let read = match tag.as_str() {
            "Subtract" => fields.get(["from", "tool"].get(k)?)?,
            "Union" | "Intersect" => fields["members"]["Spelled"].get(k)?,
            "Fillet" | "Chamfer" | "Shell" if k == 0 => fields.get("target")?,
            _ => return None,
        };
        read.as_str()
    }
}

#[derive(Default)]
struct Bijection {
    forward: BTreeMap<String, String>,
    back: BTreeMap<String, String>,
}

impl Bijection {
    fn pair(&mut self, old: &str, new: &str, at: &str) -> Result<(), String> {
        match (self.forward.get(old), self.back.get(new)) {
            (None, None) => {
                self.forward.insert(old.to_owned(), new.to_owned());
                self.back.insert(new.to_owned(), old.to_owned());
                Ok(())
            }
            (Some(image), _) if image == new => Ok(()),
            (Some(image), _) => Err(format!("{at}: {old} reads as {new}, earlier as {image}")),
            (None, Some(pre)) => Err(format!("{at}: {new} is {old}'s image, earlier {pre}'s")),
        }
    }
}

fn is_id(text: &str) -> bool {
    editor_core::MintId::parse(text).is_some()
}

struct Walk {
    seats: Seats,
    map: Bijection,
}

impl Walk {
    fn value(&mut self, old: &Value, new: &Value, at: &str) -> Result<(), String> {
        match (old, new) {
            (Value::String(a), Value::String(b)) if is_id(a) && is_id(b) => self.map.pair(a, b, at),
            (Value::Object(a), Value::Object(b)) => {
                if let Some(done) = self.boolean(a, b, at)? {
                    return Ok(done);
                }
                if let Some(done) = self.name(a, b, at)? {
                    return Ok(done);
                }
                self.object(a, b, at)
            }
            (Value::Array(a), Value::Array(b)) => self.array(a, b, at),
            (a, b) if a == b => Ok(()),
            (a, b) => Err(format!("{at}: {a} against {b}")),
        }
    }

    fn array(&mut self, a: &[Value], b: &[Value], at: &str) -> Result<(), String> {
        if a.len() != b.len() {
            return Err(format!("{at}: {} elements against {}", a.len(), b.len()));
        }
        for (i, (va, vb)) in a.iter().zip(b).enumerate() {
            self.value(va, vb, &format!("{at}[{i}]"))?;
        }
        Ok(())
    }

    fn object(
        &mut self,
        a: &serde_json::Map<String, Value>,
        b: &serde_json::Map<String, Value>,
        at: &str,
    ) -> Result<(), String> {
        if a.len() != b.len() {
            return Err(format!("{at}: {} entries against {}", a.len(), b.len()));
        }
        for ((ka, va), (kb, vb)) in in_mint_order(a).into_iter().zip(in_mint_order(b)) {
            let here = format!("{at}.{ka}");
            if ka == "chain" && at.ends_with(".mint") {
                continue;
            }
            if is_id(ka) && is_id(kb) {
                self.map.pair(ka, kb, &here)?;
            } else if ka != kb {
                return Err(format!("{at}: key {ka} against {kb}"));
            }
            self.value(va, vb, &here)?;
        }
        Ok(())
    }

    /// The node map: a `Boolean` against the node it is, and a union's
    /// member list against that list spelled.
    fn boolean(
        &mut self,
        a: &serde_json::Map<String, Value>,
        b: &serde_json::Map<String, Value>,
        at: &str,
    ) -> Result<Option<()>, String> {
        if let (Some(old), 1) = (a.get("Boolean"), a.len()) {
            let op = old["op"].as_str().unwrap_or_default();
            let (tag, seats): (&str, [&str; 2]) = match op {
                "Subtract" => ("Subtract", ["from", "tool"]),
                "Union" | "Intersect" => (op, ["", ""]),
                other => return Err(format!("{at}: a Boolean of op {other:?}")),
            };
            let Some(new) = b.get(tag) else {
                return Err(format!("{at}: Boolean {op} against {b:?}"));
            };
            let here = format!("{at}.{tag}");
            let (na, nb) = if tag == "Subtract" {
                (&new[seats[0]], &new[seats[1]])
            } else {
                let members = new["members"]["Spelled"]
                    .as_array()
                    .filter(|m| m.len() == 2)
                    .ok_or_else(|| format!("{here}: not two spelled members"))?;
                (&members[0], &members[1])
            };
            self.value(&old["a"], na, &format!("{here}[0]"))?;
            self.value(&old["b"], nb, &format!("{here}[1]"))?;
            self.value(&old["declare"], &new["declare"], &format!("{here}.declare"))?;
            return Ok(Some(()));
        }
        if let (Some(old), Some(new), 1, 1) = (a.get("Union"), b.get("Union"), a.len(), b.len())
            && old["members"].is_array()
        {
            let here = format!("{at}.Union");
            self.value(
                &old["members"],
                &new["members"]["Spelled"],
                &format!("{here}.members"),
            )?;
            self.value(&old["declare"], &new["declare"], &format!("{here}.declare"))?;
            return Ok(Some(()));
        }
        Ok(None)
    }

    /// The name map: a stable name whose path opens with a retired carry
    /// segment, against the regenerated name's `From`, its read checked
    /// against the seat of the node that minted it.
    fn name(
        &mut self,
        a: &serde_json::Map<String, Value>,
        b: &serde_json::Map<String, Value>,
        at: &str,
    ) -> Result<Option<()>, String> {
        let (Some(old_path), Some(new_path)) = (
            a.get("path").and_then(Value::as_array),
            b.get("path").and_then(Value::as_array),
        ) else {
            return Ok(None);
        };
        let carries = old_path.iter().any(|seg| {
            ["FromA", "FromB", "FromMember", "FromTarget"]
                .iter()
                .any(|tag| seg.get(tag).is_some())
        });
        if !carries {
            return Ok(None);
        }
        self.value(&a["kind"], &b["kind"], &format!("{at}.kind"))?;
        self.value(&a["node"], &b["node"], &format!("{at}.node"))?;
        let minter = b["node"].as_str().ok_or(format!("{at}: a name's node"))?;
        if old_path.len() != new_path.len() {
            return Err(format!(
                "{at}: a path of {} against {}",
                old_path.len(),
                new_path.len()
            ));
        }
        for (i, (os, ns)) in old_path.iter().zip(new_path).enumerate() {
            let here = format!("{at}.path[{i}]");
            let Some(from) = ns.get("From") else {
                self.value(os, ns, &here)?;
                continue;
            };
            let read = from["read"]
                .as_str()
                .ok_or(format!("{here}: From's read"))?;
            let of = if let Some(of) = os.get("FromA").or_else(|| os.get("FromTarget")) {
                self.seat_is(minter, 0, read, &here)?;
                of
            } else if let Some(of) = os.get("FromB") {
                self.seat_is(minter, 1, read, &here)?;
                of
            } else if let Some(member) = os.get("FromMember") {
                let node = self
                    .seats
                    .defined_by
                    .get(read)
                    .ok_or(format!("{here}: {read} is no output"))?
                    .clone();
                let old_node = member["member"].as_str().ok_or(format!("{here}: member"))?;
                self.map.pair(old_node, &node, &here)?;
                &member["of"]
            } else {
                return Err(format!("{here}: {os} against a From"));
            };
            self.value(of, &from["of"], &format!("{here}.of"))?;
        }
        Ok(Some(()))
    }

    fn seat_is(&self, minter: &str, k: usize, read: &str, at: &str) -> Result<(), String> {
        match self.seats.seat(minter, k) {
            Some(seat) if seat == read => Ok(()),
            Some(seat) => Err(format!(
                "{at}: read {read}, but seat {k} of {minter} reads {seat}"
            )),
            None => Err(format!("{at}: {minter} has no seat {k}")),
        }
    }
}

/// An object's entries, ids in mint order and every other key after
/// them in text order.
fn in_mint_order(object: &serde_json::Map<String, Value>) -> Vec<(&String, &Value)> {
    let mut entries: Vec<_> = object.iter().collect();
    entries.sort_by_key(|(key, _)| {
        (
            editor_core::MintId::parse(key).is_none(),
            editor_core::MintId::parse(key),
            key.as_str(),
        )
    });
    entries
}

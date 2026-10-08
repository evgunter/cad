//! **One saved document against another, up to minted ids**: the
//! comparator that says a change moved nothing but ids and the output
//! variables an operation defines (D10).
//!
//! Both are read as JSON bodies. The newer one's output variables are
//! set aside first — their rows in the variable table, their names and
//! their mint-log entries — and the two are then walked in step: every
//! string spelling an id (`3:3fa9c1d2a0b1c3d4`), as a value or as an
//! object key, must read as its image under ONE map, built as the walk
//! meets ids and held injective both ways; everything else is equal.
//! The mint chain is not compared: it is a digest over the ids.
//!
//! Where a refactoring hands over the node map it carried with, two
//! live documents are `fixture::round_trip::same_up_to_ids`'s instead:
//! this walk is for two saves no map joins.
//!
//! A map built by the walk is a bijection of the ids the two documents
//! spell, so a dropped node, an added one, a moved value or two ids
//! swapped in one place and not another all refuse, each naming the
//! path where the two parted.

use std::collections::BTreeMap;

use serde_json::Value;

/// `doc` with every output variable removed: its row, its name and its
/// mint-log entry.
pub fn without_outputs(doc: &Value) -> Value {
    let mut doc = doc.clone();
    let snapshot = &mut doc["snapshot"];
    let outputs: Vec<String> = snapshot["vars"]
        .as_object()
        .expect("a variable table")
        .iter()
        .filter(|(_, var)| var["def"].get("Output").is_some())
        .map(|(id, _)| id.clone())
        .collect();
    let vars = snapshot["vars"].as_object_mut().expect("a variable table");
    for id in &outputs {
        vars.remove(id);
    }
    if let Some(names) = snapshot.get_mut("var_names").and_then(Value::as_object_mut) {
        for id in &outputs {
            names.remove(id);
        }
    }
    snapshot["mint"]["log"]
        .as_array_mut()
        .expect("a mint log")
        .retain(|entry| {
            !entry
                .get("var")
                .and_then(Value::as_str)
                .is_some_and(|id| outputs.iter().any(|o| o == id))
        });
    doc
}

/// Whether `text` spells an id: an ordinal in decimal, a colon and
/// sixteen lowercase hex digits.
fn is_id(text: &str) -> bool {
    editor_core::MintId::parse(text).is_some()
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
            (Some(image), _) => Err(format!(
                "{at}: {old} reads as {new}, and earlier as {image}"
            )),
            (None, Some(preimage)) => Err(format!(
                "{at}: {new} is the image of {old}, and earlier of {preimage}"
            )),
        }
    }
}

/// **`old` and `new` are one document up to minted ids**, `new`'s
/// output variables set aside. `Err` names the first path where they
/// part.
pub fn equal_up_to_ids(old: &Value, new: &Value) -> Result<(), String> {
    let new = without_outputs(new);
    let mut map = Bijection::default();
    walk(old, &new, "$", &mut map)
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

fn walk(old: &Value, new: &Value, at: &str, map: &mut Bijection) -> Result<(), String> {
    match (old, new) {
        (Value::String(a), Value::String(b)) if is_id(a) && is_id(b) => map.pair(a, b, at),
        (Value::Object(a), Value::Object(b)) => {
            if a.len() != b.len() {
                return Err(format!("{at}: {} entries against {}", a.len(), b.len()));
            }
            // Entries pair in mint order: minting more ids keeps the
            // order of the ones both documents hold, where the text of
            // an id does not.
            for ((ka, va), (kb, vb)) in in_mint_order(a).into_iter().zip(in_mint_order(b)) {
                let here = format!("{at}.{ka}");
                if ka == "chain" && at.ends_with(".mint") {
                    continue;
                }
                if is_id(ka) && is_id(kb) {
                    map.pair(ka, kb, &here)?;
                } else if ka != kb {
                    return Err(format!("{at}: key {ka} against {kb}"));
                }
                walk(va, vb, &here, map)?;
            }
            Ok(())
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(format!("{at}: {} elements against {}", a.len(), b.len()));
            }
            for (i, (va, vb)) in a.iter().zip(b).enumerate() {
                walk(va, vb, &format!("{at}[{i}]"), map)?;
            }
            Ok(())
        }
        (a, b) if a == b => Ok(()),
        (a, b) => Err(format!("{at}: {a} against {b}")),
    }
}

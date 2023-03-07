// For the sake of this example, an item is not represented by a `Struct`,
// but by a `item`/`item_description` pair.

// A list of `item` and `item_description` pairs. This is the the starting
// equipment of every n00b adventurer!
pub const STARTING_ITEMS: &str = r#"{
    "health_potion": "An health potion",
    "c++_sword": "Trusty, not rusty, kinda verbose too. A legacy of the past",
    "c++_shield": "That's what I call a proper header!"
}"#;

// A simple declarative macro that will help us defining list of items
// in a quick way. It lets us define a `HashMap<String, String>` using
// a pretty much standard rocket notation.
//
// For example, to declare a set of two different items, you can do:
// let items = items![
//      "item_one" => "Item one description",
//      "item_two" => "Item two description"
// ]
#[macro_export]
macro_rules! items {
    // Here we are matching for a specific pattern, pretty much standard
    // regexp notation: a sequence of expression pairs (`expr`), in our
    // case `&str` separated by `=>`, each pair is separated by a comma
    // and we require at least one pair.
    ( $( $name:expr => $description:expr),+ ) => {
        {
            // Let's declare a new `HashMap`...
            let mut items_hash_map: HashMap<String, String> = HashMap::new();
            // ... and push some items into that! the enclosing `$()*` specifies
            // that this operation should be performed for each `=>` separated pair.
            $(
                items_hash_map.insert(
                    $name.to_string(),
                    $description.to_string(),
                );
            )*
            // Then, we return the map. Easy peasy!
            items_hash_map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn items_should_be_properly_instantiated() {
        let items = items![
            "item" => "Description"
        ];
        let mut expected_items = HashMap::new();
        expected_items.insert("item".to_string(), "Description".to_string());
        assert_eq!(expected_items, items);
    }
}

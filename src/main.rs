use adventure_rs::{adventurer::Adventurer, enemy::Enemy, item::STARTING_ITEMS, items};
use game_actor::GameActor;
use game_actor_derive::{add_game_actor_attributes, GameActor};

use std::collections::HashMap;

fn main() {
    let starting_items: HashMap<String, String> = serde_json::from_str(STARTING_ITEMS).unwrap();
    let noob_adventurer = Adventurer {
        items: starting_items,
        health: 10,
        mana: 10,
    };

    println!("Here comes a new adventurer!\n{:?}", noob_adventurer);
    println!(
        "\nThe noob_adventurer actor type is: {}",
        noob_adventurer.get_game_actor_type()
    );
    println!(
        "Noob adventurer health: {}\nNoob adventurer mana: {}",
        noob_adventurer.get_health(),
        noob_adventurer.get_mana()
    );

    let experienced_adventurer_items = items![
        "mana_potion" => "A mana potion",
        "rusty_sword" => "Rusty, thus blazingly fast!",
        "rusty_shield" => "The fastest of the shields (whatever that means)"
    ];

    let experienced_adventurer = Adventurer {
        items: experienced_adventurer_items,
        health: 99,
        mana: 99,
    };

    println!(
        "\nHere comes an experienced adventurer!\n{:?}",
        experienced_adventurer
    );
    println!(
        "\nThe experienced_adventurer actor type is: {}",
        experienced_adventurer.get_game_actor_type()
    );
    println!(
        "Experienced adventurer health: {}\nExperienced adventurer mana: {}",
        experienced_adventurer.get_health(),
        experienced_adventurer.get_mana()
    );

    let mut skeleton = Enemy { health: 5, mana: 5 };

    println!("\nOh no! A Skeleton!\n{:?}", skeleton);
    println!(
        "\nThe skeleton actor type is: {}",
        skeleton.get_game_actor_type()
    );
    println!(
        "Skeleton health: {}\nSkeleton mana: {}",
        skeleton.get_health(),
        skeleton.get_mana()
    );

    println!("\nThe noob_adventurer strikes a blow!");
    skeleton.set_health(skeleton.get_health() - 1);
    println!("Now the skeleton has {} HP!", skeleton.get_health());

    println!("\nThe experienced_adventurer strikes a blow!");
    skeleton.set_health(skeleton.get_health() - 4);
    println!("Now the skeleton has {} HP!", skeleton.get_health());
}

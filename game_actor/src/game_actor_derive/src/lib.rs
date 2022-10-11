use proc_macro::TokenStream;
use quote::quote;
use syn;

// This is a procedural-type Derive macro, it is used to
// automatically implement the `GameActor` trait
// defined in `game_actor/src/lib.rs` for every struct
// that needs to *Act* in our game.
//
// For adventure_rs, *Act*ing means having the possibility
// to obtain the current amount of `health` and `mana` points,
// as well as modifying these values according to what happens
// during the scene.
//
// The `proc_macro_derive` macro takes the name of the `Trait`
// to be derived as an input.
#[proc_macro_derive(GameActor)]
pub fn game_actor_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate. More specifically, here we are
    // parsing the `input`, so the original Rust representation
    // of the `struct` that needs to implement the `GameActor`
    // trait
    let ast = syn::parse(input).unwrap();

    // Here we are just calling the `impl_game_actor` trait,
    // a helper function that just writes the `Impl` block
    // to implement the functions listed in the definition
    // of the `GameActor` trait
    impl_game_actor(&ast)
}

// Helper function that takes a `syn::DeriveInput` as input
// and returns the updated `TokenStream` that will extend
// the original definition through with an `Impl` block
fn impl_game_actor(ast: &syn::DeriveInput) -> TokenStream {
    // The `name` can be obtained through the `ident` token,
    // which is a `syn::Ident` and represents the name of the
    // struct
    let name = &ast.ident;

    // Here we are using the `quote!` macro to produce a
    // `proc_macro2::TokenStream` from some Rust code, in
    // this case the implementation of the `GameActor`
    // trait.
    let gen = quote! {
        // We use #name to interpolate the proper name of
        // the implementor within the `quote!` macro call
        // for the second parameter of the `Impl` block...
        impl GameActor for #name {
            // ... and use it again to implement the
            // `get_game_actor_type` method!
            fn get_game_actor_type(&self) -> &str {
                return stringify!(#name)
            }

            // The following methods are kinda self explanatory,
            // they are used to retrieve/set the current amount
            // of health/mana respectively.
            fn get_health(&self) -> &u64 {
                &self.health
            }

            // Wait, where are the `health` and `mana` attributes
            // coming from?
            fn get_mana(&self) -> &u64 {
                &self.mana
            }

            // Heresy! Some other black magic must have been used
            // to produce this kind of sorcery!
            fn set_health(&mut self, amount: u64) -> () {
                self.health = amount;
            }

            // Could it be another type of macromajig?!
            fn set_mana(&mut self, amount: u64) -> () {
                self.mana = amount;
            }
        }
    };

    // Finally, the output is converted to a proper `proc_macro::TokenStream`,
    // which is different from the `proc_macro2::TokenStream` produced by the
    // `quote` macro.
    gen.into()
}

// This is a attribute-like procedural macro. It is used to add specific attributes
// to a specific `Struct` that does not have them. In our case, the `add_game_actor_attributes`
// will be used in pair with `#[derive(GameActor)]` to add the `health` and `mana`
// attributes references in the implementation of the `GameActor` `Trait`.
#[proc_macro_attribute]
pub fn add_game_actor_attributes(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Alternate way to parse the `input` `TokenStream` w.r.t. `syn::parse`.
    // In this case, we are specifying the input variable to parse as well
    // as the type that we expect to receive (`syn::DeriveInput`)
    let mut ast = syn::parse_macro_input!(input as syn::DeriveInput);
    // Let's match on ast.data, we are searching for a `syn::Data:Struct`,
    // since we want to add attributes to something.
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            // Let's match one more time on the `Struct`,
            // we are looking for the struct `fields`!
            match &mut struct_data.fields {
                // Since we got a mutable reference to the `fields`,
                // we are now able to `push` some more without altering
                // the original structure!
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        // This should feel familiar at this point:
                        // - parse the input to obtain a `proc_macro2::TokenStream`
                        // - the first argument specified what we are adding, a `Field`
                        // - the second argument is an actual token stream obtained through
                        //   the `quote!` macro
                        // We have now added a `u64` `health` field to our `Struct`!
                        syn::parse::Parser::parse2(
                            syn::Field::parse_named,
                            quote! { pub health: u64 },
                        )
                        .unwrap(),
                    );
                    fields.named.push(
                        // Same thing all over again, let's give the actor some `mana` to
                        // play with!
                        syn::parse::Parser::parse2(
                            syn::Field::parse_named,
                            quote! { pub mana: u64 },
                        )
                        .unwrap(),
                    );
                }
                // We do not want to do anything else to something different than a `Struct`
                _ => (),
            }

            // Finally, let's call `quote!` one more time and return!
            return quote! {
                #ast
            }
            .into();
        }
        // If we reach this point, no struct was actually provided and the macro that we wrote won't work
        _ => panic!("`add_field` has to be used with structs "),
    }
}

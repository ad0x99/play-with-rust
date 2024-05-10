mod never_type;
mod sized_types_and_sized_trait;
mod type_synonyms_with_type_aliases;

pub fn run_advanced_types() {
    println!("========Running type synonyms with type aliases========");
    type_synonyms_with_type_aliases::type_synonyms_with_type_aliases();

    println!("========Running sized types and sized trait========");
    sized_types_and_sized_trait::sized_types_and_sized_trait();
}

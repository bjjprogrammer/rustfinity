use std::collections::hash_map::Entry;
use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    match registry.entry(section.to_string()) {
        Entry::Vacant(entry) => {
            entry.insert(vec![animal.to_string()]);
        }
        Entry::Occupied(mut entry) => {
            if !entry.get().contains(&animal.to_string()) {
                entry.get_mut().push(animal.to_string());
            }
        }
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    match registry.get(section) {
        Some(animals) => {
            let mut animals = animals.clone();
            animals.sort();
            animals
        }
        None => Vec::<String>::new(),
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut animals = Vec::<String>::new();
    for (_, animals_section) in registry.iter() {
        animals.extend(animals_section.clone());
    }
    animals.sort();
    animals
}

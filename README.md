# queriable_storage
[![Crates.io](https://img.shields.io/crates/v/queriable_storage.svg)](https://crates.io/crates/queriable_storage)
[![Docs](https://docs.rs/queriable_storage/badge.svg?version=0.2.1)](https://docs.rs/queriable_storage)

__Queriable storage implementation in Rust__

This crate provides the QueriableDataStore struct that can be queried by multiple filters.


## Example
```
use queriable_storage::QueriableDataStore;
struct Person {
    first_name: &'static str,
    last_name: &'static str,
    age: u32,
}
let persons:Vec<Person> = vec![/* ...*/];
let storage: QueriableDataStore<Person> = persons.into();
let first_name_index = storage.get_index(|v| v.first_name);
let last_name_index = storage.get_index(|v| v.last_name);
let age_index = storage.get_index(|v| v.age);
let filtered: Vec<&Person> = storage
    .filter(
        (first_name_index.filter_eq("Isaiah") & last_name_index.filter_eq("Mccarthy"))
            | (age_index.filter_lt(30) | age_index.filter_gte(60)),
    )
    .collect();
```
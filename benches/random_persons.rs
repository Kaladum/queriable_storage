#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use queriable_storage::QueriableDataStore;
    use std::ops::Bound::*;
    struct Person {
        first_name: String,
        last_name: String,
        birthday: NaiveDate,
    }

    use test::{black_box, Bencher};

    fn get_data() -> Vec<Person> {
        let text = include_str!("./persons.csv");
        let rows: Vec<Vec<&str>> = text.split("\n").map(|v| v.split(",").collect()).collect();

        let persons: Vec<Person> = rows
            .iter()
            .map(|v| Person {
                first_name: v[1].to_string(),
                last_name: v[2].to_string(),
                birthday: NaiveDate::parse_from_str(v[3], "%Y-%m-%d").unwrap(),
            })
            .collect();
        persons
    }

    #[bench]
    fn bench_1k_random_persons_eq(b: &mut Bencher) {
        let persons = get_data();
        let data: QueriableDataStore<Person> = persons.into();

        let first_name_index = data.get_index(|v| v.first_name.clone());
        let last_name_index = data.get_index(|v| v.last_name.clone());

        b.iter(|| {
            for _ in 1..100 {
                black_box(
                    data.filter(
                        first_name_index.filter_eq("Jerry".to_string())
                            & last_name_index.filter_eq("Tondeur".to_string()),
                    )
                    .count(),
                );
            }
        });
    }

    #[bench]
    fn bench_1k_random_persons_lte(b: &mut Bencher) {
        let persons = get_data();
        let data: QueriableDataStore<Person> = persons.into();

        let birthday_index = data.get_index(|v| v.birthday);

        b.iter(|| {
            for _ in 1..100 {
                black_box(
                    data.filter(birthday_index.filter_lte(NaiveDate::from_ymd(1950, 1, 1)))
                        .count(),
                );
            }
        });
    }

    #[bench]
    fn bench_1k_random_persons_and(b: &mut Bencher) {
        let persons = get_data();
        let data: QueriableDataStore<Person> = persons.into();

        let first_name_index = data.get_index(|v| v.first_name.clone());
        let last_name_index = data.get_index(|v| v.last_name.clone());

        b.iter(|| {
            for _ in 1..100 {
                black_box(
                    data.filter(
                        first_name_index
                            .filter_range((Included("A".to_string()), Excluded("H".to_string())))
                            & last_name_index.filter_range((
                                Included("A".to_string()),
                                Excluded("H".to_string()),
                            )),
                    )
                    .count(),
                );
            }
        });
    }

    #[bench]
    fn bench_1k_random_persons_or(b: &mut Bencher) {
        let persons = get_data();
        let data: QueriableDataStore<Person> = persons.into();

        let last_name_index = data.get_index(|v| v.last_name.clone());

        b.iter(|| {
            for _ in 1..100 {
                black_box(
                    data.filter(
                        last_name_index
                            .filter_range((Included("A".to_string()), Excluded("E".to_string())))
                            | last_name_index.filter_gte("V".to_string()),
                    )
                    .count(),
                );
            }
        });
    }
}

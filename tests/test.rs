use queriable_storage::QueriableDataStore;

struct TestData {
    first_name: &'static str,
    last_name: &'static str,
    age: u32,
}

fn get_test_data() -> QueriableDataStore<TestData> {
    let data = vec![
        TestData {
            first_name: "Isaiah",
            last_name: "Mccarthy",
            age: 32,
        },
        TestData {
            first_name: "Bella",
            last_name: "Crawford",
            age: 58,
        },
        TestData {
            first_name: "Dexter",
            last_name: "O'Brien",
            age: 75,
        },
        TestData {
            first_name: "Catherine",
            last_name: "Hunt",
            age: 16,
        },
        TestData {
            first_name: "Haris",
            last_name: "Burke",
            age: 28,
        },
        TestData {
            first_name: "Meghan",
            last_name: "Berry",
            age: 42,
        },
        TestData {
            first_name: "Brett",
            last_name: "Holmes",
            age: 37,
        },
        TestData {
            first_name: "Daniella",
            last_name: "Edwards",
            age: 28,
        },
        TestData {
            first_name: "Aaron",
            last_name: "Mcbride",
            age: 8,
        },
        TestData {
            first_name: "Sharon",
            last_name: "Snyder",
            age: 63,
        },
    ];

    data.into()
}

#[test]
fn test_eq() {
    let data = get_test_data();
    let first_name_index = data.get_index(|v| v.first_name);
    let last_name_index = data.get_index(|v| v.last_name);
    let filtered: Vec<&TestData> = data
        .filter([
            first_name_index.filter_eq("Isaiah"),
            last_name_index.filter_eq("Mccarthy"),
        ])
        .collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].first_name, "Isaiah");
    assert_eq!(filtered[0].last_name, "Mccarthy");
}

#[test]
fn test_gt() {
    let data = get_test_data();
    let age_index = data.get_index(|v| v.age);
    let filtered: Vec<&TestData> = data.filter([age_index.filter_gt(63)]).collect();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].age, 75);
}

#[test]
fn test_gte() {
    let data = get_test_data();
    let age_index = data.get_index(|v| v.age);
    let filtered: Vec<&TestData> = data.filter([age_index.filter_gte(63)]).collect();
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].age, 75);
    assert_eq!(filtered[1].age, 63);
}

#[test]
fn test_lt() {
    let data = get_test_data();
    let age_index = data.get_index(|v| v.age);
    let filtered: Vec<&TestData> = data.filter([age_index.filter_lt(30)]).collect();
    assert_eq!(filtered.len(), 4);
}

#[test]
fn test_lte() {
    let data = get_test_data();
    let age_index = data.get_index(|v| v.age);
    let filtered: Vec<&TestData> = data.filter([age_index.filter_lte(20)]).collect();
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_between() {
    let data = get_test_data();
    let age_index = data.get_index(|v| v.age);
    let filtered: Vec<&TestData> = data.filter([age_index.filter_between(30, 50)]).collect();
    assert_eq!(filtered.len(), 3);
}

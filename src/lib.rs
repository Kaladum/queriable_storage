use std::{
    collections::BTreeMap,
    ops::{BitAnd, BitOr, Bound::*, RangeBounds},
};

///Data structure that can be queried by multiple filters.
///Its not allowed to modify data after the generation of the data store.
pub struct QueriableDataStore<T> {
    items: Vec<T>,
}

impl<T> QueriableDataStore<T> {
    ///Get all entries of the [DataStore](QueriableDataStore) that match the filter.
    pub fn filter(&self, filter: DataFilter) -> impl Iterator<Item = &T> {
        filter.indices.into_iter().map(move |v| &self.items[v])
    }

    ///Get a new [Index](SortedIndex) for the [DataStore](QueriableDataStore) for the provided key.
    pub fn get_index<F, U>(&self, index_provider: F) -> SortedIndex<U>
    where
        F: Fn(&T) -> U,
        U: Ord,
    {
        SortedIndex::new(self, index_provider)
    }

    ///Iterate over all items in the [DataStore](QueriableDataStore).
    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> From<Vec<T>> for QueriableDataStore<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items }
    }
}

///Index of a [DataStore](QueriableDataStore).
#[derive(Clone, Eq, PartialEq)]
pub struct SortedIndex<T> {
    pairs: BTreeMap<T, Vec<usize>>,
}

impl<T> SortedIndex<T>
where
    T: Ord,
{
    ///Creates a new [Index](SortedIndex) from a [DataStore](QueriableDataStore) for the index provided by the index_provider function.
    pub fn new<F, U>(data_store: &QueriableDataStore<U>, index_provider: F) -> Self
    where
        F: Fn(&U) -> T,
    {
        let mut pairs: BTreeMap<T, Vec<usize>> = BTreeMap::new();

        for (index, item) in data_store.items().enumerate() {
            let key = index_provider(item);
            pairs.entry(key).or_insert_with(|| vec![]).push(index);
        }

        Self { pairs }
    }

    ///Get a new [DataFilter](DataFilter) for all items in the given range.
    pub fn filter_range<R>(&self, range: R) -> DataFilter
    where
        R: RangeBounds<T>,
    {
        let filtered = self
            .pairs
            .range(range)
            .into_iter()
            .flat_map(|(_, indices)| indices.iter().cloned());
        DataFilter::from_unsorted(filtered)
    }

    ///Get a new [DataFilter](DataFilter) for all items between the given values (including lower and upper value).
    pub fn filter_between(&self, lower_inclusive: T, upper_inclusive: T) -> DataFilter {
        self.filter_range((Included(lower_inclusive), Included(upper_inclusive)))
    }

    ///Get a new [DataFilter](DataFilter) for all items that are equivalent to the given value.
    pub fn filter_eq(&self, value: T) -> DataFilter {
        if let Some(keys) = self.pairs.get(&value) {
            DataFilter::from_unsorted(keys.iter().cloned())
        } else {
            DataFilter::default()
        }
    }

    ///Get a new [DataFilter](DataFilter) for all items that are greater than the given value.
    pub fn filter_gt(&self, lower_limit: T) -> DataFilter {
        self.filter_range((Excluded(lower_limit), Unbounded))
    }

    ///Get a new [DataFilter](DataFilter) for all items that are greater than ore equal to the given value.
    pub fn filter_gte(&self, lower_limit: T) -> DataFilter {
        self.filter_range((Included(lower_limit), Unbounded))
    }

    ///Get a new [DataFilter](DataFilter) for all items that are less than the given value.
    pub fn filter_lt(&self, upper_limit: T) -> DataFilter {
        self.filter_range((Unbounded, Excluded(upper_limit)))
    }

    ///Get a new [DataFilter](DataFilter) for all items that are less than ore equal to the given value.
    pub fn filter_lte(&self, upper_limit: T) -> DataFilter {
        self.filter_range((Unbounded, Included(upper_limit)))
    }
}

///Contains all items that match a given filter.
///Can be combined with the bitwise logical operators (& |).
#[derive(Default)]
pub struct DataFilter {
    indices: Vec<usize>,
}

impl DataFilter {
    ///Creates a [DataFilter](DataFilter) from an unsorted list of indices.
    fn from_unsorted<T>(unsorted_indices: T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        let mut indices: Vec<usize> = unsorted_indices.collect();
        indices.sort();
        Self { indices }
    }

    fn indices<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.indices.iter().cloned()
    }
}

impl BitAnd for DataFilter {
    type Output = DataFilter;

    fn bitand(self, other: DataFilter) -> Self::Output {
        let mut indices: Vec<usize> = vec![];
        let mut other_iterator = other.indices().peekable();

        for index in self.indices() {
            while other_iterator.peek().and_then(|v| Some(*v < index)) == Some(true) {
                other_iterator.next();
            }
            if other_iterator.peek().cloned() == Some(index) {
                indices.push(index);
            }
        }

        Self { indices }
    }
}

impl BitOr for DataFilter {
    type Output = DataFilter;

    fn bitor(self, other: DataFilter) -> Self::Output {
        let mut indices: Vec<usize> = vec![];
        let mut other_iterator = other.indices().peekable();

        for index in self.indices() {
            loop {
                if let Some(peek) = other_iterator.peek() {
                    if *peek < index {
                        indices.push(*peek);
                        other_iterator.next();
                    } else if *peek == index {
                        other_iterator.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            indices.push(index);
        }

        for index in other_iterator {
            indices.push(index);
        }

        Self { indices }
    }
}

use std::{
    collections::BTreeMap,
    ops::{Bound::*, RangeBounds},
};

pub struct QueriableDataStore<T> {
    items: Vec<T>,
}

impl<T> QueriableDataStore<T> {
    pub fn filter<F>(&self, filter_iterator: F) -> impl Iterator<Item = &T>
    where
        F: Into<Vec<DataFilter>>,
    {
        let mut filters: Vec<DataFilter> = filter_iterator.into();
        filters.sort_by_key(|v| v.indices.len());
        let mut filter_iterators: Vec<_> = filters
            .iter()
            .skip(1)
            .map(|v| v.indices.iter().peekable())
            .collect();
        let mut indices: Vec<usize> = vec![];

        let mut check_index_is_in_other_filters = |index: usize| {
            for iter in filter_iterators.iter_mut() {
                if iter.peek().cloned().cloned() != Some(index) {
                    loop {
                        if let Some(filter_index) = iter.next().cloned() {
                            if filter_index > index {
                                return false;
                            } else if filter_index == index {
                                break;
                            }
                        }
                    }
                }
            }
            true
        };

        if filters.len() > 0 {
            for item in filters[0].indices.iter().cloned() {
                if check_index_is_in_other_filters(item) {
                    indices.push(item);
                }
            }
        }
        indices.into_iter().map(move |v| &self.items[v])
    }

    pub fn get_index<F, U>(&self, index_provider: F) -> SortedIndex<U>
    where
        F: Fn(&T) -> U,
        U: Ord,
    {
        SortedIndex::new(self, index_provider)
    }

    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> From<Vec<T>> for QueriableDataStore<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SortedIndex<T> {
    pairs: BTreeMap<T, Vec<usize>>,
}

impl<T> SortedIndex<T>
where
    T: Ord,
{
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

    pub fn filter_between(&self, lower_inclusive: T, upper_inclusive: T) -> DataFilter {
        self.filter_range((Included(lower_inclusive), Included(upper_inclusive)))
    }

    pub fn filter_eq(&self, value: T) -> DataFilter {
        if let Some(keys) = self.pairs.get(&value) {
            DataFilter::from_unsorted(keys.iter().cloned())
        } else {
            DataFilter::default()
        }
    }

    pub fn filter_gt(&self, lower_limit: T) -> DataFilter {
        self.filter_range((Excluded(lower_limit), Unbounded))
    }

    pub fn filter_gte(&self, lower_limit: T) -> DataFilter {
        self.filter_range((Included(lower_limit), Unbounded))
    }

    pub fn filter_lt(&self, upper_limit: T) -> DataFilter {
        self.filter_range((Unbounded, Excluded(upper_limit)))
    }

    pub fn filter_lte(&self, upper_limit: T) -> DataFilter {
        self.filter_range((Unbounded, Included(upper_limit)))
    }
}

#[derive(Default)]
pub struct DataFilter {
    pub indices: Vec<usize>,
}

impl DataFilter {
    fn from_unsorted<T>(unsorted_indices: T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        let mut indices: Vec<usize> = unsorted_indices.collect();
        indices.sort();
        Self { indices }
    }
}

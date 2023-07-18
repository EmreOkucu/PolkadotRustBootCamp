struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

fn custom_filter<T: PartialEq>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> {
    let mut result = Vec::new();

    for item in collection {
        if condition.is_match(&item) {
            result.push(item);
        }
    }

    result
}

fn main() {
    let my_collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { condition: 5 };

    let filtered_collection = custom_filter(my_collection, &filter_condition);

    println!("{:?}", filtered_collection);
}

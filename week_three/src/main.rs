// Define a struct that will hold the filter condition.
struct FilterCondition<T> {
    condition: T,
}

// Implement a method for the FilterCondition struct.
impl<T: PartialEq> FilterCondition<T> {
    // The method takes a reference to an item and checks if it matches the filter condition.
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

// Define a function that filters a collection based on a given condition.
fn custom_filter<T: PartialEq>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> {
    // Create an empty vector to hold the filtered items.
    let mut result = Vec::new();

    // Iterate over the items in the collection.
    for item in collection {
        // If the item matches the filter condition, add it to the result vector.
        if condition.is_match(&item) {
            result.push(item);
        }
    }

    // Return the vector with the filtered items.
    result
}

fn main() {
    // Create a collection (a vector, in this case).
    let my_collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Initialize a FilterCondition object with the desired filter condition.
    let filter_condition = FilterCondition { condition: 5 };

    // Call the custom_filter function with the collection and the FilterCondition object,
    // and store the result in a new variable.
    let filtered_collection = custom_filter(my_collection, &filter_condition);

    // Print the filtered collection to the console.
    println!("{:?}", filtered_collection);
}

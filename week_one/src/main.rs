// Function to concatenate two strings
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    // Create two string variables and initialize them with values
    let string1 = String::from("Emre ");
    let string2 = String::from("Okucu");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated string to the console
    println!("{}", concatenated_string);
}

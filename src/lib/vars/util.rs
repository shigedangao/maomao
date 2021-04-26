use toml::{
    Value,
    value::Map
};

/// Table To Toml Inline Array
///
/// # Description
/// Convert a toml inline table to an inline table string representation
/// i.e: foo = {boo = "bar"}
///
/// By default value.to_string() will generate [[]] => boo = "bar" (next line)
/// This util method will output { boo = "bar" }
///
/// # Arguments
/// * `table` - &Map<String, Value>
///
/// # Return
/// String 
pub fn table_to_toml_inline_array(table: &Map<String, Value>) -> String {
    let mut str = Vec::new();
    for (key, value) in table.into_iter() {
        str.push(format!(" {} = {}", key, value.to_string()));
    }

    let mut jointed_vec = str.join(",");
    jointed_vec.insert(0, '{');
    jointed_vec.push_str(" }");

    jointed_vec
}
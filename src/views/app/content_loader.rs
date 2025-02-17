use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap().to_string()
}

pub fn add_component(component_flag: String, html_data: String) -> String {
    let css_tag = component_flag.to_uppercase() + "_CSS";
    let html_tag = component_flag.to_uppercase() + "_HTML";
    let css_path = format!("./templates/components/{}.css", component_flag.to_lowercase());
    let css_loaded = read_file(&css_path);
    let html_path = format!("./templates/components/{}.html", component_flag.to_lowercase());
    let html_loaded = read_file(&html_path);
    html_data.replace(&html_tag, &html_loaded).replace(&css_tag, &css_loaded)
}
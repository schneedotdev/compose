use std::{fs::File, path::Path, io::Write};
use super::args::{Entity, StructProperties};

pub fn generate_code(name: String, template: String) -> () {
    let output_dir = "dist";

    // Create the output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        std::fs::create_dir(output_dir).expect("Failed to create output directory");
    }

    let filename = format!("{}/{}.rs", output_dir, name);
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(template.as_bytes()).expect("Failed to write to file");
}

pub fn create_template(entity: Entity) -> (String, String) {
    let (name, template) = match entity {
        Entity::Struct(props) => (props.name.clone(), create_struct(props)),
    };

    return (name, template);
}

pub fn create_struct(props: StructProperties) -> String {
    format!(
        r#"pub struct {0} {{
    // Add fields and methods
}}

impl {0} {{
    pub fn new() -> Self {{
        {0} {{
            // Initialize fields
        }}
    }}
}}
"#,
        props.name
    )
}
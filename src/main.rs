use serde_derive::{Deserialize};
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String,
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

fn main() {
    let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs">Test</Item>
        </Project>
    "##;
    let project: Project = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", project);
}
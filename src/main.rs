use serde_derive::{Deserialize};
use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::Read;

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
    let b = do_des("D:/xmlreader/src/file.xml");
    match b {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing xml: {:?}", e),
    };
}

#[derive(Debug, Deserialize)]
struct To {
    #[serde(rename = "$value")]
    pub value: String
}

fn do_des(filename: &str) -> Result<To, Box<Error>> {
    let mut doc = File::open(filename)?;
    let mut doc_str = String::new();
    doc.read_to_string(&mut doc_str)?;
    if let Some(idl_ix) = doc_str.find("<to>") {
        let idlist: To = from_reader(doc_str[idl_ix..].as_bytes())?;
        Ok(idlist)
    } else {
        Err("no <to> in XML document")?
    }
}
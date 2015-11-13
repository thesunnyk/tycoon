/*
 * SVG Path Parser
 */

extern crate xml;

use std::path::Path;
use std::fs::File;

use self::xml::reader::EventReader;
use self::xml::reader::XmlEvent;

pub fn get_paths(path: &str) -> Vec<String> {
    let mut f = File::open(path).unwrap();

    let mut reader = EventReader::new(f);

    let mut v = Vec::<String>::new();

    for event in reader {
        let edata = event.unwrap();
        match edata {
            XmlEvent::StartElement { name, attributes, .. } => if &name.local_name == "path" {
                for attr in attributes {
                    if attr.name.local_name == "d" {
                        v.push(attr.value)
                    }
                }
            },
            _ => ()

        }
    }

    v
}

extern crate proc_macro;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
struct Tileset {
    first_gid: u32,
    source: String,
    image_source: String,
}

#[derive(Debug, Clone)]
struct Layer {
    id: u32,
    name: String,
    width: u32,
    height: u32,
    data: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Object {
    id: u32,
    class: String,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct ObjectGroup {
    id: u32,
    name: String,
    class: String,
    objects: Vec<Object>,
}

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
    object_groups: Vec<ObjectGroup>,
}

fn parse_tmx_file(file_path: &str) -> Map {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let parser = EventReader::new(contents.as_bytes());

    let mut map = Map {
        width: 0,
        height: 0,
        tilewidth: 0,
        tileheight: 0,
        tilesets: Vec::new(),
        layers: Vec::new(),
        object_groups: Vec::new(),
    };

    let mut current_layer = Layer {
        id: 0,
        name: String::new(),
        width: 0,
        height: 0,
        data: Vec::new(),
    };

    let mut current_object_group = ObjectGroup {
        id: 0,
        name: String::new(),
        class: String::new(),
        objects: Vec::new(),
    };

    let mut current_object = Object {
        id: 0,
        class: String::new(),
        x: 0.0,
        y: 0.0,
    };

    let mut in_layer_data = false;
    let mut in_object_group = false;
    let mut in_object = false;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "map" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "width" => map.width = attr.value.parse().unwrap(),
                            "height" => map.height = attr.value.parse().unwrap(),
                            "tilewidth" => map.tilewidth = attr.value.parse().unwrap(),
                            "tileheight" => map.tileheight = attr.value.parse().unwrap(),
                            _ => {}
                        }
                    }
                }
                "tileset" => {
                    let mut first_gid = 0;
                    let mut source = String::new();
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "firstgid" => first_gid = attr.value.parse().unwrap(),
                            "source" => source = attr.value.clone(),
                            _ => {}
                        }
                    }
                    let image_source = parse_tsx_file(&source);
                    map.tilesets.push(Tileset {
                        first_gid,
                        source,
                        image_source,
                    });
                }
                "layer" => {
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "id" => current_layer.id = attr.value.parse().unwrap(),
                            "name" => current_layer.name = attr.value.clone(),
                            "width" => current_layer.width = attr.value.parse().unwrap(),
                            "height" => current_layer.height = attr.value.parse().unwrap(),
                            _ => {}
                        }
                    }
                }
                "data" => {
                    in_layer_data = true;
                }
                "objectgroup" => {
                    in_object_group = true;
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "id" => current_object_group.id = attr.value.parse().unwrap(),
                            "name" => current_object_group.name = attr.value.clone(),
                            "class" => current_object_group.class = attr.value.clone(),
                            _ => {}
                        }
                    }
                }
                "object" => {
                    in_object = true;
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "id" => current_object.id = attr.value.parse().unwrap(),
                            "x" => current_object.x = attr.value.parse().unwrap(),
                            "y" => current_object.y = attr.value.parse().unwrap(),
                            "type" => current_object.class = attr.value.clone(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::Characters(data)) => {
                if in_layer_data {
                    current_layer.data =
                        data.split(',').map(|s| s.trim().parse().unwrap()).collect();
                    map.layers.push(current_layer.clone());
                    in_layer_data = false;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "layer" {
                    current_layer = Layer {
                        id: 0,
                        name: String::new(),
                        width: 0,
                        height: 0,
                        data: Vec::new(),
                    };
                } else if name.local_name == "objectgroup" {
                    in_object_group = false;
                    map.object_groups.push(current_object_group.clone());
                    current_object_group = ObjectGroup {
                        id: 0,
                        name: String::new(),
                        class: String::new(),
                        objects: Vec::new(),
                    };
                } else if name.local_name == "object" {
                    in_object = false;
                    current_object_group.objects.push(current_object.clone());
                    current_object = Object {
                        id: 0,
                        class: String::new(),
                        x: 0.0,
                        y: 0.0,
                    };
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    map
}

fn parse_tsx_file(file_path: &str) -> String {
    let path = PathBuf::from("assets/tilesets").join(file_path);
    let mut file = File::open(path).expect("Unable to open tsx file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read tsx file");

    let parser = EventReader::new(contents.as_bytes());
    let mut image_source = String::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "image" {
                    for attr in attributes {
                        if attr.name.local_name == "source" {
                            image_source = attr.value.clone();
                            // Remove leading "../" if it exists
                            if image_source.starts_with("../") {
                                image_source = image_source[3..].to_string();
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    image_source
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("generated_code.rs");

    let map = parse_tmx_file("assets/maps/map1.tmx");

    let tilesets: Vec<TokenStream> = map
        .tilesets
        .iter()
        .map(|tileset| {
            let first_gid = tileset.first_gid;
            let source = &tileset.source;
            let image_source = &tileset.image_source;
            quote! {
                Tileset { first_gid: #first_gid, source: #source.to_string(), image_source: #image_source.to_string() }
            }
        })
        .collect();

    let layers: Vec<TokenStream> = map
        .layers
        .iter()
        .map(|layer| {
            let id = layer.id;
            let name = &layer.name;
            let width = layer.width;
            let height = layer.height;
            let data = &layer.data;
            quote! {
                Layer {
                    id: #id,
                    name: #name.to_string(),
                    width: #width,
                    height: #height,
                    data: vec![#(#data),*],
                }
            }
        })
        .collect();

    let object_groups: Vec<TokenStream> = map
        .object_groups
        .iter()
        .map(|object_group| {
            let id = object_group.id;
            let name = &object_group.name;
            let class = &object_group.class;
            let objects: Vec<TokenStream> = object_group
                .objects
                .iter()
                .map(|object| {
                    let obj_id = object.id;
                    let class = &object.class;
                    let x = object.x;
                    let y = object.y;
                    quote! {
                        Object { id: #obj_id, class: #class.to_string(), x: #x, y: #y }
                    }
                })
                .collect();
            quote! {
                ObjectGroup {
                    id: #id,
                    name: #name.to_string(),
                    class: #class.to_string(),
                    objects: vec![#(#objects),*],
                }
            }
        })
        .collect();

    let map_width = map.width;
    let map_height = map.height;
    let map_tilewidth = map.tilewidth;
    let map_tileheight = map.tileheight;

    let generated_code = quote! {
        use once_cell::sync::Lazy;

        #[derive(Debug, Clone)]
        struct Tileset {
            first_gid: u32,
            source: String,
            image_source: String,
        }

        #[derive(Debug, Clone)]
        struct Layer {
            id: u32,
            name: String,
            width: u32,
            height: u32,
            data: Vec<u32>,
        }

        #[derive(Debug, Clone)]
        struct Object {
            id: u32,
            class: String,
            x: f32,
            y: f32,
        }

        #[derive(Debug, Clone)]
        struct ObjectGroup {
            id: u32,
            name: String,
            class: String,
            objects: Vec<Object>,
        }

        #[derive(Debug)]
        struct Map {
            width: u32,
            height: u32,
            tilewidth: u32,
            tileheight: u32,
            tilesets: Vec<Tileset>,
            layers: Vec<Layer>,
            object_groups: Vec<ObjectGroup>,
        }

        static MAP: Lazy<Map> = Lazy::new(|| Map {
            width: #map_width,
            height: #map_height,
            tilewidth: #map_tilewidth,
            tileheight: #map_tileheight,
            tilesets: vec![#(#tilesets),*],
            layers: vec![#(#layers),*],
            object_groups: vec![#(#object_groups),*],
        });
    };

    let mut file = File::create(dest_path).unwrap();
    write!(file, "{}", generated_code).unwrap();
}

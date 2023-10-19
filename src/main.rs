use std::fs;

use patternizer::digraph;

const RESOURCE_DIR: &'static str = "res";
const OUTPUT_DIR: &'static str = "out";

fn main() {
    {
        let filename = "IMG_5653.JPG";
        let filepath = format!("{RESOURCE_DIR}/{filename}");
        let pattern_img_filename = format!("{OUTPUT_DIR}/{filename}.raw.out.png");

        let img = image::open(filepath).unwrap();
        let bytes = img.as_rgb8().unwrap();
        let output_bytes = digraph::create_digraph_bytes(bytes);
        let _save_result = image::save_buffer(
            &pattern_img_filename,
            &output_bytes,
            digraph::DIM as u32,
            digraph::DIM as u32,
            image::ColorType::Rgba8,
        );
    }
    {
        let filename = "paten.png";
        let filepath = format!("{RESOURCE_DIR}/{filename}");
        let pattern_img_filename = format!("{OUTPUT_DIR}/{filename}.raw.out.png");

        let img = image::open(filepath).unwrap();
        let bytes = img.as_rgba8().unwrap();
        let output_bytes = digraph::create_digraph_bytes(bytes);
        let _save_result = image::save_buffer(
            &pattern_img_filename,
            &output_bytes,
            digraph::DIM as u32,
            digraph::DIM as u32,
            image::ColorType::Rgba8,
        );
    }
    {
        let filename = "paten-mini.png";
        let filepath = format!("{RESOURCE_DIR}/{filename}");
        let pattern_img_filename = format!("{OUTPUT_DIR}/{filename}.raw.out.png");

        let img = image::open(filepath).unwrap();
        let bytes = img.as_rgba8().unwrap();
        let output_bytes = digraph::create_digraph_bytes(bytes);
        let _save_result = image::save_buffer(
            &pattern_img_filename,
            &output_bytes,
            digraph::DIM as u32,
            digraph::DIM as u32,
            image::ColorType::Rgba8,
        );
    }
    {
        let filename = "architecture.png";
        let filepath = format!("{RESOURCE_DIR}/{filename}");
        let pattern_img_filename = format!("{OUTPUT_DIR}/{filename}.raw.out.png");

        let img = image::open(filepath).unwrap();
        let bytes = img.as_rgb8().unwrap();
        let output_bytes = digraph::create_digraph_bytes(bytes);
        let _save_result = image::save_buffer(
            &pattern_img_filename,
            &output_bytes,
            digraph::DIM as u32,
            digraph::DIM as u32,
            image::ColorType::Rgba8,
        );
    }

    for entry in fs::read_dir(RESOURCE_DIR).unwrap() {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            let pattern_img_filename = format!("{OUTPUT_DIR}/{filename}.out.png");

            match fs::read(path) {
                Ok(bytes) => {
                    let output_bytes = digraph::create_digraph_bytes(&bytes);
                    let save_result = image::save_buffer(
                        &pattern_img_filename,
                        &output_bytes,
                        digraph::DIM as u32,
                        digraph::DIM as u32,
                        image::ColorType::Rgba8,
                    );
                    if let Err(e) = save_result {
                        println!("Error saving file [{}]: {:?}", &pattern_img_filename, e);
                    }
                }
                Err(e) => {
                    println!("Error reading file [{}]: {:?}", &filename, e);
                }
            }
        }
    }
}

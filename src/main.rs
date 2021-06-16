use std::path::Path;

#[derive(Debug)]
pub enum Stanice {
    BROD,
    STAVIDLA
}


fn main() {
    use std::fs;
    use walkdir::WalkDir;
    use std::env;

    let args: Vec<String> = env::args().collect();

    for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok())
         .filter(|x| x.path().to_str().unwrap().ends_with("jpg") ) {
         read_jpeg(&entry.path());
     }
}

fn read_jpeg(path: &Path) {
    use std::fs::File;
    use std::io::BufReader;
    use jpeg_decoder::Decoder;

    let file = File::open(path).expect("failed to open file");
    let mut decoder = Decoder::new(BufReader::new(file));
    decoder.read_info();
    let metadata = decoder.info().unwrap();

    let x = if (metadata.width as f32 / metadata.height as f32) > 1.0 {
        Stanice::BROD
    } else {
        Stanice::STAVIDLA
    };


    println!("{:?}", x);

}

fn show_focal_length(path: &str) -> Option<i32> {
    use exif::{In, Tag};
    let file = std::fs::File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let field = exif.get_field(Tag::FocalLengthIn35mmFilm, In::PRIMARY);

    for f in field {
        let ggg = f.value.get_uint(0).unwrap();
        println!("{}", ggg);
    }
    Option::Some(10)
}
pub enum Stanice {
    BROD,
    STAVIDLA
}


fn main() {
    use std::fs;
    use walkdir::WalkDir;
    use std::env;

    let args: Vec<String> = env::args().collect();

    for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok()) {
        show_focal_length(entry.path().to_str().unwrap());
    }

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
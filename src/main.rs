use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use regex::Regex;

#[derive(Debug)]
pub enum Stanice {
    BROD,
    STAVIDLA
}


fn main() {
    use walkdir::WalkDir;
    use std::env;
    let args: Vec<String> = env::args().collect();

    let re = Regex::new(r"^brod_(\d{4})(\d{2})(\d{2})(\d{2})(\d{2})\.jpg$").unwrap();

    for entry in WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok())
        .filter(|x| x.path().to_str().unwrap().ends_with("jpg") ) {
        let x = read_jpeg(&entry.path());
        match x {
            io::Result::Ok(Stanice::STAVIDLA) => {
                let format = format_stavidla(&re, &args[2], &entry.path());
                match format {
                    Option::Some(path) => {
                        println!("Prejnemovavam soubor : {}", path);
                    }
                    _ => println!("Nemohu formatovat cestu")
                }
            },
            io::Result::Ok(Stanice::BROD) => println!("ignoruji, stanice brod"),
            x => {
                println!("jina stanice nez brod nebo chyba {:?}", x)
            }
        }
     }
}


fn copy_file(source: &Path, _dest: &str) {

    let dest = Path::new(_dest);
    let dest_dir = dest.parent().unwrap();
    if !dest_dir.exists() {
        std::fs::create_dir_all(dest_dir).unwrap();
    }
    std::fs::rename(source, &dest).unwrap();

}

fn format_stavidla(re: &Regex, path_prefix: &str, path: &Path) -> Option<String> {
    let filename = path.file_name()?.to_str()?;
    let capture_groups = re.captures(filename)?;
    //println!("{:?}", capture_groups);

    let stavidla_path = format!("{}/{}/{}/{}/brod-stavidla_{}{}{}{}{}.jpg", path_prefix,
                                &capture_groups[1], &capture_groups[2], &capture_groups[3],
                                &capture_groups[1], &capture_groups[2], &capture_groups[3], &capture_groups[4], &capture_groups[5]
    );
    //println!("{}", stavidla_path);

    Option::Some(stavidla_path.to_string())
}


fn read_jpeg(path: &Path) -> io::Result<Stanice> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::ErrorKind;

    use jpeg_decoder::Decoder;

    let file = File::open(path)?;
    let mut decoder = Decoder::new(BufReader::new(file));
    let _ = decoder.read_info().map_err(map_read_error)?;
    let metadata = decoder.info().ok_or(Error::new(ErrorKind::Other, "Unable to get info"))?;

    if (metadata.width as f32 / metadata.height as f32) > 1.0 {
        io::Result::Ok(Stanice::BROD)
    } else {
        io::Result::Ok(Stanice::STAVIDLA)
    }

}

fn map_read_error(err: jpeg_decoder::Error) -> std::io::Error {
    Error::new(ErrorKind::Other, format!("Nemohu precist ifo JPEG {}", err))
}


// to generate Rust structs from .proto file use:
// pb-rs.exe src/mangaplus_pb.proto -D --owned
// the -owned and -D flags are very important to make it work, since not using them will make it
// unable to return the proto structs to use them out of the API functions
use std::io::prelude::*;
use clap::{App, Arg};

mod mangaplus_api;
mod mangaplus_pb;

fn main() {
    let args = clap::App::new("mplusdownloader-rs")
                .version("0.1")
                .arg(clap::Arg::with_name("chapter_id")
                    .short("c")
                    .help("chapter_id of the chapter to be downloaded")
                    .required(false)
                    .takes_value(true))
                .arg(clap::Arg::with_name("ouput_dir")
                    .short("d")
                    .help("Directory to write downloaded files")
                    .required(false)
                    .takes_value(true))
                .get_matches();
    
    let chapter_id = args.value_of("chapter_id");
    let output_dir = args.value_of("ouput_dir").unwrap_or("./");

    let mut mplus = mangaplus_api::MangaPlusClient::new();
    if let Ok(device_secret) = mplus.get_device_secret() {
        println!("Got device_secret={}", device_secret);
        //println!("{:?}", mplus.get_title_details(100003));
        if let Some(chapter_id) = chapter_id {
            if let Ok(pages) = mplus.download_pages(chapter_id.parse().unwrap()) {
                let m_viewer = pages.0.clone();
                let chapter_name = m_viewer.chapter_name.unwrap();
                for (p_num, page) in pages.clone().1.iter().enumerate() {
                    let filename = format!("{}/{}_{}_{}.jpg", output_dir, chapter_name, chapter_id, p_num);
                    if let Ok(mut f) = std::fs::File::create(filename.clone()) {
                        println!("Writing page {}/{} (in {})", p_num, pages.1.len(), filename);
                        f.write_all(page.as_slice());
                    }
                }
            }
        }
    }
}

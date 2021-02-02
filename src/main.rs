extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;
use std::path::PathBuf;
use actix_multipart::Multipart;
use actix_web::{
    web::{post, resource, Data},
    App, HttpResponse, HttpServer,
};
use form_data::{handle_multipart, Error, Field, FilenameGenerator, Form};
use futures::Future;
use std::error::Error as E;
use std::fs::File;
use std::process;

struct FileNamer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    ci: u64,
    name: String,
    gender: char,
    date: String,
    state: String,
    number_phon: String,
    address: String,
    email: String,
}


impl FilenameGenerator for FileNamer {
    fn next_filename(&self, _: &mime::Mime) -> Option<PathBuf> {
        let mut p = PathBuf::new();
        p.push(format!("uploaded-csvfile/{}.csv", "file"));
        Some(p)
    }
}
fn run() -> Result<(), Box<dyn E>> {
    let file = File::open("C:/Users/Kuris/postgre-migration/uploaded-csvfile/file.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


fn upload((mp, state): (Multipart, Data<Form>)) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        handle_multipart(mp, state.get_ref().clone()).map(|uploaded_content| {
            println!("Uploaded Content: {:?}", uploaded_content);
            if let Err(err) = run() {
                println!("{}", err);
                process::exit(1);
            }
            HttpResponse::Created().finish()
        }),
    )
}

fn main() -> Result<(), failure::Error> {
    let form = Form::new()
        .field("files", Field::array(Field::file(FileNamer)));
 
    println!("{:?}", form);
    HttpServer::new(move || {
        App::new()
            .data(form.clone())
            .service(resource("/upload").route(post().to(upload)))
    })
        .bind("127.0.0.1:8080").unwrap()
        .run()
        .unwrap();
 
    Ok(())
}
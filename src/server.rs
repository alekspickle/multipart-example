//! ## Server
//! Separate server setup for easier multi-server setup.
//!
//!
//!

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, App, HttpServer, HttpResponse, Responder};
use futures::future::{err, Either};
use futures::{Future, Stream};
use web::{Data, Form, get, post, resource, route};

use std::fs::File;
use std::io::{ErrorKind,Write};
use std::time::{Duration, SystemTime};

///Server struct for each server to create
pub struct Server {
    pub name: String,
    pub address: String,
    pub port: String,
}


impl Server {
    ///start function:
    /// - define app data
    /// - define routes
    /// - bind domain and port to server instance
    /// - run the server
    pub fn start(&mut self) {
        let path = self.address.to_owned() + ":" + &self.port;

        let server = HttpServer::new(|| {
            App::new()
                .service(resource("/").route(get().to(|| HttpResponse::Ok())))
                .service(
                    resource("/multipart_image")
                        .route(get().to(multipart_image))
                        .route(post().to_async(load_image)),
                )
                // set default route to 404
                .default_service(route().to(p404))
        })
        .bind(path)
        .expect(&format!("{}{}", "Could not bind to port ", &self.port));

        println!("Server is running on {}:{}", &self.address, &self.port);

        match server.run() {
            Ok(_) => println!("\nServer is gracefully shut down"),
            Err(why) => println!("There was a problem stoping the server: {}", why),
        };
    }
}
///process multipart image file
pub fn load_image(
    multipart: Multipart,
) -> impl Future<Item = impl Responder, Error = Error> {
    println!("load image process initiated.  ",);

    //actually upload it to the server
    upload(multipart)
}


pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let base = "downloads/upload_".to_owned();
    let code = match SystemTime::now().duration_since(<std::time::SystemTime>::UNIX_EPOCH) {
        Ok(now) => now,
        Err(_) => Duration::new(0, 0),
    };
    let file_path_string = base.clone() + &code.as_millis().to_string() + ".png";
    let file = match File::create(file_path_string.clone()) {
        Ok(file) => file,
        Err(match_e) => {
            if match_e.kind() == ErrorKind::NotFound {
                println!("Create 'downloads' directory in the root of the project please");
                File::create(file_path_string).expect("Second file create attempt failed")
            } else {
                return Either::A(err(error::ErrorInternalServerError(match_e)));
            }
        }
    };

    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    multipart: Multipart,
) -> impl Future<Item = impl Responder, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|_sizes| multipart_image())
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

///multipart request image page
pub fn multipart_image() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Ready for multipart upload!\n"))
}

///404 page
pub fn p404() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound()
.body("Not found, try another one!\n"))
}
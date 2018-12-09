extern crate base64;
extern crate term_painter;
extern crate tiny_http;

use self::base64::encode;
use self::term_painter::Color::*;
use self::term_painter::ToStyle;
use self::tiny_http::Server;
use spotify_api::EasyAPI;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;
use std::thread;

pub fn retrieve_tokens(handle: &mut EasyAPI) -> Result<(), std::io::Error> {
    println!(
        "{}",
        Red.bold()
            .paint("Automatic token retrieval procedure activated")
    );
    println!("{}", Blue.paint("Enter the clientId of the application"));
    let clientid: String = text_io::read!("{}\n");
    // TODO Only ask secret, and rebuild base64 from that
    println!(
        "{}",
        Blue.paint("Enter the clientSecret of the application.")
    );
    let clientsecret: String = text_io::read!("{}\n");
    let base64 = encode(&format!("{}:{}", clientid, clientsecret));
    File::create("base_64_secret").unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .open("base_64_secret")
        .unwrap();
    f.write(base64.as_bytes()).unwrap();

    thread::spawn(move || {
        webbrowser::open(&format!("https://accounts.spotify.com/authorize/?client_id={}&response_type=code&redirect_uri=http%3A%2F%2Flocalhost:8000%2F&scope=user-read-private%20user-read-email%20playlist-read-private%20playlist-read-collaborative%20playlist-modify-public%20playlist-modify-private%20user-follow-modify%20user-follow-read%20user-library-read%20user-library-modify%20user-read-private%20user-read-birthdate%20user-read-email%20user-top-read%20ugc-image-upload%20user-read-playback-state%20user-modify-playback-state%20user-read-currently-playing%20user-read-recently-played",clientid.as_str())).unwrap();
    });
    let server = Server::http("localhost:8000").unwrap();
    let (tx, rx) = mpsc::channel();
    let thread_handle = thread::spawn(move || {
        for request in server.incoming_requests() {
            let code = (&request.url()[7..]).to_string();
            println!("token {}", code.as_str());
            tx.send(code).unwrap();
            let response = tiny_http::Response::from_string(
                "Success! You can now close this window!".to_string(),
            );
            let _ = request.respond(response);
            break;
        }
    });

    thread_handle.join().unwrap();

    let refresh_token = (*handle)
        .retrieve_refresh_token(base64, rx.recv().unwrap())
        .unwrap();

    File::create("refresh_token").unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .open("refresh_token")
        .unwrap();
    f.write(refresh_token.as_bytes()).unwrap();

    Ok(())
}

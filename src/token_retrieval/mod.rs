use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn retrieve_tokens() -> Result<(), std::io::Error> {
    println!("Automatic token retrieval procedure activated");
    println!("Enter the clientid of the application");
    let clientid: String = text_io::read!("{}\n");
    // TODO Only ask secret, and rebuild base64 from that
    println!("Enter the base64 of the clientid:clientsecret here. A browser will open and ask you to connect to your Spotify Account");
    println!("You'll have to copy the token code in the URL and to paste it here");
    let base64: String = text_io::read!("{}\n");
    File::create("base_64_secret").unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .open("base_64_secret")
        .unwrap();
    f.write(base64.as_bytes()).unwrap();

    webbrowser::open(&format!("https://accounts.spotify.com/authorize/?client_id={}&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%2F&scope=user-read-private%20user-read-email%20playlist-read-private%20playlist-read-collaborative%20playlist-modify-public%20playlist-modify-private%20user-follow-modify%20user-follow-read%20user-library-read%20user-library-modify%20user-read-private%20user-read-birthdate%20user-read-email%20user-top-read%20ugc-image-upload%20user-read-playback-state%20user-modify-playback-state%20user-read-currently-playing%20user-read-recently-played",clientid.as_str())).unwrap();
    println!("Paste now the token : ");
    let code: String = text_io::read!("{}\n");
    // TODO doesnt work from now !! We dont really have a refresh token but only a code. We need to POST a request
    // to have the token bacK
    // File::create("refresh_token").unwrap();
    // let mut f = OpenOptions::new()
    //     .append(true)
    //     .write(true)
    //     .open("refresh_token")
    //     .unwrap();
    // f.write(refresh_token.as_bytes()).unwrap();

    // easy_api.refresh().unwrap();
    Ok(())
}

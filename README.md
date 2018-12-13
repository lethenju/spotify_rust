# Spotify Rust

`spotify_rust` can control your Spotify playback without having to use your
mouse.

`spotify_rust` is made of two elements :

- An API for Spotify, `spotify_api` , controlling playback, searching for
  tracks, albums or playlists, and much more.
- A client in text user interface, `spotify_rust`, directly in your terminal, to
  browse your spotify Library and play tracks.

## Example for using the API

```rust
// Creating the handle
let mut handle = EasyAPI::new();

// Getting a refresh token
handle.refresh().unwrap();

// Getting the currently playing track and artist names, if there is a track playing right now
let current_artist_name = match easy_api.get_currently_playing_artist().unwrap() {
    Some(artist) => artist.name,
    None => "".to_string(),
};
let current_track_name = match easy_api.get_currently_playing_track().unwrap() {
    Some(track) => track.name,
    None => "".to_string(),
};

println!("You are now listening to {} by {}", current_track_name.as_str(), current_artist_name.as_str());
```

## Getting Started

Just clone the repository and `cargo run`!

Keep in mind before using this that we are in a very early stage, and drastic
changes can occur anytime. It is not recommended to use the API until the beta
stage.

### Prerequisites

Before you can run there are some prerequisites:

- The _Rust_ compiler
- A _Spotify client id/secret pair_, and a _refresh token_

#### Client ID / Secret

If you plan to use the API alone, you can register your application at
[Spotify](https://developer.spotify.com/dashboard/login). You will get your
spotify client id and secret pair.

Generate a base64 of `<client:id>:<client:secret>` and put the result in a file
named `base_64_secret` on the root of the repository.

If you plan to use only the terminal client, sorry but you will have to wait for
binary packages to be released.

#### Refresh token

To get the refresh token, for now its a bit more complex. You will have to open
a browser and paste that URL in your navigation bar with <YOUR_CLIENT_ID> as
your client ID.

```

https://accounts.spotify.com/authorize/?client_id=<YOUR_CLIENT_ID>&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%2Fcallback&scope=user-read-private%20user-read-email%20playlist-read-private%20playlist-read-collaborative%20playlist-modify-public%20playlist-modify-private%20user-follow-modify%20user-follow-read%20user-library-read%20user-library-modify%20user-read-private%20user-read-birthdate%20user-read-email%20user-top-read%20ugc-image-upload%20user-read-playback-state%20user-modify-playback-state%20user-read-currently-playing%20user-read-recently-played

```

Authorize the application to use your data, and you will be redirected to
localhost with a token in the URI. Then you will have to POST the token in a
specific request to finally have your refresh token.

All the procedure is documented here :
https://developer.spotify.com/documentation/general/guides/authorization-guide/

An automated process has been developed for the application. You will only have
to enter the cliend ID and secret, then log on in a friendly interface while all
the background token retrieval process is going on.

## Documentation

Run `cargo doc --open` to build and open the documentation on the browser
directly.

The data structures are the same as in the Spotify API , available
[here](https://developer.spotify.com/web-api/object-model)

You can find the rust structures in the `model` directory, under
`spotify_api/src/model`.

## Running the tests

`cd spotify_api` to get in the library module.

You can run the tests with `cargo test`.

## Built With

External crates:

- `base64` for encoding the application credentials pair.
- `clap` for the command line interpreter
- `curl` for sending HTTP requests.
- `failure`
- `percent-encoding` to encode search queries in percent encoding.
- `serde_json` for JSON parsing.
- `tiny_http` for the automated refresh token generation process
- `term-painter` for colorful stdout during the automated refresh token
  generation process
- `termion` for the text user interface
- `text_io` for easy user input
- `tui` for the text user interface
- `webbrowser` for the automated refresh token generation process

## Contributing

Thank you for your support !

Please read the [Contributing file](CONTRIBUTING.md) before contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

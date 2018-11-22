# Spotify Rust

Spotify_rust can control your Spotify playback without having to use your mouse.

spotify-rust is made of two elements :

- An API for Spotify, controlling playback, searching for tracks, albums or
  playlists, and much more
- A client in text user interface, directly in your terminal, to browse your
  spotify Library and play tracks.

## Getting Started

Just clone the repository and `cargo run` !

Keep in mind before using this that we are in a very early stage, and drastic
changes can occur anytime. It is not recommanded to use the API until the beta
stage.

### Prerequisites

Well before you can run there are some prerequisites :

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
localhost witha token in the URI. Put that token in a file named `refresh_token`
in the root folder of the repository and you should be ok!

I plan to automate that process as soon as possible to make it easier for you :)

## Documentation

### Data structures

#### Album

- name : String
- id : String
- tracks : Vec<Track> // TODO

#### Track

- name : String
- id : String

#### Artist

- name : String // TODO
- id : String // TODO
- albums : Vec<Album> // TODO

#### Playlist

- name : String // TODO
- id : String // TODO
- track : Vec<Track> // TODO

### APIs

#### Player

#### Library


## Running the tests

Some integration tests are being developped, the coverage isnt really perfect
for now. You can run the tests with `cargo test`

## Built With

External crates :

- `curl` for sending HTTP requests.
- `serde_json` for JSON parsing.
- `percent-encoding` to encode search queries in percent encoding.
- `termion` and `tui` for the text user interface
- `clap` for the command line interpreter

## Contributing

As I would love to have contributors, the project is unfortunately not mature
enough to be on the hands of the masses..

I would rather finish to get a strong basis on which contributors could expand
the software.

Feel free to open pull requests or mail me if you have any question.

## Authors

- **Julien LE THENO** - _Initial work_ - [lethenju](https://github.com/lethenju)

## License

This project is licensed under the MIT License - see the
[LICENSE.md](LICENSE.md) file for details

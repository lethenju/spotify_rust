# Spotify Rust

Spotify_rust can control your Spotify playback without having to use your mouse. 

spotify-rust is made of two elements :
- An API for Spotify, controlling playback, searching for tracks, albums or playlists, and much more
- A client in text user interface, directly in your terminal, to browse your spotify Library and play tracks.

## Getting Started

Just clone the repository and `cargo run` !

Keep in mind before using this that we are in a very early stage, and drastic changes can occur anytime. 
It is not recommanded to use the API until the beta stage.


### Prerequisites

Well before you can run there are some prerequisites :
- The *Rust* compiler 
- A *Spotify client id/secret pair*, and a *refresh token*


## Running the tests

Some integration tests are being developped, the coverage isnt really perfect for now.
You can run the tests with `cargo test`

## Built With

External crates :
- `curl` for sending HTTP requests. 
- `serde_json` for JSON parsing.
- `percent-encoding` to encode search queries in percent encoding.
- `termion` and `tui` for the text user interface 
- `clap` for the command line interpreter

## Contributing

As I would love to have contributors, the project is unfortunately not mature enough to be on the hands of the masses..

I would rather finish to get a strong basis on which contributors could expand the software.

Feel free to open pull requests or mail me if you have any question.


## Authors

* **Julien LE THENO** - *Initial work* - [lethenju](https://github.com/lethenju)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

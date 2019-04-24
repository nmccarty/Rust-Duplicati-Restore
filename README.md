# Rust Duplicati Restore

Rust program for duplicati disaster recovery quick, fast, and in a hurry.
Uses rayon to process files across many threads, to maximze restore speed.

## Getting Started

Simply run the rust-duplicati-restore from the commandline.
It doesn't accept any flags and will prompt you for all information.

### Prerequisites

You must have sqlite3 installed on your system for this program to function.


### Installing

Simply run
```
cargo build --release
```

Or download the latest binary from the artifacts

## Limitations

* Currently does not verify restored files
* Does not yet support encrypted backups, I reccomend combining aescrypt with gnu parallel for decryption
* Does not support remote repositories yet, I reccomend using rclone to pull donw a local copy


## Built With

* [Rust](https://www.rust-lang.org/) 
* [SQLite](https://www.sqlite.org)
* [Rayon](https://github.com/rayon-rs/rayon)
* And may more, see Cargo.toml for full list

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Ben Fisher - His python script included in the Duplicati reposistory inspired
  this project, and this project was roughly based on it.

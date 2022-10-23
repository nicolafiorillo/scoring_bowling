# Scoring Bowling Kata

Calculate score for bowling game.
Written in Rust.

## Build, run tests, and execute

```Bash
cargo build
cargo test
cargo run
```

## Using via Docker

### Build using docker image

```Bash
docker build -t scoring_bowling .
```
Note: could take some time to build the image.

### Run docker image

```Bash
docker run -i scoring_bowling
```
Note the `-i` flag to interact with console (STDIN is open).


### Remove built docker image

```Bash
docker rmi -i scoring_bowling:latest
```

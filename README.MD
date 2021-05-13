# Docker client

[![Build Status](https://travis-ci.org/tuplecats/docker-client.svg?branch=master)](https://travis-ci.org/tuplecats/docker-client)

## Usage

```toml
[dependencies]
docker_client = "0.1.0-alpha4"
```

## Examples
Create container example.

```rust
extern crate docker_client;

use docker_client::DockerClient;
use docker_client::container::Config;

fn main() {

    let client = DockerClient::new();

    let config = Config::with_image("alpine")
        .name("test")
        .build();


    match client.create_container(config) {
        Ok(c) => {
            println!("Container created: {:?}", c)
        },
        Err(e) => {
            println!("Error {:?}", e)
        }
    }

}
```
# Docker client

[![Build Status](https://travis-ci.org/tuplecats/docker-client.svg?branch=0.1.0-alpha1)](https://travis-ci.org/tuplecats/docker-client)

## Examples
Create container example.

```rust
use docker_client::DockerClient;
use docker_client::container::Creator;

fn main() {

    let client = match DockerClient::connect("/var/run/docker.sock") {
        Ok(client) => client,
        Err(e) => panic!("Cannot connect to socket!"),
    };

    let creator = Creator::with_image("alpine")
        .name(Some("test"))
        .build();


    match client.create_container(creator) {
        Ok(_) => {},
        Err(_) => {}
    }

}
```
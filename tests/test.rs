extern crate docker_client;

use docker_client::DockerClient;
use docker_client::container::{Remover, Killer, Creator};

fn client() -> DockerClient {
    DockerClient::connect("/var/run/docker.sock").unwrap()
}

#[test]
fn test_kill() {
    let client = client();

    let killer = Killer::new()
        .id("123")
        .signal("SIGABORT")
        .signal("dsdsd")
        .build();

    match client.kill_container(killer) {
        Ok(()) => {},
        Err(_) => {},
    }
}

#[test]
fn test_remove() {
    let client = client();

    let remover = Remover::new()
        .id("trait")
        .with_force_delete(false)
        .with_remove_link(false)
        .with_remove_volumes(false)
        .build();

    match client.remove_container(remover) {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_rename() {
    let client = client();

    match client.rename_container("purge", "purge1") {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_start() {
    let client = client();

    match client.start_container("12", "") {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_pause() {
    let client = client();

    match client.pause_container("123") {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_unpause() {
    let client = client();

    match client.unpause_container("123") {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_stop() {
    let client = client();

    match client.stop_container("123", None) {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[test]
fn test_fs_changes() {
    let client = client();

    match client.get_fs_changes("purge") {
        Ok(c) => {dbg!(c);},
        Err(_) => {}
    }
}

#[test]
fn test_create() {
    let client = client();

    let creator = Creator::with_image("alpine")
        .name("trait")
        .hostname("localhost")
        .domain_name("www.ddd.com")
        .network_disabled(false)
        .entry_point("hello")
        .expose_port("22/tcp")
        .build();


    match client.create_container(creator) {
        Ok(c) => {dbg!(c);},
        Err(_) => {}
    }
}
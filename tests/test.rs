extern crate docker_client;

use docker_client::{DockerClient, DockerError};
use docker_client::container::{Remover, Killer, Config, HealthCheck};

fn client() -> DockerClient {
    DockerClient::connect("/var/run/docker.sock")
}

#[test]
fn test_kill() {
    let client = client();

    let killer = Killer::new()
        .id("test")
        .signal("SIGINT")
        .build();

    match client.kill_container(killer) {
        Ok(()) => { println!("Container killed."); },
        Err(DockerError::BadParameters(m)) => { println!("Request bad parameters: {}.", m.message); },
        Err(DockerError::NotFound(m)) => { println!("Container not found: {}.", m.message); },
        Err(DockerError::NotRunning(m)) => { println!("Container not running: {}.", m.message); },
        Err(DockerError::ServerError(m)) => { println!("Server error: {}.", m.message); },
        Err(DockerError::UnknownStatus) => { println!("Unknown response status."); },
        Err(DockerError::ClosedConnection) => { println!("Connection closed."); },
        _ => {}
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

    let config = Config::with_image("alpine")
        .name("trait")
        .hostname("localhost")
        .domain_name("www.ddd.com")
        .network_disabled(false)
        .entry_point("hello")
        .expose_port("22/tcp")
        .build();


    match client.create_container(config) {
        Ok(c) => {dbg!(c);},
        Err(e) => {dbg!(e);},
    }
}

#[test]
fn test_inspect_container() {
    let client = client();

    match client.inspect_container("test", true) {
        Ok(c) => {dbg!(c);},
        Err(e) => {dbg!(e);}
    }
 }

#[test]
fn test_health_check() {

    let client = DockerClient::connect("/var/run/docker.sock");
    let health_check = HealthCheck::new().test("echo test").build();
    let config = Config::with_image("alpine")
        .name("name")
        .health_check(Some(health_check))
        .build();

    match client.create_container(config) {
        Ok(container) => { println!("{:?}", container); },
        Err(_) => {},
    }
}

#[test]
fn test_full() {
    let client = DockerClient::connect("/var/run/docker.sock");

    let config = Config::with_image("alpine")
        .name("test_full")
        .build();

    match client.create_container(config) {
        Ok(c) => println!("{:?}", c),
        Err(_) => return,
    }

    let info = client.inspect_container("test_full", false);

    match info {
        Ok(info) => { dbg!(info); },
        Err(e) => println!("Error: {:?}", e),
    }

    let remover = Remover::new()
        .id("test_full")
        .build();

    match client.remove_container(remover) {
        Ok(_) => {},
        Err(e) => println!("Error {:?}", e)
    }
}

#[test]
fn test_log() {
    let client = DockerClient::connect("/var/run/docker.sock");

    match client.get_container_log("psql") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error {:?}", e),
    }
}
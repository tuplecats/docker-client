extern crate docker_client;

use docker_client::{DockerClient, DockerError};
use docker_client::container::{Remover, Killer, Config, HealthCheck, WaitCondition, Create};
use docker_client::volume::VolumeCreator;
//use docker_client::container::Request;
use docker_client::container::inspect::Inspect;
use docker_client::container::processes_list::ProcessesList;
use std::path::Path;

fn client() -> DockerClient {
    DockerClient::new()
}

// #[test]
// fn test_list_containers() {
//     let client = client();
//
//     let req = Request::new().all(true).build();
//
//     match client.containers_list(req) {
//         Ok(v) => { println!("{:?}", v); },
//         Err(DockerError::BadParameters(m)) => { println!("Request bad parameters: {}.", m.message); },
//         Err(DockerError::ServerError(m)) => { println!("Server error: {}.", m.message); },
//         _ => { println!("Disconnected"); }
//     }
// }

// #[test]
// fn test_kill() {
//     let client = client();
//
//     let killer = Killer::new()
//         .id("test")
//         .signal("SIGINT")
//         .build();
//
//     match client.kill_container(killer).await {
//         Ok(()) => { println!("Container killed."); },
//         Err(DockerError::BadParameters(m)) => { println!("Request bad parameters: {}.", m.message); },
//         Err(DockerError::NotFound(m)) => { println!("Container not found: {}.", m.message); },
//         Err(DockerError::NotRunning(m)) => { println!("Container not running: {}.", m.message); },
//         Err(DockerError::ServerError(m)) => { println!("Server error: {}.", m.message); },
//         Err(DockerError::UnknownStatus) => { println!("Unknown response status."); },
//         Err(DockerError::ClosedConnection) => { println!("Connection closed."); },
//         _ => {}
//     }
// }

#[tokio::test]
async fn test_remove() {
    let client = client();

    let remover = Remover::new()
        .id("trait")
        .with_force_delete(false)
        .with_remove_link(false)
        .with_remove_volumes(false)
        .build();

    match client.remove_container(remover).await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_rename() {
    let client = client();

    match client.rename_container("purge", "purge1").await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_start() {
    let client = client();

    match client.start_container("12", "").await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_pause() {
    let client = client();

    match client.pause_container("123").await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_unpause() {
    let client = client();

    match client.unpause_container("123").await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_stop() {
    let client = client();

    match client.stop_container("123", None).await {
        Ok(()) => {},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_fs_changes() {
    let client = client();

    match client.get_fs_changes("purge").await {
        Ok(c) => {dbg!(c);},
        Err(_) => {}
    }
}

#[tokio::test]
async fn test_create() {
    let client = client();

    let request = Create::new()
        .config(
            Config::with_image("alpine")
                .build()
        )
        .name("test").build();

    match client.create_container(request).await {
        Ok(c) => {dbg!(c);},
        Err(e) => {dbg!(e);},
    }
}

#[tokio::test]
async fn test_inspect_container() {
    let client = client();

    let request = Inspect::container("vigilant_antonelli".to_string());

    match client.inspect_container(request).await {
        Ok(c) => {dbg!(c);},
        Err(e) => {dbg!(e);}
    }
 }

#[tokio::test]
async fn test_health_check() {

    let client = client();
    let health_check = HealthCheck::new().test("echo test").build();

    let request = Create::new().name("name").config(
        Config::with_image("alpine")
            .health_check(Some(health_check)).build()
    ).build();

    match client.create_container(request).await {
        Ok(container) => { println!("{:?}", container); },
        Err(_) => {},
    }
}

#[tokio::test]
async fn test_top() {
    let client = client();

    match client.top(ProcessesList::container("vigilant_antonelli".to_string())).await {
        Ok(v) => println!("{:?}", v),
        Err(_) => return
    }
}

#[tokio::test]
async fn test_full() {
    let client = client();

    let request = Create::new().name("test_full").config(
        Config::with_image("alpine").build()
    ).build();

    match client.create_container(request).await {
        Ok(c) => println!("{:?}", c),
        Err(_) => return,
    }

    let request_inspect = Inspect::container("vigilant_antonelli".to_string());
    match client.inspect_container(request_inspect).await {
        Ok(info) => { dbg!(info); },
        Err(e) => println!("Error: {:?}", e),
    }

    let remover = Remover::new()
        .id("test_full")
        .build();

    match client.remove_container(remover).await {
        Ok(_) => {},
        Err(e) => println!("Error {:?}", e)
    }
}

#[tokio::test]
async fn test_log() {
    let client = client();

    match client.get_container_log("psql").await {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn test_wait_container() {
    let client = client();

    match client.wait_container("test", WaitCondition::default()).await {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn test_export_container() {
    let client = client();

    let mut path = std::env::temp_dir();
    path.push("export_container");
    path.set_extension("tar");

    match client.export_container("test", path.as_path()).await {
        Ok(_) => {},
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn test_image_list() {
    let client = client();

    match client.get_image_list().await {
        Ok(info) => { dbg!(info); },
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn create_volume() {
    let client = client();

    let volume = VolumeCreator::builder()
        .name("volume-test")
        .label("label1", "label-value")
        .label("label2", "label-3")
        .build();

    match client.create_volume(volume).await {
        Ok(_) => {},
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn inspect_volume() {
    let client = client();

    match client.inspect_volume("volume-test").await {
        Ok(info) => { dbg!(info); },
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn delete_unused_volumes() {
    let client = client();

    match client.delete_unused_volumes().await {
        Ok(deleted) => { dbg!(deleted); },
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn get_volumes_list() {
    let client = client();

    match client.get_volumes_list().await {
        Ok(list) => { dbg!(list); },
        Err(e) => println!("Error {:?}", e),
    }
}

#[tokio::test]
async fn test_pull_image() {
    let client = client();

    let request = docker_client::image::create::RequestBuilder::new()
        .image("ubuntu")
        .tag("latest")
        .build();

    match client.pull_image(request).await {
        Ok(_) => {},
        Err(e) => println!("Error {:?}", e)
    }
}
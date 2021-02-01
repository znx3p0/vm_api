


use actix_web::{get, web, App, HttpServer, Responder, delete};
use std::{collections::HashMap, sync::{Mutex, atomic::{AtomicUsize, Ordering}}, thread::spawn};
use std::sync::Arc;
use rand::random;
use regex::Regex;

use serde::{Serialize};

mod sh;
use sh::{sh_silent, sh};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Clone, Debug)]
struct Creator {
    servers: Arc<Mutex<HashMap<String, Server>>>
}

impl Creator {
    fn create_server(&self) -> (String, String, String) {
        let (id, passwd, ip) = create_server();
        self.servers.lock().unwrap().insert(id.clone(), Server{id: id.clone(), password: passwd.clone(), ip:ip.clone()});
        println!("{:#?}", self.servers.lock().unwrap());
        (id, passwd, ip)
    }

    fn delete_server(&self, id: String) {
        delete_server(id.clone());
        self.servers.lock().unwrap().remove(&id).unwrap();
    }
}


#[get("/hetzner_create_server")]
async fn hetzner_create(data: web::Data<Creator>) -> impl Responder {
    let (id, passwd, ip) = data.create_server();
    let server = hetzner_server::new(id, passwd, ip);
    serde_json::to_string(&server)
}

#[derive(Debug, Serialize)]
struct hetzner_server {
    id: String,
    passwd: String,
    ip: String,
}

impl hetzner_server {
    fn new(id: String, passwd: String, ip: String) -> Self {
        Self {
            id: id,
            passwd: passwd,
            ip: ip
        }
    }
}

#[delete("/hetzner/delete/{id}")]
async fn hetzner_delete(path: web::Path<(String,)>) -> impl Responder {
    delete_server(path.into_inner().0);
    "Success"
}


fn rand_str() -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(24)
            .collect::<String>()
}

fn create_server() -> (String, String, String) {

    let id = rand_str();
    let passwd = rand_str();
    let cmd = format!("sudo podman inspect {} | grep IPAddress", id);

    let re = Regex::new(r#"[0-9.]+"#).unwrap();

    btl::shell! {
        "sudo podman run -d --cpus 1 -m 1g --rm -e PASSWD=\"{}\" --name={} ssh_server" passwd id;
    };
    let addr = sh(cmd.as_str());
    println!("{:?}", addr);

    let addr = re.find(addr.as_str()).unwrap().as_str();
    println!("ADDR: {}\nid: {}\nPASSWD:{}", addr, id, passwd);
    (id, passwd, addr.to_string())
}

fn delete_server(id: String) {
    println!("Executing: {:#?}", format!("sudo podman rm -f {}", id).as_str());
    btl::shell! {
        "sudo podman rm -f {}" id;
    };
}

#[derive(Debug)]
struct Server {
    id: String,
    password: String,
    ip: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    btl::exec!{
        "sudo echo Running!";
    };
    spawn(||{keep_alive()});

    let app = Creator{servers: Arc::new(Mutex::new(HashMap::new()))};

    HttpServer::new(move || {
        App::new()
            .data(app.clone())
            .service(hetzner_create)
            .service(hetzner_delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}

fn keep_alive() {

    std::thread::sleep(std::time::Duration::from_millis(10000));

    sh_silent("sudo echo a");

    keep_alive()
}
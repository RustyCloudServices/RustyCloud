mod service;
mod fmt;
mod database;

use colored::*;
use database::create_pool_postgresql;
use fmt::{print_console, ConsoleOutputTypes};
use service::{Service, ServiceGroup, StartedService};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use crate::fmt::hex_to_customcolor;

pub struct CloudNode {
    database: DatabaseTypes,
    service_identifier_count: i32,
    services: Vec<StartedService>,
    service_id_map: HashMap<i32, usize>,
    groups: Vec<ServiceGroup>,
    group_id_map: HashMap<i32, usize>,
}

impl CloudNode {
    fn new(database: DatabaseTypes) -> CloudNode {
        CloudNode {
            database,
            service_identifier_count: 0,
            services: Vec::new(),
            service_id_map: HashMap::new(),
            groups: Vec::new(),
            group_id_map: HashMap::new(),
        }
    }
    
    fn add_service(&mut self, service: StartedService) {
        if self.service_id_map.contains_key(&service.get_id()) {
            print_console("Service ID already exists.", ConsoleOutputTypes::ERROR);
            return;
        }
        let index = self.services.len();
        self.services.push(service.clone());
        self.service_id_map.insert(service.get_id(), index);
    }

    fn remove_service(&mut self, id: i32) -> Option<StartedService> {
        if let Some(index) = self.service_id_map.remove(&id) {
            let removed_service = self.services.swap_remove(index);
            Some(removed_service)
        } else {
            None
        }
    }

    fn get_service(&self, id: i32) -> Option<&StartedService> {
        if let Some(&index) = self.service_id_map.get(&id) {
            Some(&self.services[index])
        } else {
            None
        }
    }

    fn get_free_id(&mut self) -> i32 {
        self.service_identifier_count += 1;
    
        self.service_identifier_count
    }
}

fn main() {
    setup();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!(
            "{}",
            "RustyCloud".custom_color(hex_to_customcolor("FF5733"))
        );
        println!("{}", line.unwrap().blue());
    }
}

fn setup() {
    print_console("Hello, world! :D", ConsoleOutputTypes::INFO);
    let mut local_cloud = CloudNode::new(DatabaseTypes::POSTGRESQL);

    // Create a database pool. This isn't currently needed and can be safely removed for testing.
    let pool = create_pool_postgresql("postgres://postgres:local@localhost:5432/postgres");
    print_console("Connecting to database..", ConsoleOutputTypes::INFO);

    print_console("Loading Demo Groups and Services..", ConsoleOutputTypes::INFO);

    let mut group1 = ServiceGroup::load("Lobby");
    let mut group2 = ServiceGroup::load("Survival");

    let service1 = Service::new(&mut local_cloud, &mut group1)
        .set_name("Lobby".to_string())
        .set_max_memory(1024)
        .start(&mut local_cloud, &mut group1);

    let service2 = Service::new(&mut local_cloud, &mut group2)
        .set_name("Survival1".to_string())
        .set_max_memory(2048)
        .start(&mut local_cloud, &mut group2);

    service2.kill(&mut local_cloud);
    service1.stop(&mut local_cloud);

    println!("");
    print_console("CLI currently not working correctly.", ConsoleOutputTypes::WARN);
    print_console("If you want to play around with service management you'll need to fork this repo and edit the source code.", ConsoleOutputTypes::INFO);
    println!("");
}

enum DatabaseTypes {
    POSTGRESQL,
    SQLITE,
    MARIADB,
    MYSQL,
}
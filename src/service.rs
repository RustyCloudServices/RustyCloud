use crate::{print_console, CloudNode, ConsoleOutputTypes};

#[derive(Clone)]
pub struct ServiceGroup {
    name: String,
    max_players: i32,
    max_memory: u32,
}

impl ServiceGroup {
    // TODO: Implement loading from database
    pub fn load(name: &str) -> ServiceGroup {
        print_console(
            &format!("Loading service group: {}", String::from(name)),
            ConsoleOutputTypes::INFO,
        );

        print_console(
            &format!("Loaded service group: {}", String::from(name)),
            ConsoleOutputTypes::SUCCESS,
        );

        ServiceGroup {
            name: String::from(name),
            max_players: 20,
            max_memory: 2048,
        }
    }
}

#[derive(Clone)]
pub struct Service {
    name: String,
    identifier: i32,
    max_players: i32,
    max_memory: u32,
}

impl Service {
    pub fn new(cloud: &mut CloudNode, group: &mut ServiceGroup) -> Service {
        Service {
            name: String::from("NewService"),
            identifier: cloud.get_free_id(),
            max_players: 20,
            max_memory: 512,
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_id(&mut self, number: i32) -> &mut Self {
        self.identifier = number;
        self
    }

    pub fn set_max_players(&mut self, max_players: i32) -> &mut Self {
        self.max_players = max_players;
        self
    }

    pub fn set_max_memory(&mut self, max_memory: u32) -> &mut Self {
        self.max_memory = max_memory;
        self
    }

    pub fn start(&self, cloud: &mut CloudNode, group: &mut ServiceGroup) -> StartedService {
        print_console(
            &format!("Starting service: {}", self.name),
            ConsoleOutputTypes::INFO,
        );
        let identifier = cloud.get_free_id();

        let service = StartedService {
            name: self.name.to_string(),
            identifier: identifier,
            max_players: self.max_players,
            max_memory: self.max_memory,
        };
        cloud.add_service(service.clone());
        service
    }
}

#[derive(Clone)]
pub struct StartedService {
    name: String,
    identifier: i32,
    max_players: i32,
    max_memory: u32,
}

impl StartedService {
    pub fn get_name(&self) -> &str {
        &self.name[..] as &str
    }

    pub fn get_id(&self) -> i32 {
        self.identifier
    }

    pub fn get_max_players(&self) -> i32 {
        self.max_players
    }

    pub fn get_max_memory(&self) -> u32 {
        self.max_memory
    }

    pub fn restart(&self, cloud: &mut CloudNode) { // TODO: Implement restart
        cloud.remove_service(self.identifier);
        print_console(
            &format!("Restarting service: {}", self.name),
            ConsoleOutputTypes::INFO,
        );
    }

    pub fn stop(&self, cloud: &mut CloudNode) {
        cloud.remove_service(self.identifier);
        print_console(
            &format!("Stopping service: {}", self.name),
            ConsoleOutputTypes::INFO,
        );
    }

    pub fn kill(&self, cloud: &mut CloudNode) {
        cloud.remove_service(self.identifier);
        print_console(
            &format!("Killed service: {}", self.name),
            ConsoleOutputTypes::WARN,
        );
    }
}
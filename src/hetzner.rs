
pub mod CREATE_SERVER_RESPONSE {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub server: Server,
        action: Action,
        next_actions: Vec<Action>,
        pub root_password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        id: i64,
        command: String,
        status: String,
        progress: i64,
        started: String,
        finished: Option<serde_json::Value>,
        resources: Vec<Resource>,
        error: Error,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Error {
        code: String,
        message: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resource {
        id: i64,
        #[serde(rename = "type")]
        resource_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Server {
        pub id: i64,
        pub name: String,
        status: String,
        created: String,
        pub public_net: PublicNet,
        private_net: Vec<PrivateNet>,
        server_type: ServerType,
        datacenter: Datacenter,
        image: Image,
        iso: Iso,
        rescue_enabled: bool,
        locked: bool,
        backup_window: String,
        outgoing_traffic: i64,
        ingoing_traffic: i64,
        included_traffic: i64,
        protection: ServerProtection,
        labels: Labels,
        volumes: Vec<Option<serde_json::Value>>,
        load_balancers: Vec<Option<serde_json::Value>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Datacenter {
        id: i64,
        name: String,
        description: String,
        location: Location,
        server_types: ServerTypes,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Location {
        id: i64,
        name: String,
        description: String,
        country: String,
        city: String,
        latitude: f64,
        longitude: f64,
        network_zone: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerTypes {
        supported: Vec<i64>,
        available: Vec<i64>,
        available_for_migration: Vec<i64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Image {
        id: i64,
        #[serde(rename = "type")]
        image_type: String,
        status: String,
        name: String,
        description: String,
        image_size: f64,
        disk_size: i64,
        created: String,
        created_from: CreatedFrom,
        bound_to: Option<serde_json::Value>,
        os_flavor: String,
        os_version: String,
        rapid_deploy: bool,
        protection: ImageProtection,
        deprecated: String,
        labels: Labels,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreatedFrom {
        id: i64,
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Labels {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageProtection {
        delete: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Iso {
        id: i64,
        name: String,
        description: String,
        #[serde(rename = "type")]
        iso_type: String,
        deprecated: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrivateNet {
        network: i64,
        ip: String,
        alias_ips: Vec<Option<serde_json::Value>>,
        mac_address: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerProtection {
        delete: bool,
        rebuild: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PublicNet {
        pub ipv4: Ipv4,
        ipv6: Ipv6,
        floating_ips: Vec<i64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ipv4 {
        pub ip: String,
        blocked: bool,
        dns_ptr: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ipv6 {
        ip: String,
        blocked: bool,
        dns_ptr: Vec<DnsPtr>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DnsPtr {
        ip: String,
        dns_ptr: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerType {
        id: i64,
        name: String,
        description: String,
        cores: i64,
        memory: i64,
        disk: i64,
        deprecated: bool,
        prices: Vec<Price>,
        storage_type: String,
        cpu_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Price {
        location: String,
        price_hourly: PriceHourlyClass,
        price_monthly: PriceHourlyClass,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PriceHourlyClass {
        net: String,
        gross: String,
    }
}

// DELETE /servers/{id}
pub mod DELETE_SERVER_RESPONSE {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Welcome {
        action: Action,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        id: i64,
        command: String,
        status: String,
        progress: i64,
        started: String,
        finished: String,
        resources: Vec<Resource>,
        error: Error,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Error {
        code: String,
        message: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resource {
        id: i64,
        #[serde(rename = "type")]
        resource_type: String,
    }
}

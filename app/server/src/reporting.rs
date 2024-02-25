#![allow(unused)]
#![allow(clippy::needless_return)]

use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex};
use crate::utils::{SharedClient, SharedClientList};
use crate::packet::Packet;

pub struct Reporting {
    clients: SharedClientList,
    admin_client: Arc<RwLock<Option<SharedClient>>>,
    admin_client_id: Mutex<Option<u64>>,
    admin_client_connection: RwLock<AdminClientConnectionHealth>,
}

#[derive(PartialEq)]
enum AdminClientConnectionHealth {
    StayedConnected,
    HasBeenDisconnected,
}

impl Reporting {

    pub fn new(clients: SharedClientList) -> Self {

        let admin_client_value: Option<SharedClient> = None;
        let admin_client_lock = RwLock::new(admin_client_value);
        let admin_client = Arc::new(admin_client_lock);

        let admin_client_id_value: Option<u64> = None;
        let admin_client_id = Mutex::new(admin_client_id_value);

        let admin_client_connection = RwLock::new(AdminClientConnectionHealth::StayedConnected);

        return Reporting {
            clients,
            admin_client,
            admin_client_id,
            admin_client_connection,
        }
    }

    pub fn start(&self) {
        println!("hee......................................");
        spawn(move || self.run());
    }

    pub fn run(&self) {

        loop {
            println!("wait...................................");
            self.wait_for_admin_client();
            println!("report.................................");
            self.report_to_admin_client();
        }

    }

    fn wait_for_admin_client(&self) {

        *self.admin_client_connection.write().unwrap() = AdminClientConnectionHealth::StayedConnected;

        loop {

            { // drop() is not enough, must enclose in a scope
                let lock: &RwLock<Option<SharedClient>> = &self.admin_client;
                let opt: &Option<SharedClient> = &lock.read().unwrap();
                if !opt.is_none() { return; }
            }

            sleep(Duration::from_secs(1));
        }

    }

    fn report_to_admin_client(&self) {

        let mut first_round = true;

        loop {

            // it contains at least one element: the admin client
            let client_ids = self.get_client_ids_snapshot();
            if *self.admin_client_connection.read().unwrap()
                == AdminClientConnectionHealth::HasBeenDisconnected {
                return;
            }

            for client_id in client_ids {

                if self.report_to_admin_single(client_id, first_round) {
                    return;
                }

                sleep(Duration::from_millis(200));

                if *self.admin_client_connection.read().unwrap()
                    == AdminClientConnectionHealth::HasBeenDisconnected {
                    return;
                }

            }

            first_round = false;
        }
    }

    fn get_client_ids_snapshot(&self) -> Vec<u64> {

        let mut client_ids = Vec::new();

        let hash_map = self.clients.read().unwrap();
        for (id, _shared_client) in hash_map.iter() {
            client_ids.push(*id);
        }

        return client_ids;
    }

    pub fn report_to_admin_single(&self, client_id: u64, force: bool) -> bool {

        let lock: &RwLock<Option<SharedClient>> = &self.admin_client;
        let opt: &Option<SharedClient> = &lock.read().unwrap();
        let Some(shared_admin_client) = opt else {
            return true;
        };

        let mcid_opt = *self.admin_client_id.lock().unwrap();
        let Some(admin_id) = mcid_opt else {
            return true;
        };

        let hash_map = self.clients.read().unwrap();
        let Some(shared_client) = hash_map.get(&client_id) else {
            return false;
        };

        let mut client = shared_client.write().unwrap();

        let should_send_report = match force {
            true => true,
            false => client.check_and_clear(),
        };

        if should_send_report {

            let packet = client.create_report(admin_id);

            if client_id == admin_id {
                client.send_packet(&packet);
            } else {
                let admin_client = shared_admin_client.read().unwrap();
                admin_client.send_packet(&packet);
            }

        }

        return false;
    }


    pub fn report_client_creation(&self, client_id: u64) {
        self.report_client_life_event(client_id, true);
    }

    pub fn report_client_destruction(&self, client_id: u64) {
        self.report_client_life_event(client_id, false);
    }

    pub fn set_admin_client(&self, shared_client: &SharedClient) {
        *self.admin_client.write().unwrap() = Some(shared_client.clone());
    }

    pub fn set_admin_client_id(&self, client_id: u64) {
        *self.admin_client_id.lock().unwrap() = Some(client_id);
    }

    pub fn clear_admin_on_match(&self, client_id: u64) {

        let mut guarded_id_opt = self.admin_client_id.lock().unwrap();
        let id_opt = *guarded_id_opt;
        let admin_id = match id_opt {
            Some(value) => value,
            None => return,
        };

        if admin_id != client_id { return; }

        *guarded_id_opt = None;
        drop(guarded_id_opt);

        *self.admin_client.write().unwrap() = None;

        *self.admin_client_connection.write().unwrap() =
            AdminClientConnectionHealth::HasBeenDisconnected;

    }

    pub fn report_client_life_event(&self, client_id: u64, creation: bool) {

        let lock: &RwLock<Option<SharedClient>> = &self.admin_client;
        let opt: &Option<SharedClient> = &lock.read().unwrap();
        let Some(shared_admin_client) = opt else {
            return;
        };
        let admin_client = shared_admin_client.read().unwrap();

        if !creation {
            let Some(admin_id) = *self.admin_client_id.lock().unwrap() else {
                return;
            };
            if client_id == admin_id {
                return;
            }
        }

        let mut packet = Packet::new();
        packet.set_type(match creation {
            true => "CONNECT",
            false => "DISCONNECT",
        });
        packet.set_num(0, client_id as i64);

        admin_client.send_packet(&packet);

    }

    pub fn sync_client_list(&self) {

        let client_ids = self.get_client_ids_snapshot();
        for client_id in client_ids {
            self.report_to_admin_single(client_id, true);
            //self.report_client_life_event(client_id, true);
        }
    }

}

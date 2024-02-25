#![allow(unused)]
#![allow(clippy::needless_return)]

use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex};
use crate::utils::{SharedClient, SharedClientList};
use crate::packet::Packet;

pub struct Reporting {
    clients: SharedClientList,
    master_client: Arc<RwLock<Option<SharedClient>>>,
    master_client_id: Mutex<Option<u64>>,
    master_client_connection: RwLock<MasterClientConnectionHealth>,
}

#[derive(PartialEq)]
enum MasterClientConnectionHealth {
    StayedConnected,
    HasBeenDisconnected,
}

impl Reporting {

    pub fn new(clients: SharedClientList) -> Self {

        let master_client_value: Option<SharedClient> = None;
        let master_client_lock = RwLock::new(master_client_value);
        let master_client = Arc::new(master_client_lock);

        let master_client_id_value: Option<u64> = None;
        let master_client_id = Mutex::new(master_client_id_value);

        let master_client_connection = RwLock::new(MasterClientConnectionHealth::StayedConnected);

        return Reporting {
            clients,
            master_client,
            master_client_id,
            master_client_connection,
        }
    }

    pub fn start(self: Arc<Self>) {
        spawn(move || self.run());
    }

    pub fn run(&self) {

        loop {
            self.wait_for_master_client();
            self.report_to_master_client();
        }

    }

    fn wait_for_master_client(&self) {

        *self.master_client_connection.write().unwrap() =
            MasterClientConnectionHealth::StayedConnected;

        loop {

            { // drop() is not enough, must enclose in a scope
                let lock: &RwLock<Option<SharedClient>> = &self.master_client;
                let opt: &Option<SharedClient> = &lock.read().unwrap();
                if !opt.is_none() { return; }
            }

            sleep(Duration::from_secs(1));
        }

    }

    fn report_to_master_client(&self) {

        let mut first_round = true;

        loop {

            // it contains at least one element: the master client
            let client_ids = self.get_client_ids_snapshot();
            if *self.master_client_connection.read().unwrap()
                == MasterClientConnectionHealth::HasBeenDisconnected {
                return;
            }

            for client_id in client_ids {

                if self.report_to_master_single(client_id, first_round) {
                    return;
                }

                sleep(Duration::from_millis(200));

                if *self.master_client_connection.read().unwrap()
                    == MasterClientConnectionHealth::HasBeenDisconnected {
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

    pub fn report_to_master_single(&self, client_id: u64, force: bool) -> bool {

        let lock: &RwLock<Option<SharedClient>> = &self.master_client;
        let opt: &Option<SharedClient> = &lock.read().unwrap();
        let Some(shared_master_client) = opt else {
            return true;
        };

        let mcid_opt = *self.master_client_id.lock().unwrap();
        let Some(master_id) = mcid_opt else {
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

            let packet = client.create_report(master_id);

            if client_id == master_id {
                client.send_packet(&packet);
            } else {
                let master_client = shared_master_client.read().unwrap();
                master_client.send_packet(&packet);
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

    pub fn set_master_client(&self, shared_client: &SharedClient) {
        *self.master_client.write().unwrap() = Some(shared_client.clone());
    }

    pub fn set_master_client_id(&self, client_id: u64) {
        *self.master_client_id.lock().unwrap() = Some(client_id);
    }

    pub fn clear_master_on_match(&self, client_id: u64) {

        let mut guarded_id_opt = self.master_client_id.lock().unwrap();
        let id_opt = *guarded_id_opt;
        let master_id = match id_opt {
            Some(value) => value,
            None => return,
        };

        if master_id != client_id { return; }

        *guarded_id_opt = None;
        drop(guarded_id_opt);

        *self.master_client.write().unwrap() = None;

        *self.master_client_connection.write().unwrap() =
            MasterClientConnectionHealth::HasBeenDisconnected;

    }

    pub fn report_client_life_event(&self, client_id: u64, creation: bool) {

        let lock: &RwLock<Option<SharedClient>> = &self.master_client;
        let opt: &Option<SharedClient> = &lock.read().unwrap();
        let Some(shared_master_client) = opt else {
            return;
        };
        let master_client = shared_master_client.read().unwrap();

        if !creation {
            let Some(master_id) = *self.master_client_id.lock().unwrap() else {
                return;
            };
            if client_id == master_id {
                return;
            }
        }

        let mut packet = Packet::new();
        packet.set_type(match creation {
            true => "CONNECT",
            false => "DISCONNECT",
        });
        packet.set_num(0, client_id as i64);

        master_client.send_packet(&packet);

    }

    pub fn sync_client_list(&self) {

        let client_ids = self.get_client_ids_snapshot();
        for client_id in client_ids {
            self.report_client_life_event(client_id, true);
        }
    }

}

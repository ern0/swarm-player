#![allow(unused)]

use crate::utils::{SharedClient, SharedClientList};
use crate::utils::ClientId;

const MADTER_CHANNEL_ID: ClientId = 0;
const PUBLIC_CHANNEL_COUNT: usize = 4;

pub struct ChannelManager {
	channel_list: [Vec<ClientId>; 1 + PUBLIC_CHANNEL_COUNT],
}

impl ChannelManager {

    pub fn new(clients: SharedClientList) -> Self {

			return ChannelManager {  
				channel_list: Default::default(),
			}

		}

		pub fn report_client_creation(&self, client_id: ClientId) {

			let channel = &self.channel_list[0];
		}

		pub fn report_client_destruction(&self, client_id: ClientId) {
		}
}

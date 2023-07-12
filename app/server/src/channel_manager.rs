#![allow(unused)]

use crate::utils::{SharedClient, SharedClientList};
use crate::utils::ClientId;

const MADTER_CHANNEL_ID: ClientId = 0;
const PUBLIC_CHANNEL_COUNT: usize = 4;

pub struct ChannelManager {
  client_list: SharedClientList,
	channel_list: [Vec<ClientId>; 1 + PUBLIC_CHANNEL_COUNT],
}

impl ChannelManager {

    pub fn new(client_list: SharedClientList) -> Self {

			return ChannelManager {  
				client_list,
				channel_list: Default::default(),
			}

		}

		pub fn report_client_creation(&mut self, client_id: ClientId) {

			let mut channel = &mut self.channel_list[0];
			channel[0] = 22;
		}

		pub fn report_client_destruction(&mut self, client_id: ClientId) {
		}
}

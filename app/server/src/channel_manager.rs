#![allow(unused)]
#![allow(clippy::needless_return)]

use crate::utils::{SharedClient, SharedClientList};
use crate::utils::ClientId;

pub type ChannelId = u8;
pub type ChannelMask = u8;

const CHANNEL_MASKS: [ChannelMask; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
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
			self.reorganize();
		}

		pub fn report_client_destruction(&mut self, client_id: ClientId) {
			self.reorganize();
		}

		fn reorganize(&mut self) {

		}
}

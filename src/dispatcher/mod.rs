use crate::errors::NetworkError;
use crate::{
	config::*,
	core::{Apps, Ops},
	prochandler::HandleOps,
	runner::Regress,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::default::Default;
use zmq::{self, Context, Message};

#[derive(Serialize, Deserialize, Default)]
pub struct DispatchMsg {
	pub proctype: HandleOps,
	pub func: Option<Ops>,
	pub application: Option<Apps>,
	pub refresh: bool,
	pub regress_counter: Option<Regress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMsg {
	pub code: String,
	pub message: String,
}

impl Default for ResponseMsg {
	fn default() -> ResponseMsg {
		ResponseMsg {
			code: String::from("E000"),
			message: String::from(
				"A generic, unspecified error occured. See backtrace for more details.",
			),
		}
	}
}

pub fn send(d_msg: DispatchMsg) -> Result<ResponseMsg, NetworkError> {
	let context = Context::new();
	let requester = context.socket(zmq::REQ)?;

	requester.connect(get_config()?.client_url.as_str())?;

	let mut msg = Message::new();

	debug!("Sending Request...");
	requester.send(&serde_json::to_string(&d_msg)?, 0)?;

	requester.recv(&mut msg, 0)?;

	let retmsg: ResponseMsg = serde_json::from_str(msg.as_str().unwrap_or_default())?;

	debug!("Received response: {:?}", retmsg);

	Ok(retmsg)
}

pub fn heartbeat() -> Result<ResponseMsg, NetworkError> {
	// TODO: Define an early exit if daemon isn't running yet.
	let context = Context::new();
	let requester = context.socket(zmq::REQ)?;

	requester.connect(get_config()?.client_url.as_str())?;

	// QUESTION: These params are a hack. Is there a better way to detect this?
	requester.set_linger(1)?;
	requester.set_rcvtimeo(2000)?;

	let mut msg = Message::new();

	debug!("Sending Heartbeat...");

	let snd_msg = DispatchMsg {
		proctype: HandleOps::Heartbeat,
		..Default::default()
	};

	requester.send(&serde_json::to_string(&snd_msg)?, 0)?;

	requester.recv(&mut msg, 0)?;

	let retmsg: ResponseMsg = serde_json::from_str(msg.as_str().unwrap_or_default())?;

	// TODO: Check against expected heartbeat output.

	debug!("Received response: {:?}", retmsg);

	Ok(retmsg)
}

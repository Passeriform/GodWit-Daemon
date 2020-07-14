//! Runner
//!
//! Defines a thread-runnable class. Contains list of runnable operations.
use crate::errors::{NetworkError, RunError};
use crate::{
	config::*,
	core::{Apps, Ops},
	dispatcher::{self, DispatchMsg, ResponseMsg},
	prochandler::{self, HandleOps},
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use zmq::{self, Context, Message};

#[derive(Debug, Serialize, Deserialize)]
pub enum Regress {
	Once,
	Infinite,
}

pub fn init_daemon() -> Result<(), NetworkError> {
	let pool = ThreadPool::new(get_config()?.max_threads);

	let context = Context::new();
	let responder = context.socket(zmq::REP).unwrap();

	responder.bind(get_config()?.daemon_url.as_str())?;

	debug!("Started daemon listening on: {}", get_config()?.daemon_url);

	let (tx, rx) = channel();

	loop {
		let tx = tx.clone();

		let mut msg = Message::new();

		responder.recv(&mut msg, 0)?;

		debug!("Captured request {:?}", msg.as_str().unwrap_or_default());

		let DispatchMsg {
			proctype,
			func,
			application,
			refresh,
			regress_counter,
		} = serde_json::from_str(msg.as_str().unwrap_or_default())?;

		match regress_counter.unwrap_or(Regress::Once) {
			Regress::Once => {
				pool.execute(move || {
					let retmsg = match prochandler::handle(proctype, func, application, refresh) {
						Ok(_) => Ok(ResponseMsg {
							code: String::from("S000"),
							message: String::from("Process successfully cleared."),
						}),
						Err(e) => Err(ResponseMsg {
							code: String::from("E002"),
							message: e.to_string(),
						}),
					};

					thread::sleep(Duration::from_millis(1000)); // Emulate processing

					tx.send(retmsg.unwrap())
						.expect("Channel blocked for the pool");
				});

				let retmsg = rx.recv().unwrap_or_default();
				println!("{:?}", retmsg);

				responder
					.send(
						&serde_json::to_string(&retmsg).expect(
							format!("Tried to serialize invalid message string: {:?}", retmsg)
								.as_str(),
						),
						0,
					)
					.expect("Error occured while sending return message.");
			}
			Regress::Infinite => {
				let ack_msg = ResponseMsg {
					code: String::from("S001"),
					message: String::from("Regression started on thread."),
				};

				responder
					.send(
						&serde_json::to_string(&ack_msg).expect(
							format!("Tried to serialize invalid message string: {:?}", ack_msg)
								.as_str(),
						),
						0,
					)
					.expect("Error occured while sending return message.");
				loop {
					let tx = tx.clone();

					pool.execute(move || {
						let retmsg = prochandler::handle(proctype, func, application, refresh);

						thread::sleep(Duration::from_millis(1000)); // Emulate processing

						// TODO: Write concise retmsg stream to file.
					});
				}
			}
		}
	}
}

pub fn daemon_running() -> bool {
	dispatcher::heartbeat().is_ok()
}

pub fn run(
	func: Ops,
	application: Apps,
	refresh: bool,
	regress_counter: Regress,
) -> Result<(), RunError> {
	if !daemon_running() {
		init_daemon().map_err(Into::into)
	} else {
		dispatcher::send(DispatchMsg {
			proctype: HandleOps::Run,
			func: Some(func),
			application: Some(application),
			refresh: refresh,
			regress_counter: Some(regress_counter),
		})?;
		Ok(())
	}
}

pub fn kill(func: Ops, application: Apps) -> Result<(), RunError> {
	dispatcher::send(DispatchMsg {
		proctype: HandleOps::Kill,
		func: Some(func),
		application: Some(application),
		..Default::default()
	})?;
	Ok(())
}

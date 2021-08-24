use crate::kdeconnect_client::{Connection, Daemon, MessageType};
use crate::Message;
use dbus::channel::Token;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::Duration;

pub struct SMSReceiver {
    rx: Receiver<Message>,
    kde_connection: Connection,
    _tokens: Vec<Token>,
}

impl SMSReceiver {
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let kde_connection = Connection::new()?;
        let kde_daemon = Daemon::new(&kde_connection);
        let devices = kde_daemon.get_devices(true, true)?;

        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let mut tokens = Vec::new();
        for device in devices {
            if device.has_plugin("kdeconnect_sms")? {
                let sms_plugin = device.get_sms_plugin();
                let device_name = device.get_name()?;

                let tx_clone = tx.clone();
                let device_name_clone = device_name.clone();
                tokens.push(sms_plugin.on_conversation_created(move |sms| {
                    if let MessageType::Inbox = sms.msg_type {
                        if !sms.sender.is_empty() && !sms.body.is_empty() {
                            tx_clone
                                .send(Message {
                                    sender: sms.sender,
                                    body: sms.body,
                                    date: sms.date,
                                    thread_id: sms.thread_id,
                                    unique_id: sms.unique_id,
                                    device_name: device_name_clone.clone(),
                                    sim_id: sms.sim_id,
                                })
                                .unwrap();
                        }
                    }
                })?);

                let tx_clone = tx.clone();
                let device_name_clone = device_name.clone();
                tokens.push(sms_plugin.on_conversation_updated(move |sms| {
                    if let MessageType::Inbox = sms.msg_type {
                        if !sms.sender.is_empty() && !sms.body.is_empty() {
                            tx_clone
                                .send(Message {
                                    sender: sms.sender,
                                    body: sms.body,
                                    date: sms.date,
                                    thread_id: sms.thread_id,
                                    unique_id: sms.unique_id,
                                    device_name: device_name_clone.clone(),
                                    sim_id: sms.sim_id,
                                })
                                .unwrap();
                        }
                    }
                })?);
            }
        }

        Ok(Self {
            rx,
            kde_connection,
            _tokens: tokens,
        })
    }

    pub fn check_for_message(&self) -> Result<Option<Message>, Box<dyn std::error::Error>> {
        self.kde_connection.process(Duration::from_millis(100))?;

        match self.rx.try_recv() {
            Ok(msg) => Ok(Some(msg)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(Box::new(TryRecvError::Disconnected).into()),
        }
    }
}

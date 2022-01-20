use channel::Receiver;
use futures::StreamExt;
use logger::prelude::*;
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub enum IsSyncingMessage {
    Check,
    Terminate(SyncSender<()>),
}

pub struct IsSyncingChecker {
    gaiad_endpoint: String,
    receiver: Receiver<IsSyncingMessage>,
}

impl IsSyncingChecker {
    pub fn new(gaiad_endpoint: String, receiver: Receiver<IsSyncingMessage>) -> Self {
        Self {
            gaiad_endpoint,
            receiver,
        }
    }
    pub async fn run(mut self) {
        while let Some(message) = self.receiver.next().await {
            match message {
                IsSyncingMessage::Check => {
                    match gaiacli::get_client(self.gaiad_endpoint.as_str())
                        .lock()
                        .await
                        .fetch_syncing()
                        .await
                    {
                        Ok(syncing) => {
                            if syncing {
                                error!(
                                    "the gaia daemon: {} is syncing",
                                    self.gaiad_endpoint.as_str()
                                );
                            } else {
                                info!(
                                    "the gaia daemon: {} is synced",
                                    self.gaiad_endpoint.as_str()
                                );
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                IsSyncingMessage::Terminate(sender) => {
                    info!("is syncing checker will be terminated soon...");
                    let _ = sender.send(());
                    break;
                }
            }
        }
    }
}

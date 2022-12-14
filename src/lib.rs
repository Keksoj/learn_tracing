use std::time::{Duration, Instant};

use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt, StreamExt,
};
use rand::{thread_rng, Rng};
use tracing::{debug, error, info, instrument, trace};

#[derive(Debug)]
pub struct BitcoinMiningFacility {
    mining_rigs: i32,
    /// receives mined bitcoins from miners (rig_id, bitcoins_mined)
    receiver: Receiver<(i32, i32)>,
    /// passed to mining rigs for them to send bitcoins back (rig_id, bitcoins_mined)
    sender: Sender<(i32, i32)>,
    bitcoin_produced: i32,
}

impl BitcoinMiningFacility {
    pub fn new(number_of_rigs: i32) -> Self {
        let (sender, receiver) = mpsc::channel(1000);
        info!("Created facility with {} mining rigs", number_of_rigs);
        Self {
            mining_rigs: number_of_rigs,
            bitcoin_produced: 0,
            receiver,
            sender,
        }
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) {
        info!("Launching mining rigs");
        let mut mining_rig_id = 0;

        std::env::set_var("SMOL_THREADS", self.mining_rigs.to_string());

        while mining_rig_id < self.mining_rigs {
            smol::spawn(launch_mining_rig(
                mining_rig_id.clone(),
                self.sender.clone(),
            ))
            .detach();

            mining_rig_id += 1;
        }

        // awaiting bitcoins from mining rigs
        info!("Waiting for minings rigs to die and give off the bitcoins they mined");
        let mut number_of_rigs_that_responded = 0;
        while let Some((rig_id, bitcoins_mined)) = self.receiver.next().await {
            self.bitcoin_produced += bitcoins_mined;
            info!(
                self.bitcoin_produced,
                "Rig {} sent {} bitcoins", rig_id, bitcoins_mined,
            );
            number_of_rigs_that_responded += 1;
            if number_of_rigs_that_responded == self.mining_rigs {
                info!(
                    "All rigs answered. {} bitcoins produced total",
                    self.bitcoin_produced
                );
                break;
            }
        }
    }
}

#[instrument(skip_all)]
async fn launch_mining_rig(rig_id: i32, mut sender: Sender<(i32, i32)>) {
    let mut bitcoins_mined: i32 = 0;
    let mut now = Instant::now();

    debug!(rig_id, "Mining rig {} is starting", rig_id);

    loop {
        if now.elapsed() >= Duration::from_secs(1) {
            if mining_rig_finds_a_bitcoin() {
                bitcoins_mined += 1;
                info!(rig_id, bitcoins_mined, "mined 1 bitcoin!");
            }

            if mining_rig_dies() {
                error!(rig_id, "Rig {} dies unexpectedly", rig_id);

                match sender.send((rig_id, bitcoins_mined)).await {
                    Ok(()) => {
                        trace!(rig_id, "Mining rig successfuly returned its bitcoins")
                    }
                    Err(send_error) => error!(
                        rig_id,
                        "Error while sending bitcoins through the sender: {}", send_error
                    ),
                }
                break;
            }

            now = Instant::now();
        }
    }
}

fn mining_rig_finds_a_bitcoin() -> bool {
    let mut rng = thread_rng();
    rng.gen::<f64>() > 0.95
}

fn mining_rig_dies() -> bool {
    let mut rng = thread_rng();
    rng.gen::<f64>() > 0.97
}

use cosmwasm_std::Event;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CliKeyringAccount {
    pub key: &'static str,
    pub address: &'static str,
    pub mnemonic: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CliTxResponse {
    pub height: String,
    pub txhash: String,
    pub code: u32,
    pub raw_log: String,
    pub events: Vec<Event>,
}

impl CliTxResponse {
    pub fn is_success(&self) -> bool {
        return self.code == 0;
    }

    pub fn wait_for_tx_and_assert_success(&self, tx_type: Option<&str>) {
        if let Some(message) = tx_type {
            print!("Processing: {}", message);
        }
        if !self.is_success() {
            panic!(
                "Tx failed: {}\nCode: {}\n{}",
                self.txhash, self.code, &self.raw_log
            );
        }
        // Sleeping for an estimate of a block time creation so we make sure 
        // a success TX is included in a block and the txhash is able to be queried
        for _ in 0..7 {
            print!(".");
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
        println!("OK!");
    }

    pub fn get_attr(&self, event_type: &str, attribute_key: &str) -> Option<String> {
        self.events
            .clone()
            .into_iter()
            .find(|e| e.ty == event_type)
            .and_then(|e| e.attributes.into_iter().find(|a| a.key == attribute_key))
            .map(|a| a.value.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CliWasmQueryResponse<T> {
    pub data: T,
}

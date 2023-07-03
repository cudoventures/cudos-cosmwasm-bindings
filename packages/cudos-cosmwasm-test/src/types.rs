use cosmwasm_std::Event;
use serde::{Deserialize, Serialize};

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

    pub fn assert_success(&self) {
        if !self.is_success() {
            panic!(
                "Tx failed: {}\nCode: {}\n{}",
                self.txhash, self.code, &self.raw_log
            );
        }
    }

    pub fn get_attr(&self, event_type: &str, attribute_key: &str) -> String {
        self.events
            .clone()
            .into_iter()
            .find(|e| e.ty == event_type)
            .and_then(|e| {
                e.attributes
                    .into_iter()
                    .find(|a| a.key == base64::encode(attribute_key))
            })
            .map(|a| {
                String::from_utf8(base64::decode(a.value).unwrap()).unwrap()
            })
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CliWasmQueryResponse<T> {
    pub data: T,
}

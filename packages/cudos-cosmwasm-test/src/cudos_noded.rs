use crate::types::{CliKeyringAccount, CliTxResponse, CliWasmQueryResponse};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{
    env::{self},
    ffi::OsStr,
    path::Path,
    process::Command,
    sync::OnceLock,
    u64,
};

static INSTANCE: OnceLock<CudosNoded> = OnceLock::new();

#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct CudosNoded {}

impl CudosNoded {
    pub const INSTALL_PATH: &str = "/tmp/cudos-test-node";
    pub const CUDOS_HOME: &str = "/tmp/cudos-test-data";
    // todo set tagged version on next cudos-node release, e.g. "v1.2.0"
    pub const VERSION: &str = "cudos-dev-cosmos-v0.47.3";
    pub const CHAIN_ID: &str = "cudos-test-network";
    pub const ALICE: CliKeyringAccount = CliKeyringAccount {
    key: "validator",
    address: "cudos1phaxpevm5wecex2jyaqty2a4v02qj7qmk3ceu6",
    mnemonic: "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn",
};
    pub const BOB: CliKeyringAccount = CliKeyringAccount {
    key: "orchestrator",
    address: "cudos1cyyzpxplxdzkeea7kwsydadg87357qna7hvj7h",
    mnemonic: "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius"
};

    pub fn instance() -> &'static CudosNoded {
        INSTANCE.get_or_init(|| CudosNoded::start())
    }

    fn start() -> CudosNoded {
        env::set_var("CUDOS_HOME", CudosNoded::CUDOS_HOME);

        let setup_script = format!("{}/src/cudos-node.setup.sh", env!("CARGO_MANIFEST_DIR"));

        CudosNoded::run_command(
            &mut Command::new(setup_script)
                .env("VERSION", CudosNoded::VERSION)
                .env("INSTALL_PATH", CudosNoded::INSTALL_PATH)
                .env("CHAIN_ID", CudosNoded::CHAIN_ID)
                .env("VALIDATOR_MNEMONIC", CudosNoded::ALICE.mnemonic)
                .env("ORCHESTRATOR_MNEMONIC", CudosNoded::BOB.mnemonic),
        );

        CudosNoded {}
    }

    pub fn upload_contract(self, wasm_file_path: &Path, from: CliKeyringAccount) -> CliTxResponse {
        self.execute_tx(["wasm", "store", wasm_file_path.to_str().unwrap()], from)
    }

    pub fn instantiate_contract(
        self,
        code_id: u64,
        msg: &impl Serialize,
        label: String,
        admin: Option<String>,
        from: CliKeyringAccount,
    ) -> CliTxResponse {
        let admin_flag = match admin {
            Some(addr) => format!("--admin={}", addr),
            None => "--no-admin".to_string(),
        };

        self.execute_tx(
            [
                "wasm",
                "instantiate",
                &code_id.to_string(),
                &serde_json::to_string(&msg).unwrap(),
                &format!("--label={}", label),
                &admin_flag,
            ],
            from,
        )
    }

    pub fn query_tx_by_hash(&self, tx_hash: &str) -> CliTxResponse {
        let response = CudosNoded::run_command(&mut Command::new("cudos-noded").args([
            "q",
            "tx",
            tx_hash,
            "--output=json",
            &format!("--home={}", CudosNoded::CUDOS_HOME)
        ]));

        serde_json::from_str(&response).unwrap()
    }

    pub fn wasm_execute<T>(
        &self,
        contract_address: String,
        msg: &T,
        from: CliKeyringAccount,
    ) -> CliTxResponse
    where
        T: ?Sized + Serialize,
    {
        self.execute_tx(
            [
                "wasm",
                "execute",
                &contract_address,
                &serde_json::to_string(msg).unwrap(),
            ],
            from,
        )
    }

    pub fn wasm_query<T, U>(&self, contract_address: String, msg: &T) -> U
    where
        T: ?Sized + Serialize,
        U: DeserializeOwned,
    {
        let response = CudosNoded::run_command(&mut Command::new("cudos-noded").args([
            "query",
            "wasm",
            "contract-state",
            "smart",
            &contract_address,
            &serde_json::to_string(&msg).unwrap(),
            "--output=json",
            &format!("--home={}", CudosNoded::CUDOS_HOME)
        ]));

        serde_json::from_str::<CliWasmQueryResponse<U>>(&response)
            .unwrap()
            .data
    }

    pub fn execute_tx<I, S>(&self, args: I, from: CliKeyringAccount) -> CliTxResponse
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let tx_result = CudosNoded::run_command(
            &mut Command::new("cudos-noded")
                .arg("tx")
                .args(args)
                .arg(&format!("--from={}", from.key))
                .arg("--keyring-backend=test")
                .arg("--gas=auto")
                .arg(&format!("--chain-id={}", CudosNoded::CHAIN_ID))
                .arg("--yes")
                .arg("--output=json")
                .arg(&format!("--home={}", CudosNoded::CUDOS_HOME))
        );

        serde_json::from_str(&tx_result).unwrap()
    }

    fn run_command(cmd: &mut Command) -> String {
        match cmd.output() {
            Ok(output) => {
                let stdout =
                    String::from_utf8(output.stdout).expect("error parsing command stdout");
                let stderr =
                    String::from_utf8(output.stderr).expect("error parsing command stderr");

                if output.status.code().unwrap_or(-1) != 0 {
                    panic!("command error {}", stderr);
                }

                stdout
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        }
    }
}

use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cudos_cosmwasm::{CudosMsg, CudosMsgWrapper, CudosQuery, CudosQueryWrapper, DenomResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(CudosMsgWrapper), &out_dir);
    export_schema(&schema_for!(CudosMsg), &out_dir);
    export_schema(&schema_for!(CudosQueryWrapper), &out_dir);
    export_schema(&schema_for!(CudosQuery), &out_dir);
    export_schema(&schema_for!(DenomResponse), &out_dir);
}

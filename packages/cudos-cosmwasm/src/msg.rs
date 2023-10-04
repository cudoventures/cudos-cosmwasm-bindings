use cosmwasm_std::{CosmosMsg, CustomMsg, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// implement custom msg
impl CustomMsg for CudosMsg {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<CudosMsg> for CosmosMsg<CudosMsg> {
    fn from(original: CudosMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosMsg {
    IssueDenomMsg {
        id: String,
        name: String,
        symbol: String,
        schema: Option<String>,
        sender: String,
        contract_address_signer: String,
        traits: Option<String>,
        minter: Option<String>,
        description: Option<String>,
        data: Option<String>,
    },
    MintNftMsg {
        denom_id: String,
        name: String,
        uri: Option<String>,
        data: Option<String>,
        sender: String,
        contract_address_signer: String,
        recipient: String,
    },
    EditNftMsg {
        denom_id: String,
        token_id: String,
        name: Option<String>,
        uri: Option<String>,
        data: Option<String>,
        sender: String,
        contract_address_signer: String,
    },
    TransferNftMsg {
        denom_id: String,
        token_id: String,
        from: String,
        to: String,
        sender: String,
        contract_address_signer: String,
    },
    TransferDenomMsg {
        denom_id: String,
        to: String,
        sender: String,
        contract_address_signer: String,
    },
    BurnNftMsg {
        denom_id: String,
        token_id: String,
        sender: String,
        contract_address_signer: String,
    },
    ApproveNftMsg {
        denom_id: String,
        token_id: String,
        approved_address: String,
        sender: String,
        contract_address_signer: String,
    },
    ApproveAllMsg {
        approved_operator: String,
        approved: bool,
        sender: String,
        contract_address_signer: String,
    },
    RevokeApprovalMsg {
        denom_id: String,
        token_id: String,
        address_to_revoke: String,
        sender: String,
        contract_address_signer: String,
    },
    PublishCollectionMsg {
        creator: String,
        denom_id: String,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
    },
    CreateAddressMsg {
        creator: String,
        network: String,
        label: String,
        value: String,
    },
    UpdateAddressMsg {
        creator: String,
        network: String,
        label: String,
        value: String,
    },
    DeleteAddressMsg {
        creator: String,
        network: String,
        label: String,
    },
    PublishNftMsg {
        creator: String,
        token_id: String,
        denom_id: String,
        price: Coin,
    },
    BuyNftMsg {
        creator: String,
        id: u64,
    },
    MintNftMarketplaceMsg {
        creator: String,
        denom_id: String,
        recipient: String,
        price: Coin,
        name: String,
        uri: Option<String>,
        data: Option<String>,
        uid: String,
    },
    RemoveNftMsg {
        creator: String,
        id: u64,
    },
    VerifyCollectionMsg {
        creator: String,
        id: u64,
    },
    UnverifyCollectionMsg {
        creator: String,
        id: u64,
    },
    CreateCollectionMsg {
        creator: String,
        id: String,
        name: String,
        symbol: String,
        schema: Option<String>,
        traits: Option<String>,
        minter: Option<String>,
        description: Option<String>,
        data: Option<String>,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
        verified: bool,
    },
    UpdateRoyaltiesMsg {
        creator: String,
        id: u64,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
    },
    UpdatePriceMsg {
        creator: String,
        id: u64,
        price: Coin,
    },
    AddAdminMsg {
        creator: String,
        address: String,
    },
    RemoveAdminMsg {
        creator: String,
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Royalty {
    pub address: String,
    pub percent: String,
}

pub fn create_issue_denom_msg(
    id: String,
    name: String,
    symbol: String,
    schema: Option<String>,
    sender: String,
    contract_address_signer: String,
    traits: Option<String>,
    minter: Option<String>,
    description: Option<String>,
    data: Option<String>,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::IssueDenomMsg {
        id,
        name,
        symbol,
        schema,
        sender,
        contract_address_signer,
        traits,
        minter,
        description,
        data,
    }
    .into()
}

pub fn create_mint_nft_msg(
    denom_id: String,
    name: String,
    uri: Option<String>,
    data: Option<String>,
    recipient: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::MintNftMsg {
        denom_id,
        name,
        uri,
        data,
        sender,
        recipient,
        contract_address_signer,
    }
    .into()
}

pub fn create_edit_nft_msg(
    denom_id: String,
    token_id: String,
    name: Option<String>,
    uri: Option<String>,
    data: Option<String>,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::EditNftMsg {
        token_id,
        denom_id,
        name,
        uri,
        data,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_burn_nft_msg(
    denom_id: String,
    token_id: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::BurnNftMsg {
        token_id,
        denom_id,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_transfer_nft_msg(
    denom_id: String,
    token_id: String,
    from: String,
    to: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::TransferNftMsg {
        denom_id,
        token_id,
        from,
        to,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_transfer_denom_msg(
    denom_id: String,
    to: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::TransferDenomMsg {
        denom_id,
        to,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_approve_nft_msg(
    denom_id: String,
    token_id: String,
    approved_address: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::ApproveNftMsg {
        token_id,
        denom_id,
        approved_address,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_approve_all_msg(
    approved_operator: String,
    approved: bool,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::ApproveAllMsg {
        approved_operator,
        approved,
        sender,
        contract_address_signer,
    }
    .into()
}

pub fn create_revoke_msg(
    denom_id: String,
    token_id: String,
    address_to_revoke: String,
    sender: String,
    contract_address_signer: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::RevokeApprovalMsg {
        denom_id,
        token_id,
        address_to_revoke,
        sender,
        contract_address_signer,
    }
    .into()
}

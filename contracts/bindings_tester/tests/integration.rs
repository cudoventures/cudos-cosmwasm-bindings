use bindings_tester::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::Coin;
use cudos_cosmwasm::{
    CollectionResponse, CollectionsResponse, Denom, DenomResponse, DenomsResponse, MarketplaceNft,
    OwnerCollectionResponse, QueryAdressResponse, QueryAllAdressesResponse,
    QueryAllCollectionsResponse, QueryAllNftsResponse, QueryApprovalsResponse,
    QueryApprovedForAllResponse, QueryCollectionByDenomIdResponse,
    QueryCollectionMarketplaceResponse, QueryListAdminsResponse, QueryNFTResponse,
    QueryNftMarketplaceResponse, Royalty, SupplyResponse, NFT,
};
use cudos_cosmwasm_test::cudos_noded::CudosNoded;
use std::path::Path;

const WASM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/bindings_tester.wasm"
);

#[test]
fn bindings_work() {
    let node = CudosNoded::instance();
    let alice = CudosNoded::ALICE;
    let bob = CudosNoded::BOB;

    let raw_upload_res = node.upload_contract(Path::new(WASM_PATH), alice);
    raw_upload_res.wait_for_tx_and_assert_success(Some("UploadContract"));

    let upload_res = node.query_tx_by_hash(&raw_upload_res.txhash);

    let code_id = upload_res
        .get_attr("store_code", "code_id")
        .ok_or_else(|| String::from("Attribute 'code_id' not found in 'store_code' event"))
        .and_then(|attr| {
            attr.parse::<u64>()
                .map_err(|e| format!("Failed to parse code_id: {}", e))
        })
        .unwrap();

    let raw_instantiate_res = node.instantiate_contract(
        code_id,
        &InstantiateMsg {},
        "bindings-tester".to_string(),
        Some(alice.address.to_string()),
        alice,
    );

    raw_instantiate_res.wait_for_tx_and_assert_success(Some("InstantiateMsg"));
    let instantiate_res = node.query_tx_by_hash(&raw_instantiate_res.txhash);

    let contract_address = instantiate_res
        .get_attr("instantiate", "_contract_address")
        .ok_or_else(|| {
            String::from("Attribute 'contract_address' not found in 'instantiate' event")
        })
        .unwrap();

    let denom_id = "testdenom";

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::IssueDenomMsg {
            id: denom_id.to_string(),
            name: denom_id.to_string(),
            symbol: denom_id.to_string(),
            schema: None,
            traits: None,
            minter: None,
            description: Some("somedesc".to_string()),
            data: None,
        },
        alice,
    )
    .wait_for_tx_and_assert_success(Some("IssueDenomMsg"));

    let denom_res: DenomResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryDenomById {
            denom_id: denom_id.to_string(),
        },
    );

    let expected_denom = Denom {
        id: denom_id.to_string(),
        name: denom_id.to_string(),
        schema: Some("".to_string()),
        creator: alice.address.to_string(),
        symbol: denom_id.to_string(),
        traits: Some("".to_string()),
        minter: Some("".to_string()),
        description: Some("somedesc".to_string()),
        data: Some("".to_string()),
    };

    assert_eq!(denom_res.denom, expected_denom.clone(),);

    let denom_res: DenomResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryDenomByName {
            denom_name: denom_id.to_string(),
        },
    );

    assert_eq!(denom_res.denom, expected_denom.clone(),);

    let denom_res: DenomResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryDenomBySymbol {
            denom_symbol: denom_id.to_string(),
        },
    );

    assert_eq!(denom_res.denom, expected_denom.clone(),);

    let all_denoms_res: DenomsResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryDenoms { pagination: None },
    );

    assert!(all_denoms_res
        .denoms
        .unwrap()
        .iter()
        .any(|d| d.id == denom_id.to_string()));

    let mut nft_name = "testnft".to_string();

    let raw_mint_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::MintNftMsg {
            denom_id: denom_id.to_string(),
            name: nft_name.clone(),
            uri: None,
            data: None,
            recipient: alice.address.to_string(),
        },
        alice,
    );

    raw_mint_res.wait_for_tx_and_assert_success(Some("MintNftMsg"));

    let mint_res = node.query_tx_by_hash(&raw_mint_res.txhash);

    let token_id = mint_res
        .get_attr("mint_nft", "token_id")
        .ok_or_else(|| String::from("Attribute 'token_id' not found in 'mint_nft' event"))
        .unwrap();

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::TransferNftMsg {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
            from: alice.address.to_string(),
            to: bob.address.to_string(),
        },
        alice,
    )
    .wait_for_tx_and_assert_success(Some("TransferNftMsg"));

    nft_name.push_str(nft_name.clone().as_str());

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::EditNftMsg {
            denom_id: denom_id.to_string(),
            name: Some(nft_name.to_string()),
            token_id: token_id.clone(),
            uri: None,
            data: None,
        },
        bob,
    )
    .wait_for_tx_and_assert_success(Some("EditNftMsg"));

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::ApproveNftMsg {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
            approved_address: alice.address.to_string(),
        },
        bob,
    )
    .wait_for_tx_and_assert_success(Some("ApproveNftMsg"));

    let approvals_res: QueryApprovalsResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryApprovals {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
        },
    );

    assert_eq!(
        approvals_res.approved_addresses,
        vec![alice.address.to_string()]
    );

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::RevokeApprovalMsg {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
            address_to_revoke: alice.address.to_string(),
        },
        bob,
    )
    .wait_for_tx_and_assert_success(Some("RevokeApprovalMsg"));

    let token_res: QueryNFTResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryToken {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
        },
    );

    assert_eq!(
        token_res.nft,
        NFT {
            id: token_id.clone(),
            name: Some(nft_name.to_string()),
            uri: Some("".to_string()),
            data: Some("".to_string()),
            owner: bob.address.to_string(),
            approved_addresses: None
        }
    );

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::ApproveAllMsg {
            approved_operator: bob.address.to_string(),
            approved: true,
        },
        alice,
    )
    .wait_for_tx_and_assert_success(Some("ApproveAllMsg"));

    let all_approvals_res: QueryApprovedForAllResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryApprovedForAll {
            owner_address: alice.address.to_string(),
            operator_address: bob.address.to_string(),
        },
    );

    assert!(all_approvals_res.is_approved);

    let collection_res: CollectionResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryCollection {
            denom_id: denom_id.to_string(),
            pagination: None,
        },
    );

    assert_eq!(
        collection_res.clone().collection.unwrap().denom.id,
        denom_id.to_string()
    );

    assert_eq!(
        collection_res
            .clone()
            .collection
            .unwrap()
            .nfts
            .unwrap()
            .first()
            .unwrap()
            .id,
        token_id.clone()
    );

    let collections_res: CollectionsResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryCollectionsByDenomIds {
            denom_ids: vec![denom_id.to_string()],
        },
    );

    assert_eq!(
        collections_res
            .clone()
            .collections
            .unwrap()
            .first()
            .unwrap()
            .denom
            .id,
        denom_id.to_string()
    );

    assert_eq!(
        collections_res
            .clone()
            .collections
            .unwrap()
            .first()
            .unwrap()
            .clone()
            .nfts
            .unwrap()
            .first()
            .unwrap()
            .id,
        token_id.clone()
    );

    let supply_res: SupplyResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QuerySupply {
            denom_id: denom_id.to_string(),
        },
    );

    assert_eq!(supply_res.amount, 1);

    let owner_res: OwnerCollectionResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryOwner {
            denom_id: Some(denom_id.to_string()),
            address: bob.address.to_string(),
            pagination: None,
        },
    );

    assert_eq!(
        owner_res
            .owner
            .id_collections
            .first()
            .unwrap()
            .token_ids
            .first()
            .unwrap(),
        &token_id.clone()
    );

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::BurnNftMsg {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
        },
        bob,
    )
    .wait_for_tx_and_assert_success(Some("BurnNftMsg"));

    let supply_res: SupplyResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QuerySupply {
            denom_id: denom_id.to_string(),
        },
    );

    assert_eq!(supply_res.amount, 0);

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::TransferDenomMsg {
            denom_id: denom_id.to_string(),
            to: bob.address.to_string(),
        },
        alice,
    )
    .wait_for_tx_and_assert_success(Some("TransferDenomMsg"));

    let royalties = vec![Royalty {
        address: alice.address.to_string(),
        percent: "100".to_string(),
    }];

    let raw_publish_collection_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::PublishCollectionMsg {
            denom_id: denom_id.to_string(),
            mint_royalties: royalties.clone(),
            resale_royalties: royalties.clone(),
        },
        bob,
    );

    raw_publish_collection_res.wait_for_tx_and_assert_success(Some("PublishCollectionMsg"));

    let publish_collection_res = node.query_tx_by_hash(&raw_publish_collection_res.txhash);

    let collection_id = publish_collection_res
        .get_attr("publish_collection", "collection_id")
        .ok_or_else(|| {
            String::from("Attribute 'collection_id' not found in 'publish_collection' event")
        })
        .and_then(|attr| {
            attr.parse::<u64>()
                .map_err(|e| format!("Failed to parse collection_id: {}", e))
        })
        .unwrap();

    let all_collections_res: QueryAllCollectionsResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryAllCollections { pagination: None },
    );

    assert_eq!(all_collections_res.collections.len(), 1);

    let collection_res: QueryCollectionMarketplaceResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryCollectionMarketplace { id: collection_id },
    );

    assert_eq!(collection_res.collection.id, collection_id);

    let collection_by_denom_res: QueryCollectionByDenomIdResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryCollectionByDenomId {
            denom_id: denom_id.to_string(),
        },
    );

    assert_eq!(
        collection_by_denom_res.collection.owner,
        bob.address.to_string()
    );

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::AddAdminMsg {
            address: bob.address.to_string(),
        },
        alice,
    )
    .wait_for_tx_and_assert_success(Some("AddAdminMsg"));

    let list_admins_res: QueryListAdminsResponse =
        node.wasm_query(contract_address.clone(), &QueryMsg::QueryListAdmins {});

    assert_eq!(list_admins_res.admins, vec![bob.address.to_string()]);

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::VerifyCollectionMsg { id: collection_id },
        bob,
    )
    .wait_for_tx_and_assert_success(Some("VerifyCollectionMsg"));

    let raw_mint_nft_marketplace_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::MintNftMarketplaceMsg {
            denom_id: denom_id.to_string(),
            recipient: alice.address.to_string(),
            price: Coin::new(1000, "acudos"),
            name: nft_name.clone(),
            uri: None,
            data: None,
            uid: "1".to_string(),
        },
        bob,
    );

    raw_mint_nft_marketplace_res.wait_for_tx_and_assert_success(Some("MintNftMarketplaceMsg"));
    let mint_nft_marketplace_res = node.query_tx_by_hash(&raw_mint_nft_marketplace_res.txhash);

    let token_id = mint_nft_marketplace_res
        .get_attr("marketplace_mint_nft", "token_id")
        .ok_or_else(|| {
            String::from("Attribute 'token_id' not found in 'marketplace_mint_nft' event")
        })
        .unwrap();

    let raw_publish_nft_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::PublishNftMsg {
            token_id: token_id.clone(),
            denom_id: denom_id.to_string(),
            price: Coin::new(1000, "acudos"),
        },
        bob,
    );

    raw_publish_nft_res.wait_for_tx_and_assert_success(Some("PublishNftMsg"));
    let publish_nft_res = node.query_tx_by_hash(&raw_publish_nft_res.txhash);

    let nft_id = publish_nft_res
        .get_attr("publish_nft", "nft_id")
        .ok_or_else(|| String::from("Attribute 'nft_id' not found in 'publish_nft' event"))
        .and_then(|attr| {
            attr.parse::<u64>()
                .map_err(|e| format!("Failed to parse collection_id: {}", e))
        })
        .unwrap();

    let query_nft_res: QueryNftMarketplaceResponse =
        node.wasm_query(contract_address.clone(), &QueryMsg::QueryNft { id: nft_id });

    assert_eq!(
        query_nft_res.nft,
        MarketplaceNft {
            id: nft_id,
            token_id,
            denom_id: denom_id.to_string(),
            price: Coin::new(1000, "acudos"),
            owner: bob.address.to_string()
        }
    );

    let all_nfts_res: QueryAllNftsResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryAllNfts { pagination: None },
    );

    assert_eq!(all_nfts_res.nfts.len(), 1);

    // ADDRESSBOOK
    let network = "network".to_string();
    let label = "label".to_string();
    let value = "value".to_string();
    let updated_value = "updated_value".to_string();

    // TEST Create new addressbook
    let create_address_msg = &ExecuteMsg::CreateAddressMsg {
        network: network.clone(),
        label: label.clone(),
        value: value.clone(),
    };
    let raw_create_addressbook_address_res =
        node.wasm_execute(contract_address.clone(), create_address_msg, bob);
    raw_create_addressbook_address_res.wait_for_tx_and_assert_success(Some("CreateAddressMsg"));

    // TEST Addressbook Queries
    if let ExecuteMsg::CreateAddressMsg {
        network,
        label,
        value,
    } = &create_address_msg
    {
        // TEST Single address query
        let single_address_query_res: QueryAdressResponse = node.wasm_query(
            contract_address.clone(),
            &QueryMsg::QueryAddress {
                creator: bob.address.to_string(),
                network: network.to_string(),
                label: label.to_string(),
            },
        );
        let mut created_address = single_address_query_res.address.clone();
        assert_eq!(created_address.network, *network);
        assert_eq!(created_address.label, *label);
        assert_eq!(created_address.value, *value);
        assert_eq!(created_address.creator, bob.address);

        // TEST All-addresses query
        let all_addresses_res: QueryAllAdressesResponse = node.wasm_query(
            contract_address.clone(),
            &QueryMsg::QueryAllAddresses { pagination: None },
        );
        assert_eq!(all_addresses_res.address.len(), 1);
        created_address = all_addresses_res.address[0].clone();
        assert_eq!(created_address.network, *network);
        assert_eq!(created_address.label, *label);
        assert_eq!(created_address.value, *value);
        assert_eq!(created_address.creator, bob.address);
    }

    // TEST Update addressbook
    let update_address_msg = &ExecuteMsg::UpdateAddressMsg {
        network: network.clone(),
        label: label.clone(),
        value: updated_value.clone(),
    };
    let raw_updated_addressbook_address_res =
        node.wasm_execute(contract_address.clone(), update_address_msg, bob);
    raw_updated_addressbook_address_res.wait_for_tx_and_assert_success(Some("UpdateAddressMsg"));

    // ASSERT UPDATED ADDRESS CHANGED VALUES
    if let ExecuteMsg::UpdateAddressMsg {
        network,
        label,
        value,
    } = &update_address_msg
    {
        let all_addresses_res: QueryAllAdressesResponse = node.wasm_query(
            contract_address.clone(),
            &QueryMsg::QueryAllAddresses { pagination: None },
        );
        assert_eq!(all_addresses_res.address.len(), 1);
        let updated_address = all_addresses_res.address[0].clone();
        assert_eq!(updated_address.network, *network);
        assert_eq!(updated_address.label, *label);
        assert_eq!(updated_address.value, *value);
        assert_eq!(updated_address.creator, bob.address);
    }

    // TEST Delete addressbook
    let delete_address_msg = &ExecuteMsg::DeleteAddressMsg {
        network: network.clone(),
        label: label.clone(),
    };
    let raw_delete_addressbook_address_res =
        node.wasm_execute(contract_address.clone(), delete_address_msg, bob);
    raw_delete_addressbook_address_res.wait_for_tx_and_assert_success(Some("DeleteAddressMsg"));

    //ASSERT EMPTY ADDRESSBOOK AFTER SINGLE RECORD DELETION
    let all_addresses_res: QueryAllAdressesResponse = node.wasm_query(
        contract_address.clone(),
        &QueryMsg::QueryAllAddresses { pagination: None },
    );
    assert_eq!(all_addresses_res.address.len(), 0);
}

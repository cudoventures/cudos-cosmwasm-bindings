use cosmwasm_std::Coin;
use cudos_cosmwasm::{
    CollectionResponse, CollectionsResponse, Denom, DenomResponse, DenomsResponse, MarketplaceNft,
    OwnerCollectionResponse, QueryAllCollectionsResponse, QueryAllNftsResponse,
    QueryApprovalsResponse, QueryApprovedForAllResponse, QueryCollectionByDenomIdResponse,
    QueryCollectionMarketplaceResponse, QueryListAdminsResponse, QueryNFTResponse,
    QueryNftMarketplaceResponse, Royalty, SupplyResponse, NFT,
};
use cudos_cosmwasm_test::cudos_noded::CudosNoded;
use nft_bindings_tester::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use std::path::Path;

const WASM_PATH: &str = "../../artifacts/nft_bindings_tester.wasm";

#[test]
fn bindings_work() {
    println!("Starting Test Node...");
    let node = CudosNoded::instance();
    let alice = CudosNoded::ALICE;
    let bob = CudosNoded::BOB;

    println!("Uploading contract from {}...", WASM_PATH);
    let upload_res = node.upload_contract(Path::new(WASM_PATH), alice);

    upload_res.assert_success();

    let code_id = upload_res
        .get_attr("store_code", "code_id")
        .parse::<u64>()
        .unwrap();

    println!("Instantiating uploaded contract with code id {}.", code_id);
    let instantiate_res = node.instantiate_contract(
        code_id,
        &InstantiateMsg {},
        "nft-bindings-tester".to_string(),
        Some(alice.address.to_string()),
        alice,
    );

    instantiate_res.assert_success();

    let contract_address = instantiate_res.get_attr("instantiate", "_contract_address");
    println!("Instantiated. Contract address: {}.", contract_address);
    println!("Start testing...");

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
    .assert_success();

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

    let mint_res = node.wasm_execute(
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

    mint_res.assert_success();

    let token_id = mint_res.get_attr("mint_nft", "token_id");

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
    .assert_success();

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
    .assert_success();

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::ApproveNftMsg {
            denom_id: denom_id.to_string(),
            token_id: token_id.clone(),
            approved_address: alice.address.to_string(),
        },
        bob,
    )
    .assert_success();

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
    .assert_success();

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
    .assert_success();

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
    .assert_success();

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
    .assert_success();

    let royalties = vec![Royalty {
        address: alice.address.to_string(),
        percent: "100".to_string(),
    }];

    let publish_collection_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::PublishCollectionMsg {
            denom_id: denom_id.to_string(),
            mint_royalties: royalties.clone(),
            resale_royalties: royalties.clone(),
        },
        bob,
    );

    publish_collection_res.assert_success();

    let collection_id = publish_collection_res
        .get_attr("publish_collection", "collection_id")
        .parse::<u64>()
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
    .assert_success();

    let list_admins_res: QueryListAdminsResponse =
        node.wasm_query(contract_address.clone(), &QueryMsg::QueryListAdmins {});

    assert_eq!(list_admins_res.admins, vec![bob.address.to_string()]);

    node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::VerifyCollectionMsg { id: collection_id },
        bob,
    )
    .assert_success();

    let mint_nft_marketplace_res = node.wasm_execute(
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

    mint_nft_marketplace_res.assert_success();

    let token_id = mint_nft_marketplace_res.get_attr("marketplace_mint_nft", "token_id");

    let publish_nft_res = node.wasm_execute(
        contract_address.clone(),
        &ExecuteMsg::PublishNftMsg {
            token_id: token_id.clone(),
            denom_id: denom_id.to_string(),
            price: Coin::new(1000, "acudos"),
        },
        bob,
    );

    publish_nft_res.assert_success();

    let nft_id = publish_nft_res
        .get_attr("publish_nft", "nft_id")
        .parse::<u64>()
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
}

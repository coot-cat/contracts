use crate::{
    error::ContractError,
    msg::ExecMsg,
    multitest::contract::{CodeId, Contract},
};
use cat::CatMetadata;
use cosmwasm_std::{coin, coins, Addr, Api, BlockInfo, Timestamp};
use cw_multi_test::App;
use sha2::{digest::Update, Digest, Sha256};

const ADMIN: &str = "admin";
const SENDER: &str = "sender";
const OWNER: &str = "owner";

use crate::state::{CONFIG, NETX_CAT_ID};
#[test]
fn secp256k1_verify_test() {
    let app = App::default();
    let msg = b"test";
    let hash = Sha256::new().chain(msg).finalize();

    let msg_hash =
        hex_literal::hex!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    assert_eq!(hash.as_slice(), &msg_hash);

    let sign = hex_literal::hex!("9231daabb433b2fcd94cedaf0eff9ccf7e8c9db383075a74b893153c432a7e6e1fa2e3349883ce3676f85c81cdfd71d2ed07a49ab81a16a4a9ac557b54267738");
    let pubkey =
        hex_literal::hex!("03efe62cf30083241a03eb531d750efb47e442df8e50bd90dd2b9b5b4651438cb1");
    let result = app
        .api()
        .secp256k1_verify(msg_hash.as_slice(), sign.as_slice(), pubkey.as_slice())
        .unwrap();

    assert!(result, "secp256k1 verify failed~~~!!!");
}

fn init_test() -> (App, Contract) {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(SENDER), coins(10000000, "unibi"))
            .unwrap();
    });

    let code_id = CodeId::store_code(&mut app);
    let cat_code_id = cat::multitest::CodeId::store_code(&mut app);
    let gear_code_id = gear::multitest::CodeId::store_code(&mut app);
    let token_id_prefix = "http://localhost:300/nft".to_string();
    let owner = OWNER.to_string();
    let mint_price = coin(10000, "unibi");
    let checkin_pubkey =
        "03f40368599183a34155478ac9f76bff94400c9a64d6d8f3ae453cbfbed7ffba5e".to_string();
    let sender = Addr::unchecked(SENDER);
    let admin = ADMIN.to_string();

    // 初始化
    let contract = Contract::instantiate(
        &mut app,
        code_id.id(),
        owner,
        token_id_prefix,
        mint_price.clone(),
        cat_code_id.id(),
        gear_code_id.id(),
        checkin_pubkey,
        sender.clone(),
        admin,
    )
    .unwrap();
    (app, contract)
}

#[test]
fn test_mint() {
    let (mut app, contract) = init_test();
    // 初始化后猫猫Id应为1
    let querier = app.wrap();
    let next_cat_id = NETX_CAT_ID.query(&querier, contract.addr()).unwrap();
    assert_eq!(next_cat_id, 1);

    let cat_name = "Kitty";
    contract
        .mint(
            &mut app,
            SENDER,
            &coins(10000, "unibi"),
            cat_name.to_string(),
            1,
            2,
            1,
        )
        .unwrap();

    let querier = app.wrap();
    let next_cat_id = NETX_CAT_ID.query(&querier, contract.addr()).unwrap();
    assert_eq!(next_cat_id, 2);

    let config = CONFIG.query(&querier, contract.addr()).unwrap();

    let cat_tokens: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(
            config.cat_contract.clone(),
            &cat::QueryMsg::Tokens {
                owner: SENDER.to_string(),
                start_after: None,
                limit: None,
            },
        )
        .unwrap();

    assert_eq!(
        cat_tokens,
        cw721::TokensResponse {
            tokens: vec!["1".to_string()]
        }
    );

    let cat_info: cw721::NftInfoResponse<CatMetadata> = app
        .wrap()
        .query_wasm_smart(
            config.cat_contract,
            &cat::QueryMsg::NftInfo {
                token_id: "1".to_string(),
            },
        )
        .unwrap();
    assert_eq!(cat_info.extension.name, cat_name);
}

#[test]
fn test_checkin() {
    let (mut app, contract) = init_test();
    app.set_block(BlockInfo {
        chain_id: "dev-test-chain".to_string(),
        height: 100,
        time: Timestamp::from_seconds(3600 * 1000),
    });

    let msg_hash = "dc82bc70f188933325e6ee9b048f9b583d5e0557115ab0e6c76aeaa76d23801c".to_string();
    let msg_signature= "2c717e8baabd802ad6ad83b0c1cd9e3f7eeee8f505d31df6bae4b37f3a25b67a15da3b1c2a46aedeab4eccf0401adde4186896b33db7c8b6baa87e8652d9ce02".to_string();

    contract
        .mint(
            &mut app,
            SENDER,
            &coins(10000, "unibi"),
            "TestKitty".to_string(),
            1,
            1,
            1,
        )
        .unwrap();

    contract
        .checkin(&mut app, SENDER, &[], 0, msg_hash, msg_signature)
        .unwrap();

    let check_count = contract
        .query_checkin_count(&app, SENDER.to_string())
        .unwrap();
    assert_eq!(check_count, 1);
}

#[test]
fn test_withdraw() {
    let (mut app, contract) = init_test();
    contract
        .mint(
            &mut app,
            SENDER,
            &coins(100000, "unibi"),
            "TestKitty".to_string(),
            1,
            1,
            1,
        )
        .unwrap();

    contract.withdraw(&mut app, OWNER).unwrap();
}

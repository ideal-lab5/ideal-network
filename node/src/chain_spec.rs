//! Substrate chain configurations.

use beefy_primitives::bls_crypto::AuthorityId as BeefyId;
use cumulus_primitives_core::ParaId;
use ideal_nw_runtime as runtime;
use runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    RuntimeAppPublic,
};

use ark_serialize::CanonicalSerialize;
use ark_std::UniformRand;
use etf_crypto_primitives::dpss::acss::DoubleSecret;
use rand::rngs::OsRng;
use w3f_bls::{DoublePublicKey, DoublePublicKeyScheme, EngineBLS, SerializableToBytes, TinyBLS377};
/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed.
pub fn get_pair_from_seed<TPublic: Public>(seed: &str) -> TPublic::Pair {
    TPublic::Pair::from_string(&format!("//{}", seed), None).expect("static values are valid; qed")
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

pub fn get_beefy_etf_keys_from_seed(seed: &str) -> BeefyId {
    get_from_seed::<BeefyId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(aura_keys: AuraId, beefy_keys: BeefyId) -> runtime::SessionKeys {
    runtime::SessionKeys {
        aura: aura_keys,
        beefy: beefy_keys,
    }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "IDN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(),
            // You MUST set this to the correct network!
            para_id: 2000,
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
                get_beefy_etf_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
                get_beefy_etf_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        2000.into(),
    ))
    .build()
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "IDN".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(),
            // You MUST set this to the correct network!
            para_id: 2000,
        },
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
                get_beefy_etf_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
                get_beefy_etf_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        2000.into(),
    ))
    .with_protocol_id("template-local")
    .with_properties(properties)
    .build()
}

/// Helper function to prepare initial secrets and resharing for ETF conensus
/// return a vec of (authority id, resharing, pubkey commitment) along with ibe public key against the master secret
pub fn etf_genesis<E: EngineBLS>(
    initial_authorities: Vec<BeefyId>,
    seeds: Vec<&str>,
) -> (Vec<u8>, Vec<(BeefyId, Vec<u8>)>) {
    let msk_prime = E::Scalar::rand(&mut OsRng);
    let keypair = w3f_bls::KeypairVT::<E>::generate(&mut OsRng);
    let msk: E::Scalar = keypair.secret.0;
    let double_public: DoublePublicKey<E> = DoublePublicKey(
        keypair.into_public_key_in_signature_group().0,
        keypair.public.0,
    );

    let double_secret = DoubleSecret::<E>(msk, msk_prime);

    let mut double_public_bytes = Vec::new();
    double_public
        .serialize_compressed(&mut double_public_bytes)
        .unwrap();

    let genesis_resharing = double_secret
        .reshare(
            &initial_authorities
                .iter()
                .map(|authority| {
                    w3f_bls::single::PublicKey::<E>(
                        w3f_bls::double::DoublePublicKey::<E>::from_bytes(&authority.to_raw_vec())
                            .unwrap()
                            .1,
                    )
                })
                .collect::<Vec<_>>(),
            initial_authorities.len() as u8, // threshold = full set of authorities for now
            &mut OsRng,
        )
        .unwrap();

    let resharings = initial_authorities
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let pok = &genesis_resharing[idx].1;
            let mut bytes = Vec::new();
            pok.serialize_compressed(&mut bytes).unwrap();

            let seed = seeds[idx];
            let test = get_pair_from_seed::<BeefyId>(seed);
            let t = sp_core::bls::Pair::<TinyBLS377>::from(test);
            let o = t
                .acss_recover(&bytes, initial_authorities.len() as u8)
                .expect("genesis shares should be well formatted");
            let etf_id = BeefyId::from(o.public());
            (etf_id, bytes)
        })
        .collect::<Vec<_>>();
    (double_public_bytes, resharings)
}

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, AuraId, BeefyId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
    id: ParaId,
) -> serde_json::Value {
    let (round_key, genesis_shares) = etf_genesis::<TinyBLS377>(
        initial_authorities
            .iter()
            .map(|x| x.3.clone())
            .collect::<Vec<_>>(),
        vec!["Alice", "Bob"],
    );
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "collatorSelection": {
            "invulnerables": initial_authorities.iter().cloned().map(|(acc, _, _, _)| acc).collect::<Vec<_>>(),
            "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
        },
        "session": {
            "keys": initial_authorities
                .into_iter()
                .map(|(acc, _, aura, beefy_etf)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura, beefy_etf), // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(root) },
        "etf": {
            "genesisResharing": genesis_shares,
            "roundPubkey": round_key,
        },
        "beefy": {
            "authorities": Vec::<BeefyId>::new(),
            "genesisBlock": Some(1),
        },
    })
}

use ethereum_consensus::primitives::{BlsPublicKey, BlsSignature, U256};
pub use ethereum_consensus::{
    bellatrix::mainnet::MAX_TRANSACTIONS_PER_PAYLOAD, builder::SignedValidatorRegistration,
    deneb::mainnet as spec,
};
use ssz_rs::prelude::*;

// NOTE: type alias here to call out the important types clearly, in lieu of just `pub use ...`
pub type ExecutionPayload = spec::ExecutionPayload;
pub type ExecutionPayloadHeader = spec::ExecutionPayloadHeader;
pub type SignedBlindedBeaconBlock = spec::SignedBlindedBeaconBlock;
pub type BlindedBlobSidecar = spec::BlindedBlobSidecar;
pub type SignedBlindedBlobSidecar = spec::SignedBlindedBlobSidecar;
pub type Blob = spec::Blob;

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuilderBid {
    pub header: spec::ExecutionPayloadHeader,
    pub value: U256,
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
    pub blinded_blob_sidecars: List<BlindedBlobSidecar, MAX_TRANSACTIONS_PER_PAYLOAD>,
}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBuilderBid {
    pub message: BuilderBid,
    pub signature: BlsSignature,
}

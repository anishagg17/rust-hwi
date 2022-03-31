use bitcoin::util::address::Address;
use bitcoin::util::bip32::ExtendedPubKey;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct HWIExtendedPubKey {
    pub xpub: ExtendedPubKey,
}

#[derive(Deserialize)]
pub struct HWISignature {
    pub signature: String,
}

#[derive(Deserialize)]
pub struct HWIAddress {
    pub address: Address,
}

#[derive(Deserialize)]
pub struct HWIPartiallySignedTransaction {
    pub psbt: String,
}

// TODO: use Descriptors
#[derive(Deserialize)]
pub struct HWIDescriptor {
    pub internal: Vec<String>,
    pub receive: Vec<String>,
}

#[derive(Deserialize)]
pub struct HWIKeyPoolElement {
    pub desc: String,
    pub range: Vec<u32>,
    pub timestamp: String,
    pub internal: bool,
    pub keypool: bool,
    pub watchonly: bool,
}

#[derive(Deserialize)]
pub struct HWISetup {
    pub success: bool,
}

#[derive(Clone)]
pub enum HWIAddressType {
    Pkh,
    ShWpkh,
    Wpkh,
}

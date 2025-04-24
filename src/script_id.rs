use anyhow::Result;
use zcash_address::{ToAddress, ZcashAddress};

use zewif::{Network, u160};
use zewif::{parse, parser::prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScriptId(u160);

impl ScriptId {
    pub fn to_string(&self, network: Network) -> String {
        // Create proper 20-byte array for the script hash
        let mut script_hash = [0u8; 20];
        script_hash.copy_from_slice(self.0.as_ref());

        // Create a transparent P2SH address using the proper constructor
        let addr = ZcashAddress::from_transparent_p2sh(network.into(), script_hash);
        addr.to_string()
    }
}

impl Parse for ScriptId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let script_id = parse!(p, "script_id")?;
        Ok(ScriptId(script_id))
    }
}

impl From<u160> for ScriptId {
    fn from(script_id: u160) -> Self {
        ScriptId(script_id)
    }
}

impl From<ScriptId> for u160 {
    fn from(script_id: ScriptId) -> Self {
        script_id.0
    }
}

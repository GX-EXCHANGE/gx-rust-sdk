use alloy::{
    primitives::B256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};

use crate::{eip712::Eip712, prelude::*, signature::agent::l1, Error};

pub(crate) fn sign_l1_action(
    wallet: &PrivateKeySigner,
    connection_id: B256,
    is_mainnet: bool,
) -> Result<Signature> {
    let source = if is_mainnet { "a" } else { "b" }.to_string();
    let payload = l1::Agent {
        source,
        connectionId: connection_id,
    };
    sign_typed_data(&payload, wallet)
}

pub(crate) fn sign_typed_data<T: Eip712>(
    payload: &T,
    wallet: &PrivateKeySigner,
) -> Result<Signature> {
    wallet
        .sign_hash_sync(&payload.eip712_signing_hash())
        .map_err(|e| Error::SignatureFailure(e.to_string()))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{UsdSend, Withdraw3};

    fn get_wallet() -> Result<PrivateKeySigner> {
        let priv_key = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e";
        priv_key
            .parse::<PrivateKeySigner>()
            .map_err(|e| Error::Wallet(e.to_string()))
    }

    #[test]
    fn test_sign_l1_action() -> Result<()> {
        let wallet = get_wallet()?;
        let connection_id =
            B256::from_str("0xde6c4037798a4434ca03cd05f00e3b803126221375cd1e7eaaaf041768be06eb")
                .map_err(|e| Error::GenericParse(e.to_string()))?;

        let expected_mainnet_sig = "0xfa8a41f6a3fa728206df80801a83bcbfbab08649cd34d9c0bfba7c7b2f99340f53a00226604567b98a1492803190d65a201d6805e5831b7044f17fd530aec7841c";
        assert_eq!(
            sign_l1_action(&wallet, connection_id, true)?.to_string(),
            expected_mainnet_sig
        );
        let expected_testnet_sig = "0x1713c0fc661b792a50e8ffdd59b637b1ed172d9a3aa4d801d9d88646710fb74b33959f4d075a7ccbec9f2374a6da21ffa4448d58d0413a0d335775f680a881431c";
        assert_eq!(
            sign_l1_action(&wallet, connection_id, false)?.to_string(),
            expected_testnet_sig
        );
        Ok(())
    }

    #[test]
    fn test_sign_usd_transfer_action() -> Result<()> {
        let wallet = get_wallet()?;

        let usd_send = UsdSend {
            signature_chain_id: 421614,
            gx_exchange_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        let expected_sig = "0x65094b50968f92118c65e40f87dfacf1707abbfd9bb040425f5dacca4c7143b177b0feeba6b7529ef6cf1fac15ebfb9e48d00b247e28061e954ec5c00bd7076b1b";
        assert_eq!(
            sign_typed_data(&usd_send, &wallet)?.to_string(),
            expected_sig
        );
        Ok(())
    }

    #[test]
    fn test_sign_withdraw_from_bridge_action() -> Result<()> {
        let wallet = get_wallet()?;

        let usd_send = Withdraw3 {
            signature_chain_id: 421614,
            gx_exchange_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        let expected_sig = "0x682430327651a24ee3d40e3bbc464f0bf33361e7342edb087d9ade1b8eb7fb1e14ee8526a65d9a0e5b2b997bf17aa81d184dfd536275ec49005df7434d9ac88e1b";
        assert_eq!(
            sign_typed_data(&usd_send, &wallet)?.to_string(),
            expected_sig
        );
        Ok(())
    }
}

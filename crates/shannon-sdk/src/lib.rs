use k256::ecdsa::{Signature, SigningKey, signature::Signer};
use shannon_protos::{
    prost::Message,
    tx::v1beta1::{AuthInfo, SignDoc, TxRaw},
};

#[derive(Debug, Clone, Copy)]
pub struct AccountNumber(pub u64);

pub struct ShannonClient<S> {
    pub signer: S,
    pub account_number: AccountNumber,
    pub auth_info: AuthInfo,
}

pub trait Wallet {
    fn sign_msg(pk: &SigningKey, bytes: &[u8]) -> Signature {
        pk.sign(bytes)
    }
    fn signing_key(&self) -> SigningKey;
}

pub trait Client {
    type Signer: Wallet;

    fn auth_info_mut(&mut self) -> &mut AuthInfo;
    fn auth_info(&self) -> AuthInfo;
    fn account_number(&self) -> AccountNumber;
    fn signer(&self) -> &Self::Signer;
    fn sign<M: Message>(&self, msg: &M) -> TxRaw {
        let signed: Signature = self.signer().signing_key().sign(
            &SignDoc {
                body_bytes: msg.encode_to_vec(),
                auth_info_bytes: self.auth_info().encode_to_vec(),
                chain_id: "pocket".to_string(),
                account_number: self.account_number().0,
            }
            .encode_to_vec(),
        );

        TxRaw {
            body_bytes: msg.encode_to_vec(),
            auth_info_bytes: self.auth_info().encode_to_vec(),
            signatures: vec![signed.to_vec()],
        }
    }
}

impl<S> Client for ShannonClient<S>
where
    S: Wallet,
{
    type Signer = S;

    fn auth_info(&self) -> AuthInfo {
        self.auth_info.clone()
    }

    fn account_number(&self) -> AccountNumber {
        self.account_number
    }

    fn signer(&self) -> &Self::Signer {
        &self.signer
    }

    fn auth_info_mut(&mut self) -> &mut AuthInfo {
        &mut self.auth_info
    }
}

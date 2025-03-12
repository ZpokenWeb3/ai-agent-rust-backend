use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use std::error::Error;
use tokio;

const CONTRACT_ABI_PATH: &str = "prize_contract.abi";
const BONDING_ABI_PATH: &str = "bonding_curve_contract.abi";

pub struct SMCDriver {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
    prize_contract: Contract<Provider<Http>>,
    bonding_contract: Contract<Provider<Http>>,
}

impl SMCDriver {
    pub async fn new(
        rpc_url: &str,
        contract_address: &str,
        bonding_contract_address: &str,
        private_key: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(1);
        let address = wallet.address();

        let prize_contract = Contract::from_json(
            provider.clone(),
            contract_address.parse::<Address>()?,
            std::fs::read_to_string(CONTRACT_ABI_PATH)?.as_bytes(),
        )?;

        let bonding_contract = Contract::from_json(
            provider.clone(),
            bonding_contract_address.parse::<Address>()?,
            std::fs::read_to_string(BONDING_ABI_PATH)?.as_bytes(),
        )?;

        Ok(Self {
            provider,
            wallet,
            prize_contract,
            bonding_contract,
        })
    }

    pub async fn get_prize_pool_balance(&self) -> Result<U256, Box<dyn Error>> {
        let balance = self.provider.get_balance(self.wallet.address(), None).await?;
        Ok(balance)
    }

    pub async fn transfer_prize(&self, user_address: &str) -> Result<TxHash, Box<dyn Error>> {
        let to: Address = user_address.parse()?;
        let balance = self.get_prize_pool_balance().await?;
        let gas_price = self.provider.get_gas_price().await?;
        let gas_limit = 21000.into();
        let transaction_fee = gas_price * gas_limit;

        let value_to_send = balance - transaction_fee;

        let tx = TransactionRequest::new()
            .to(to)
            .value(value_to_send)
            .from(self.wallet.address());

        let pending_tx = self.provider.send_transaction(tx, None).await?;
        let tx_hash = pending_tx.tx_hash();
        Ok(tx_hash)
    }
}

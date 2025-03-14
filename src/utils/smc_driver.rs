use ethers::prelude::*;
use ethers::abi::Abi;
use ethers::signers::{LocalWallet, Signer};
use ethers::contract::Contract;
use std::sync::Arc;
use std::error::Error;
use std::fs;

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
        // Initialize provider
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

        // Parse wallet
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(1u64);

        // Read and parse contract ABIs
        let prize_abi = serde_json::from_str::<Abi>(&fs::read_to_string(CONTRACT_ABI_PATH)?)?;
        let bonding_abi = serde_json::from_str::<Abi>(&fs::read_to_string(BONDING_ABI_PATH)?)?;

        // Create contract instances using `Contract::new`
        let prize_contract = Contract::new(
            contract_address.parse::<Address>()?,
            prize_abi,
            provider.clone(),
        );

        let bonding_contract = Contract::new(
            bonding_contract_address.parse::<Address>()?,
            bonding_abi,
            provider.clone(),
        );

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
        let gas_limit = U256::from(21000);
        let transaction_fee = gas_price * gas_limit;

        let value_to_send = balance.checked_sub(transaction_fee).ok_or("Insufficient balance")?;

        let tx = TransactionRequest::new()
            .to(to)
            .value(value_to_send)
            .from(self.wallet.address());

        let pending_tx = self.provider.send_transaction(tx, None).await?;
        let tx_hash = pending_tx.tx_hash();
        Ok(tx_hash)
    }
}

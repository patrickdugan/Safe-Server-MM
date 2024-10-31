use hex::FromHex;
use near_sdk::{env, ext_contract, require, Gas, NearToken, Promise, AccountId};
use serde::Deserialize;
use std::collections::HashSet;

const MPC_CONTRACT_ACCOUNT_ID: &str = "v1.signer-prod.testnet";
const GAS: Gas = Gas::from_tgas(250);
const ATTACHED_DEPOSIT: NearToken = NearToken::from_yoctonear(50000000000000000000000);
const COST: NearToken = NearToken::from_near(1);

#[derive(Deserialize)]
struct Psbt {
    // Define your structure based on the PSBT specification
    outputs: Vec<Output>,
}

#[derive(Deserialize)]
struct DecodedTransaction {
    txid: String,
    hash: String,
    size: u64,
    vsize: u64,
    weight: u64,
    version: i32,
    locktime: u32,
    vin: Vec<Input>,
    vout: Vec<Output>,
}


#[derive(Deserialize)]
struct Input {
    txid: String,
    vout: u32,
    scriptSig: ScriptSig,
    txinwitness: Vec<String>,
    sequence: u32,
}

#[derive(Deserialize)]
struct ScriptSig {
    asm: String,
    hex: String,
}

#[derive(Deserialize)]
struct Output {
    value: f64,
    n: u32,
    scriptPubKey: ScriptPubKey,
}

// Interface for cross contract call to MPC contract
#[ext_contract(mpc)]
trait MPC {
    fn sign(&self, request: SignRequest) -> Promise;
}

#[near(contract_state)]
pub struct Contract {
    whitelist: UnorderedSet<AccountId>,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            whitelist: UnorderedSet::new(b"w".to_vec()),
        }
    }

    // Function to add an address to the whitelist
    pub fn add_to_whitelist(&mut self, account_id: AccountId) {
        self.whitelist.insert(&account_id);
    }

    // Function to decode PSBT from hexadecimal
    fn decode_raw_transaction(raw_tx_hex: &str) -> Result<Transaction, String> {
        let bytes = hex::decode(raw_tx_hex).map_err(|e| e.to_string())?;
        let tx: Transaction = deserialize(&bytes).map_err(|e| e.to_string())?;
        Ok(tx)
    }

    // Function to validate outputs against the whitelist
    pub fn validate_outputs(&self, decoded_tx: &DecodedTransaction) -> bool {
        let whitelist: HashSet<String> = self.whitelist.iter().map(|addr| addr.to_string()).collect();

        for output in &decoded_tx.vout {
            for address in &output.scriptPubKey.addresses {
                if !whitelist.contains(address) {
                    return false; // Address not whitelisted
                }
            }
        }

        true // All addresses are valid
    }

    // Process the PSBT and validate outputs
    #[payable]
    pub fn process_transaction(&mut self, hex_psbt: String, path: String, key_version: u32) -> Promise {
        let psbt = self.decode_raw_transaction(&hex_psbt).expect("Failed to decode PSBT");

        require!(self.validate_outputs(&psbt), "Outputs are not valid according to whitelist");

        // Generate a payload for the signature request
        let payload: [u8; 32] = env::keccak256_array(&[0u8; 32]); // Replace with actual payload generation logic

        // Create the sign request
        let sign_request = SignRequest {
            payload,
            path,
            key_version,
        };

        // Check deposit requirement
        let deposit = env::attached_deposit();
        require!(deposit >= COST, "Insufficient deposit for signing");

        // Call the MPC contract to request a signature
        mpc::ext(MPC_CONTRACT_ACCOUNT_ID.parse().unwrap())
            .with_static_gas(GAS)
            .with_attached_deposit(ATTACHED_DEPOSIT)
            .sign(sign_request)
    }
}

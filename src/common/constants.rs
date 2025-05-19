pub static PROJECT_NAME: &str = "MEV_Bot_Solana";

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or(String::from(""))
}

#[derive(Debug, Clone)]
pub struct Env {
    pub block_engine_url: String,
    pub mainnet_rpc_url: String,
    pub rpc_url_tx: String,
    pub devnet_rpc_url: String,
    pub rpc_url: String,
    pub wss_rpc_url: String,
    pub geyser_url: String,
    pub geyser_access_token: String,
    pub simulator_url: String,
    pub ws_simulator_url: String,
    pub payer_keypair_path: String,
    pub database_name: String,

}

impl Env {
    pub fn new() -> Self {
        Env {
            block_engine_url: get_env("BLOCK_ENGINE_URL"),
            rpc_url: get_env("RPC_URL"),
            mainnet_rpc_url: get_env("MAINNET_RPC_URL"),
            rpc_url_tx: get_env("RPC_URL_TX"),
            devnet_rpc_url: get_env("DEVNET_RPC_URL"),
            wss_rpc_url: get_env("WSS_RPC_URL"),
            geyser_url: get_env("GEYSER_URL"),
            geyser_access_token: get_env("GEYSER_ACCESS_TOKEN"),
            simulator_url: get_env("SIMULATOR_URL"),
            ws_simulator_url: get_env("WS_SIMULATOR_URL"),
            payer_keypair_path: get_env("PAYER_KEYPAIR_PATH"),
            database_name: get_env("DATABASE_NAME")
        }
    }
}

pub static COINBASE: &str = "0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5"; // Flashbots Builder

pub static WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub static USDT: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
pub static USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

/*
Can figure out the balance slot of ERC-20 tokens using the:
EvmSimulator::get_balance_slot method

However, note that this does not work for all tokens.
Especially tokens that are using proxy patterns.
*/
pub static WETH_BALANCE_SLOT: i32 = 3;
pub static USDT_BALANCE_SLOT: i32 = 2;
pub static USDC_BALANCE_SLOT: i32 = 9;

pub static WETH_DECIMALS: u8 = 18;
pub static USDT_DECIMALS: u8 = 6;
pub static USDC_DECIMALS: u8 = 6;

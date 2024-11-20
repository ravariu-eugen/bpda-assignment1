#![allow(non_snake_case)]

mod proxy;

use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use proxy::CardProperties;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        // actions
        "deploy" => interact.deploy().await,
        "solve" => solve().await,
        "upgrade" => interact.upgrade().await,
        //"issueNft" => interact.issue_nft().await,
        //"createNftWithAttributes" => interact.create_nft_with_attributes().await,
        "getYourNftCardProperties" => _=interact.get_your_nft_card_properties().await,
        //"exchangeNft" => interact.exchange_nft().await,
        "swap" => execute_swap().await,
        // views
        "getTokenId" => interact.get_token_id().await,
        "getTokenData" => interact.get_token_data().await,
        "tokenId" => interact.token_id().await,
        "nftSupply" => _=interact.nft_supply().await,
        "cardsProperties" => interact.cards_properties().await,
        "studentsCards" => interact.students_cards().await,
        "studentsAddresses" => interact.student_address().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}


/// sets the smart contract address to the one used for completing the assignment
async fn setRealSCAddress() {
    println!("setRealSCAddress");
    setAddress("erd1qqqqqqqqqqqqqpgqrqz7r8yl5dav2z0fgnn302l2w7xynygruvaq76m26j").await;

}

/// sets the interactor contract address
async fn setAddress(address: &str) {
    let mut interact = ContractInteract::new().await;
    interact.state.set_address(Bech32Address::from_bech32_string(address.to_string()));
    
}
async fn solve() {


    // get the required card

    setRealSCAddress().await;
    let mut interact = ContractInteract::new().await;
    let card_properties = interact.get_your_nft_card_properties().await;

    // parse the card properties into integers
    let class = match card_properties.class {
        proxy::Class::Warrior => 0,
        proxy::Class::Mage => 1,
        proxy::Class::Rogue => 2,
        proxy::Class::Priest => 3,
        proxy::Class::Hunter => 4,
        proxy::Class::Warlock => 5,
        proxy::Class::Shaman => 6,
        proxy::Class::Druid => 7,
        proxy::Class::Paladin => 8,
        
    };

    let rarity = match card_properties.rarity {
        proxy::Rarity::Common => 0,
        proxy::Rarity::Rare => 1,
        proxy::Rarity::Epic => 2,
        proxy::Rarity::Legendary => 3,
    };

    let power = match card_properties.power {
        proxy::Power::Low => 0,
        proxy::Power::Medium => 1,
        proxy::Power::High => 2,
    };

    // create the properties hex string
    let hex_properties = format!("0x{:02x}{:02x}{:02x}", class, rarity, power);
    println!("Hex Properties: {}", hex_properties);


    // get the nft nonce
    
    let attributes =interact.nft_supply().await;
    let nonce = attributes.iter().position(|x| x == &hex_properties).unwrap() as u64 + 1;
    println!("Nonce: {}", nonce);


    // create required nft into my wallet

    getNft("ravariu.eugen", class, rarity, power).await;
    
}

/// execute the exchange with the token id taken from the multiversx explorer and the nonce obtained from the solve function
async fn execute_swap() {
    setRealSCAddress().await;
    let mut interact = ContractInteract::new().await;
    interact.exchange_nft("BPDAR-9be2d0".to_string(), 55).await;
}


async fn issueTestNFT() {
    let mut interact = ContractInteract::new().await;

    let token_display_name = "BPDARavariu";
    let token_ticker = "BPDAR";

    interact.issue_nft(50_000_000_000_000_000, token_display_name.to_string(), token_ticker.to_string()).await;
    println!("----------");
    println!("Issued NFT: {} {}", token_display_name, token_ticker);
    println!("----------");
}



async fn createUserNft(name : &str, class : u8, rarity : u8, power : u8){
    println!("Creating NFT: {} class={} rarity={} power={}", name, class, rarity, power);
    let mut interact = ContractInteract::new().await;
    interact.create_nft_with_attributes(name.to_string(), class, rarity, power).await;

}

/// obtains the necessary nft by deploying a modified version of the contract that can create the nft 
/// and send it back to my wallet;
async fn getNft(name : &str, class : u8, rarity : u8, power : u8){
    let mut interactor = ContractInteract::new().await;
    // deploy sc
    interactor.deploy().await;
    
    // issue nft

    issueTestNFT().await;
    
    // create the nft
    println!("Creating NFTs...");
    createUserNft(name, class, rarity, power).await;

    // send the nft
    interactor.send_nft_to_owner(1).await;
}



#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
            self.save_state();
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }


        pub fn save_state(&self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
        }
}
    
impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        self.save_state();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;


        let pem_path = Path::new("/mnt/e/Facultate/master/an1/BPDA/lab/intro/new_wallet.pem");
        let pem = std::fs::read_to_string(pem_path).expect("Failed to read PEM file");
        let wallet = Wallet::from_pem_file_contents(pem).expect("Invalid PEM file");
        let wallet_address = interactor.register_wallet(wallet.clone());
        
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/tema-1.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }

        
    }

    async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(90_000_000u64)
            .typed(proxy::Tema1Proxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::Tema1Proxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn issue_nft(&mut self, 
        egld: u128, 
        token_display_name: String,
        token_ticker: String) {
        let egld_amount = BigUint::<StaticApi>::from(egld);

        let token_display_name = ManagedBuffer::new_from_bytes(&token_display_name.as_bytes());
        let token_ticker = ManagedBuffer::new_from_bytes(&token_ticker.as_bytes());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::Tema1Proxy)
            .issue_nft(token_display_name, token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn send_nft_to_owner(&mut self, nonce: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::Tema1Proxy)
            .send_nft_to_owner(nonce)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn create_nft_with_attributes(&mut self, name: String, class: u8, rarity: u8, power: u8) {
        let name = ManagedBuffer::new_from_bytes(&name.as_bytes());


        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::Tema1Proxy)
            .create_nft_with_attributes(name, class, rarity, power)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn get_your_nft_card_properties(&mut self) -> CardProperties {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::Tema1Proxy)
            .get_your_nft_card_properties()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
        return response;
    }

    async fn exchange_nft(&mut self, token_id: String, nonce: u64) {
        let token_nonce = 1u64;
        let token_amount = BigUint::<StaticApi>::from(1u128);


        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::Tema1Proxy)
            .exchange_nft(nonce)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn get_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .get_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_token_data(&mut self) {
        let token_nonce = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .get_token_data(token_nonce)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn nft_supply(&mut self) -> Vec<String> {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .nft_supply()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;


        let card_properties_vec = result_value.into_vec();
        let attributes_vec : Vec<String> = card_properties_vec.iter().map(|x| x.attributes.hex_expr().to_string().clone()).collect();
        for card_properties in card_properties_vec {
            println!("Name: {}", card_properties.name);
            println!("Attributes: {}", card_properties.attributes.hex_expr());
            println!();
        }
        //println!("Result: {result_value:?}");

        return attributes_vec;
    }

    async fn cards_properties(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .cards_properties()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

            for x in result_value.into_vec() {
                println!("Result: {x:?}");
            }
    }

    async fn students_cards(&mut self) {
        let student_address = bech32::decode("erd1z58pjw7w9ggxcqk7srkwlnhugy8r8efkpvqvt29c7fn6ggq8ryrs54mq6y");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .students_cards(student_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn student_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::Tema1Proxy)
            .student_address()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        for x in result_value.into_vec() {
            println!("Result: {x:?}");
        }
    }

}

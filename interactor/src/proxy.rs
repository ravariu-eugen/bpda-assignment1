// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct Tema1Proxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for Tema1Proxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = Tema1ProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        Tema1ProxyMethods { wrapped_tx: tx }
    }
}

pub struct Tema1ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> Tema1ProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> Tema1ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> Tema1ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn issue_nft<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issueNft")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn create_nft_with_attributes<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<u8>,
        Arg2: ProxyArg<u8>,
        Arg3: ProxyArg<u8>,
    >(
        self,
        name: Arg0,
        class: Arg1,
        rarity: Arg2,
        power: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createNftWithAttributes")
            .argument(&name)
            .argument(&class)
            .argument(&rarity)
            .argument(&power)
            .original_result()
    }

    pub fn get_your_nft_card_properties(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, CardProperties> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getYourNftCardProperties")
            .original_result()
    }

    pub fn exchange_nft<
        Arg0: ProxyArg<u64>,
    >(
        self,
        nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("exchangeNft")
            .argument(&nonce)
            .original_result()
    }

    pub fn send_nft_to_caller<
        Arg0: ProxyArg<u64>,
    >(
        self,
        nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("sendNftToCaller")
            .argument(&nonce)
            .original_result()
    }

    pub fn get_token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTokenId")
            .original_result()
    }

    pub fn get_token_data<
        Arg0: ProxyArg<u64>,
    >(
        self,
        token_nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, EsdtTokenData<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTokenData")
            .argument(&token_nonce)
            .original_result()
    }

    pub fn token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("tokenId")
            .original_result()
    }

    pub fn nft_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, EsdtTokenData<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nftSupply")
            .original_result()
    }

    pub fn cards_properties(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, CardProperties>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("cardsProperties")
            .original_result()
    }

    pub fn students_cards<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        student_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, CardProperties> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("studentsCards")
            .argument(&student_address)
            .original_result()
    }

    pub fn student_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("studentsAddresses")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct CardProperties {
    pub class: Class,
    pub rarity: Rarity,
    pub power: Power,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum Class {
    Warrior,
    Mage,
    Rogue,
    Priest,
    Hunter,
    Warlock,
    Shaman,
    Druid,
    Paladin,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum Power {
    Low,
    Medium,
    High,
}

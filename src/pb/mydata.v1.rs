// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumSwaps {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TradeData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeData {
    #[prost(int64, tag="1")]
    pub block_time: i64,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub base_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub quote_mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub base_amount: u64,
    #[prost(uint64, tag="7")]
    pub quote_amount: u64,
    #[prost(int32, tag="8")]
    pub is_buy: i32,
    #[prost(string, tag="9")]
    pub sol_price: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)

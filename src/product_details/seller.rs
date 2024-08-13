#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm_parser", derive(tsify::Tsify), tsify(into_wasm_abi))]
#[derive(Debug)]
/// Information about the seller of a Product.
pub struct Seller {
    /// Name of the seller.
    pub name: String,
    #[cfg_attr(feature = "wasm_parser", tsify(optional))]
    /// Rating of the seller.
    pub rating: Option<f32>,
}

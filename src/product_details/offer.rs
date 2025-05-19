#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Information about the offers available on a Product.
#[cfg_attr(feature = "wasm_parser", derive(tsify::Tsify))]
#[derive(Debug)]
pub struct Offer {
    /// The category are typically like: `Bank Offer`,
    /// `Exchange Offer`, `No Cost EMI Available`,
    /// `Patner Offer` etc.
    #[cfg_attr(feature = "wasm_parser", tsify(optional))]
    pub category: Option<String>,
    /// The description of the offer.
    pub description: String,
}

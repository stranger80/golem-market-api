extern crate market_api;
extern crate uuid;

use market_api::*;
use std::collections::HashMap;
use uuid::Uuid;

use provider::market_impl::GolemMarketProviderFacade;
use provider::MarketProviderFacade;

#[test]
fn provider_api_subscribe_returns_success() {
    let offer = Offer {
        offer_id: Uuid::new_v4(),
        provider_id: NodeId {},
        constraints: String::new(),
        properties: vec![],

        exp_properties: HashMap::new(),
        imp_properties: vec![],
    };

    let facade: GolemMarketProviderFacade = GolemMarketProviderFacade::new();

    assert_eq!(facade.subscribe(offer), Ok(0));
}

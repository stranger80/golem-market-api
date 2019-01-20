
use uuid::Uuid;

use asnom::structures::{Tag};

use super::errors::{ PrepareError };
use super::super::{ NodeId, Demand, Offer };
use super::expression::{ Expression, build_expression };
use super::properties::{ PropertySet };
use super::ldap_parser;

// PreparedOffer
// Offer parsed and converted into optimized data structures.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedOffer<'a> {
    pub offer_id : Uuid,
    pub provider_id : NodeId,

    // Properties (values, aspects)
    pub properties : PropertySet<'a>,

    // Filter expression
    pub constraints : Expression<'a>,

    constraints_tag : Tag,

    pub offer : &'a Offer,
}

impl <'a> PreparedOffer<'a> {
    pub fn from(offer : &'a Offer) -> Result<PreparedOffer<'a>, PrepareError> {
        let mut result = PreparedOffer{
            offer_id : offer.offer_id.clone(),
            provider_id : offer.provider_id.clone(),
            properties : PropertySet::from_flat_props(&offer.properties),
            constraints : Expression::Empty, //offer_cons_expr,
            constraints_tag : match ldap_parser::parse(&offer.constraints) {
                     Ok(tags) => tags,
                     Err(error) => { return Err(PrepareError::new(&format!("Error parsing Offer constraints: {}", error)))}
                 },
            offer : offer,
        };

        result.constraints = match build_expression(&result.constraints_tag) {
             Ok(expr) => expr,
             Err(error) => { return Err(PrepareError::new(&format!("Error building Offer constraints expression: {}", error)))}
        };

        Ok(result)
        
    }
}

// PreparedDemand
// Offer parsed and converted into optimized data structures.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedDemand<'a> {
    pub demand_id : Uuid,
    pub requestor_id : NodeId,

    // Properties (values, aspects)
    pub properties : PropertySet<'a>,

    // Filter expression
    pub constraints : Expression<'a>,
}

impl <'a> PreparedDemand<'a> {
    // Process a Demand to obtain a PreparedDemand
    pub fn from(demand : &'a Demand) -> Result<PreparedDemand<'a>, PrepareError> {
        let demand_cons_tags = match ldap_parser::parse(&demand.constraints) {
            Ok(tags) => tags ,
            Err(error) => { return Err(PrepareError::new(&format!("Error parsing Demand constraints: {}", error)))}
        };
        
        let demand_cons_expr = match build_expression(&demand_cons_tags) {
            Ok(expr) => expr,
            Err(error) => { return Err(PrepareError::new(&format!("Error building Demand constraints expression: {}", error)))}
        };

        let result = PreparedDemand{
            demand_id : demand.demand_id.clone(),
            requestor_id : demand.requestor_id.clone(),
            properties : PropertySet::from_flat_props(&demand.properties),
            constraints : demand_cons_expr
        };

        Ok(result)
    }
}

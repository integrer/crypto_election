use crate::api;

use exonum::{
    blockchain::{self, Transaction, TransactionSet},
    crypto::Hash,
    helpers::fabric::{self, Context},
    messages::RawTransaction,
};

use core::{constant, schema::ElectionSchema, tx_behavior::ElectionTransactions};

use exonum::api::ServiceApiBuilder;
use exonum_merkledb::Snapshot;

pub trait ElectionDataService {
    fn create_election();

    fn start_election();

    fn stop_election();

    fn create_participant();

    fn vote();

    fn get_election_list();
}

trait UserLocationService {
    fn submit_location();
}

trait LocationDataService {
    fn create_region();

    fn set_region_name();

    fn set_coordinates();
}

#[derive(Default, Debug)]
pub struct Service;

impl blockchain::Service for Service {
    fn service_id(&self) -> u16 {
        constant::BLOCKCHAIN_SERVICE_ID
    }

    fn service_name(&self) -> &str {
        constant::BLOCKCHAIN_SERVICE_NAME
    }

    fn state_hash(&self, snapshot: &dyn Snapshot) -> Vec<Hash> {
        let schema = ElectionSchema::new(snapshot);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        ElectionTransactions::tx_from_raw(raw).map(Into::into)
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        api::PublicApi::wire(builder);
    }
}

#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        core::constant::BLOCKCHAIN_SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<dyn blockchain::Service> {
        Box::new(Service)
    }
}

use std::ops::Deref;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::loading::link::model::LinkModel;
use crate::dynamic::loading::node::routing::{RoutedModel, RoutingModel};
use crate::network::Network;

pub struct DynamicNetwork<RP, FD: FundamentalDiagram> {
	network: Network,
	routing_models: Vec<RoutingModel<RP>>,
	link_models: Vec<Box<dyn LinkModel<FD>>>,
}

impl<RP, FD: FundamentalDiagram> DynamicNetwork<RP, FD> {

}

impl<RP, FD: FundamentalDiagram> Deref for DynamicNetwork<RP, FD> {
	type Target = Network;
	
	fn deref(&self) -> &Self::Target {
		&self.network
	}
}

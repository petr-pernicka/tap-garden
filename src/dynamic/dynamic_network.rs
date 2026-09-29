use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::loading::link::model::LinkModel;
use crate::dynamic::loading::node::routing::{NonRoutedModel, RoutedModel, Zone};
use crate::network::Network;
use std::ops::Deref;
use crate::dynamic::loading::node::model::NodeModel;

pub struct DynamicNetwork<RP, FD: FundamentalDiagram> {
    network: Network,
    routing_models: Vec<RoutingModel<RP>>,
    node_models: Vec<Box<dyn NodeModel>>,
    link_models: Vec<Box<dyn LinkModel<FD>>>,
    fundamental_diagrams: Vec<FD>,
    num_nonrouted_models: usize,
}

pub enum RoutingModel<RP> {
    Routed(Box<dyn RoutedModel<RP>>),
    NonRouted(Box<dyn NonRoutedModel>),
}

impl<RP, FD: FundamentalDiagram> DynamicNetwork<RP, FD> {
    pub fn new(
        network: Network,
        zones: Vec<Zone>,
        nonrouted_models: Vec<Box<dyn NonRoutedModel>>,
        routed_models: Vec<Box<dyn RoutedModel<RP>>>,
        node_models: Vec<Box<dyn NodeModel>>,
        link_models: Vec<Box<dyn LinkModel<FD>>>,
        fundamental_diagrams: Vec<FD>,
    ) -> Self {
        debug_assert_eq!(zones.len(), network.zones().len());
        debug_assert_eq!(
            zones.len() + nonrouted_models.len() + routed_models.len(),
            network.nodes().len()
        );
        debug_assert_eq!(routed_models.len(), node_models.len());
        debug_assert_eq!(link_models.len(), network.links().len());

        let num_nonrouted_models = zones.len() + nonrouted_models.len();

        let mut routing_models: Vec<RoutingModel<RP>> =
            Vec::with_capacity(zones.len() + nonrouted_models.len() + routed_models.len());

        for zone in zones {
            routing_models.push(RoutingModel::NonRouted(Box::new(zone)));
        }

        for nonrouted_model in nonrouted_models {
            routing_models.push(RoutingModel::NonRouted(nonrouted_model));
        }

        for routed_model in routed_models {
            routing_models.push(RoutingModel::Routed(routed_model));
        }

        Self {
            network,
            routing_models,
            node_models,
            link_models,
            fundamental_diagrams,
            num_nonrouted_models,
        }
    }

    pub fn link_models(&self) -> &Vec<Box<dyn LinkModel<FD>>> {
        &self.link_models
    }

    pub fn fundamental_diagrams(&self) -> &Vec<FD> {
        &self.fundamental_diagrams
    }
    
    pub fn routing_models(&self) -> &Vec<RoutingModel<RP>> {
        &self.routing_models
    }
    
    pub fn node_models(&self) -> &Vec<Box<dyn NodeModel>> {
        &self.node_models
    }
}

impl<RP, FD: FundamentalDiagram> Deref for DynamicNetwork<RP, FD> {
    type Target = Network;

    fn deref(&self) -> &Self::Target {
        &self.network
    }
}

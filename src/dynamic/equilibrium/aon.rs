use std::marker::PhantomData;
use crate::dynamic::DynamicNetwork;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::routing::MixtureOutgoingFractions;

pub struct AON<RP, FD> {
    rp: PhantomData<RP>,
    fd: PhantomData<FD>,
}

impl<RP, FD: FundamentalDiagram> AON<RP, FD> {
    pub fn conmpute_routing_parameters(network: &DynamicNetwork<RP, FD>, time_steps: usize) -> MixtureOutgoingFractions {
        let mut mofs = MixtureOutgoingFractions::new(network, time_steps);

        

        mofs
    }
}
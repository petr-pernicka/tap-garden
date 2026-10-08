use crate::io::gmns::{DemandData, LinkData, NodeData};
use crate::network::{Network, NodeID, NodeIndex};
use crate::static_::ODM;
use std::collections::HashMap;

pub fn from_gmns_data(
    node_data: &[NodeData],
    link_data: &[LinkData],
    demand_data: &[DemandData],
) -> (Network, ODM, HashMap<NodeID, NodeIndex>) {
    let zone_id2node_id = node_data
        .iter()
        .filter(|nd| nd.zone_id.is_some())
        .map(|nd| (nd.zone_id.unwrap(), nd.node_id))
        .collect::<HashMap<_, _>>();
    let zone_ids = zone_id2node_id.keys().cloned().collect::<Vec<_>>();

    let links = link_data
        .iter()
        .map(|ld| (ld.from_node_id, ld.to_node_id))
        .collect::<Vec<_>>();

    let (network, id2index) = Network::new(&links, &zone_ids);

    let mut odm = ODM::new(zone_ids.len(), zone_ids.len(), 0.0);
    for dd in demand_data {
        let DemandData {
            o_zone_id: origin_id,
            d_zone_id: destination_id,
            volume: demand,
        } = dd;

        let origin_index = id2index[&zone_id2node_id[origin_id]];
        let destination_index = id2index[&zone_id2node_id[destination_id]];
        odm[(origin_index, destination_index)] = *demand;
    }

    (network, odm, id2index)
}

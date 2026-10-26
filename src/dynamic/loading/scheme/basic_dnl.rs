use crate::data::function::IntPiecewiseLinearFunction;
use crate::dynamic::dynamic_network::RoutingModel;
use crate::dynamic::loading::link::diagram::FundamentalDiagram;
use crate::dynamic::loading::outgoing_mixture_flow;
use crate::dynamic::loading::scheme::DnlScheme;
use crate::dynamic::{Cvn, DynamicNetwork};

pub struct BasicDnl;

impl<RP, FD: FundamentalDiagram> DnlScheme<RP, FD> for BasicDnl {
    fn run(
        &self,
        network: &DynamicNetwork<RP, FD>,
        time_steps: usize,
        step_size: f64,
        routing_parameters: &RP,
    ) -> Vec<(Cvn, Cvn)> {
        let mut cvns = Vec::with_capacity(network.links().len());
        for _ in 0..network.links().len() {
            let mut cinflow = Cvn::new(time_steps + 1);
            cinflow.push_value(0.);

            let mut coutflow = Cvn::new(time_steps + 1);
            coutflow.push_value(0.);

            cvns.push((cinflow, coutflow));
        }

        let mut mixture_inflows_per_link = Vec::with_capacity(network.links().len());
        for _ in 0..network.links().len() {
            let mixture_inflow = IntPiecewiseLinearFunction::new(time_steps);
            mixture_inflows_per_link.push(mixture_inflow);
        }

        for tn in 0..time_steps {
            debug_assert!(
                cvns.iter()
                    .all(|(cinflow, _)| cinflow.num_points() == tn + 1)
            );
            debug_assert!(
                cvns.iter()
                    .all(|(_, coutflow)| coutflow.num_points() == tn + 1)
            );
            debug_assert!(
                mixture_inflows_per_link
                    .iter()
                    .all(|mi| mi.num_points() == tn)
            );

            for node in network.nodes() {
                let mixture_sending_flows = network
                    .incoming_links(node)
                    .iter()
                    .map(|link| {
                        let i = link.index();
                        let model = &network.link_models()[i];
                        let fd = &network.fundamental_diagrams()[i];
                        let sending_flow = model
                            .sending_flow(tn, step_size, fd, &cvns[i].0, &cvns[i].1)
                            .0;

                        outgoing_mixture_flow(
                            tn,
                            sending_flow,
                            &cvns[i].0,
                            &cvns[i].1,
                            &mixture_inflows_per_link[i],
                        )
                    })
                    .collect::<Vec<_>>();

                let receiving_flows = network
                    .outgoing_links(node)
                    .iter()
                    .map(|link| link.index())
                    .map(|i| {
                        let model = &network.link_models()[i];
                        let fd = &network.fundamental_diagrams()[i];
                        model
                            .receiving_flow(tn, step_size, fd, &cvns[i].0, &cvns[i].1)
                            .0
                    })
                    .collect::<Vec<_>>();

                let (routing_model, node_model) = network.routing_model(node);
                let (mixture_inflows, mixture_outflows) = match routing_model {
                    RoutingModel::Routed(routed_model) => routed_model.mixture_inflows_outflows(
                        &mixture_sending_flows,
                        &receiving_flows,
                        tn,
                        routing_parameters,
                        node_model.unwrap(),
                    ),
                    RoutingModel::NonRouted(nonrouted_model) => nonrouted_model
                        .mixture_inflows_outflows(&mixture_sending_flows, &receiving_flows, tn),
                };

                debug_assert_eq!(mixture_inflows.len(), network.incoming_links(node).len());
                debug_assert_eq!(mixture_outflows.len(), network.outgoing_links(node).len());

                mixture_inflows
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, mixture_inflow)| {
                        let i = network.incoming_links(node)[i].index();

                        // The inflow of this node is outflow for the incoming link
                        let new_value = cvns[i].1[tn] + mixture_inflow.total_flow();
                        cvns[i].1.push_value(new_value);
                    });

                mixture_outflows
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, mixture_outflow)| {
                        let i = network.outgoing_links(node)[i].index();

                        // The outflow of this node is inflow for the outgoing link
                        let new_value = cvns[i].0[tn] + mixture_outflow.total_flow();
                        cvns[i].0.push_value(new_value);
                        mixture_inflows_per_link[i].push_value(mixture_outflow);
                    });
            }

            print!(
                "\r[Time {}] Total flow on network: {}      ",
                tn,
                cvns.iter()
                    .map(|(cinflow, coutflow)| cinflow[tn + 1] - coutflow[tn + 1])
                    .sum::<f64>()
            )
        }

        println!();

        cvns
    }
}

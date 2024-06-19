///////////////////////////////////////////////////////////////////////////////

use std::{
    f32::consts::{E, PI},
    path::PathBuf,
};

use cs_240_library::{
    algorithms::graphs::{dfs::depth_first_search, dijkstras::dijkstras_explore},
    data_structures::graphs::{
        weighted_graph::WeightedGraph, IDefiniteGraph, IGraph, IGraphEdgeWeightedMut, IGraphMut,
        IWeightedGraph,
    },
};
use egui::{
    ahash::{HashMap, HashMapExt},
    emath::TSTransform,
    Pos2, Vec2,
};

///////////////////////////////////////////////////////////////////////////////

fn typing(x: usize) -> f32 {
    f32::from(i16::try_from(x).unwrap())
}

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum Page {
    Blank,
    Project(Project),
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GraphDef {
    nodes: Vec<String>,
    edges: Vec<(String, String, i32)>,
}

//---------------------------------------------------------------------------//

impl GraphDef {
    fn new() -> Self {
        Default::default()
    }

    fn with_nodes(mut self, nodes: Vec<&str>) -> Self {
        self.nodes = nodes.into_iter().map(|x| x.to_owned()).collect();
        self
    }

    fn with_edges(mut self, edges: Vec<(&str, &str, i32)>) -> Self {
        self.edges = edges
            .into_iter()
            .map(|(x, y, z)| (x.to_owned(), y.to_owned(), z))
            .collect();
        self
    }

    fn directed_graph(&self) -> WeightedGraph<String, i32> {
        let mut graph = WeightedGraph::new();

        for node in &self.nodes {
            graph.insert_node(node.to_owned());
        }

        for (from, to, weight) in &self.edges {
            graph.insert_edge_weighted(from.to_owned(), to.to_owned(), weight.to_owned());
        }

        graph
    }

    fn undirected_graph(&self) -> WeightedGraph<String, i32> {
        let mut graph = WeightedGraph::new();

        for node in &self.nodes {
            graph.insert_node(node.to_owned());
        }

        for (from, to, weight) in &self.edges {
            graph.insert_edge_weighted(from.to_owned(), to.to_owned(), weight.to_owned());
            graph.insert_edge_weighted(to.to_owned(), from.to_owned(), weight.to_owned());
        }

        graph
    }

    fn build(self) -> (WeightedGraph<String, i32>, WeightedGraph<String, i32>) {
        (self.directed_graph(), self.undirected_graph())
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    pub path: Option<PathBuf>,
    pub view: PanZoom,
    pub graph: WeightedGraph<String, i32>,
    pub undirected_graph: WeightedGraph<String, i32>,
    pub text: String,
    pub graphic: HashMap<String, Pos2>,
}

//---------------------------------------------------------------------------//

impl Project {
    //---------------------------------------------------------------------------//

    pub fn new() -> Self {
        // let mut graph = WeightedGraph::new();

        // graph.insert_node("node 1".to_owned());
        // graph.insert_node("node 2".to_owned());
        // graph.insert_node("node 3".to_owned());
        // graph.insert_node("node 4".to_owned());

        // graph.insert_edge_weighted("node 1".to_owned(), "node 2".to_owned(), 1);
        // graph.insert_edge_weighted("node 1".to_owned(), "node 3".to_owned(), 2);
        // graph.insert_edge_weighted("node 2".to_owned(), "node 4".to_owned(), 1);

        let graphdef = GraphDef::new()
            .with_nodes(vec!["node 1", "node 2", "node 3", "node 4"])
            .with_edges(vec![
                ("node 1", "node 2", 1),
                ("node 1", "node 3", 2),
                ("node 2", "node 4", 1),
            ]);

        let text = serde_yaml::to_string(&graphdef).unwrap_or_default();

        let (graph, undirected_graph) = graphdef.build();

        let mut res = Self {
            path: None,
            view: Default::default(),
            graph,
            undirected_graph,
            text,
            graphic: Default::default(),
        };

        res.update_graphic();

        res
    }

    //---------------------------------------------------------------------------//

    pub fn update_graph(&mut self) {
        match serde_yaml::from_str::<GraphDef>(&self.text) {
            Ok(val) => {
                let (graph, undirected_graph) = val.build();
                if self.check(&graph) {
                    self.graph = graph;
                    self.undirected_graph = undirected_graph;
                    self.update_graphic();
                }
            }
            Err(_) => {}
        }
    }

    //---------------------------------------------------------------------------//

    pub fn check(&mut self, graph: &WeightedGraph<String, i32>) -> bool {
        for node in graph.get_all() {
            for adj in graph.get_adj(&node) {
                if !graph.contains(&adj) {
                    return false;
                }
            }
        }
        true
    }

    //---------------------------------------------------------------------------//

    fn update_graphic(&mut self) {
        self.graphic.clear();
        let (roots, _, cyclic) = depth_first_search(self.graph.clone());

        if cyclic {
            println!("Cyclic graph");
            let nodes = self.graph.get_all();
            let len = typing(nodes.len());

            for (i, node) in nodes.into_iter().enumerate() {
                self.graphic.insert(
                    node,
                    Pos2 {
                        x: (2.0 * PI * typing(i) / len).cos() * 50.0 * len + 300.0,
                        y: (2.0 * PI * typing(i) / len).sin() * 50.0 * len + 300.0,
                    },
                );
            }
            // self.simulate();
            // self.smacof();
            self._sgd();
        } else {
            println!("Acyclic graph");

            let mut x = 0.0;

            for root in roots {
                x = self.update_graphic_rec(root, x + 100.0, 0.0, 1);
            }
        }
    }

    //---------------------------------------------------------------------------//

    fn update_graphic_rec(&mut self, root: String, mut x: f32, mut y: f32, weight: i32) -> f32 {
        let mut adj = self.graph.get_adj_weighted(&root).into_iter();

        let offset = f32::from(weight as u16) * 100.0;
        println!("{}", offset);
        y += offset;

        self.graphic.insert(root.clone(), Pos2 { x, y });

        if let Some((first, weight)) = adj.next() {
            x = self.update_graphic_rec(first, x, y, weight);
        }

        for (node, weight) in adj {
            // self.graphic.insert(node.clone(), Pos2 { x, y });
            x = self.update_graphic_rec(node, x + 100.0, y, weight);
        }

        x
    }

    //---------------------------------------------------------------------------//

    pub fn _smacof(&mut self) {}

    //---------------------------------------------------------------------------//

    pub fn _sgd(&mut self) {
        // https://arxiv.org/pdf/2008.10376.pdf

        fn eta(t: f32) -> f32 {
            let rate = 2.0;
            let n = 10.0_f32.powi(12);

            n * E.powf(-rate * t)
        }

        fn mu(t: f32, ideal: f32) -> f32 {
            1.0_f32.min(ideal.powi(-2) * eta(t))
        }

        let max = 15;
        let nodes = self.graph.get_all();

        for t in 1..max + 1 {
            println!("--- iteration: {}/{}", t, max);

            for node in &nodes {
                println!("  - {}", node);

                let map = dijkstras_explore(&self.undirected_graph, node);

                for other in &nodes {
                    let pos = self.graphic.get(node).expect("pos not found").clone();
                    let other_pos = self
                        .graphic
                        .get(other)
                        .expect("other pos not found")
                        .clone();

                    if !pos.any_nan() {
                        println!("    - OK so far - {:?} >< {:?}", pos, other_pos);
                    }

                    let ideal_dist = f32::from(*map.get(other).unwrap_or(&8) as i16) * 100.0;

                    let diff = pos.to_vec2() - other_pos.to_vec2();
                    let real_dist = diff.length();

                    if !ideal_dist.is_nan() && !real_dist.is_nan() {
                        println!("    - OK so far - {:?} -- {:?}", ideal_dist, real_dist);
                    }

                    let sigma = if real_dist != 0.0 {
                        (real_dist - ideal_dist) / (real_dist) * (diff)
                    } else {
                        diff
                    };

                    if !sigma.any_nan() {
                        println!("    - OK so far - {:?}", sigma);
                    }

                    let mu = mu(f32::from(t as i16), ideal_dist);

                    if !sigma.any_nan() && !mu.is_nan() {
                        println!("    - OK so far >>> {:?}", mu / 2.0 * sigma);
                    }

                    self.graphic
                        .insert(node.to_string(), pos - mu / 2.0 * sigma);
                    self.graphic
                        .insert(other.to_string(), other_pos + mu / 2.0 * sigma);
                }
            }
        }

        println!("Finished!");
        println!("{:?}", self.graphic);
    }

    //---------------------------------------------------------------------------//

    pub fn _simulate(&mut self) {
        let mut velocities = HashMap::new();
        let mut accelerations = HashMap::new();

        for node in self.graph.get_all() {
            velocities.insert(node.clone(), Vec2 { x: 0.0, y: 0.0 });
            accelerations.insert(node, Vec2 { x: 0.0, y: 0.0 });
        }

        let steps = 300;
        for _ in 1..steps + 1 {
            // println!("--- {}/{}", step, steps);

            for node in self.graph.get_all() {
                let node_pos = self.graphic.get(&node).unwrap();
                // println!("> {} @ {:?}", node, node_pos);

                let adj_nodes = self.graph.get_adj_weighted(&node);

                for other in self.graph.get_all() {
                    if other != node {
                        let adj_pos = self.graphic.get(&other).unwrap();

                        let diff = adj_pos.to_vec2() - node_pos.to_vec2();
                        let dist = (diff).length();

                        let spring_char;
                        let relaxed_dist;
                        let node_mass;

                        let mut num_connections = 0;
                        let mut total_weight = 0;

                        for (_, weight) in adj_nodes.iter().filter(|(x, _)| *x == other) {
                            num_connections += 1;
                            total_weight += weight;
                        }

                        for (_, weight) in self
                            .graph
                            .get_adj_weighted(&other)
                            .iter()
                            .filter(|(x, _)| *x == node)
                        {
                            num_connections += 1;
                            total_weight += weight;
                        }

                        if num_connections > 0 {
                            let mean_weight = total_weight / num_connections;

                            spring_char = 0.7;
                            relaxed_dist = f32::from(mean_weight as u16) * 100.0;
                            node_mass = 100.0;
                        } else {
                            spring_char = 0.5;
                            relaxed_dist = 300.0;
                            node_mass = 500.0;
                        }

                        // if adj_nodes.iter().any(|(x, _)| *x == other)
                        //     || self.graph.get_adj(&other).contains(&node)
                        // {
                        //     spring_char = 0.7;
                        //     relaxed_dist = 100.0;
                        //     node_mass = 100.0;
                        // } else {
                        //     spring_char = 0.5;
                        //     relaxed_dist = 150.0;
                        //     node_mass = 500.0;
                        // }

                        let restoring_force = -spring_char * (dist - relaxed_dist);
                        let acc = restoring_force / node_mass;

                        // println!(
                        //     "> {} @ {:?} <<< {:?} | {:?} | {:?} >>> {} @ {:?}",
                        //     node, node_pos, dist, restoring_force, acc, other, adj_pos
                        // );

                        let acc_vec = acc * diff.normalized() + diff.normalized();

                        let node_acc = accelerations.get_mut(&node).unwrap();
                        *node_acc = *node_acc - acc_vec;

                        let other_acc = accelerations.get_mut(&other).unwrap();
                        *other_acc = *other_acc + acc_vec;
                    }
                }

                // for adj in self.graph.get_adj(&node) {
                //     let adj_pos = self.graphic.get(&adj).unwrap();

                //     let diff = adj_pos.to_vec2() - node_pos.to_vec2();
                //     let dist = (diff).length();

                //     let spring_char = 0.7;
                //     let relaxed_dist = 100.0;
                //     let node_mass = 50.0;

                //     let restoring_force = -spring_char * (dist - relaxed_dist);
                //     let acc = restoring_force / node_mass;

                //     let acc_vec = acc * diff.normalized() + diff.normalized();

                //     let node_acc = accelerations.get_mut(&node).unwrap();
                //     *node_acc = *node_acc - acc_vec;

                //     let adj_acc = accelerations.get_mut(&adj).unwrap();
                //     *adj_acc = *adj_acc + acc_vec;
                // }
            }

            for node in self.graph.get_all() {
                let node_pos = self.graphic.get_mut(&node).unwrap();
                let node_vel = velocities.get_mut(&node).unwrap();
                let node_acc = accelerations.get_mut(&node).unwrap();

                *node_vel += *node_acc;
                *node_vel -= node_vel.normalized();
                *node_pos += *node_vel;

                // println!(
                //     "> {} @ {:?} v {:?} a {:?}",
                //     node, node_pos, node_vel, node_acc
                // );
                *node_acc = Vec2 { x: 0.0, y: 0.0 };
            }
        }
    }

    //---------------------------------------------------------------------------//
}

//---------------------------------------------------------------------------//

impl Default for Project {
    fn default() -> Self {
        Self {
            path: None,
            view: Default::default(),
            graph: WeightedGraph::new(),
            undirected_graph: WeightedGraph::new(),
            text: Default::default(),
            graphic: Default::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PanZoom {
    pub transform: TSTransform,
    pub drag_value: f32,
}

///////////////////////////////////////////////////////////////////////////////

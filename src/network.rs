use std::{fs::File, io::Write, collections::HashMap};

use crate::{edge_options::{EdgeOptions}, node_options::NodeOptions};

#[test]
fn test_graph() {
    let mut net = Network::new();
    net.add_node(0, "Cool", None);
    net.add_node(1, "Cooler", Some(vec![NodeOptions::Hex("#ff0000"), NodeOptions::Shape("hexagon"), NodeOptions::Title("not slime boy")]));

    net.add_edge(0, 1, Some(vec![EdgeOptions::Hex("#ff0000"), EdgeOptions::Opacity(0.3)]), false);

    net.add_node(2, "Coolerer", None);
    net.add_node(3, "Coolererer", Some(vec![NodeOptions::Hex("#ff0000"), NodeOptions::Shape("hexagon")]));


    net.add_edge(2, 3, Some(vec![EdgeOptions::Hex("#ff0000"), EdgeOptions::Opacity(0.3)]), false);
    net.create("funny.html").unwrap();
}

/// A graphviz network
/// 
/// # Example
/// ```
/// use network_vis::{Network, node_options::NodeOptions, edge_options::EdgeOptions};
/// 
/// let mut net = Network::new();
/// net.add_node(0, "Example1", None);
/// net.add_node(1, "Example2", None);
/// 
/// net.add_edge(0, 1, None);
/// net.create("example.html").unwrap();
/// ```
pub struct Network<'a> {
    nodes: Vec<(u128, String, Option<Vec<NodeOptions<'a>>>)>,
    edges: Vec<(u128, u128, Option<Vec<EdgeOptions<'a>>>, bool)>,
}

impl<'a> Network<'a> {
    /// Create a new network
    /// 
    /// # Example
    /// ```
    /// use network_vis::Network;
    /// 
    /// let net = Network::new();
    /// ```
    pub fn new() -> Self {
        Network {
            nodes: vec![],
            edges: vec![],
        }
    }

    /// Add a node to the network
    /// 
    /// # Example
    /// ```
    /// use network_vis::Network;
    /// 
    /// let mut net = Network::new();
    /// 
    /// // Takes an id and a name and a vector of node options
    /// net.add_node(0, "Example1", None);
    /// net.add_node(1, "Example2", None);
    /// ```
    pub fn add_node(&mut self, id: u128, name: &str, node_options: Option<Vec<NodeOptions<'a>>>) {
        if self.nodes.iter().any(|(id_, _, _)| *id_ == id) {
            return;
        }

        self.nodes.push((id, name.to_string(), node_options));
    }

    /// Add an edge to the network
    /// 
    /// # Example
    /// ```
    /// use network_vis::Network;
    /// 
    /// let mut net = Network::new();
    /// net.add_node(0, "Example1", None);
    /// net.add_node(1, "Example2", None);
    /// 
    /// // Takes 2 ids and then a vector of edge options
    /// net.add_edge(0, 1, None);
    /// ```
    pub fn add_edge(&mut self, from: u128, to: u128, edge_options: Option<Vec<EdgeOptions<'a>>>, arrow: bool) {
        if self.edges.iter().any(|(from_, to_, _, _)| *from_ == from && *to_ == to) {
            return;
        }

        self.edges.push((from, to, edge_options, arrow));
    }

    /// Create the html graph file
    /// 
    /// # Example
    /// ```
    /// use network_vis::Network;
    /// 
    /// let mut net = Network::new();
    /// net.add_node(0, "Example1", None);
    /// net.add_node(1, "Example2", None);
    /// 
    /// net.add_edge(0, 1, None);
    /// 
    /// net.create("example.html").unwrap();
    /// ```
    pub fn create(self, name: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(name)?;

        // Create the javascript box
        file.write_all(r#"<html><body id="mynetwork"><script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>"#.as_bytes())?;
        file.write_all("\n".as_bytes())?;
        file.write_all(r#"<script type="text/javascript">"#.as_bytes())?;
        file.write_all("\n".as_bytes())?;


        // Nodes
        file.write_all(r#"var nodes = new vis.DataSet(["#.as_bytes())?;

        let mut write_str = String::new();
        for (id, name, node_options) in self.nodes {
            let mut node_options_value = String::new();
            let node_options_match = match node_options {
                Some(options) => options,
                None => vec![NodeOptions::Hex("#73ef81")]
            };
            for x in node_options_match {
                node_options_value.push_str(format!("{x}").as_str());
            }
            node_options_value.push('}');
            write_str.push_str(format!("{{ id: {id}, label: \"{name}\", {node_options_value},\n").as_str());
        }
        file.write_all(write_str.as_bytes())?;

        file.write_all(r#"]);"#.as_bytes())?;
        file.write_all("\n".as_bytes())?;

        // Edges
        file.write_all(r#"var edges = new vis.DataSet(["#.as_bytes())?;

        let mut write_str = String::new();
        for (from, to, edge_options, arrow) in self.edges {
            let mut edge_options_value = "{".to_string();
            let edge_options_match = match edge_options {
                Some(options) => options,
                None => vec![EdgeOptions::Name("black")]
            };
            for x in edge_options_match {
                edge_options_value.push_str(format!("{x}").as_str());
            }
            edge_options_value.push('}');
            let mut arrow_string = String::new();
            if arrow == true {
                arrow_string.push_str("arrows: { to: { enabled: true, type: \"arrow\" }}")
            }
            write_str.push_str(format!("{{ from: {from}, to: {to}, color: {edge_options_value}, {arrow_string} }},\n").as_str());
        }
        file.write_all(write_str.as_bytes())?;

        file.write_all(r#"]);"#.as_bytes())?;
        file.write_all("\n".as_bytes())?;

        // Create the network

        file.write_all(r#"var container = document.getElementById("mynetwork");
var data = {
    nodes: nodes,
    edges: edges
};
var options = { nodes: {shape: "dot" }};
var network = new vis.Network(container, data, options);
"#.as_bytes())?;

        file.write_all("\n".as_bytes())?;
        file.write(r#"</script></body></html>"#.as_bytes())?;

        Ok(())
    }
}
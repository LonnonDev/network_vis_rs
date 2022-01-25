use std::fmt::Display;

/// Node options
/// 
/// # Examples
/// ```
/// use network_vis::node_options::NodeOptions;
/// 
/// let node_options = NodeOptions::RGB(255, 0, 0);
/// ```
pub enum NodeOptions<'a> {
    Shape(&'a str),
    Hex(&'a str),
    Title(&'a str),
}

impl<'a> Display for NodeOptions<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NodeOptions::Shape(shape) => format!("shape: \"{shape}\","),
            NodeOptions::Hex(color) => format!("color: \"{color}\","),
            NodeOptions::Title(title) => format!("title: \"{title}\","),
        })
    }
}
type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(Debug)]
pub struct NodeData {
    id: String,
    first_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
pub struct EdgeData {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, id: &str) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            id: String::from(id),
            first_edge: None,
        });
        index
    }

    pub fn add_edge(&mut self, src: NodeIndex, dest: NodeIndex) {
        let index = self.edges.len();
        let data = &mut self.nodes[src];
        self.edges.push(EdgeData {
            target: dest,
            next_edge: data.first_edge,
        });
        data.first_edge = Some(index);
    }

    pub fn dump(&self) {
        println!("v: {:?}", self.nodes);
        println!("e: {:?}", self.edges);
    }
}

#[cfg(test)]
mod test {}

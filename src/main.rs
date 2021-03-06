mod graph;
use graph::Graph;

fn main() {
    let mut g = Graph::new();
    let n1 = g.add_node("first", 1);
    let n2 = g.add_node("second", 1);
    let n3 = g.add_node("third", 1);
    g.add_edge(n1, n2);
    g.add_edge(n2, n3);
    g.add_edge(n3, n1);
    g.dump();

    use std::fs::File;
    let mut f = File::create("output.dot").unwrap();
    dot::render(&g, &mut f).unwrap()
}
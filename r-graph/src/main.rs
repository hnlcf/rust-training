pub mod graph;

use graph::Graph;

fn main() {
    let mut g: Graph<String> = Graph::new();
    if g.is_empty() {
        println!("{}", g);
    }

    g.insert_value(3, "Changfeng Lou".to_string());
    g.connect_edge(0, 5, 2);
    println!("{}", g);
}

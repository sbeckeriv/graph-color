extern crate petgraph;
use  petgraph::EdgeType;
use petgraph::graph::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

trait GraphColoring{
    fn lazy_color(&self) -> HashMap<usize, usize>;
}

impl<N, E, Ty, Ix>  GraphColoring for Graph<N, E, Ty, Ix>
where
Ty: EdgeType,
Ix: IndexType+Hash,
{
    fn lazy_color(&self) -> HashMap<usize, usize>{
        let mut colors = HashSet::new();
        let mut colored: HashMap<usize, usize> = HashMap::new();
        for node in self.node_indices(){
            let mut used_colors = HashSet::new();
            for neighbor in self.neighbors_undirected(node){
                if let Some(color) = colored.get(&neighbor.index()) {
                    used_colors.insert(color.clone());
                }
            }
            match colors.difference(&used_colors).cloned().nth(0){
                None =>{
                    let next_color = colors.len() + 1;
                    colored.insert(node.index(), next_color.clone());
                    colors.insert(next_color.clone());
                }
                Some(next_color) =>{
                    colored.insert(node.index(),next_color.clone());
                }
            }
        }
        colored
    }
}

#[cfg(test)]
mod tests {

    use petgraph::graph::*;
    use std::ops::Index;
    use super::GraphColoring;
    #[test]
    fn it_works() {
        let mut deps = Graph::<&str, &str>::new();
        let pg = deps.add_node("petgraph");
        let fb = deps.add_node("fixedbitset");
        let qc = deps.add_node("quickcheck");
        let rand = deps.add_node("rand");
        let libc = deps.add_node("libc");
        deps.extend_with_edges(&[
                               (pg, fb), (pg, qc),
                               (qc, rand), (rand, libc), (qc, libc),
        ]);
        let colors = deps.lazy_color();
        println!("{:?}", colors);
        for (key, value) in colors{
            println!("Node: {:?} color: {}",
                     deps.index(NodeIndex::new(key)),
                     value);
        }

    }
}

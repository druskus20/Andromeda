use anyhow::Result;
use anyhow::{bail, Context};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl Id {
    pub fn new(id: usize) -> Id {
        Id(id)
    }
}

pub trait HasId {
    fn get_id(&self) -> Id;
    fn set_id(&mut self, id: Id);
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    value: T,
    id: Id,
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub adj_matrix: Vec<Vec<bool>>,
}


// TODO: Maybe custom trait iterate with Ids, where Ids are actually Node Ids not indexes
// TODO: is this needed?
impl<T> IntoIterator for Graph<T> {
    type Item = Node<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            adj_matrix: vec![],
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Node<T>> {
        self.nodes.iter()
    }

    /* (buffet's implementation bc "elkowar is dumb and stoopid and wrong") */
    fn get_new_id(&self) -> Id {
        let ids = self
            .iter()
            .map(|t| t.get_id().0)
            .sorted()
            .collect::<Vec<_>>();
        let mut next = 0;
        for id in ids.iter() {
            if *id != next {
                return Id(next);
            }
            next = *id + 1;
        }
        Id(next)
    }

    pub fn add_node(&mut self, value: T) -> Result<()> {
        let n_nodes = self.nodes.len();
        let node = Node::new(value, self.get_new_id());
        let id = node.id.0;

        // Insert nodes in "holes" in the graph
        if id < n_nodes {
            let row_length = self.adj_matrix[0].clone().len(); // Required because nodes != matrix size
            *self
                .adj_matrix
                .get_mut(id)
                .with_context(|| format!("Error indexing adj_matrix at row: {}", id))? =
                vec![false; row_length];
            for row in &mut self.adj_matrix {
                *row.get_mut(id)
                    .with_context(|| format!("Error indexing adj_matrix at col: {}", id))? = false;
            }
            self.nodes.insert(id, node);
        } else if id == n_nodes {
            // Insert nodes at the end of the graph
            // Can use n_nodes because nodes == matrix size
            self.adj_matrix.push(vec![false; n_nodes]);
            self.adj_matrix.iter_mut().for_each(|c| c.push(false));
            self.nodes.push(node);
        } else {
            bail!("The node id chain has been broken")
        }
        Ok(())
    }

    pub fn remove_node(&mut self, id: Id) -> Result<()> {
        let n_nodes = self.nodes.len();
        let id = id.0;
        if id > n_nodes {
            bail!(format!(
                "Error trying to remove node with id = {id} from graph with n_nodes = {n_nodes}"
            ))
        }
        self.adj_matrix.iter_mut().for_each(|c| c[id] = false);
        self.adj_matrix[id] = vec![false; n_nodes];
        self.nodes.remove(id);
        Ok(())
    }

    // TODO: Maybe optimize by not fully removing nodes, instead leaving blank space so that
    //  its not required to iterate over list, just list.get(id) -> Some()
    fn get_node(&self, id: Id) -> Option<&Node<T>> {
        self.iter().find(|n| n.get_id() == id)
    }

    fn get_connection(&mut self, id1: Id, id2: Id) -> Option<&mut bool> {
        self.adj_matrix.get_mut(id1.0)?.get_mut(id2.0)
    }

    fn change_adj_matrix_at(&mut self, id1: Id, id2: Id, value: bool) -> Result<()> {
        *self.get_connection(id1, id2).with_context(|| {
            format!("The connection {} -> {} could not be added", id1.0, id2.0)
        })? = value;
        Ok(())
    }

    fn is_node_in_graph(&self, id: Id) -> Result<()> {
        self.get_node(id)
            .with_context(|| format!("Node with id {} does not exist in the graph", id.0))?;
        Ok(())
    }

    fn change_connection(&mut self, id1: Id, id2: Id, value: bool) -> Result<()> {
        self.is_node_in_graph(id1)?;
        self.is_node_in_graph(id2)?;
        self.change_adj_matrix_at(id1, id2, value)?;
        self.change_adj_matrix_at(id2, id1, value)?;
        Ok(())
    }

    pub fn add_connection(&mut self, id1: Id, id2: Id) -> Result<()> {
        self.change_connection(id1, id2, true)
    }

    pub fn remove_connection(&mut self, id1: Id, id2: Id) -> Result<()> {
        self.change_connection(id1, id2, false)
    }
}

impl<T> Node<T> {
    pub fn new(value: T, id: Id) -> Self {
        Node { value, id }
    }
}

impl<T> HasId for Node<T> {
    fn get_id(&self) -> Id {
        self.id
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

#[cfg(test)]
mod test {
    use crate::graph::{Graph, Id};
    use crate::note::Note;

    #[test]
    fn test_insertion_and_removal() {
        let mut g = Graph::<Note>::new();

        g.add_node(Note::new()).unwrap();
        assert_eq!(g.nodes.len(), 1);
        assert_eq!(g.adj_matrix.len(), 1);
        assert_eq!(g.adj_matrix[0].len(), 1);

        g.add_node(Note::new()).unwrap();
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.adj_matrix.len(), 2);
        assert_eq!(g.adj_matrix[0].len(), 2);

        g.add_node(Note::new()).unwrap();
        assert_eq!(g.nodes.len(), 3);
        assert_eq!(g.adj_matrix.len(), 3);
        assert_eq!(g.adj_matrix[0].len(), 3);

        g.remove_node(Id::new(0)).unwrap();
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.adj_matrix.len(), 3);
        assert_eq!(g.adj_matrix[0].len(), 3);

        g.add_node(Note::new()).unwrap();
        assert_eq!(g.nodes.len(), 3);
        assert_eq!(g.adj_matrix.len(), 3);
        assert_eq!(g.adj_matrix[0].len(), 3);
    }

    #[test]
    fn test_add_remove_connection() {
        let mut g = Graph::<Note>::new();

        g.add_node(Note::new()).unwrap();
        g.add_node(Note::new()).unwrap();
        g.add_node(Note::new()).unwrap();

        let id0 = Id::new(0);
        let id1 = Id::new(1);

        //     pub fn add_connection(&mut self, id1: Id, id2: Id) -> Result<()> {
        let c = *g.get_connection(id0, id1).unwrap();
        assert_eq!(c, false);
        let c = *g.get_connection(id1, id0).unwrap();
        assert_eq!(c, false);
        g.add_connection(id0, id1).unwrap();
        let c = *g.get_connection(id0, id1).unwrap();
        assert_eq!(c, true);
        let c = *g.get_connection(id1, id0).unwrap();
        assert_eq!(c, true);
        g.remove_connection(id0, id1).unwrap();
        let c = *g.get_connection(id0, id1).unwrap();
        assert_eq!(c, false);
        let c = *g.get_connection(id1, id0).unwrap();
        assert_eq!(c, false);

        g.add_connection(id0, id1).unwrap();
        g.remove_connection(id1, id0).unwrap();
        let c = *g.get_connection(id0, id1).unwrap();
        assert_eq!(c, false);
        let c = *g.get_connection(id1, id0).unwrap();
        assert_eq!(c, false);
    }
}



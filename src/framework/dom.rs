use std::collections::HashMap;

use crate::widgets::WidgetDesc;

pub struct Dom {
    elements: HashMap<NodeId, Node>,
    root: HashMap<Name, NodeId>,
    last_id: NodeId,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            root: HashMap::new(),
            last_id: NodeId(0),
        }
    }
    pub fn add_node(&mut self, name: &str, path: &str, node: NodeData) -> NodeId {
        if "/" == path {
            match self.elements.insert(
                self.last_id,
                Node {
                    parent: None,
                    children: HashMap::new(),
                    data: node,
                },
            ) {
                Some(a) => {
                    log::warn!("Insert node that is ")
                }
                None => {}
            }
            self.root.insert(Name(name.to_string()), self.last_id);
            let id = self.last_id;
            self.last_id.0 += 1;
            return id;
        }
        let node_id = match self.get(path) {
            Some(id) => id,
            None => {
                log::error!("Path in add node invalid. Inserting to root");
                self.add_node(name, "/", node)
            }
        };
        todo!()
    }
    pub fn get(&self, path: &str) -> Option<NodeId> {
        let path: Vec<&str> = path.strip_prefix('/')?.split("/").collect();
        let node_id = self.root.get(&Name(path.get(0)?.to_string()))?;
        let node = self.get_node_from_id(node_id)?;

        node.get_from_children(self, path, 1, node_id)
    }
    pub fn get_node_from_id(&self, id: &NodeId) -> Option<&Node> {
        self.elements.get(id)
    }
}

pub struct Node {
    data: NodeData,
    parent: Option<NodeId>,
    children: HashMap<Name, NodeId>,
}
impl Node {
    fn get_from_children(
        &self,
        dom: &Dom,
        path: Vec<&str>,
        me: usize,
        prev_id: &NodeId,
    ) -> Option<NodeId> {
        if me == path.len() - 1 {
            return Some(prev_id.clone());
        }
        let node_id = self.children.get(&Name(path.get(me)?.to_string()))?;
        let node = dom.get_node_from_id(node_id)?;
        node.get_from_children(dom, path, me + 1, node_id)
    }
}

pub struct NodeData {
    pub widget: WidgetDesc,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Name(String);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct NodeId(u32);

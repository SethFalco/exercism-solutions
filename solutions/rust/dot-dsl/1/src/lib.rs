/// Adds methods to set and get `attrs` with the type
macro_rules! with_attrs {
    () => {
        pub fn with_attrs<'a>(&'a mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key, value);
            }

            self.clone()
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).copied()
        }
    };
}

pub mod graph {
    use std::collections::HashMap;
    use graph_items::{edge::Edge, node::Node};

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub attrs: HashMap<&'static str, &'static str>
            }

            impl Edge {
                pub fn new(body: &'static str, body2: &'static str) -> Self {
                    Self {
                        attrs: HashMap::from([(body, body2)])
                    }
                }

                with_attrs!();
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: &'static str,
                pub attrs: HashMap<&'static str, &'static str>
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Self {
                        name,
                        attrs: HashMap::new()
                    }
                }

                with_attrs!();
            }
        }
    }

    #[derive(Clone)]
    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes<'a>(&'a mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().map(|x| x.to_owned()));
            self.clone()
        }

        pub fn with_edges<'a>(&'a mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().map(|x| x.to_owned()));
            self.clone()
        }

        pub fn with_attrs<'a>(&'a mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();
            self.clone()
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            Some(self.nodes.iter().find(|node| node.name == name)?.to_owned())
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            Some(&self.attrs.get(key)?)
        }
    }
}

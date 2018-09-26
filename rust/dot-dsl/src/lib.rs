pub mod graph {
    pub struct Graph{
        nodes: Vec<Node>,
    }

    impl Graph {
        pub fn new() -> Self {
            unimplemented!("Construct a new Graph struct.");
        }
    }

    pub struct Node {
        value: String,
        attrs: Vec<String>,
    }
    
    impl Node {
        pub fn new(value: &str) -> Self {
            Node {
                value: value.to_string(),
                attrs: vec![]
            }
        }
    }
}

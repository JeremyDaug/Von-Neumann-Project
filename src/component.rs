pub struct Component {
    /// The unique id for the component. No relation to other Ids.
    pub id: usize,
    /// The name of the component.
    pub name: String,
    /// The goods which go into making this component.
    pub parts: HashMap<usize, f64>,
    
    pub consumes: HashMap<usize, f64>,
}
use std::collections::HashMap;

/// # Recipe
/// 
/// A transformation of one set of materials into another set of 
/// materials.
pub struct Recipe {
    /// The Recipe id.
    pub id: usize,
    /// The name of the recipe, may drop.
    pub name: String,
    /// The type of recipe it is, and what kind of components can do it.
    pub recipe_type: RecipeType,
    // TODO add token image here?

    // Recipe Parts
    /// What the recipe consumes in the process.
    pub inputs: HashMap<usize, f64>,
    /// What the recipe outputs in the processes.
    pub outputs: HashMap<usize, f64>,

    /// The Base energy cost for the process.
    pub energy_cost: f64,
    /// The base time cost for the process.
    pub time_cost: f64,
}

/// # Recipe Type
/// 
/// The kinds of recipies that exist.
/// 
/// Defines what components can do which.
pub enum RecipeType {
    /// Refining, typically for turning raw resources into another resource.
    Refining,
    /// Oil Refining, used for processing organic material into other oil based products.
    OilRefining,
    /// Fabrication, taking goods and reforming it into new shapes for later use.
    /// Predominantly used for simple components.
    Fabricating,
    /// Assembly, takes constituent parts and puts them together to create a 
    /// new item. Most often used for final products, but also used for more
    /// complex components.
    Assembly,
}
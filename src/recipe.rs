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
}

/// # Recipe Type
/// 
/// The kinds of recipies that exist.
/// 
/// Defines what components can do which.
pub enum RecipeType {
    /// Refining, typically for turning raw resources into another resource.
    Refining,
    Fabricating,
    Assembly,
}
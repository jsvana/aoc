use std::collections::{BTreeSet, BTreeMap};

use anyhow::{format_err, Result};
use maplit::btreeset;

/*
Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8
Butterscotch: capacity -1, durability 0, flavor 5, texture 0, calories 6
Sugar: capacity 0, durability 0, flavor -2, texture 2, calories 1
 */

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum IngredientName {
    Frosting,
    Candy,
    Butterscotch,
    Sugar,
    /* TEST INGREDIENTS
    Butterscotch,
    Cinnamon,
    */
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    //calories: i32,
}

impl Ingredient {
    fn from_name(name: &IngredientName) -> Self {
        match name {
                IngredientName::Frosting => {
                    Self {
                        capacity: 4,
                        durability: -2,
                        flavor: 0,
                        texture: 0,
                        //calories: 5,
                    }
                }
                IngredientName::Candy => {
                    Self {
                        capacity: 0,
                        durability: 5,
                        flavor: -1,
                        texture: 0,
                        //calories: 8,
                    }
                }
                IngredientName::Butterscotch => {
                    Self {
                        capacity: -1,
                        durability: 0,
                        flavor: 5,
                        texture: 0,
                        //calories: 6,
                    }
                }
                IngredientName::Sugar => {
                    Self {
                        capacity: 0,
                        durability: 0,
                        flavor: -2,
                        texture: 2,
                        //calories: 1,
                    }
                }
                /* TEST INGREDIENTS
            IngredientName::Butterscotch => Self {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                //calories: 8,
            },
            IngredientName::Cinnamon => Self {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                //calories: 3,
            },
            */
        }
    }

    fn score_count(&self, count: i32) -> IngredientScore {
        IngredientScore {
            count,

            capacity: self.capacity * count,
            durability: self.durability * count,
            flavor: self.flavor * count,
            texture: self.texture * count,
            //calories: self.calories * count,
        }
    }
}

#[derive(Debug)]
struct IngredientScore {
    count: i32,

    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    //calories: i32,
}

impl IngredientScore {
    fn zero() -> Self {
        Self {
            count: 0,
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            //calories: 0,
        }
    }

    fn merge(&self, other: Self) -> Self {
        Self {
            count: self.count + other.count,

            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            //calories: self.calories + other.calories,
        }
    }

    fn multiply(&self) -> i32 {
        std::cmp::max(0, self.capacity)
            * std::cmp::max(0, self.durability)
            * std::cmp::max(0, self.flavor)
            * std::cmp::max(0, self.texture)
            // * std::cmp::max(0, self.calories)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Recipe {
    ingredients: BTreeMap<IngredientName, i32>,
}

impl Recipe {
    fn zero() -> Self {
        Self { ingredients: BTreeMap::new() }
    }

    fn ingredient_count(&self) -> usize {
        let mut count: usize = 0;

        for ingredient_count in self.ingredients.values() {
            count += *ingredient_count as usize;
        }

        count
    }

    fn add_ingredient(&self, ingredient_name: IngredientName) -> Self {
        let mut ingredients = self.ingredients.clone();
        *ingredients.entry(ingredient_name).or_insert(0) += 1;

        Self {
            ingredients,
        }
    }

    fn score(&self) -> i32 {
        let mut score = IngredientScore::zero();

        for (name, count) in self.ingredients.iter() {
            score = score.merge(Ingredient::from_name(name).score_count(*count));
        }

        score.multiply()
    }
}

fn populate_recipes(base_recipe: Recipe, required_size: usize) -> BTreeSet<Recipe> {
    let missing_count = required_size - base_recipe.ingredient_count();
    if missing_count == 0 {
        return btreeset! { base_recipe };
    }

    if missing_count == 50 {
        println!("50 left");
    }

    let mut all_recipes = BTreeSet::new();

    // Add one of each then recurse
    all_recipes.append(&mut populate_recipes(base_recipe.add_ingredient(IngredientName::Frosting), required_size));
    all_recipes.append(&mut populate_recipes(base_recipe.add_ingredient(IngredientName::Candy), required_size));
    all_recipes.append(&mut populate_recipes(base_recipe.add_ingredient(IngredientName::Butterscotch), required_size));
    all_recipes.append(&mut populate_recipes(base_recipe.add_ingredient(IngredientName::Sugar), required_size));

    all_recipes
}

fn main() -> Result<()> {
    let mut max_score = 0;

    let recipes = populate_recipes(Recipe::zero(), 100);

    println!("Generated {} recipes", recipes.len());

    for recipe in recipes.into_iter() {
        max_score = std::cmp::max(max_score, recipe.score());
    }

    println!("{}", max_score);

    Ok(())
}

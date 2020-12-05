use std::collections::BTreeMap;

use anyhow::{format_err, Result};

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

#[derive(Debug)]
struct Recipe {
    ingredients: BTreeMap<IngredientName, i32>,
}

impl Recipe {
    fn zero() -> Self {
        Self { ingredients: BTreeMap::new() }
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

fn max_for_next_addition(base_recipe: &Recipe) -> Result<Recipe> {
    println!("Starting with {}: {:?}", base_recipe.score(), base_recipe.ingredients);
    let mut max_recipe: Option<Recipe> = None;

    let add_frosting = base_recipe.add_ingredient(IngredientName::Frosting);
    match &max_recipe {
        Some(recipe) => {
            if add_frosting.score() > recipe.score() {
                max_recipe = Some(add_frosting);
            }
        }
        None => {
            max_recipe = Some(add_frosting);
        }
    }

    let add_candy = base_recipe.add_ingredient(IngredientName::Candy);
    match &max_recipe {
        Some(recipe) => {
            if add_candy.score() > recipe.score() {
                max_recipe = Some(add_candy);
            }
        }
        None => {
            max_recipe = Some(add_candy);
        }
    }

    let add_butterscotch = base_recipe.add_ingredient(IngredientName::Butterscotch);
    match &max_recipe {
        Some(recipe) => {
            if add_butterscotch.score() > recipe.score() {
                max_recipe = Some(add_butterscotch);
            }
        }
        None => {
            max_recipe = Some(add_butterscotch);
        }
    }

    let add_sugar = base_recipe.add_ingredient(IngredientName::Sugar);
    match &max_recipe {
        Some(recipe) => {
            if add_sugar.score() > recipe.score() {
                max_recipe = Some(add_sugar);
            }
        }
        None => {
            max_recipe = Some(add_sugar);
        }
    }

    println!("Ending with {}: {:?}", max_recipe.as_ref().unwrap().score(), max_recipe.as_ref().unwrap().ingredients);

    max_recipe.ok_or_else(|| format_err!("no max found"))
}

fn calculate(ingredient_count: usize) -> Result<i32> {
    let mut max_recipe = Recipe::zero();

    for _ in 0..ingredient_count {
        max_recipe = max_for_next_addition(&max_recipe)?;
    }

    Ok(max_recipe.score())
}

fn main() -> Result<()> {
    println!("{}", calculate(100)?);

    Ok(())
}

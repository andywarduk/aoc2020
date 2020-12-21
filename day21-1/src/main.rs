use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct Ingredient {
    lang: String,
    english: Option<String>
}

#[derive(Default)]
struct Allergen {
    lang: Option<String>,
    foods: Vec<usize>
}

type Ingredients = Vec<Ingredient>;
type Foods = Vec<Ingredients>;
type AllergenMap = HashMap<String, Allergen>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut foods, mut allergen_map) = load_ingredients()?;

    translate_allergens(&mut foods, &mut allergen_map);

    let non_allergens = count_non_allergens(&foods);

    println!("{} non-allergen ingredients", non_allergens);

    Ok(())
}

fn load_ingredients() -> Result<(Foods, AllergenMap), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input21.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut foods = Foods::new();
    let mut allergens = AllergenMap::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let terms: Vec<String> = line.split("(contains ").map(|s| s.to_string()).collect();

        // Build food
        let ingredients = terms[0].split_whitespace().map(|s| {
            Ingredient {
                lang: s.to_string(),
                english: None
            }
        }).collect();

        // Add allergens to allergen map
        for a in terms[1].split(")").next().unwrap().split(",") {
            let allergen_str = a.trim().to_string();

            if let Some(allergen) = allergens.get_mut(&allergen_str) {
                allergen.foods.push(foods.len())
            } else {
                let mut allergen: Allergen = Default::default();
                allergen.foods.push(foods.len());
                allergens.insert(allergen_str, allergen);
            }
        }

        foods.push(ingredients);
    }

    Ok((foods, allergens))
}

fn translate_allergens(foods: &mut Foods, allergens: &mut AllergenMap) {
    loop {
        match translate_allergen_attempt(foods, allergens) {
            None => {}
            Some(n) => {
                if n == 0 {
                    break
                } else {
                    panic!("Couldn't translate {} allrgens", n)
                }
            }
        }
    }
}

fn translate_allergen_attempt(foods: &mut Foods, allergens: &mut AllergenMap) -> Option<usize> {
    let mut unknown = 0;

    for (allergen_eng, allergen) in allergens {
        if allergen.lang == None {
            if let Some(translation) = translate_allergen(foods, allergen_eng, &allergen.foods) {
                println!("Translation for {} is {}", allergen_eng, translation);
                allergen.lang = Some(translation.to_string());
                apply_translation(foods, allergen_eng, &translation);
                return None
            } else {
                unknown += 1
            }
        }
    }

    Some(unknown)
}

fn translate_allergen(foods: &Foods, allergen_eng: &String, allergen_vec: &Vec<usize>) -> Option<String> {
    let mut allergen_iter = allergen_vec.iter();

    // Build list of possible ingredients from the first food
    let mut translations: Vec<&String> = foods[*allergen_iter.next().unwrap()].iter().filter_map(|i| {
        if i.english == None {
            Some(&i.lang)
        } else {
            None
        }
    }).collect();

    // Process the other foods
    for &food_elem in allergen_iter {
        let mut valid_trans = Vec::new();

        for trans_word in translations {
            for ingredient in &foods[food_elem] {
                if ingredient.english == None && ingredient.lang == *trans_word {
                    valid_trans.push(trans_word);
                    break
                }
            }
        }

        translations = valid_trans;
    }

    if translations.len() == 1 {
        Some(translations[0].to_string())
    } else {
        println!("Possible translations for {}: {:?}", allergen_eng, translations);
        None
    }
}

fn apply_translation(foods: &mut Foods, eng: &String, lang: &String) {
    for food in foods {
        for ingredient in food {
            if ingredient.lang == *lang {
                ingredient.english = Some(eng.to_string())
            }
        }
    }
}

fn count_non_allergens(foods: &Foods) -> usize {
    foods.iter().map(|food| {
        food.iter().filter(|i| i.english == None).count()
    }).sum()
}

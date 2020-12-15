use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("30,000,000th number is {}", calculate(vec![0,20,7,16,1,18,15], 30_000_000));

    Ok(())
}

fn calculate(numbers: Vec<u32>, iters: u32) -> u32 {
    let mut last_ent: HashMap<u32, usize> = HashMap::new();

    for i in 0..numbers.len() {
        last_ent.insert(numbers[i], i);
    }

    let mut last = numbers[numbers.len() - 1];

    for i in numbers.len()..iters as usize {
        let next = match last_ent.get(&last) {
            None => 0,
            Some(elem) => (i - *elem - 1) as u32
        };

        match last_ent.get_mut(&last) {
            None => {
                last_ent.insert(last, i - 1);
                ()
            },
            Some(ent) => *ent = i - 1
        }

        last = next;
    }

    last
}

#[test]
fn test_calculate() {
    assert!(calculate(vec![0,3,6], 10) == 0, "0,3,6 for 10 should be 0");
    assert!(calculate(vec![0,3,6], 2020) == 436, "0,3,6 for 2020 should be 436");
    assert!(calculate(vec![0,20,7,16,1,18,15], 2020) == 1025, "0,20,7,16,1,18,15 for 2020 should be 1025");
}

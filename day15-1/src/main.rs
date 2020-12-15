fn main() -> Result<(), Box<dyn std::error::Error>> {
    // calculate(&mut vec![0,3,6], 10);
    // calculate(&mut vec![0,3,6], 2020);
    calculate(&mut vec![0,20,7,16,1,18,15], 2020);

    Ok(())
}

fn calculate(numbers: &mut Vec<u32>, iters: u32) {
    for i in numbers.len()..iters as usize {
        let last = numbers[i - 1];

        let mut found: isize = -1;

        for j in (0..i - 1).rev() {
            if numbers[j] == last {
                found = j as isize;
                break
            }
        }

        let next: u32;

        if found != -1 {
            next = (i - found as usize - 1) as u32;
        } else {
            next  = 0;
        }

        numbers.push(next);
    }

    println!("Number {} is {}", iters, numbers[iters as usize - 1])
}
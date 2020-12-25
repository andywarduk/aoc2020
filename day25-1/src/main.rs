type NumType = u64;

const MODULUS: NumType = 20201227;

fn main() {
    let pk1: NumType = 10604480;
    let pk2: NumType = 4126658;

    let subject: NumType = 7;

    let loop1 = calc_loop(pk1, subject);
    println!("Loop 1 is {}", loop1);

    let loop2 = calc_loop(pk2, subject);
    println!("Loop 2 is {}", loop2);

    let ek1: NumType = calc_ek(pk1, loop2);
    println!("Encryption key is {}", ek1);

    let ek2: NumType = calc_ek(pk2, loop1);
    println!("Crosscheck encryption key is {}", ek2);
}

fn calc_loop(pk: NumType, subject: NumType) -> NumType {
    let mut acc: NumType = 1;
    let mut loop_cnt: NumType = 0;

    loop {
        acc *= subject;
        acc = acc % MODULUS;
        loop_cnt += 1;

        if acc == pk {
            break
        }
    }

    loop_cnt
}

fn calc_ek(pk: NumType, loop_cnt: NumType) -> NumType {
    let mut acc: NumType = 1;

    for _ in 0..loop_cnt {
        acc *= pk;
        acc = acc % MODULUS;
    }

    acc
}

#[test]
fn test_calc_loop() {
    let pk1: NumType = 5764801;
    let pk2: NumType = 17807724;

    let loop1 = calc_loop(pk1, 7);
    assert!(loop1 == 8, "Loop 1 size incorrect");

    let loop2 = calc_loop(pk2, 7);
    assert!(loop2 == 11, "Loop 2 size incorrect");

    let ek1 = calc_ek(pk1, loop2);
    assert!(ek1 == 14897079, "Encryption key 1 incorrect");

    let ek2 = calc_ek(pk2, loop1);
    assert!(ek2 == 14897079, "Encryption key 2 incorrect");
}

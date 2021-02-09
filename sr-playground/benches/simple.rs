

#[macro_use]
extern crate bencher;

use bencher::Bencher;

use minisketch_rs::{Minisketch, MinisketchError};

fn create_sketch(elements: impl IntoIterator<Item = u64>) -> Result<Minisketch, MinisketchError> {
    let mut sketch = Minisketch::try_new(64, 0, 400)?;
    for item in elements.into_iter() {
        sketch.add(item);
    }

    Ok(sketch)
}

fn create_sketch_alice() -> Result<Minisketch, MinisketchError> {
    //let set = 3_000_000..4_000_000;
    let set = 1000000_3_00000u64..1000000_301000u64;
    //println!(
        //"Alice's set: {:?}",
        //set.clone().into_iter().collect::<Vec<_>>()
    //);

    Ok(create_sketch(set)?)
}

fn create_sketch_bob() -> Result<Minisketch, MinisketchError> {
    // let set = 3_001_000..4_001_000;
    let set = 1000000_3_00200..1000000_3_01200u64;
    //println!(
        //"Bob's set: {:?}",
        //set.clone().into_iter().collect::<Vec<_>>()
    //);

    Ok(create_sketch(set)?)
}

fn reconcile_with_bob(msg_alice: &[u8]) -> Result<(), MinisketchError> {
    let mut sketch_bob = create_sketch_bob()?;

    // Restore Alice's sketch (not set!) from serialized message
    let mut sketch_alice = Minisketch::try_new(64, 0, 400)?;
    sketch_alice.deserialize(&msg_alice);

    // Reconcile sets by merging sketches
    sketch_bob.merge(&sketch_alice)?;

    // Extract difference between two sets from merged sketch
    let mut differences = [0u64; 400];
    let num_differences = sketch_bob.decode(&mut differences[..])?;

    // println!("Differences between Alice and Bob: {}", num_differences);
    assert!(num_differences > 0);

    // Sort differences since they may come in arbitrary order from Minisketch::decode()
    let mut differences = Vec::from(&differences[..]);
    differences.sort();

/*    for (i, diff) in differences.iter().enumerate() {
        println!("Difference #{}: {}", (i + 1), diff);
    }
 */
    assert_eq!(differences[0], 1000000_3_00000);
    assert_eq!(differences[1], 1000000_3_00001);
    assert_eq!(differences[2], 1000000_3_00002);
    assert_eq!(differences[3], 1000000_3_00003);

    Ok(())
}

pub fn simple_sketch(bench: &mut Bencher) {
    // Create sketch of Alice's set
    let sketch_alice = create_sketch_alice().unwrap();

    // Serialize sketch as bytes
    let mut buf_a = vec![0u8; sketch_alice.serialized_size()];
    sketch_alice.serialize(buf_a.as_mut_slice()).unwrap();

    // println!("Message: {}, {:?}", buf_a.len(), buf_a);

    // Send bytes to Bob for set reconciliation
    bench.iter(|| {
        reconcile_with_bob(&buf_a).unwrap();
    });
}

pub fn create_sketch_bench(bench: &mut Bencher) {
    bench.iter(|| {
        create_sketch(0..100000u64);
    });
}

pub fn nothing(bench: &mut Bencher) {
}

benchmark_group!(
    benches,
    nothing
    // # simple_sketch,
    //create_sketch_bench
);

benchmark_main!(benches);


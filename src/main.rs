use std::time::{Duration, Instant};
use algods::ds::heap::binary_heap_vec::BinaryHeapVec;
use algods::ds::heap::binomial_heap::BinomialHeap;
use csv::Writer;

use algods::ds::heap::leftist_heap::LeftistHeap;
use algods::ds::heap::{self as heap, generate_random_vector, Heap};


fn measure_execution<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}


#[allow(dead_code)]
fn leftist_heap_measurements() {
    let filename = "data/leftist_heap_insert.csv";

    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = LeftistHeap::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let d = measure_execution(|| {
            heap::insert_n_elements(&mut heap, n);
        });
        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}

#[allow(dead_code)]
fn leftist_heap_measurements_random() {
    let filename = "data/leftist_heap_insert_random.csv";

    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = LeftistHeap::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let vec = generate_random_vector(n);

        let d = measure_execution(|| {
            heap::insert_n_vector_elements(&mut heap, &vec);
        });

        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}


#[allow(dead_code)]
fn binary_heap_vec_measurements() {
    let filename = "data/binary_heap_vec_insert.csv";
    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = BinaryHeapVec::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let d = measure_execution(|| {
            heap::insert_n_elements(&mut heap, n);
        });
        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}

#[allow(dead_code)]
fn binary_heap_vec_measurements_random() {
    let filename = "data/binary_heap_vec_insert_random.csv";
    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = BinaryHeapVec::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let vec = generate_random_vector(n);

        let d = measure_execution(|| {
            heap::insert_n_vector_elements(&mut heap, &vec);
        });

        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}


#[allow(dead_code)]
fn binomial_heap_measurements() {
    let filename = "data/binomial_heap_insert.csv";
    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = BinomialHeap::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let d = measure_execution(|| {
            heap::insert_n_elements(&mut heap, n);
        });
        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}

#[allow(dead_code)]
fn binomial_heap_measurements_random() {
    let filename = "data/binomial_heap_insert_random.csv";
    let mut wtr = Writer::from_path(filename).unwrap();
    let mut heap = BinomialHeap::<i32>::new();

    for n in [10_000, 50_000,  100_000, 150_000, 200_000, 250_000, 300_000, 350_000, 400_000, 450_000, 500_000] {
        let vec = generate_random_vector(n);

        let d = measure_execution(|| {
            heap::insert_n_vector_elements(&mut heap, &vec);
        });

        heap.clear();
        wtr.write_record(&[format!("{}", n), format!("{}", d.as_secs_f64())]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Written {filename}")
}



fn main() {
    leftist_heap_measurements();
    leftist_heap_measurements_random();
    binary_heap_vec_measurements();
    binary_heap_vec_measurements_random();
    binomial_heap_measurements();
    binomial_heap_measurements_random();
}

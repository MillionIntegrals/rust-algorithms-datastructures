use crate::ds::heap::binary_heap_vec::BinaryHeapVec;
use crate::ds::heap::binomial_heap::BinomialHeap;
use crate::ds::heap::leftist_heap::LeftistHeap;
use crate::ds::heap::{self as heap, generate_random_vector, Heap};

use crate::analysis::utils::measure_execution;

use csv::Writer;

use super::commands::{CommandDescriptor, CommandMap};


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

fn analyze_leftist_heap() {
    leftist_heap_measurements();
    leftist_heap_measurements_random();
}


pub fn register_commands(cm: &mut CommandMap) {
    cm.add(CommandDescriptor::new("analyze-heap-leftist", "Run runtime analysis for leftist heap", analyze_leftist_heap))
}
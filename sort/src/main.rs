mod scratch_file;

use scratch_file::scratch;

extern crate rand;

use std::time::Instant;
use rand::thread_rng;
use rand::Rng;

fn generate_random_list() -> Vec<i32> {
    let mut rand_list: Vec<i32> = Vec::new();

    let mut rng = thread_rng();
    for i in 1..25{
        rand_list.append(rng.gen_range(0,99));
    }

    rand_list
}

fn selection_sort(unsorted:Vec<i32>) -> Vec<i32> {
    let mut i:usize;

    let mut sorted: Vec<i32> = unsorted;
    let length = unsorted.len();
    let before = Instant::now();

    for minimum_index in 0..length{
        // create trackers to hold data
        let mut min_num:i32 = 100;
        let mut index: usize = minimum_index;
        let mut min_index: usize;

        //We only want to go through the part of the list we haven't already sorted
        for number in sorted[minimum_index..length]{
            if number < min_num{
                min_num = number;
                min_index = index;
            }
            index += 1;
            //TODO: change the array
        }
    }

    println!("Elapsed time: {:.2?}", before.elapsed());

    sorted
}

fn print_list(x:Vec<i32>){
    for item in x{
        print!("{}, ", item);
    }
}

fn main() {
    let mut list = generate_random_list();
    let sorted_list = selection_sort(list);

    print_list(sorted_list);

    // scratch();
}
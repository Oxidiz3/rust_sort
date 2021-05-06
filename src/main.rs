// Author Porter Mecham
// Date 05/06/2021
extern crate rand;

use rand::{Rng};
use rand::{thread_rng};
use std::time::{Instant, Duration};

fn generate_random_list(list_length: usize) -> Vec<i32> {
    let mut rand_list: Vec<i32> = Vec::new();
    let mut rng = thread_rng();

    // take a random value from 0 to 10 and add it to the end of the list
    for _ in 1..list_length {
        rand_list.push(rng.gen_range(0, 10));
    }

    rand_list
}

fn selection_sort(unsorted: &Vec<i32>, length: usize) -> Vec<i32> {
    // let before = Instant::now();
    let mut sorting_list = unsorted.clone();

    // go through the Vector one at a time
    for min_index in 0..length - 1 {

        // create tracker to hold smallest number
        let mut min_num: i32 = 100;
        let mut min_num_index: usize = 0;
        let mut index = min_index;

        //only go through the part of the Vector we haven't already sorted
        while index < length{

            // If indexed number is smaller then our min_num change min num
            if sorting_list[index] < min_num{
                min_num = sorting_list[index];
                min_num_index = index;
            }
            index += 1;
        }

        let temp = sorting_list[min_index];
        sorting_list[min_index] = min_num;
        sorting_list[min_num_index] = temp;
        // Add lowest number to sorted array
    }

    sorting_list
}

fn bubble_sort(list: &Vec<i32>, length: usize){
    let mut sorting_list = list.clone();
    let before = Instant::now();
    let mut sorted = true;

    // If the list was sorted last round check if there is more to sort
    while sorted {
        sorted = false;

        for index in 0..length - 2 {
            // Check if two numbers are out of order
            if sorting_list[index + 1] < sorting_list[index] {
                // swap numbers positions if out of order
                let temp = sorting_list[index];
                sorting_list[index] = sorting_list[index + 1];
                sorting_list[index + 1] = temp;
                sorted = true;
            }
        }
    }

    println!("\nBubble Sort time: {:.2?}", before.elapsed());
}

fn counting_sort(list: &Vec<i32>){
    let mut count:Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0];
    let before = Instant::now();

    // Sort out the array by number
    for number in list{
        count[*number as usize] += 1;
    }


    //Print out the entirety of the sorted array
    let mut i = 0;
    let mut sorted_list = Vec::new();
    for x in count{
        for _ in 0..x{
            sorted_list.push(i);
        }
        i += 1;
    }

    println!("Counting Sort: {:?}", before.elapsed());
}

fn tim_sort(list: &Vec<i32>) -> Vec<Vec<i32>> {
    // Take the list and split it up into equal groups no bigger than size x
    // then send the lists through the iterator sort
    // then merge the lists back together

    // Split up the list into manageable groups
    let before = Instant::now();
    let max_size = 32;

    let split_lists= split_up_list(list, max_size);
    let mut temp_list:Vec<Vec<i32>> = Vec::new();
    let mut sorted_lists:Vec<Vec<i32>> = Vec::new();

    // Sort each list individually
    for list in &split_lists{
        sorted_lists.push(selection_sort(&list, max_size));
        // println!("{:?}", selection_sort(&list, max_size));
    }

    // If the sorted lists is still not one array
    while sorted_lists.len() > 1 {
        let mut i = 0;

        // Combine two arrays together
        // print!("{}", sorted_lists.len());
        while i < sorted_lists.len()/2{
            // Push the merged array into our new temp list
            temp_list.push(merge_lists(&sorted_lists[2 * i], &sorted_lists[2 * i + 1]));
            // print!("{:?}", temp_list.last());
            i+=1;
        }

        // make the sorted_lists equal it's more condensed self
        sorted_lists = temp_list.clone();
        temp_list.clear();
    }
    // Get time after now

    println!("'Tim' Sort: {:?}", before.elapsed());

    sorted_lists
}

fn split_up_list(list: &Vec<i32>, max_size: usize) -> Vec<Vec<i32>> {
    let mut split_lists:Vec<Vec<i32>> = Vec::new();

    // Split up the list into manageable groups
    if list.len() > max_size as usize {
        let num_lists = ((list.len()/ max_size) as f32).ceil() as usize;

        // Make the list have a determined side
        split_lists.resize(num_lists, Vec::new());

        // Go through the whole list and push each chunk into the split_lists vector
        for i in 0..num_lists{
            // There is a built in function for vectors that chunks up the vec
            split_lists[i as usize] = list[i * max_size..(i + 1) * max_size].to_owned();
        }
    }

    split_lists
}

fn merge_lists(r_list: &Vec<i32>, l_list: &Vec<i32>) -> Vec<i32> {
    let mut r = 0;
    let mut l = 0;
    let mut merged_list = Vec::new();

    for _ in 0..(r_list.len()  + l_list.len()){

        // If r_list has been sorted out
        if r > r_list.len()-1{
            merged_list.push(l_list[l]);
            l += 1;
        // If l_list has been sorted out
        } else if l > l_list.len()-1 {
            merged_list.push(r_list[r]);
            r += 1;
        }
            // if neither array has been sorted out
        else {
            // Push the lesser number to the back of the merged_list
            if r_list[r] <= l_list[l] {
                merged_list.push(r_list[r]);
                r += 1;
            }
            else {
                merged_list.push(l_list[l]);
                l += 1;
            }
        }
    }

    merged_list
}

fn main() {
    let list_length: usize = 4096;
    let random_list = generate_random_list(list_length);

    // time and use the selection_sort
    let before = Instant::now();
    selection_sort(&random_list, list_length - 1);
    print!("Selection Sort: {:?}", before.elapsed());

    // use and time bubble sort
    bubble_sort(&random_list, list_length);
    // use and time the counting sort
    counting_sort(&random_list);
    let sorted_list = tim_sort(&random_list);
    println!("{:?}", sorted_list);
}
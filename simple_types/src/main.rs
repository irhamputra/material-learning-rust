// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

// import library using use statement
// rules name_lib::collection::{HashMap}
use simple_types::{on_off, print_array, print_difference, ding, print_distance};

fn main() {
    // create a tuples for
    let coords: (f32, f32) = (6.3, 15.0);

    // destructuring tuples
    let (float_one, float_two) = coords;

    // you can also do like this (coords.0, coords.1)
    print_difference( float_one, float_two );


    // assign an array
    let coords_arr:[f32; 2] = [float_one, float_two];
    print_array(coords_arr);


    // array indexing finding 13
    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len() - 1]);

    // finding complex index
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    // destructure mess variable
    let (arr, number, arr_tuples, num_int, str) = mess;

    // find the last arr_tuples index and access the first index
    on_off(arr_tuples[arr_tuples.len() - 1].0);

    print_distance(coords);
}


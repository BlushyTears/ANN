
use crate::lin_reg;

// 
// pub fn reshape_to_2d(arr: Vec<i32>, cols: usize, rows: usize) {
//     let mut new_arr: Vec<i32> = Vec::new();

//     if arr.len() % cols != 0 { return; }

//     for (i, x) in new_arr.iter().enumerate() {
//         if i % cols == 0 {
//             let index = [
//                 Vec2d {
//                     col: x,
//                     row: ,
//                 },
//             ];
//         }
//         else if i % cols == 1 {  
//             let index = [
//                 Vec2d {
//                     col: x,
//                     row: ,
//                 },
//             ];
//         }   
//         new_arr.push(index);
//     }
// }

pub fn reshape_to_list(vec_2d: Vec<[lin_reg::Vec2d; 1]>) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();

    for i in vec_2d {
        list.push(i[0].row);     
        list.push(i[0].col);
    }
    list
}

// Splits a Vector between index x and y
pub fn split_list(arr: &Vec<i32>, x: usize, y: usize) -> Vec<i32> {

    let mut new_arr: Vec<i32> = Vec::new();

    for i in 0..arr.len() {
        println!("{}", i);
        if i >= x {
            if i <= y { new_arr.push(arr[i]); }
        }
    }
    new_arr
}

// Copies a Vector
pub fn copy_list(arr: &Vec<i32>) -> &Vec<i32> {
    arr
}

// Sums a Vector
pub fn sum_list(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}

// Subtracts a Vector
pub fn sub_list(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum -= i;
    }
    sum
}


// Check if x1*w1 + x2*w2 + x3*w3 - b > 0 and return as boolean 
// If weights are higher then they weigh more in the final calculation
// X are just input values from whatever image, text etc read from

// Will Sweden qualify for the next Eurocup
// Arguments: Elanga plays (w = 3), Kulusevski plays (w = 3), Lindelof plays (w = 2)

// #[derive(Serialize, Deserialize)]

// Recreate some of the functionalities from tensorflow

// Tensorflow ideas:
// Link: https://github.com/aymericdamien/TensorFlow-Examples/blob/master/tensorflow_v2/notebooks/3_NeuralNetworks/neural_network.ipynb


// Todo: Lib for creating layers
// use lin_reg::print_word;

pub mod lin_reg;
pub mod ann;
pub mod listops;

use crate::lin_reg::*;
use crate::listops::*;
use crate::ann::*;

fn main() {
    let two_dim = 2;
    let vec_x = vec![1, 3, -5];
    let vec_y = vec![4, -2, -1];
    let vec_z = vec![1, 2, 3, 4];


    // Different functions: 
    println!("Basic Neural network prediction: ", );
    predict_outcome(init_nn());
    println!("\n Dot product: {}", calc_dot_product(&vec_x, &vec_y));

    let matrix = init_matrix(&two_dim);
    println!("2D matrix: {:?}", matrix);
    println!("Matrix, converted to 1 dimensional list {:?}", reshape_to_list(matrix));

}

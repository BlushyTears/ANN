
// Check if x1*w1 + x2*w2 + x3*w3 - b > 0 and return as boolean 
// If weights are higher then they weigh more in the final calculation
// X are just input values from whatever image, text etc read from

// Will Sweden qualify for the next Eurocup
// Arguments: Elanga plays (w = 3), Kulusevski plays (w = 3), Lindelof plays (w = 2)

// #[derive(Serialize, Deserialize)]

use rand::prelude::*;

#[derive(Debug)]
struct Perception {
    x: f32,
    weight: f32,
}

fn init_nn() -> Vec<[Perception; 1]> {

    let mut percep_vec = Vec::new();

    for i in 0..11 {
        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen();
        let rand_weight: f32 = rng.gen();

        let percep_struct = [
            Perception {
                x: rand_x,
                weight: rand_weight,
            }
        ];
        percep_vec.push(percep_struct);
    }
    
    percep_vec
}

fn predict_outcome(data: Vec<[Perception; 1]>) {
    
    let mut bias = 2.0;

    for i in data {
        bias -= i[0].x * i[0].weight;
    }
    
    if bias > 0.0 {
        println!("True. Bias: {}", bias);
    }
    else {
        println!("False. Bias: {}", bias);
    }

}

fn main() {


    predict_outcome(init_nn());
    

}

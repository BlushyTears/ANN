
use rand::prelude::*;

#[derive(Debug)]
pub struct Perception {
    x: f32,
    weight: f32,
}

// Executes a random prediction with no substance
pub fn predict_outcome(data: Vec<[Perception; 1]>) {
    
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

pub fn init_nn() -> Vec<[Perception; 1]> {

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



// Beyond amazing reference: https://github.com/henryfo/polyfit
// https://medium.com/analytics-vidhya/linear-regression-machine-learning-ef8b8899922a
  

// static matrix_t *createMatrix( int rows, int cols )
// {
//     matrix_t *rVal = (matrix_t *) calloc(1, sizeof(matrix_t));
//     if(NULL != rVal)
//     {
//         rVal->rows = rows;
//         rVal->cols = cols;
//         rVal->pContents = (double *) calloc( rows * cols, sizeof( double ));
//         if(NULL == rVal->pContents)
//         {
//             free( rVal );
//             rVal = NULL;
//         }
//     }

//     return rVal;
// }

#[derive(Debug)]
pub struct Vec2d {
    pub row: i32,
    pub col: i32,
}

// Simple matrix implementation
pub fn init_matrix(rows: &i32) -> Vec<[Vec2d; 1]> {
    let mut vec_struct = Vec::new();

    for i in 0..*rows {
            let index = [
                Vec2d {
                    row: i,
                    col: i + 1,
                },
            ];
        vec_struct.push(index);
    }

    vec_struct
}

// Might wanna get absolute value (not sure yet how it works for negative values)
pub fn calc_dot_product(vec_x: &Vec<i32>, vec_y: &Vec<i32>) -> i32  {
    let mut sum = 0;
    if vec_x.len() != vec_y.len() {panic!("rows and columns has to be the same length!")};

    for i in 0..vec_x.len() {
        sum += vec_x[i] * vec_y[i];
    }
    sum
}


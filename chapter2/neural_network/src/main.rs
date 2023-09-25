use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
mod utils;
use utils::{ActivationType, ActivationFuncs, LossType, LossFuncs};

fn main() {
    // let X: Array1<f64> = array![10.,100.,1000., 10000.,];
    // let b: Array1<f64> = Array::range(1., 32., 1.);     // b = [0., 1., 2., 3, ]
    // let i: Array1<f64> = Array::ones(2);
    // let test2: Array2<f64> = Array2::random((2,1), Uniform::new(-1., 1.));
    // println!("{:?}", i);
    // println!("{:?}", i.dot(&test2));
}

#[allow(dead_code)]
struct Layer {
    n: usize,
    weights : Array2<f64>,
    biases : Array1<f64>,
    activation : ActivationFuncs,
    delta: Array1<f64>,
}

#[allow(dead_code)]
impl Layer {
    fn new(input: usize, n: usize, act_type: ActivationType) -> Self {
        let weights: Array2<f64> = Array2::random((input, n), Uniform::new(-1., 1.));
        let biases: Array1<f64> = Array::random(n, Uniform::new(-1., 1.));
        let activation = ActivationFuncs::new(act_type);
        let delta: Array1<f64> = Array::zeros(n);
        Layer {
            n,
            weights,
            biases,
            activation,
            delta,
        }
    }
}
#[allow(dead_code)]
struct Network {
    layers: Vec<Layer>,
    loss: LossFuncs
}

#[allow(dead_code)]
impl Network {
    fn new(loss: LossType) -> Self {
        let loss = LossFuncs::new(loss);
        Self {
            layers: vec![],
            loss
        }
    }
    fn add(mut self, n: usize, act_type: ActivationType, input: Option<usize>) {
        if self.layers.is_empty() {
            self.layers.push(Layer::new(input.unwrap(), n, act_type));
        }
        else {
            self.layers.push(Layer::new(self.layers[self.layers.len() -1].n, n, act_type));
        }
    }

    fn foward(self, x: Array2<f64>) -> Array2<f64> {
        let mut z: Array2<f64> = x.dot(&self.layers[0].weights) + &self.layers[0].biases;
        let mut a: Array2<f64> = z.map(self.layers[0].activation.activation);
        for i in 1..self.layers.len() {
            z = z.dot(&self.layers[i].weights) + &self.layers[i].biases;
            a =  z.map(self.layers[i].activation.activation);
        }
        a
    }

    fn backpropagation(self, x: Array2<f64>, y: Array2<f64>) {}
}
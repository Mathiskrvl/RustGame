#[allow(dead_code)]
pub enum ActivationType {
    Relu,
    Sigmoid,
    Tanh,
    Identité,
}
#[allow(dead_code)]
pub enum LossType {
    MSE,
    BCE,
    CCE,
}
#[allow(dead_code)]
pub struct LossFuncs {
    activation: fn(&f64) -> f64,
    derivative: fn(&f64) -> f64,
}
#[allow(dead_code)]
impl LossFuncs {
    pub fn new(loss_type: LossType) -> Self {
        match loss_type {
            LossType::MSE => Self {
                activation: relu_activation,
                derivative: relu_derivative,
            },
            LossType::BCE => Self {
                activation: sigmoid_activation,
                derivative: sigmoid_derivative,
            },
            LossType::CCE => Self {
                activation: tanh_activation,
                derivative: tanh_derivative,
            },
            // Ajoutez d'autres types d'activation au besoin
        }
    }
}


#[allow(dead_code)]
pub struct ActivationFuncs {
    pub activation: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}
#[allow(dead_code)]
impl ActivationFuncs {
    pub fn new(activation_type: ActivationType) -> Self {
        match activation_type {
            ActivationType::Relu => Self {
                activation: relu_activation,
                derivative: relu_derivative,
            },
            ActivationType::Sigmoid => Self {
                activation: sigmoid_activation,
                derivative: sigmoid_derivative,
            },
            ActivationType::Tanh => Self {
                activation: tanh_activation,
                derivative: tanh_derivative,
            },
            ActivationType::Identité => Self {
                activation: |x| *x,
                derivative: |_| 1.,
            },
            // Ajoutez d'autres types d'activation au besoin
        }
    }
}

fn relu_activation(x: &f64) -> f64 {
    if x > &0.0 {
        *x
    } else {
        0.0
    }
}

fn relu_derivative(x: &f64) -> f64 {
    if x > &0.0 {
        1.0
    } else {
        0.0
    }
}

fn sigmoid_activation(x: &f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: &f64) -> f64 {
    let sigmoid_x = sigmoid_activation(x);
    sigmoid_x * (1.0 - sigmoid_x)
}

fn tanh_activation(x: &f64) -> f64 {
    x.tanh()
}

fn tanh_derivative(x: &f64) -> f64 {
    1.0 - x.tanh().powi(2)
}
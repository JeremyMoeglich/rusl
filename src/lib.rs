use rand::Rng;
use rand_distr::Normal;

fn initialize_coefficients(std_dev: f64, n: usize) -> Vec<f64> {
    let normal = Normal::new(0.0, std_dev).unwrap();
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.sample(normal)).collect()
}

pub trait Model {
    fn predict(&self, inputs: &[f64]) -> f64;
}

pub trait GradientModel: Model + CostFunction {}

pub struct LinearRegression {
    weights: Vec<f64>,
    bias: f64,
}

impl Model for LinearRegression {
    fn predict(&self, inputs: &[f64]) -> f64 {
        let mut sum = 0.0;
        for (i, input) in inputs.iter().enumerate() {
            sum += input * self.weights[i];
        }
        sum + self.bias
    }
}

pub struct DataPoint {
    inputs: Vec<f64>,
    actual: f64,
}

pub trait CostFunction {
    fn cost<T: Model>(&self, model: &T, datapoints: &[DataPoint]) -> f64;
    fn cost_gradient<T: Model>(&self, model: &T, datapoints: &[DataPoint]) -> Vec<f64>;
}

pub struct MeanSquaredError {}

impl CostFunction for MeanSquaredError {
    fn cost<T: Model>(&self, model: &T, datapoints: &[DataPoint]) -> f64 {
        datapoints.iter().fold(0.0, |acc, datapoint| {
            let predicted = model.predict(&datapoint.inputs);
            let error = predicted - datapoint.actual;
            acc + error * error
        }) / (2.0 * datapoints.len() as f64)
    }

    fn cost_gradient<T: Model>(&self, model: &T, datapoints: &[DataPoint]) -> Vec<f64> {
        let init_gradients = vec![0.0; datapoints[0].inputs.len()];
        datapoints
            .iter()
            .fold(init_gradients, |gradients, datapoint| {
                let predicted = model.predict(&datapoint.inputs);
                let error = predicted - datapoint.actual;
                gradients
                    .iter()
                    .zip(datapoint.inputs.iter())
                    .map(|(g, input)| g + error * input)
                    .collect()
            })
            .iter()
            .map(|g| g / datapoints.len() as f64)
            .collect()
    }
}

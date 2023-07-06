use argmin::core::observers::{ObserverMode, SlogLogger};
use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::simulatedannealing::{Anneal, SATempFunc, SimulatedAnnealing};
use argmin_testfunctions::easom;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::sync::{Arc, Mutex};

struct Easom {
    lower_bound: f64,
    upper_bound: f64,
    rng: Arc<Mutex<Xoshiro256PlusPlus>>,
}

impl Easom {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
            rng: Arc::new(Mutex::new(Xoshiro256PlusPlus::from_entropy())),
        }
    }
}

impl CostFunction for Easom {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        Ok(easom(param) + 1.0)
        // Ok(easom(param))
    }
}

impl Anneal for Easom {
    type Param = Vec<f64>;
    type Output = Vec<f64>;
    type Float = f64;

    fn anneal(&self, param: &Self::Param, temperature: Self::Float) -> Result<Self::Output, Error> {
        let mut rng = self.rng.lock().unwrap();

        let temperature_n = temperature.max(0.00001);

        let perturbation = rng.sample(Uniform::new_inclusive(-1.0, 1.0)) * temperature;

        println!(
            "New perturbation: {}, temperature: {}",
            perturbation, temperature
        );

        let param_n = param
            .clone()
            .iter()
            .map(|&x| {
                let new_value = x + perturbation;
                new_value.max(self.lower_bound).min(self.upper_bound)
            })
            .collect();

        println!("New param: {:?}, pertubation: {:?}", param_n, perturbation);

        Ok(param_n)
    }
}

fn main() -> Result<(), Error> {
    let temperature = 100.0;
    let lower_bound = -100.0;
    let upper_bound = 100.0;
    let initial_point = vec![50.0_f64, -50.0_f64];

    let operator = Easom::new(lower_bound, upper_bound);

    let solver = SimulatedAnnealing::new(temperature)?
        // .with_temp_func(SATempFunc::Boltzmann)
        .with_temp_func(SATempFunc::Linear(0.1))
        .with_stall_best(5000)
        .with_stall_accepted(5000)
        // Optional: Reanneal after 1000 iterations (resets temperature to initial temperature)
        // .with_reannealing_fixed(1000)
        // Optional: Reanneal after no accepted solution has been found for `iter` iterations
        // .with_reannealing_accepted(500)
        // Optional: Start reannealing after no new best solution has been found for 800 iterations
        .with_reannealing_best(800);

    let res = Executor::new(operator, solver)
        .configure(|state| {
            state
                .param(initial_point)
                // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
                .max_iters(10_000)
                // Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
                .target_cost(0.0)
        })
        // Optional: Attach a observer
        // .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Wait a second (lets the logger flush everything before printing again)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Print result
    println!("{res}");

    Ok(())
}

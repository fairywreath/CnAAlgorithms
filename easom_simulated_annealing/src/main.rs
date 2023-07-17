use argmin::core::observers::{ObserverMode, SlogLogger};
use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::simulatedannealing::{Anneal, SATempFunc, SimulatedAnnealing};
use argmin_testfunctions::easom;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::sync::{Arc, Mutex};

use plotters::prelude::*;

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

        // println!(
        //     "New perturbation: {}, temperature: {}",
        //     perturbation, temperature
        // );

        let param_n = param
            .clone()
            .iter()
            .map(|&x| {
                let new_value = x + perturbation;
                new_value.max(self.lower_bound).min(self.upper_bound)
            })
            .collect();

        // println!("New param: {:?}, pertubation: {:?}", param_n, perturbation);

        Ok(param_n)
    }
}

fn main() -> Result<(), Error> {
    let lower_bound = -100.0;
    let upper_bound = 100.0;

    // Best from b.i
    let initial_point = vec![-61.93053801633628, 25.608809544535887];

    // Best from b.ii
    let initial_temperature = 82.62615541634152_f64;

    let mut rng = rand::thread_rng();
    let mut initial_points = vec![];
    for _ in 0..10 {
        let initial_point = vec![
            rng.gen_range(lower_bound..upper_bound),
            rng.gen_range(lower_bound..upper_bound),
        ];
        initial_points.push(initial_point);
    }

    let temperatures: Vec<f64> = (0..10).map(|_| rng.gen_range(10.0..100.0)).collect();

    let mut results = Vec::new();

    // for point in &initial_points {
    // for temperature in &temperatures {
    for _ in 0..10 {
        let operator = Easom::new(lower_bound, upper_bound);
        let solver = SimulatedAnnealing::new(initial_temperature.clone())?
            // .with_temp_func(SATempFunc::Boltzmann)
            // .with_temp_func(SATempFunc::Linear(0.1))
            // .with_temp_func(SATempFunc::Linear(0.1))
            .with_temp_func(SATempFunc::Exponential(0.95))
            // .with_temp_func(SATempFunc::TemperatureSlow(0.000001))
            .with_stall_best(50000)
            .with_stall_accepted(50000)
            // Optional: Reanneal after 1000 iterations (resets temperature to initial temperature)
            // .with_reannealing_fixed(1000)
            // Optional: Reanneal after no accepted solution has been found for `iter` iterations
            // .with_reannealing_accepted(500)
            // Optional: Start reannealing after no new best solution has been found for 800 iterations
            .with_reannealing_best(800);

        let res = Executor::new(operator, solver)
            .configure(|state| {
                state
                    // .param(point.clone())
                    .param(initial_point.clone())
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

        results.push(res);
    }

    let root = BitMapBackend::new("scatter_plot_zoomed.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("SA Scatter", ("sans-serif", 40))
        .build_cartesian_2d(3.0_f64..3.25_f64, 3.0_f64..3.25_f64)
        .unwrap();

    let mut draw_points = Vec::new();
    let mut draw_points_xy = Vec::new();

    for res in &results {
        let x = initial_point[0];
        let y = initial_point[1];
        let best_x = res.state.best_param.as_ref().unwrap()[0];
        let best_y = res.state.best_param.as_ref().unwrap()[1];

        let best_xy = (best_x, best_y);

        draw_points.push((initial_point[0], initial_point[1]));
        draw_points_xy.push(best_xy);
    }

    ctx.configure_mesh().draw()?;

    // ctx.draw_series(
    //     draw_points
    //         .iter()
    //         .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
    // )?;

    ctx.draw_series(
        draw_points_xy
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
    )?;

    // Write results to a CSV file
    // let file = std::fs::File::create("q1_b_iii_slow.csv")?;
    // let mut writer = csv::Writer::from_writer(file);

    // // Write header
    // writer.write_record(&["Optimal Solution", "Cost"])?;

    // for (i, res) in results.iter().enumerate() {
    //     writer.write_record(&[
    //         // format!("{:?}", initial_points[i]),
    //         // format!("{:?}", temperatures[i]),
    //         format!("{:?}", res.state.best_param.clone().unwrap()),
    //         format!("{:?}", res.state.best_cost - 1.0),
    //     ])?;
    // }

    // writer.flush()?;

    Ok(())
}

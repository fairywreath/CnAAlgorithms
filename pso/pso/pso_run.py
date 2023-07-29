import numpy as np
import pandas as pd
import pyswarms as ps
import sys

data_points = pd.read_csv('data_points.csv')
x_data = data_points['x'].values
y_data = data_points['y'].values
z_data = data_points['z'].values

a_min, a_max = -5, 5
b_min, b_max = -50, 50
c_min, c_max = 0.01, 10


def fitness_function(params):
    a, b, c = params[:, 0], params[:, 1], params[:, 2]
    print(a)
    func_output = (a[:, np.newaxis] * x_data ** 2 + y_data ** 2 +
                   b[:, np.newaxis]) * np.sin(c[:, np.newaxis] * x_data + y_data)
    return np.mean((func_output - z_data) ** 2, axis=1)


num_particles = 50
dimensions = 3
options = {'c1': 1.5, 'c2': 1.5, 'w': 0.5}  # PSO parameters
# velocity_clamp = (-10000000000000000, 10000000000000000)
# velocity_clamp = (float('-inf'), float('+inf'))
velocity_clamp = None
# velocity_clamp = (sys.float_info.min, sys.float_info.max)

bounds = (np.array([a_min, b_min, c_min]), np.array([a_max, b_max, c_max]))
optimizer = ps.single.GlobalBestPSO(
    n_particles=num_particles, dimensions=dimensions, options=options, bounds=bounds, velocity_clamp=velocity_clamp)


best_cost, best_params = optimizer.optimize(fitness_function, iters=100)

best_a, best_b, best_c = best_params
print("Optimal values for a, b, and c:", best_a, best_b, best_c)

# Check the function output with actual z values
best_func_output = (best_a * x_data ** 2 + y_data ** 2 +
                    best_b) * np.sin(best_c * x_data + y_data)
print("Length of best_func_output: " + str(len(best_func_output)))
difference = best_func_output - z_data

print("Function output and difference between function output and actual z values:")
for i, (x_val, y_val, z_val, func_output_val, diff) in enumerate(zip(x_data, y_data, z_data, best_func_output, difference)):
    print(f"Row {i+1}: x={x_val}, y={y_val}, z_actual={z_val}, z_function_output={func_output_val}, Difference: {diff}")

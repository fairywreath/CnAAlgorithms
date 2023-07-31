import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


class ParticleSwarmOptimizer:
    def __init__(self, func, dimensions, num_particles, max_iter, w, c1, c2, upper_bound, lower_bound, velocity_limit, use_randomness=True):
        self.dimensions = dimensions
        self.num_particles = num_particles
        self.max_iter = max_iter
        self.w = w
        self.c1 = c1
        self.c2 = c2
        self.velocity_limit = velocity_limit
        self.upper_bound = np.array(upper_bound) * np.ones(self.dimensions)
        self.lower_bound = np.array(lower_bound) * np.ones(self.dimensions)
        self.use_randomness = use_randomness
        self.func = func

        assert np.all(self.upper_bound > self.lower_bound)
        assert self.dimensions == len(
            self.upper_bound) == len(self.lower_bound)

        self.set_initial_positions_and_velocities()

        # Set initial costs
        self.update_costs()

        # Set initial best values
        self.pbest_pos = self.particle_positions.copy()
        self.pbest_costs = np.array([[np.inf]] * self.num_particles)
        self.gbest_pos = self.pbest_pos.mean(axis=0).reshape(1, -1)
        self.gbest_cost = np.inf
        self.gbest_cost_history = []
        self.update_gbest()

    def set_initial_positions_and_velocities(self):
        # Set initial positions
        self.particle_positions = np.random.uniform(low=self.lower_bound, high=self.upper_bound, size=(
            self.num_particles, self.dimensions))

        # Set initial velocities
        maximum_allowable_velocity = self.upper_bound - self.lower_bound
        self.particle_velocities = np.random.uniform(
            low=-maximum_allowable_velocity, high=maximum_allowable_velocity, size=(self.num_particles, self.dimensions))

    def update_costs(self):
        self.costs = self.func(self.particle_positions).reshape(-1, 1)
        return self.costs

    def update_gbest(self):
        min_index = self.pbest_costs.argmin()
        if self.gbest_cost > self.pbest_costs[min_index]:
            self.gbest_pos = self.particle_positions[min_index, :].copy()
            self.gbest_cost = self.pbest_costs[min_index]

    def update_pbest(self):
        self.require_update = self.costs < self.pbest_costs
        self.pbest_pos = np.where(
            self.require_update, self.particle_positions, self.pbest_pos)
        self.pbest_costs = np.where(
            self.require_update, self.costs, self.pbest_costs)

    def update_velocities(self):
        r1 = 0.5
        r2 = 0.5

        if self.use_randomness:
            r1 = np.random.rand(self.num_particles, self.dimensions)
            r2 = np.random.rand(self.num_particles, self.dimensions)

        self.particle_velocities = self.w * self.particle_velocities + \
            self.c1 * r1 * (self.pbest_pos - self.particle_positions) + \
            self.c2 * r2 * (self.gbest_pos - self.particle_positions)

        if self.velocity_limit is not None:
            self.particle_velocities = np.clip(
                self.particle_velocities, -self.velocity_limit, self.velocity_limit)

    def update_positions(self):
        self.particle_positions = self.particle_positions + self.particle_velocities
        self.particle_positions = np.clip(
            self.particle_positions, self.lower_bound, self.upper_bound)

    def optimize(self):
        for curr_iter in range(self.max_iter):
            self.update_velocities()
            self.update_positions()
            self.update_costs()
            self.update_pbest()
            self.update_gbest()

            self.gbest_cost_history.append(self.gbest_cost)

        self.best_pos = self.gbest_pos
        self.best_cost = self.gbest_cost

        return self.best_pos, self.best_cost


# Load data points from CSV file
data_points = pd.read_csv('data_points.csv')
x_data = data_points['x'].values
y_data = data_points['y'].values
z_data = data_points['z'].values

# Define search space constraints
a_min, a_max = -5, 5
b_min, b_max = -50, 50
c_min, c_max = 0.01, 10


max_iter = 100
num_particles = 100
dimensions = 3
lower_bound = [a_min, b_min, c_min]
upper_bound = [a_max, b_max, c_max]
w = 0.5
c1 = 2.0
c2 = 2.0
velocity_limit = np.array([5, 50, 5])
use_randomness = True


def fitness_function(params):
    a, b, c = params[:, 0], params[:, 1], params[:, 2]
    func_output = (a[:, np.newaxis] * x_data ** 2 + y_data ** 2 +
                   b[:, np.newaxis]) * np.sin(c[:, np.newaxis] * x_data + y_data)
    return np.mean((func_output - z_data) ** 2, axis=1)


pso = ParticleSwarmOptimizer(func=fitness_function, dimensions=dimensions, num_particles=num_particles, max_iter=max_iter,
                             w=w, c1=c1, c2=c2, upper_bound=upper_bound, lower_bound=lower_bound, velocity_limit=velocity_limit, use_randomness=use_randomness)

best_params, best_cost = pso.optimize()

best_a, best_b, best_c = best_params
print("Optimal values for a, b, and c:", best_a, best_b, best_c)
print(f"Best cost: {best_cost}")

# 3D Plot
x_range = np.linspace(-10, 10, 100)
y_range = np.linspace(-10, 10, 100)
x_grid, y_grid = np.meshgrid(x_range, y_range)

# Calculate function values using the optimized parameters
function_output = (best_a * x_grid ** 2 + y_grid ** 2 +
                   best_b) * np.sin(best_c * x_grid + y_grid)

# Plot the 3D surface
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.plot_surface(x_grid, y_grid, function_output, cmap='viridis')

# Set axis labels
ax.set_xlabel('x')
ax.set_ylabel('y')
ax.set_zlabel('f(x, y, θ)')

# Set plot title
plt.title('3D Plot')
plt.show()

best_costs_history = pso.gbest_cost_history

# Create a list of iterations from 1 to max_iter
iterations = list(range(1, max_iter + 1))

# Plot the best cost versus iterations
plt.plot(iterations, best_costs_history)
plt.xlabel("Iterations")
plt.ylabel("Best Cost")
plt.title("Best Cost vs Iterations in PSO")
plt.grid(True)
plt.show()

# w_values = [0.1, 0.3,  0.5, 0.7, 0.9]

# # Store best costs for each run
# best_costs_runs = []

# # Loop over different w values
# for curr_w in w_values:
#     pso = ParticleSwarmOptimizer(func=fitness_function, dimensions=dimensions, num_particles=num_particles, max_iter=max_iter,
#                                  w=curr_w, c1=c1, c2=c2, upper_bound=upper_bound, lower_bound=lower_bound, velocity_limit=velocity_limit, use_randomness=use_randomness)

#     # Run PSO optimization
#     best_params, best_cost = pso.optimize()

#     # Store best cost history for the current run
#     best_costs_runs.append(pso.gbest_cost_history)

#     print(f"Best params for w = {curr_w}: {best_params}, cost: {best_cost}")

# # Plot best cost versus iterations for each run with different w values
# plt.figure(figsize=(8, 6))
# for i, w in enumerate(w_values):
#     plt.plot(range(max_iter), best_costs_runs[i], label=f"w={w}")

# plt.xlabel("Iterations")
# plt.ylabel("Best Cost")
# plt.title("Best Cost vs. Iterations for Different Particle Inertia (w) Values")
# plt.legend()
# plt.grid(True)
# plt.show()

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


class AntColonyOptimizer_TSP:
    def __init__(self, func, num_nodes, num_ants, max_iter, alpha, beta, rho, distances):
        self.num_nodes = num_nodes
        self.num_ants = num_ants
        self.max_iter = max_iter
        self.alpha = alpha
        self.beta = beta
        self.rho = rho
        self.func = func

        # Heuristic information, with extra addition to avoid division by zero
        self.local_heuristics = 1 / \
            (distances + 1e-10 * np.eye(self.num_nodes, self.num_nodes))

        # Pheromones
        self.tau_matrix = np.ones((self.num_nodes, self.num_nodes))

        self.current_paths = np.zeros(
            (self.num_ants, self.num_nodes)).astype(np.int32)
        self.current_costs = None

        self.best_path = None
        self.best_cost = None
        self.all_best_paths = []
        self.all_best_costs = []

        self.best_costs_history = []

    def __calculate_probability_matrix(self):
        probability_matrix = (self.tau_matrix ** self.alpha) * \
            (self.local_heuristics) ** self.beta
        return probability_matrix

    def __update_pheromones_and_bests(self):
        # Current path costs
        costs = np.array([self.func(i) for i in self.current_paths])
        best_cost_index = costs.argmin()
        best_path = self.current_paths[best_cost_index, :].copy()
        best_cost = costs[best_cost_index].copy()

        # Cache current bests
        self.all_best_paths.append(best_path)
        self.all_best_costs.append(best_cost)

        # Update pheromones
        delta_tau = np.zeros((self.num_nodes, self.num_nodes))
        for j in range(self.num_ants):
            for k in range(self.num_nodes - 1):
                source = self.current_paths[j, k]
                destination = self.current_paths[j, k + 1]
                delta_tau[source, destination] += 1 / costs[j]
            # Full lap TSP
            source = self.current_paths[j, self.num_nodes - 1]
            destination = self.current_paths[j, 0]
            delta_tau[source, destination] += 1 / costs[j]

        self.tau_matrix = (1 - self.rho) * self.tau_matrix + delta_tau

    def optimize(self):
        for i in range(self.max_iter):
            probability_matrix = self.__calculate_probability_matrix()
            for j in range(self.num_ants):
                self.current_paths[j, 0] = 0

                for k in range(self.num_nodes - 1):
                    visited = set(self.current_paths[j, :k + 1])
                    nodes_to_visit = list(set(range(self.num_nodes)) - visited)

                    # Calculate probability
                    probability = probability_matrix[self.current_paths[j,
                                                                        k], nodes_to_visit]
                    probability = probability / probability.sum()

                    # Select next path based on probability
                    next_node = np.random.choice(
                        nodes_to_visit, p=probability, size=1)[0]
                    self.current_paths[j, k + 1] = next_node

            self.__update_pheromones_and_bests()

            self.best_costs_history.append(min(self.all_best_costs))

        best_index = np.array(self.all_best_costs).argmin()
        self.best_path = self.all_best_paths[best_index]
        self.best_cost = self.all_best_costs[best_index]

        return self.best_path, self.best_cost


file_path = 'adjacency.txt'
distance_matrix = np.loadtxt(file_path, delimiter=',')
num_nodes = 8


def calculate_total_cost(routine):
    # Cost function
    num_nodes, = routine.shape
    return sum([distance_matrix[routine[i % num_nodes], routine[(i + 1) % num_nodes]] for i in range(num_nodes)])


num_ants = 20
max_iter = 5
evaporation_rate = 0.1  # Pheromone trail persistence
pheromone_intensity = 1  # Relative mportance of pheromone trail
local_intensity = 2  # Relative importante of local heuristic

# aco = AntColonyOptimizer_TSP(func=calculate_total_cost, num_nodes=num_nodes, num_ants=num_ants, max_iter=max_iter, distances=distance_matrix,
#                              alpha=pheromone_intensity, beta=local_intensity, rho=evaporation_rate)

# best_path, best_cost = aco.optimize()

# print(f"Best path: {best_path}, cost: {best_cost}")

# final_cost = calculate_total_cost(best_path)
# print(f"Final cost: {final_cost}")

# all_best_costs = aco.best_costs_history

# # Create an array of iterations from 1 to max_iter
# iterations = np.arange(1, max_iter + 1)

# # Plot the best cost versus iteration
# plt.plot(iterations, all_best_costs, marker='o')
# plt.xlabel('Iteration')
# plt.ylabel('Best Cost')
# plt.title('Best Cost vs. Iteration in ACO')
# plt.grid(True)
# plt.show()

num_runs = 5  # Number of runs with different evaporation rates

# Specify a range of evaporation rates to test
evaporation_rates = np.linspace(0.1, 0.9, num_runs)

# Store best costs for each run
best_costs_runs = []

for current_evaporation_rate in evaporation_rates:
    # Create and run ACO optimizer for each evaporation rate
    aco = AntColonyOptimizer_TSP(func=calculate_total_cost, num_nodes=num_nodes, num_ants=num_ants, max_iter=max_iter,
                                 distances=distance_matrix, alpha=pheromone_intensity, beta=local_intensity, rho=current_evaporation_rate)
    best_path, best_cost = aco.optimize()

    # Store best cost history for the current run
    best_costs_runs.append(aco.best_costs_history)

# Plot best cost versus iterations for each run
plt.figure(figsize=(8, 6))
for i, evaporation_rate in enumerate(evaporation_rates):
    plt.plot(range(max_iter),
             best_costs_runs[i], label=f"Evap. Rate: {evaporation_rate:.2f}")

plt.xlabel("Iterations")
plt.ylabel("Best Cost")
plt.title("Best Cost vs. Iterations for Different Evaporation Rates")
plt.legend()
plt.grid(True)
plt.show()

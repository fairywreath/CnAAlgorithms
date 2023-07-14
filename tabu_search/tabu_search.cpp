#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <random>
#include <algorithm>
#include <limits>

const int NUM_DEPARTMENTS = 20; // Number of departments
const int TABU_TENURE = 15;     // Number of iterations a move remains tabu
/* FOR DYNAMIC TABU LIST ONLY */
#define TABU_TENURE_LOWER_BOUND  25
#define TABU_TENURE_UPPER_BOUND  5
#define TABU_TENURE_UPDATE_INTERVAL  50

std::vector<std::vector<int>> flow_matrix;
std::vector<std::vector<int>> distance_matrix;

void print_permutation(std::vector<int> &permutation)
{
  for (const auto &element : permutation)
  {
    std::cout << element << " ";
  }
  std::cout << std::endl;
}

void read_csv(const std::string &filename)
{
  std::ifstream file(filename);
  std::vector<std::vector<int>> matrix;

  if (file.is_open())
  {
    std::string line;
    while (std::getline(file, line))
    {
      std::vector<int> row;
      std::stringstream ss(line);
      std::string cell;
      while (std::getline(ss, cell, ','))
      {
        // std::cout << "cell " << cell << std::endl;
        row.push_back(std::stoi(cell));
      }
      if (filename == "Distance.csv")
      {
        distance_matrix.push_back(row);
      }
      if (filename == "Flow.csv")
      {
        flow_matrix.push_back(row);
      }
    }

    file.close();
  }
  else
  {
    std::cerr << "Error opening file: " << filename << std::endl;
  }

  return;
}

// Function to calculate the cost of a permutation
int calculate_cost(const std::vector<int> &permutation)
{
  int cost = 0;
  for (int i = 0; i < NUM_DEPARTMENTS; ++i)
  {
    for (int j = 0; j < NUM_DEPARTMENTS; ++j)
    {
      int flow = flow_matrix[i][j];
      int distance = distance_matrix[permutation[i]][permutation[j]];
      cost += flow * distance;
    }
  }
  return cost;
}

// Function to generate a random initial permutation
std::vector<int> generate_init_permutation()
{
  std::vector<int> permutation(NUM_DEPARTMENTS);
  for (int i = 0; i < NUM_DEPARTMENTS; ++i)
  {
    permutation[NUM_DEPARTMENTS - 1 - i] = i;
  }
  std::random_device rd;
  std::mt19937 g(rd());
  std::shuffle(permutation.begin(), permutation.end(), g);

  std::cout << "Initial Permutation: ";
  print_permutation(permutation);

  return permutation;
}

// Function to generate the neighborhood by generating all possible moves (swaps)
std::vector<std::pair<int, int>> generate_neighbours()
{
  std::vector<std::pair<int, int>> neighborhood;
  for (int i = 0; i < NUM_DEPARTMENTS - 1; ++i)
  {
    for (int j = i + 1; j < NUM_DEPARTMENTS; ++j)
    {
      neighborhood.push_back(std::make_pair(i, j));
    }
  }
  return neighborhood;
}

// Function to perform a move (swap) between two departments in the permutation
void do_move(std::vector<int> &permutation, int i, int j)
{
  std::swap(permutation[i], permutation[j]);
}

// Function to update the tabu list by decrementing the tenure of all moves
void update_tabu_list(std::vector<std::vector<int>> &tabuList)
{
  for (auto &row : tabuList)
  {
    for (int &value : row)
    {
      if (value > 0)
      {
        --value;
      }
    }
  }
}

// Function to search for the best non-tabu move in the neighborhood
std::pair<int, int> find_best_move(const std::vector<int> &permutation,
                                   const std::vector<std::pair<int, int>> &neighbourhood,
                                   const std::vector<std::vector<int>> &tabuList)
{
  int best_cost = std::numeric_limits<int>::max();
  std::pair<int, int> best_move;
  for (const auto &move : neighbourhood)
  {
    int i = move.first;
    int j = move.second;
    std::vector<int> dummy = permutation;
    std::swap(dummy[i], dummy[j]);
    int cost = calculate_cost(dummy);
    if (tabuList[i][j] == 0 && cost < best_cost)
    {
      best_cost = cost;
      best_move = move;
    }
  }
  return best_move;
}

// Tabu Search algorithm
void tabu_simple()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");
  std::vector<int> current_permutation = generate_init_permutation();
  std::vector<int> best_permutation = current_permutation;
  int current_cost = calculate_cost(current_permutation);
  int best_cost = current_cost;

  /* ONLY FOR DYNAMIC TABU LIST */
  std::uniform_int_distribution<int> distribution(1, 100);
  std::random_device rd_tenure;

  std::vector<std::vector<int>> tabu_list(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));

  int iterations = 0;
  while (iterations < 5000)
  { // Adjust stopping criterion as per your requirement
    std::vector<std::pair<int, int>> neighbourhood = generate_neighbours();
    std::pair<int, int> best_move = find_best_move(current_permutation, neighbourhood, tabu_list);

    do_move(current_permutation, best_move.first, best_move.second);
    int new_cost = calculate_cost(current_permutation);
    /* FOR REGULAR TABU LIST */
    // tabu_list[best_move.first][best_move.second] = TABU_TENURE;

    /* ONLY FOR DYNAMIC TABU LIST */
    // Randomly generate tabu tenure within the specified range
    if (iterations % TABU_TENURE_UPDATE_INTERVAL == 0)
    {
      std::mt19937 generator(rd_tenure());
      int random_tenure = distribution(generator);
      tabu_list[best_move.first][best_move.second] = random_tenure;
    }

    if (new_cost < best_cost)
    {
      best_permutation = current_permutation;
      best_cost = new_cost;
    }

    update_tabu_list(tabu_list);
    ++iterations;
  }

  // Print final permutation, best permutation, and best cost
  std::cout << "Final Permutation: ";
  print_permutation(current_permutation);

  std::cout << "Best Permutation: ";
  print_permutation(best_permutation);
  std::cout << "Best Cost: " << best_cost << std::endl;
}

int main()
{

  // Run Tabu Search
  tabu_simple();

  return 0;
}

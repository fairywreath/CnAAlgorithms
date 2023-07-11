#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <limits>

// Constants
const int SIZE = 20;
const int MAX_ITERATIONS = 100;
const int TABU_SIZE = 20;

// Global variables
std::vector<std::vector<int>> flow_matrix;
std::vector<std::vector<int>> distance_matrix;
std::vector<int> current_permutation(SIZE);
std::vector<int> bestPermutation(SIZE);
int best_cost = std::numeric_limits<int>::max();
std::vector<std::vector<int>> tabuList(TABU_SIZE, std::vector<int>(SIZE, -1));

// Read flow and distance matrices from files
void read_input()
{
  std::ifstream flowFile("Flow.csv");
  std::ifstream distanceFile("Distance.csv");

  if (flowFile.is_open() && distanceFile.is_open())
  {
    for (int i = 0; i < SIZE; ++i)
    {
      for (int j = 0; j < SIZE; ++j)
      {
        flowFile >> flow_matrix[i][j];
        distanceFile >> distance_matrix[i][j];
      }
    }

    flowFile.close();
    distanceFile.close();
  }
  else
  {
    std::cerr << "Error opening input files." << std::endl;
    exit(1);
  }
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
        std::cout << "cell " << cell << std::endl;
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

// Calculate the total cost of a permutation
int calculate_cost(const std::vector<int> &permutation)
{
  int cost = 0;
  for (int i = 0; i < SIZE; ++i)
  {
    for (int j = 0; j < SIZE; ++j)
    {
      int flow = flow_matrix[i][j];
      int distance = distance_matrix[permutation[i]][permutation[j]];
      cost += flow * distance;
    }
  }

  return cost;
}

void generate_init_permutation()
{
  for (int i = 0; i < SIZE; ++i)
  {
    current_permutation[SIZE - 1 - i] = i;
  }
  std::random_shuffle(current_permutation.begin(), current_permutation.end());
}

// swap two elements in a permutation
void swap_departments(std::vector<int> &permutation, int index1, int index2)
{
  int temp = permutation[index1];
  permutation[index1] = permutation[index2];
  permutation[index2] = temp;
}

bool is_tabu(const std::vector<int> &permutation)
{
  for (const auto &tabu : tabuList)
  {
    if (tabu == permutation)
    {
      return true;
    }
  }
  return false;
}

// Update the tabu list with a new permutation
void update_tabu_list(const std::vector<int> &permutation)
{
  tabuList.erase(tabuList.begin());
  tabuList.push_back(permutation);
}

// perform a do_move operation by swapping two elements in the current permutation
void do_move(std::vector<int> &permutation, int index1, int index2)
{
  swap_departments(permutation, index1, index2);
  update_tabu_list(permutation);
}

// generate the neighborhood by performing all possible moves
std::vector<std::pair<int, int>> generate_neighbourhood()
{
  std::vector<std::pair<int, int>> neighborhood;

  for (int i = 0; i < SIZE; ++i)
  {
    for (int j = i + 1; j < SIZE; ++j)
    {
      neighborhood.emplace_back(i, j);
    }
  }

  return neighborhood;
}

// Perform Tabu Search (simple ver.)
void tabu_search_simple()
{
  generate_init_permutation();
  bestPermutation = current_permutation;
  best_cost = calculate_cost(current_permutation);

  int iteration = 0;
  while (iteration < MAX_ITERATIONS)
  {
    std::cout << "iteration: " << iteration << std::endl;
    std::vector<std::pair<int, int>> neighborhood = generate_neighbourhood();
    int min_neighbour_cost = std::numeric_limits<int>::max();
    std::vector<int> best_neighbour_permutation;

    // Find the best move in the neighborhood
    for (const auto &move : neighborhood)
    {
      std::vector<int> neighbour_permutation = current_permutation;
      do_move(neighbour_permutation, move.first, move.second);
      int neighbour_cost = calculate_cost(neighbour_permutation);

      if (!is_tabu(neighbour_permutation) && neighbour_cost < min_neighbour_cost)
      {
        min_neighbour_cost = neighbour_cost;
        best_neighbour_permutation = neighbour_permutation;
      }
    }

    std::cout << "best_cost: " << best_cost << std::endl;
    std::cout << "min_neighbour_cost: " << min_neighbour_cost << std::endl;

    // Update the current permutation and best solution if necessary
    if (min_neighbour_cost < best_cost)
    {

      current_permutation = best_neighbour_permutation;
      bestPermutation = best_neighbour_permutation;
      best_cost = min_neighbour_cost;
    }

    iteration++;
  }
}

int main()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");

  tabu_search_simple();

  std::cout << "Final permutation: ";
  for (const auto &element : bestPermutation)
  {
    std::cout << element << " ";
  }
  std::cout << std::endl;

  std::cout << "Best permutation: ";
  for (const auto &element : bestPermutation)
  {
    std::cout << element << " ";
  }
  std::cout << std::endl;

  std::cout << "Best cost: " << best_cost << std::endl;

  return 0;
}

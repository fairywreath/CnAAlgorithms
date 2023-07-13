#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <random>
#include <algorithm>
#include <limits>

const int NUM_DEPARTMENTS = 20; // Number of departments
const int TABU_TENURE = 5;      // Number of iterations a move remains tabu

// Define the flow and distance matrices as global variables
std::vector<std::vector<int>> flow_matrix;
std::vector<std::vector<int>> distance_matrix;

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
  return permutation;
}

// Function to generate the neighborhood by generating all possible moves (swaps)
std::vector<std::pair<int, int>> generateNeighborhood()
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
void performMove(std::vector<int> &permutation, int i, int j)
{
  std::swap(permutation[i], permutation[j]);
}

// Function to update the tabu list by decrementing the tenure of all moves
void updateTabuList(std::vector<std::vector<int>> &tabuList)
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
std::pair<int, int> findBestMove(const std::vector<int> &permutation, const std::vector<std::pair<int, int>> &neighborhood, const std::vector<std::vector<int>> &tabuList)
{
  int bestCost = std::numeric_limits<int>::max();
  std::pair<int, int> bestMove;
  for (const auto &move : neighborhood)
  {
    int i = move.first;
    int j = move.second;
    int cost = calculate_cost(permutation);
    if (tabuList[i][j] == 0 && cost < bestCost)
    {
      bestCost = cost;
      bestMove = move;
    }
  }
  return bestMove;
}

// Tabu Search algorithm
void tabuSearch()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");
  std::vector<int> currentPermutation = generate_init_permutation();
  std::vector<int> bestPermutation = currentPermutation;
  int currentCost = calculate_cost(currentPermutation);
  int bestCost = currentCost;

  std::vector<std::vector<int>> tabuList(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));

  int iterations = 0;
  while (iterations < 1000)
  { // Adjust stopping criterion as per your requirement
    std::vector<std::pair<int, int>> neighborhood = generateNeighborhood();
    std::pair<int, int> bestMove = findBestMove(currentPermutation, neighborhood, tabuList);

    performMove(currentPermutation, bestMove.first, bestMove.second);
    int newCost = calculate_cost(currentPermutation);
    tabuList[bestMove.first][bestMove.second] = TABU_TENURE;

    if (newCost < bestCost)
    {
      bestPermutation = currentPermutation;
      bestCost = newCost;
    }

    updateTabuList(tabuList);
    ++iterations;
  }

  // Print final permutation, best permutation, and best cost
  std::cout << "Final Permutation: ";
  for (int department : currentPermutation)
  {
    std::cout << department << " ";
  }
  std::cout << std::endl;

  std::cout << "Best Permutation: ";
  for (int department : bestPermutation)
  {
    std::cout << department << " ";
  }
  std::cout << std::endl;

  std::cout << "Best Cost: " << bestCost << std::endl;
}

int main()
{
  // Read flow and distance matrices from files (Flow.csv and Distance.csv)
  // Populate the flowMatrix and distanceMatrix global variables

  // Run Tabu Search
  tabuSearch();

  return 0;
}

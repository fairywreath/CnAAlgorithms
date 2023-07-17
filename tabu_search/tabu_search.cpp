#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <random>
#include <algorithm>
#include <limits>

const int NUM_DEPARTMENTS = 20; // Number of departments
const int TABU_TENURE = 20;     // Number of iterations a move remains tabu
/* FOR DYNAMIC TABU LIST ONLY */
#define TABU_TENURE_LOWER_BOUND 25
#define TABU_TENURE_UPPER_BOUND 5
#define TABU_TENURE_UPDATE_INTERVAL 50

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
  /* FOR SEARCHING LESS THAN THE WHOLE NEIGHBOURHOOD*/
  // for (size_t q = 0; q < neighbourhood.size() - 3; ++q){
  //   int i = neighbourhood[q].first;
  //   int j = neighbourhood[q].second;
  //   std::vector<int> dummy = permutation;
  //   std::swap(dummy[i], dummy[j]);
  //   int cost = calculate_cost(dummy);
  //   if (tabuList[i][j] == 0 && cost < best_cost)
  //   {
  //     best_cost = cost;
  //     best_move = neighbourhood[q];
  //   }

  // }
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
    tabu_list[best_move.first][best_move.second] = TABU_TENURE;

    /* ONLY FOR DYNAMIC TABU LIST */
    // Randomly generate tabu tenure within the specified range
    // if (iterations % TABU_TENURE_UPDATE_INTERVAL == 0)
    // {
    //   std::mt19937 generator(rd_tenure());
    //   int random_tenure = distribution(generator);
    //   tabu_list[best_move.first][best_move.second] = random_tenure;
    // }

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

/* FOR FREQUENCY BASED SEARCH*/
// Function to update the tabu list by decrementing the tenure of all moves and adjusting frequency
void update_tabu_list_freq(std::vector<std::vector<int>> &tabuList, std::vector<std::vector<int>> &frequency)
{
  for (int i = 0; i < NUM_DEPARTMENTS; ++i)
  {
    for (int j = 0; j < NUM_DEPARTMENTS; ++j)
    {
      if (tabuList[i][j] > 0)
      {
        --tabuList[i][j];
      }
      else
      {
        // Decrease frequency for non-tabu moves
        --frequency[i][j];
        if (frequency[i][j] < 0)
        {
          frequency[i][j] = 0;
        }
      }
    }
  }
}

// Function to search for the best non-tabu move in the neighborhood using frequency-based tabu list
std::pair<int, int> find_best_move_freq(const std::vector<int> &permutation,
                                        const std::vector<std::pair<int, int>> &neighbourhood,
                                        const std::vector<std::vector<int>> &tabuList,
                                        const std::vector<std::vector<int>> &frequency)
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
    else if (frequency[i][j] > 0 && cost < best_cost)
    {
      best_cost = cost;
      best_move = move;
    }
  }

  return best_move;
}

// Tabu Search algorithm with frequency-based tabu list
void tabu_freq()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");
  std::vector<int> current_permutation = generate_init_permutation();
  std::vector<int> best_permutation = current_permutation;
  int current_cost = calculate_cost(current_permutation);
  int best_cost = current_cost;

  std::vector<std::vector<int>> tabu_list(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));
  std::vector<std::vector<int>> frequency(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));

  int iterations = 0;
  while (iterations < 5000)
  {
    std::vector<std::pair<int, int>> neighbourhood = generate_neighbours();
    std::pair<int, int> best_move = find_best_move_freq(current_permutation, neighbourhood, tabu_list, frequency);

    do_move(current_permutation, best_move.first, best_move.second);
    int new_cost = calculate_cost(current_permutation);
    tabu_list[best_move.first][best_move.second] = TABU_TENURE;

    if (new_cost < best_cost)
    {
      best_permutation = current_permutation;
      best_cost = new_cost;
    }

    update_tabu_list_freq(tabu_list, frequency);
    // Increase frequency for selected move
    ++frequency[best_move.first][best_move.second];
    ++iterations;
  }

  // Print final permutation, best permutation, and best cost
  std::cout << "Final Permutation: ";
  print_permutation(current_permutation);

  std::cout << "Best Permutation: ";
  print_permutation(best_permutation);
  std::cout << "Best Cost: " << best_cost << std::endl;
}

std::pair<int, int> find_best_move_aspiration_best_so_far(const std::vector<int> &permutation,
                                               const std::vector<std::pair<int, int>> &neighbourhood,
                                               const std::vector<std::vector<int>> &tabuList,
                                               int best_cost)
{
  int best_tabu_cost = std::numeric_limits<int>::max();
  std::pair<int, int> best_tabu_move;
  int best_non_tabu_cost = std::numeric_limits<int>::max();
  std::pair<int, int> best_non_tabu_move;

  for (const auto &move : neighbourhood)
  {
    int i = move.first;
    int j = move.second;
    std::vector<int> dummy = permutation;
    std::swap(dummy[i], dummy[j]);
    int cost = calculate_cost(dummy);

    if (cost < best_tabu_cost && tabuList[i][j] > 0)
    {
      best_tabu_cost = cost;
      best_tabu_move = move;
    }

    if (tabuList[i][j] == 0 && cost < best_non_tabu_cost)
    {
      best_non_tabu_cost = cost;
      best_non_tabu_move = move;
    }
  }

  if (best_tabu_cost < best_cost)
  {
    return best_tabu_move;
  }
  else
  {
    return best_non_tabu_move;
  }
}

// Tabu Search algorithm with aspiration criteria for best solution so far
void tabu_aspiration_best_so_far()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");
  std::vector<int> current_permutation = generate_init_permutation();
  std::vector<int> best_permutation = current_permutation;
  int current_cost = calculate_cost(current_permutation);
  int best_cost = current_cost;

  std::vector<std::vector<int>> tabu_list(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));
  std::vector<std::vector<int>> frequency(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));

  int iterations = 0;
  while (iterations < 5000)
  {
    std::vector<std::pair<int, int>> neighbourhood = generate_neighbours();
    std::pair<int, int> best_move = find_best_move_aspiration_best_so_far(current_permutation, neighbourhood, tabu_list, best_cost);
    do_move(current_permutation, best_move.first, best_move.second);
    int new_cost = calculate_cost(current_permutation);
    tabu_list[best_move.first][best_move.second] = TABU_TENURE;

    if (new_cost < best_cost)
    {
      best_permutation = current_permutation;
      best_cost = new_cost;
    }

    update_tabu_list(tabu_list);
    // Increase frequency for selected move
    ++frequency[best_move.first][best_move.second];
    ++iterations;
  }

  // Print final permutation, best permutation, and best cost
  std::cout << "Final Permutation: ";
  print_permutation(current_permutation);

  std::cout << "Best Permutation: ";
  print_permutation(best_permutation);
  std::cout << "Best Cost: " << best_cost << std::endl;
}

// Function to search for the best move in the neighborhood with aspiration criteria
std::pair<int, int> find_best_move_aspiration_best_in_neighborhood(const std::vector<int>& permutation,
                                   const std::vector<std::pair<int, int>>& neighbourhood,
                                   const std::vector<std::vector<int>>& tabuList)
{
    int best_cost = std::numeric_limits<int>::max();
    std::pair<int, int> best_move;

    for (const auto& move : neighbourhood)
    {
        int i = move.first;
        int j = move.second;
        std::vector<int> dummy = permutation;
        std::swap(dummy[i], dummy[j]);
        int cost = calculate_cost(dummy);

        if (cost < best_cost && (tabuList[i][j] == 0 || cost < calculate_cost(permutation)))
        {
            best_cost = cost;
            best_move = move;
        }
    }

    return best_move;
}

// Tabu Search algorithm with aspiration criteria for best solution in the neighborhood
void tabu_aspiration_aspiration_best_in_neighborhood()
{
  read_csv("Distance.csv");
  read_csv("Flow.csv");
  std::vector<int> current_permutation = generate_init_permutation();
  std::vector<int> best_permutation = current_permutation;
  int current_cost = calculate_cost(current_permutation);
  int best_cost = current_cost;

  std::vector<std::vector<int>> tabu_list(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));
  std::vector<std::vector<int>> frequency(NUM_DEPARTMENTS, std::vector<int>(NUM_DEPARTMENTS, 0));

  int iterations = 0;
  while (iterations < 5000)
  {
    std::vector<std::pair<int, int>> neighbourhood = generate_neighbours();
    std::pair<int, int> best_move = find_best_move_aspiration_best_in_neighborhood(current_permutation, neighbourhood, tabu_list);
    do_move(current_permutation, best_move.first, best_move.second);
    int new_cost = calculate_cost(current_permutation);
    tabu_list[best_move.first][best_move.second] = TABU_TENURE;

    if (new_cost < best_cost)
    {
      best_permutation = current_permutation;
      best_cost = new_cost;
    }

    update_tabu_list(tabu_list);
    // Increase frequency for selected move
    ++frequency[best_move.first][best_move.second];
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
  // tabu_freq();
  // tabu_aspiration_best_so_far();
  // tabu_aspiration_best_in_neighborhood();
  return 0;
}

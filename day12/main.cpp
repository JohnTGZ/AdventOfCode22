#include <iostream>
#include <sstream>
#include <fstream>

#include <cmath>

#include <vector>
#include <unordered_set>

#include <queue>
#include <stack>

class Gridmap
{
public:
  Gridmap(const int &height, const int &width)
      : width_(width), height_(height)
  {
    map.resize(height * width);
  }

  void setValue(const int &idx, const char &value)
  {
    map[idx] = value;
  }

  void setValue(std::pair<int, int> xy, const char &value)
  {
    map[xy_to_idx(xy)] = value;
  }

  char getValue(const int &idx)
  {
    return map[idx];
  }

  char getValue(std::pair<int, int> xy) const
  {
    return map[xy_to_idx(xy)];
  }

  std::pair<int, int> idx_to_xy(const int &idx) const
  {
    return std::make_pair((idx % width_), (idx / width_));
  }

  int xy_to_idx(std::pair<int, int> xy) const
  {
    return xy.second * width_ + xy.first;
  }

  void printMap() const
  {
    std::cout << "Printing map\n";
    for (int y = 0; y < height_; y++)
    {
      for (int x = 0; x < width_; x++)
      {
        std::cout << getValue({x, y}) << " ";
      }
      std::cout << "\n";
    }
    std::cout << "\n";
  }

  bool inMap(std::pair<int, int> xy)
  {
    return (xy.first >= 0 && xy.first < width_) && (xy.second >= 0 && xy.second < height_);
  }

  std::vector<int> getNeighbors(const int &idx)
  {
    std::vector<int> neighbors;

    // 4 way connected movement
    const auto cur_pos = idx_to_xy(idx);
    const int x = cur_pos.first;
    const int y = cur_pos.second;

    std::vector<std::pair<int, int>> four_way{
        {x, y - 1}, // UP
        {x, y + 1}, // DOWN
        {x - 1, y}, // LEFT
        {x + 1, y}  // RIGHT
    };

    // Check if neighbour is traversable from current cell
    for (auto nb_pos : four_way)
    {
      int step_diff = int(getValue(nb_pos)) - int(getValue(cur_pos));
      if (inMap(nb_pos) && step_diff < 2)
      {
        neighbors.push_back(xy_to_idx(nb_pos));
      }
    }

    return neighbors;
  }

  double calc_l2_dist(int idx_1, int idx_2)
  {
    auto xy_1 = idx_to_xy(idx_1);
    auto xy_2 = idx_to_xy(idx_2);
    return std::hypot(float(xy_2.first - xy_1.first), float(xy_2.second - xy_1.second));
  }

  // Depth first search, returns number of steps to goal
  std::deque<int> planDFS(const int & start_idx,
  const int & goal_idx)
  {
    std::deque<int> path;

    std::vector<int> parents(height_ * width_, -1);
    std::queue<int> visit_queue;
    std::unordered_set<int> visited;

    visit_queue.push(start_idx);
    visited.insert(start_idx);

    while (!visit_queue.empty())
    {
      auto cur_idx = visit_queue.front();
      // std::cout << "Visiting: " << cur_idx << std::endl;
      visit_queue.pop();
      visited.insert(cur_idx);

      if (cur_idx == goal_idx){
        std::cout << "Got goal!" << std::endl;
        // Trace back the path
        path.push_front(cur_idx);
        while (parents[cur_idx] != -1)
        {
          cur_idx = parents[cur_idx];
          path.push_front(cur_idx);
        }

        return path;
      }

      // Add possible neighbours to visit to the list
      for (auto nb_idx : getNeighbors(cur_idx) )
      {
        if (visited.count(nb_idx) == 0){
          visit_queue.push(nb_idx);
          parents[nb_idx] = cur_idx;
        }
      }
    }

    return path;
  }

  // Depth first search, returns number of steps to goal
  std::deque<int> planDjikstra(const int &start_idx,
               const int &goal_idx)
  {
    std::deque<int> path;
    std::unordered_set<int> visited;

    std::vector<int> parents(height_ * width_, -1);
    std::vector<int> g_cost(height_ * width_, std::numeric_limits<int>::max());

    auto cmp_g_cost = [](std::pair<int,int> cell_1, std::pair<int,int> cell_2){
      return cell_1.second > cell_2.second;
    };

    // Min priority queue that pops out the cell with the minimum g-cost 
    std::priority_queue<std::pair<int, int>, 
      std::vector<std::pair<int, int>>, 
      decltype(cmp_g_cost) > visit_pqueue(cmp_g_cost); 

    g_cost[start_idx] = 0;

    visit_pqueue.push({start_idx, g_cost[start_idx]});

    while (!visit_pqueue.empty())
    {
      auto cur_cell = visit_pqueue.top();
      visit_pqueue.pop();

      if (cur_cell.first == goal_idx)
      {
        std::cout << "Got goal!\n" << std::endl;
        // Trace back the path
        int traceback_cell_idx = cur_cell.first;
        path.push_front(traceback_cell_idx);
        while (parents[traceback_cell_idx] != -1)
        {
          traceback_cell_idx = parents[traceback_cell_idx];
          path.push_front(traceback_cell_idx);
        }
        return path;
      }

      if (visited.count(cur_cell.first) > 0)
      {
        continue;
      }
      visited.insert(cur_cell.first);
      // std::cout << "Visiting: " << cur_cell.first << std::endl;

      // Add possible neighbours to visit to the list
      for (auto nb_idx : getNeighbors(cur_cell.first))
      {
        // std::cout << "  Checking out neighbour " << nb_idx << std::endl;
        // int alt_g_cost = g_cost[cur_cell.first] + int(calc_l2_dist(cur_cell.first, nb_idx));
        int alt_g_cost = g_cost[cur_cell.first] + 1;
        // std::cout << "  alt_g_cost =  " << alt_g_cost << std::endl;
        if (alt_g_cost < g_cost[nb_idx]){
          g_cost[nb_idx] = alt_g_cost;
          parents[nb_idx] = cur_cell.first;
          visit_pqueue.push({nb_idx, g_cost[nb_idx]});
        }
      }
    }

    return path;
  }

private:
  int width_{0};
  int height_{0};
  std::vector<char> map;
};

int main()
{
  // std::string input_path = "../input/day12/test_input.txt";
  // int height = 5, width = 8;

  std::string input_path = "../input/day12/final_input.txt";
  int height = 41, width = 167;

  // Get the input
  std::ifstream fs;
  fs.open(input_path, std::ios::in);

  if (!fs.is_open())
  {
    std::cerr << "unable to open file" << std::endl;
    return 1;
  }

  Gridmap gridmap(height, width);

  /** Part 1 */

  // Find start and goal
  auto start_idx = -1;
  auto goal_idx = -1;
  // Find start and goal
  std::vector<int> start_indices;

  {
    int y = 0;
    for (std::string line; std::getline(fs, line, '\n'); y++)
    {
      for (int x = 0; x < line.size(); x++)
      {
        if (line[x] == 'S')
        {
          gridmap.setValue({x, y}, 'a');
          start_idx = gridmap.xy_to_idx({x, y});
          start_indices.push_back(start_idx);
        }
        else if (line[x] == 'E')
        {
          gridmap.setValue({x, y}, 'z');
          goal_idx = gridmap.xy_to_idx({x, y});
        }
        else
        {
          if (line[x] == 'a')
          {
            start_indices.push_back(gridmap.xy_to_idx({x, y}));
          }
          gridmap.setValue({x, y}, line[x]);
        }
      }
    }
  }

  gridmap.printMap();

  std::cout << "Start: " << start_idx << ", Goal:" << goal_idx << "\n";

  // std::deque<int> path = gridmap.planDFS(start_idx, goal_idx);
  std::deque<int> path = gridmap.planDjikstra(start_idx, goal_idx);

  // // Print path
  // for (int cell_idx : path) {
  //   const auto cell_xy = gridmap.idx_to_xy(cell_idx);
  //   std::cout << "(" << cell_xy.first << ", " << cell_xy.second << ")" ;
  // }
  // std::cout<< std::endl;

  if (path.size() == 0){
    std::cout << "No path found!" << "\n";
    return 1;
  }
  std::cout << "Part 1, no. of steps: " << path.size() -1 << "\n";

  /** Part 2 */

  int min_path_length = std::numeric_limits<int>::max();
  for (auto start : start_indices)
  {
    // std::deque<int> path = gridmap.planDFS(start_idx, goal_idx);
    std::deque<int> path = gridmap.planDjikstra(start, goal_idx);

    if (path.size() > 0) {
      if (min_path_length > path.size()){
        min_path_length = path.size();
      }
    }
  }
  std::cout << "Part 2, no. of steps: " << min_path_length -1 << "\n";

  return 0;
}
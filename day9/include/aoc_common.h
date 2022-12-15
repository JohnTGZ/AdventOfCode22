#include <iostream>
#include <fstream>
#include <sstream>

#include <cmath>
#include <memory>
#include <optional>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include <stdexcept>

enum class LOG_LEVEL { //scoped
    DEBUG,
    INFO,
    WARNING,
    ERROR,
};

class Logger {

public:
    Logger(){}

    Logger(LOG_LEVEL level): 
        current_disp_level(level){
    }

    void log(std::string msg, enum LOG_LEVEL level = LOG_LEVEL::DEBUG){
        std::string color;
        switch (level) {
            case LOG_LEVEL::DEBUG: // BLUE 
                color = "34";
                break;
            case LOG_LEVEL::INFO: // WHITE
                color = "37";
                break;
            case LOG_LEVEL::WARNING: // YELLOW
                color = "33";
                break;
            case LOG_LEVEL::ERROR: // RED
                color = "31";
                break;
            default:
                std::printf("Invalid log level, using Cyan! \n");
                color = "36";
        }

        if (level >= current_disp_level) {
            std::printf("\033[1;" "%s" "m" "%s" "\033[0m\n", color.c_str(), msg.c_str());
        }
    }

    /**
     * @brief Set the display level
     * 
     * @param current_disp_level Current log level to display
     */
    void set_level(const LOG_LEVEL& level) {
        current_disp_level = level;
    }
    
private:
    LOG_LEVEL current_disp_level{LOG_LEVEL::INFO};
};

class FileParser{

public:
    FileParser(){}

    FileParser(const std::string& file_name, std::shared_ptr<Logger> logger)
        : logger(logger)
    {
        ifstrm.open(file_name, std::ios::in);

        if (!ifstrm.is_open()) {
            logger->log((std::ostringstream{} << "Opening file '" << file_name << "' failed.").str(), 
                LOG_LEVEL::ERROR);
        }
        logger->log("Opened file stream", LOG_LEVEL::INFO);

    }

    ~FileParser(){
        logger->log("Closed file stream", LOG_LEVEL::INFO);
        ifstrm.close();
    }

    std::vector<std::string> split(const char delimiter){
        std::vector<std::string> split_contents;
        for (std::string split_elem; std::getline(ifstrm, split_elem, delimiter); ) {
            split_contents.push_back(split_elem);
        }

        return split_contents;
    }

    static std::vector<std::string> split(const std::string& str, const char delimiter) {
        std::vector<std::string> split_contents;

        std::stringstream ss(str);

        for (std::string split_elem; std::getline(ss, split_elem, delimiter); ) {
            split_contents.push_back(split_elem);
        }

        return split_contents;
    }

private:
    std::shared_ptr<Logger> logger{nullptr};
    std::ifstream ifstrm;
};

enum class STATE {
    HEAD, // Head of rope
    TAIL, // Tail of rope
    FREE // Freespace
};

enum DIRECTION {
    NORTH, 
    NORTHEAST, 
    EAST, 
    SOUTHEAST, 
    SOUTH, 
    SOUTHWEST, 
    WEST, 
    NORTHWEST, 
};

struct Position2D {
    Position2D():
        x(0), y(0)
    {}

    Position2D(const long& x, const long& y):
        x(x), y(y)
    {}

    void move(const int& x, const int& y){
        this->x += x;
        this->y += y; 
    }
    
    bool operator == (const Position2D &other) const{
        return (this->x == other.x)
            && (this->y == other.y); 
    } 

    Position2D operator + (const Position2D& other) {
        this->x += other.x;
        this->y += other.y;

        return *this; 
    } 

    Position2D operator += (const Position2D& other) {
        this->x += other.x;
        this->y += other.y;

        return *this; 
    } 

    long x{0};
    long y{0};
};

struct Vector2D {
    Vector2D(const DIRECTION& direction){
        set_direction(direction);
    }

    Vector2D(const std::string& dir_name, const int& magnitude = 1){
        const DIRECTION direction = [&]() {
            if (dir_name.compare("U") == 0){
                return NORTH;
            }
            else if (dir_name.compare("R") == 0) {
                return EAST;
            }
            else if (dir_name.compare("D") == 0) {
                return SOUTH;
            }
            else if (dir_name.compare("L") == 0) {
                return WEST;
            }
            else {
                throw std::invalid_argument("Unknown direction!");
            }
        }();

        set_direction(direction, magnitude);
    }

    Vector2D(const DIRECTION& direction, const int& magnitude){
        set_direction(direction, magnitude);
    }

    Vector2D(const int& x, const int& y): x(x), y(y){}

    void set_direction(const DIRECTION& direction, const int& magnitude = 1) {
        // Y is positive in South direction 
        // X is positive in the East direction
        switch (direction){
            case NORTH:
                x = 0; y = -1;
                break;
            case NORTHEAST:
                x = 1; y = -1;
                break;
            case EAST:
                x = 1; y = 0;
                break;
            case SOUTHEAST:
                x = 1; y = 1;
                break;
            case SOUTH:
                x = 0; y = 1;
                break;
            case SOUTHWEST:
                x = -1; y = 1;
                break;
            case WEST:
                x = -1; y = 0;
                break;
            case NORTHWEST:
                x = -1; y = -1;
                break;
            default:
                break;
        }
        x *= magnitude;
        y *= magnitude;
    }

    // Copy constructor
    Vector2D(const Vector2D& other)
    : Vector2D(other.x, other.y){}

    // Move constructor
    Vector2D(Vector2D&& other) noexcept //Move constructor
    : x(std::exchange(other.x, 0)), y(std::exchange(other.y, 0)) {}

    // Copy assignment 
    Vector2D &operator = (const Vector2D& other) //Copy Assignment
    {
        return *this = Vector2D(other);
    }

    // Move assignment 
    Vector2D &operator = (Vector2D&& other) //Move Assignment
    {
        return *this = std::move(other);
    }

    float magnitude(){
        return std::hypot(x,y);
    }

    Position2D to_position() const {
        return Position2D(x, y);
    }

    int x{0};
    int y{0};
};

std::ostream &operator << (std::ostream& os, const Position2D &pos)
{
    os << "(" << pos.x << "," << pos.y << ")";
    return os; 
}

std::ostream &operator << (std::ostream& os, const Vector2D &vec)
{
    os << "(" << vec.x << "," << vec.y << ")";
    return os; 
}

class GridMap {
public:

    GridMap(long width, long height):
    width_(width), height_(height), size_(width*height), origin_(0, 0) {
        cells_.assign(width_*height_, STATE::FREE);
    }

    /**
     * @brief Initialize gridmap with default starting values
     * 
     * @return true 
     * @return false 
     */
    bool init(const Position2D& origin) {
        this->origin_ = origin;

        // Vist the origin
        this->visit(xy_to_idx(origin_));

        // TODO Make add_knot a lambda
        return this->add_knot(this->origin_, "H", STATE::HEAD)
            && this->add_knot(this->origin_, "T", STATE::TAIL);
    }

    /**
     * @brief Initialize a STATE value at the origin point
     * 
     * @param state 
     * @return true 
     * @return false 
     */
    bool add_knot(const Position2D& position, const std::string& id, const STATE& state){
        id_map_[id] = std::make_pair(position, state);
        return this->assign(id_map_[id].first, id_map_[id].second); 
    }

    bool move_to(const std::string& id, const Position2D& position) {
        this->assign(id_map_[id].first, STATE::FREE); 

        id_map_[id].first = position;

        const long idx = this->xy_to_idx(id_map_[id].first);

        if (idx >= size_ or idx < 0){
            throw std::out_of_range(
                (std::ostringstream{} << "Gridmap index " << idx << " out of range. Size " << size_).str());
            return false;
        }

        this->assign(id_map_[id].first, id_map_[id].second); 
        return true;
    }

    /**
     * @brief Move the current 
     * 
     * @param direction 
     * @param value 
     * @return true 
     * @return false 
     */
    bool move(const std::string& id, const std::string& direction, const int& magnitude) {

        for (int i = 0; i < magnitude; i++){
            // Save previous position of the HEAD
            auto prev_h_pos = (*this->get_cell("H")).first;

            std::cout << "Got previous h position:" << prev_h_pos << "\n";

            move(id, Vector2D(direction));

            // Get direction vector
            auto t_h_vect = Vector2D(
                id_map_["H"].first.x - id_map_["T"].first.x, 
                id_map_["H"].first.y - id_map_["T"].first.y);

            // TODO Check if tail is adjacent to head (includes diagonals)
            if (t_h_vect.magnitude() <= 1.42){
                // std::cout << "Tail and Head is adjacent \n";
                continue;
            }
            std::cout << "Moving T to " << prev_h_pos << "\n";

            this->visit(xy_to_idx(prev_h_pos));
            // Move the tail to the previous position of H
            move_to("T", prev_h_pos);
        }
        return true;
    }

    /**
     * @brief Move the cell of selected id in the direction of the vector
     * 
     * @param id 
     * @param dir_vect 
     * @return true 
     * @return false 
     */
    bool move(const std::string& id, const Vector2D& dir_vect) {
        this->assign(id_map_[id].first, STATE::FREE); 

        id_map_[id].first.move(dir_vect.x, dir_vect.y);
        
        const long idx = this->xy_to_idx(id_map_[id].first);

        if (idx >= size_ or idx < 0){
            throw std::out_of_range(
                (std::ostringstream{} << "Gridmap index " << idx << " out of range. Size " << size_).str());
            return false;
        }

        this->assign(id_map_[id].first, id_map_[id].second); 

        return true;
    }

    /**
     * @brief Display the current state of the gridmap
     * 
     * @param logger 
     */
    void display(std::shared_ptr<Logger> logger) const{
        std::ostringstream ss;

        for (long i = 0; i < height_; i++){
            ss.str(std::string());
            for (long j = 0; j < width_; j++){
                switch (cells_[i * width_ + j]) {
                    case STATE::HEAD:
                        ss << "H";
                        break;
                    case STATE::TAIL:
                        ss << "T";
                        break;
                    case STATE::FREE:
                        ss << ".";
                        break;
                    default:
                        ss << "E";
                }
            }
            logger->log(ss.str());
        }

    }

    long xy_to_idx(const Position2D& position){
        return position.y * this->width_ + position.x; 
    }

    Position2D idx_to_xy(const long& idx, long& x, long& y){
        return Position2D(
            idx % this->width_,
            floor(idx / this->width_)
        );
    }

    std::optional<std::pair<Position2D, STATE>> get_cell(const std::string& id){
        if (id_map_.find(id) != id_map_.end()){
            return id_map_[id];
        }
        return {};
    }

    std::size_t get_num_visited(){
        return visited_.size();
    }

private:
    bool visit(const long& idx){
        auto p = visited_.insert(idx);
        return p.second;
    }

    bool assign(const Position2D& position, const STATE& state) {
        const long idx = this->xy_to_idx(position);

        if (idx >= size_ or idx < 0)
            return false;

        cells_[idx] = state;
        return true;
    }

private:
    std::vector<STATE> cells_;
    long width_{0};
    long height_{0};
    long size_{0};
    // Oorigin is at bottom left of the grid map
    Position2D origin_;

    std::unordered_map<std::string, std::pair<Position2D, STATE>> id_map_;

    std::unordered_set<long> visited_;

};


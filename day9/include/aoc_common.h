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

template <typename T> int sgn(T val) {
    return (T(0) < val) - (val < T(0));
}

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

class Vector2D {
public:

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

    Vector2D(const int& x, const int& y): x_(x), y_(y){}

    // Copy constructor
    Vector2D(const Vector2D& other)
    : Vector2D(other.x(), other.y()){}

    // Copy assignment 
    Vector2D &operator = (const Vector2D& other) //Copy Assignment
    {
        if (this != &other){
            this->x_ = other.x();
            this->y_ = other.y();
        }

        return *this;
    }

    bool operator == (const Vector2D &other) const{
        return (this->x_ == other.x())
            && (this->y_ == other.y()); 
    } 

    Vector2D& operator += (const Vector2D& other) {
        this->x_ += other.x();
        this->y_ += other.y();

        return *this; 
    } 

    Vector2D& operator -= (const Vector2D& other) {
        this->x_ -= other.x();
        this->y_ -= other.y();

        return *this; 
    } 

    void move(const int& x, const int& y){
        this->x_ += x;
        this->y_ += y; 
    }

    void set_direction(const DIRECTION& direction, const int& magnitude = 1) {
        // Y is positive in South direction 
        // X is positive in the East direction
        switch (direction){
            case NORTH:
                x_ = 0; y_ = -1;
                break;
            case NORTHEAST:
                x_ = 1; y_ = -1;
                break;
            case EAST:
                x_ = 1; y_ = 0;
                break;
            case SOUTHEAST:
                x_ = 1; y_ = 1;
                break;
            case SOUTH:
                x_ = 0; y_ = 1;
                break;
            case SOUTHWEST:
                x_ = -1; y_ = 1;
                break;
            case WEST:
                x_ = -1; y_ = 0;
                break;
            case NORTHWEST:
                x_ = -1; y_ = -1;
                break;
            default:
                break;
        }
        x_ *= magnitude;
        y_ *= magnitude;
    }

    float magnitude() const{
        return std::hypot(x_,y_);
    }

    Vector2D discretized_unit() const {
        int x_unit, y_unit = 0;
        x_unit = this->x_ != 0 ? int(std::copysign(1, this->x_)) : 0;
        y_unit = this->y_ != 0 ? int(std::copysign(1, this->y_)) : 0;

        return Vector2D(x_unit, y_unit);
    }

    int x() const {
        return this->x_;
    }

    int y() const {
        return this->y_;
    }

private:
    int x_{0};
    int y_{0};
};

Vector2D operator - (const Vector2D& lhs, const Vector2D& rhs)
{
    return Vector2D(
        lhs.x() - rhs.x(),
        lhs.y() - rhs.y()
    );
}

using Position2D = Vector2D;

std::ostream &operator << (std::ostream& os, const Vector2D &vec)
{
    os << "(" << vec.x() << "," << vec.y() << ")";
    return os; 
}

struct Cell {
    Cell(std::shared_ptr<Cell> parent_cell, Position2D position, STATE state)
    : parent_cell(parent_cell), position(position), state(state) {}

    std::shared_ptr<Cell> parent_cell; // ID of cell to follow
    Position2D position;
    STATE state;
};

class GridMap {
public:

    GridMap(long width, long height):
    width_(width), height_(height), size_(width*height), origin_(0, 0) {
        // cells_.assign(width_*height_, STATE::FREE);
    }

    /**
     * @brief Initialize gridmap with default starting values
     * 
     * @return true 
     * @return false 
     */
    bool init(const Position2D& origin, const unsigned int& num_tail) {
        this->origin_ = origin;

        // Vist the origin
        this->visit(xy_to_idx(origin_));

        auto add_cell = [&](
            const std::string& id, 
            const std::string& parent_id,
            const Position2D& pos, 
            const STATE& state
            ) {
            auto parent_cell = this->get_cell(id);
            id_map_[id] = std::make_shared<Cell>(parent_cell, pos, state);
        };

        // Add head
        add_cell("C0", "", this->origin_, STATE::HEAD);

        // Add tails
        for (unsigned int i = 1; i < num_tail+1; i++)
        {
            add_cell("C"+std::to_string(i), "C"+std::to_string(i-1), this->origin_, STATE::TAIL);
        }

        print_map();

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
    bool move_all_cells(const std::string& direction, const int& magnitude, bool part_2 = false) {
        for (int i = 0; i < magnitude; i++){
            std::cout << "Magnitude " << i << "\n";
            
            move_in_dir(this->get_cell("C0"), Vector2D(direction));

            for (int i = 0; i < id_map_.size()-1; i++) {
                std::cout << "==========\n";

                const std::string& parent_id = "C"+std::to_string(i);
                const std::string& child_id = "C"+std::to_string(i+1);

                auto parent_cell = this->get_cell(parent_id);
                auto child_cell = this->get_cell(child_id);

                // IF
                // 1. Head is 2 stesps directly up, down, left or right from the tail, tail 
                //      must also move one step in that direction so it remains close enough.
                // 2. If head and tail aren't touching and aren't in the same row or column, 
                //      the tail always moves one step diagonally to keep up.

                const auto& t_h_vect = parent_cell->position - child_cell->position;

                // std::cout << "  MAGNITUDE: " << t_h_vect.magnitude() << "\n";
                // std::cout << "  TH Vect: "<< t_h_vect << "\n";
                // std::cout << "  TH Discrete: "<< t_h_vect.discretized_unit() << "\n";

                // IF child and parent cell not in the same row or column
                // tail moves one step diagonally to keep up.
                if (child_cell->position.x() != parent_cell->position.x() 
                    && child_cell->position.y() != parent_cell->position.y())
                {
                    if (! (t_h_vect.magnitude() <= 1.42) ){
                        // Move diagonally in unit x and y direction of vector
                        child_cell->position += t_h_vect.discretized_unit();
                        std::cout << "Moved " << child_id << " to " << child_cell->position << "\n";
                    }
                }
                // Child and parent in the same row and column, move one one step in that direction
                else {
                    if (! (t_h_vect.magnitude() <= 1.42) ){
                        // Move one step in the direction of the parent_cell

                        child_cell->position += t_h_vect.discretized_unit();

                        std::cout << "Moved " << child_id << " to " << child_cell->position << "\n";
                    }
                }

                if (part_2){
                    if (child_id.compare("C9") == 0){
                        this->visit(xy_to_idx(child_cell->position));
                    }
                }
                else {
                    this->visit(xy_to_idx(child_cell->position));

                }

            }
            std::cout << "==========\n";

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
    bool move_in_dir(std::shared_ptr<Cell> cell, const Vector2D& dir_vect) {
        cell->position.move(dir_vect.x(), dir_vect.y());
        
        const long idx = this->xy_to_idx(cell->position);

        if (idx >= size_ or idx < 0){
            throw std::out_of_range(
                (std::ostringstream{} << "Gridmap index " << idx << " out of range. Size " << size_).str());
            return false;
        }

        return true;
    }

    bool move_to_pos(std::shared_ptr<Cell> cell, const Position2D& position) {
        cell->position = position;

        const long idx = this->xy_to_idx(cell->position);

        if (idx >= size_ or idx < 0){
            throw std::out_of_range(
                (std::ostringstream{} << "Gridmap index " << idx << " out of range. Size " << size_).str());
            return false;
        }

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
        return position.y() * this->width_ + position.x(); 
    }

    Position2D idx_to_xy(const long& idx, long& x, long& y){
        return Position2D(
            idx % this->width_,
            floor(idx / this->width_)
        );
    }

    std::shared_ptr<Cell> get_cell(const std::string& id){
        if (id_map_.find(id) != id_map_.end()){
            return id_map_[id];
        }
        return nullptr;
    }

    std::size_t get_num_visited(){
        return visited_.size();
    }

    void print_map() {
        std::cout << "Printing map \n";
        for (auto cell : id_map_ ){
            std::cout << "ID: " << cell.first << ", Cell_position: " << cell.second->position << "\n";
        }
    }

private:
    bool visit(const long& idx){
        auto p = visited_.insert(idx);
        return p.second;
    }

    bool assign(const Vector2D& position, const STATE& state) {
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

    std::unordered_map<std::string, std::shared_ptr<Cell>> id_map_;

    std::unordered_set<long> visited_;
};

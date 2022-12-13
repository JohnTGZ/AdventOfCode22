#include <iostream>
#include <fstream>
#include <sstream>

#include <memory>
#include <vector>

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

    std::vector<std::string> split(const char* delimiter){
        std::vector<std::string> split_contents;
        for (std::string split_elem; std::getline(ifstrm, split_elem, '\n'); ) {
            std::cout << split_elem << std::endl;
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

enum class DIRECTION {
    UP, 
    DOWN, 
    LEFT, 
    RIGHT 
};


class GridMap {
public:
    GridMap(long width, long height):
    width(width), height(height), size(width*height){
        cells.assign(width*height, STATE::FREE);
    }

    bool move_head(const DIRECTION& direction, const int& value) {
        switch (direction) {
            case DIRECTION::UP:
                break;
            case DIRECTION::DOWN:
                break;
            case DIRECTION::LEFT:
                break;
            case DIRECTION::RIGHT:
                break;
            default:
                break;
        }
    }

    // TODO 
    void xy_to_idx(const long& x, const long& y, long& idx){
        
    }

    // TODO 
    void idx_to_xy(const long& idx, long& x, long& y){
        
    }

    bool assign(const long& idx, const STATE& state) {
        if (idx >= size)
            return false;

        cells[idx] = state;
        return true;
    }

    void display(std::shared_ptr<Logger> logger) {
        std::ostringstream ss;

        for (long i = 0; i < height; i++){
            ss.str(std::string());
            for (long j = 0; j < width; j++){
                switch (cells[i * width + j]) {
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

private:
    std::vector<STATE> cells;
    long width;
    long height;
    long size;
};


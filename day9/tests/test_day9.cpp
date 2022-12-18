#include <gtest/gtest.h>
#include <aoc_common.h>
#include <memory>

class GridMapTest : public ::testing::Test {
    protected:
        void SetUp() override {
            logger_ = std::make_shared<Logger>(LOG_LEVEL::DEBUG);
            fileparser_ = std::make_unique<FileParser>("../../input/day9/test_input.txt", logger_);
            lines_ = fileparser_->split('\n');

            gridmap_ = std::make_unique<GridMap>(6,5);
            gridmap_->init(Position2D(0,4), 1);
        }

    void TearDown() override {
        logger_.reset();
        gridmap_.reset();
        fileparser_.reset();
        lines_.clear();
    }

    std::shared_ptr<Logger> logger_;
    std::unique_ptr<GridMap> gridmap_;
    std::unique_ptr<FileParser> fileparser_;
    std::vector<std::string> lines_;
};

TEST_F(GridMapTest, XYConversion) {
    ASSERT_EQ(gridmap_->xy_to_idx(Position2D(0,0)), 0);
    ASSERT_EQ(gridmap_->xy_to_idx(Position2D(5,0)), 5);
    ASSERT_EQ(gridmap_->xy_to_idx(Position2D(0,1)), 6);
    ASSERT_EQ(gridmap_->xy_to_idx(Position2D(1,3)), 19);
}

TEST_F(GridMapTest, MoveHeadIndividual) {

    gridmap_->move("C0", "R", 3);
    ASSERT_EQ(gridmap_->get_cell("C0")->position, Position2D(3, 4))
        << "Got " << gridmap_->get_cell("C0")->position << " against " << Position2D(3, 4);

    gridmap_->move("C0", "U", 3);
    ASSERT_EQ(gridmap_->get_cell("C0")->position, Position2D(3, 1));

    gridmap_->move("C0", "L", 1);
    ASSERT_EQ(gridmap_->get_cell("C0")->position, Position2D(2, 1));

    gridmap_->move("C0", "D", 3);
    ASSERT_EQ(gridmap_->get_cell("C0")->position, Position2D(2, 4));

}

TEST_F(GridMapTest, MoveHeadTestInput) {
    for (int i = 0; i < lines_.size(); i++){
        const auto inst = FileParser::split(lines_[i], ' ');
        gridmap_->move("C0", inst[0], std::stoi(inst[1].c_str()) );
    }
    ASSERT_EQ(gridmap_->get_cell("C0")->position, Position2D(2, 2));
}

TEST_F(GridMapTest, MoveTailIndividual) {
    gridmap_->move("C0", "R", 4);
    ASSERT_EQ(gridmap_->get_cell("C1")->position, Position2D(3, 4));

    gridmap_->move("C0", "U", 4);
    ASSERT_EQ(gridmap_->get_cell("C1")->position, Position2D(4, 1));

    gridmap_->move("C0", "L", 3);
    ASSERT_EQ(gridmap_->get_cell("C1")->position, Position2D(2, 0));

    gridmap_->move("C0", "D", 1);
    ASSERT_EQ(gridmap_->get_cell("C1")->position, Position2D(2, 0));
}

TEST_F(GridMapTest, MoveTailTestInput) {
    for (int i = 0; i < lines_.size(); i++){
        const auto inst = FileParser::split(lines_[i], ' ');
        gridmap_->move(
            "C0", inst[0], std::stoi(inst[1].c_str()));
    }
    ASSERT_EQ(gridmap_->get_cell("C1")->position, Position2D(1, 2));
}

TEST_F(GridMapTest, NumVisited) {
    for (int i = 0; i < lines_.size(); i++){
        const auto inst = FileParser::split(lines_[i], ' ');
        gridmap_->move(
            "C0", inst[0], std::stoi(inst[1].c_str()));
    }

    ASSERT_EQ(gridmap_->get_num_visited(), 13);
}


class GridMapTestPart2 : public ::testing::Test {
    protected:
        void SetUp() override {
            logger_ = std::make_shared<Logger>(LOG_LEVEL::DEBUG);
            fileparser_ = std::make_unique<FileParser>("../../input/day9/test_input2.txt", logger_);
            lines_ = fileparser_->split('\n');

            gridmap_ = std::make_unique<GridMap>(26,21);
            gridmap_->init(Position2D(11,15), 1);
        }

    void TearDown() override {
        logger_.reset();
        gridmap_.reset();
        fileparser_.reset();
        lines_.clear();
    }

    std::shared_ptr<Logger> logger_;
    std::unique_ptr<GridMap> gridmap_;
    std::unique_ptr<FileParser> fileparser_;
    std::vector<std::string> lines_;
};

TEST_F(GridMapTestPart2, MoveTailIndividual) {
    
}

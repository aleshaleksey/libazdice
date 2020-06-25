
struct Rolls {
    uint64_t len_input;
    const char *input;
    uint64_t len_dice_groups;
    const int64_t *groups;
    int64_t bonus;
    int64_t total;
};


struct ListRolls {
    uint64_t len;
    const struct Rolls *results;
};

struct XY {
    int64_t value;
    uint64_t frequency;
};

struct Distribution {
    struct XY *rolls_and_frequencies;
    uint64_t count;
    uint64_t len_input;
    char *input;
};

struct DistributionResult {
    struct Distribution *succ;
    char *err;
};

struct ResultListRolls {
    const struct ListRolls *succ;
		const char *err;
};

struct SingleRollResult {
    const int64_t roll;
    const char *err;
};

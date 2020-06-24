// NB this is a test that relies on "libhello_from_c.so" having been compiled from
// "hello_from_c.rs" using "rustc hello_from_c.rs crate_type=cdylib".
// Then this file is compiled using "gcc rust_caller.c -ldl -o rc" (or whatever name you want.)

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <dlfcn.h>


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

struct ResultListRolls {
    const struct ListRolls *succ;
		const char *err;
};

struct SingleRollResult {
    const int64_t roll;
    const char *err;
};

int main(int argc, char** argv)
{
    // Prepare variables and write a bunch of blurb.
    char *input = "5d6dl2mn2";
    uint64_t l = strlen(input);
    uint64_t n = 50;

    // Load the sensibly named "libazdice.so".
    void *lib;
    lib = dlopen("../target/release/libazdice.so",RTLD_LAZY);

    if (lib != NULL) {
        // Initialise the function pointers.
        struct ResultListRolls (*parse_n)(char **, uint64_t, uint64_t);
        struct SingleRollResult (*parse)(char **);
        int64_t (*parse2)(char **);

        // Load functions.
        *(void **)(&parse_n) = dlsym(lib,"parse_and_roll_n_times");
        *(void **)(&parse) = dlsym(lib,"parse_and_roll");
        *(void **)(&parse2) = dlsym(lib,"parse_and_roll2");

        // Run a single roll function.
        long int i = parse2(&input);
        printf("Parse test Result of one roll of %s: %ld\n", input, i);

        // Run fifty rolls and return the total.
        struct SingleRollResult parse_res = parse(&input);
        if (parse_res.err != NULL) {
            printf("We returned with an error: %s\n", parse_res.err);
        } else {
            printf("Rolled \"%s\" once and got %ld!\n", input, parse_res.roll);
        }

        // Roll fifty rolls and return details.
        struct ResultListRolls parse_res_n = parse_n(&input, l, n);
        if (parse_res_n.err != NULL) {
            printf("We returned with an error: %s\n", parse_res_n.err);
        } else {
            printf("We have rolled \"%s\", %lu times and got the following rolls:", input, n);
            int64_t total = 0;
            printf("[ ");
            for (uint64_t i=0; i<parse_res_n.succ -> len; i++) {
            	total += parse_res_n.succ -> results[i]. total;

            	printf("%ld",parse_res_n.succ -> results[i]. total);
                if (i+1==parse_res_n.succ -> len) {
                    printf(" ]\n");
                } else {
                    printf(", ");
                }
            }
            printf("Total = %ld\n", total);
        }
    } else {
        printf("Could not load liblibazdice!\n\n");
        exit(1);
    }

    exit(0);
}

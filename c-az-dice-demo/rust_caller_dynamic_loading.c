// NB this is a test that relies on "libhello_from_c.so" having been compiled from
// "hello_from_c.rs" using "rustc hello_from_c.rs crate_type=cdylib".
// Then this file is compiled using "gcc rust_caller.c -ldl -o rc" (or whatever name you want.)

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <dlfcn.h>
#include "../include/libazdice.h"

int main(int argc, char** argv)
{
    // Prepare variables and write a bunch of blurb.
    char *input = "5d6dl2mn2";
    uint64_t l = strlen(input);
    uint64_t n = 50;
    uint64_t n_dist = 50000000;

    // Load the sensibly named "liblibazdice.so". Gosh what a naming sense!
    void *lib;
    lib = dlopen("../target/release/liblibazdice.so",RTLD_LAZY);

    if (lib != NULL) {
        // Initialise the function pointers.
        struct ResultListRolls (*parse_n)(char **, uint64_t, uint64_t);
        struct SingleRollResult (*parse)(char **);
        int64_t (*parse2)(char **);
        struct DistributionResult (*parse_distribution)(char **, uint64_t, uint64_t);

        // Load functions.
        *(void **)(&parse_n) = dlsym(lib,"parse_and_roll_n_times");
        *(void **)(&parse) = dlsym(lib,"parse_and_roll");
        *(void **)(&parse2) = dlsym(lib,"parse_and_roll2");
        *(void **)(&parse_distribution) = dlsym(lib,"parse_and_generate_distribution");

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
            printf("We returned with an error from \"parse_and_roll_n_times\": %s\n", parse_res_n.err);
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

        struct DistributionResult dist_res = parse_distribution(&input, l, n_dist);
        if (dist_res.err != NULL) {
            printf("We returned with an error from \"parse_and_generate_distribution\": %s\n", parse_res_n.err);
        } else {
            printf(
                "We have made a distribution with %lu repeats from \"%s\".\n",
                n_dist,
                dist_res.succ -> input
            )
            ;
            printf("Value   | Frequency\n");
            for (i=0; i<dist_res.succ -> count; i++) {
                printf(
                    "%ld\t| %lf\n",
                    dist_res.succ -> rolls_and_frequencies[i].value,
                    100.0 * (double) dist_res.succ -> rolls_and_frequencies[i].frequency / (double) n_dist
                );
            }
        }
    } else {
        printf("Could not load liblibazdice!\n\n");
        exit(1);
    }

    exit(0);
}

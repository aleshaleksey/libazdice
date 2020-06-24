// NB this is a test that relies on "libhello_from_c.so" having been compiled from
// "hello_from_c.rs" using "rustc hello_from_c.rs crate_type=cdylib".
// Then this file is compiled using "gcc rust_caller.c -ldl -o rc" (or whatever name you want.)



#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
// #include "../src/liblibazdice.h"

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

extern struct SingleRollResult parse_and_roll(char *);
extern struct ResultListRolls parse_and_roll_n_times(char *, uint64_t, uint64_t);
extern int64_t parse_and_roll2(char *);

int main(int argc, char** argv)
{

  int64_t parse_res2;

	// Prepare variables and write a bunch of blurb.
	char *input = "5d6dl2mn2";
	uint64_t l = strlen(input);
	uint64_t n = 50;
	printf("We will attempt to parse \"%s\" from C! Hold onto your hats.\n",input);
	printf("We will then roll the dice %lu times\n", n);

		// printf("Test = %s\n", test(30));
    int64_t i = parse_and_roll2(input);
		printf("parse test %s\n", i);

		// exit(0);
		struct SingleRollResult parse_res = parse_and_roll(input);
    printf("Rolled %s and got %ld!\n", input, parse_res.roll);

		struct ResultListRolls parse_res_n = parse_and_roll_n_times(input, l, n);


		printf("input pointer: %u\n", input);
		printf("We have returned from the rust side.\n");

		if (parse_res_n.err != NULL) {
			printf("We returned with an error: %s\n", parse_res_n.err);
		} else {
			printf("We have the following rolls");
			int64_t total = 0;
			for (uint64_t i=0; i<parse_res_n.succ -> len; i++) {
				total += parse_res_n.succ -> results -> total;
				printf("%ld\n",parse_res_n.succ -> results -> total);
			}
			printf("Total = %ld", total);
		}

	exit(0);
}

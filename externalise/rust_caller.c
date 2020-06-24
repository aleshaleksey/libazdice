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
	printf("size of ResultListRolls: %lu\n",sizeof(struct ResultListRolls));
	printf("size of ListRolls: %lu\n",sizeof(struct ListRolls));
	printf("size of Rolls: %lu\n",sizeof(struct Rolls));

	// struct ResultListRolls parse_res_n;
	// struct SingleRollResult parse_res;

	// Prepare variables and write a bunch of blurb.
	char *input = "5d6dl2mn2";
	uint64_t l = strlen(input);
	uint64_t n = 50;
	printf("We will attempt to parse \"%s\" from C! Hold onto your hats.\n",input);
	printf("We will then roll the dice %lu times\n", n);

	// Load the stupidly named "liblibazdice.so".
	// Who names libraries like that?
	void *lib;
	lib = dlopen("../target/release/libazdice.so",RTLD_LAZY);

	//
	if (lib != NULL) {
		struct ResultListRolls (*parse_n)(char *, uint64_t, uint64_t);
		struct SingleRollResult (*parse)(char *);
		int64_t (*parse2)(char *);
		char *(*test)(int64_t);
    int64_t (*test2)(char *);
		*(void **)(&parse_n) = dlsym(lib,"parse_and_roll_n_times");
		*(void **)(&parse) = dlsym(lib,"parse_and_roll");
		*(void **)(&parse2) = dlsym(lib,"parse_and_roll2");
    *(void **)(&test) = dlsym(lib,"test");
		*(void **)(&test2) = dlsym(lib,"test2");

		printf("Test = %s\n", test((int64_t) l));
    printf("Test2 = %ld\n", test2(input));
    long int i = parse2(input);
		printf("Parse test %ld\n", i);

		// exit(0);
	  // struct SingleRollResult parse_res = parse(input);
    // printf("Rolled %s and got %ld!\n", input, parse_res.roll);

		struct ResultListRolls parse_res_n = parse_n(input, l, n);

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
	} else {
		printf("Could not load liblibazdice!\n\n");
		exit(1);
	}
	exit(0);
}

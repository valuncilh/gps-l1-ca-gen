/*
 * Generator pseudonoise sequence C/A code for GPS L1, SV ID 2
 * adv: vladimir_v, 05.09, r0001
 * description:
 * 	Two 10-bit LFSR (G1 & G2)
 * 	polynomial -> back_link {G1: 3, 7, G2: 2, 3, 6, 8, 9, 10}
 * */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define COUNT 10
#define B_LINK_1 2
#define B_LINK_2 6

typedef struct {
	uint8_t vec[COUNT];
} G_;

void init_g(uint8_t vec[]) {
	for(int i = 0; i < COUNT; ++i){
		vec[i] = 1;
	}
}

void spin(uint8_t vec[]) {
	for(int i = COUNT - 1; i >= 1; --i) {
		vec[i] = vec[i - 1];
	}
}

uint8_t tick(uint8_t vec_1[], uint8_t vec_2[]) { // one step of iteration
	uint8_t outG1 = vec_1[9];

	uint8_t bit = outG1 ^ (vec_2[B_LINK_1] ^ vec_2[B_LINK_2]);
	
	uint8_t newG1 = vec_1[2] ^ vec_1[9];
	uint8_t newG2 = vec_2[1] ^ vec_2[2] ^ vec_2[5] ^ vec_2[7] ^ vec_2[8] ^ vec_2[9];

	spin(vec_1); spin(vec_2);

	vec_1[0] = newG1;
	vec_2[0] = newG2;

	return bit;
}

int main() {

	G_ G1, G2; init_g(G1.vec); init_g(G2.vec);

	uint8_t buf[1023];

	for(int i = 0; i < 1023; ++i){
		buf[i] = tick(G1.vec, G2.vec);
	}

	uint8_t buf2[1023];

	for(int i = 0; i < 1023; ++i){
		buf2[i] = tick(G1.vec, G2.vec);
	}

	for(int i = 0; i < 1023; ++i){
		if(buf[i] != buf2[i]) printf("%d\n", i);
	}

	for(int i = 0; i < 10; ++i){
		printf("%d", buf2[i]);
	}

	return 0;
}

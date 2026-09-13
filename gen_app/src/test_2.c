#include "lfsr.h"

int main(int argc, const char* argv[]) {

	uint8_t buf[1024];

	LFSR(buf);

	for(int i = 0; i < 10; ++i) {
		printf("%d ", buf[i]);
	}

	printf("\n\n");

	for(int i = 1023 - 10; i < 1024; ++i) {
		printf("%d ", buf[i]);
	}

	return 0;
}

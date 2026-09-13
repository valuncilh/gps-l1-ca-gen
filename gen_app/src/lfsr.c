#include "lfsr.h"

void LFSR(uint8_t buf[PERIOD]) {

	GEN G1, G2;

	init_g(G1.bits); init_g(G2.bits); // initial gen: fill all bits '1'
	for(int i = 0; i < PERIOD; ++i) {	
		uint8_t outG1 = G1.bits[9];
		uint8_t bit = outG1 ^ ( G2.bits[BACK_LINK_1] ^ G2.bits[BACK_LINK_2]);
		
		uint8_t newG1 = G1.bits[2] ^ G1.bits[9];
		uint8_t newG2 = G2.bits[1] ^ G2.bits[2] ^ G2.bits[5] ^ G2.bits[7] ^ G2.bits[8] ^ G2.bits[9];
		
		spin(G1.bits); spin(G2.bits);

		G1.bits[0] = newG1;
		G2.bits[0] = newG2;

		buf[i] = bit;
	}
}

void spin(uint8_t bits[]){
	for(int i = COUNT - 1; i >= 1; --i){
		bits[i] = bits[i - 1];
	}
}

void init_g(uint8_t bits[]) {
	for(int i = 0; i < COUNT; ++i){
		bits[i] = 1;
	}
}

bool chek_period(uint8_t fperiod[], uint8_t speriod[]) {
	for(int i = 0; i < PERIOD; ++i){
		if(fperiod[i] != speriod[i]) return 0;
	}
	return 1;
}

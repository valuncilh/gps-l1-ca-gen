#ifndef LFSR_H
#define LFSR_H

#define COUNT  10
#define PERIOD 1023
#define BACK_LINK_1  2
#define BACK_LINK_2  6

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h> 

typedef struct {
	uint8_t bits[COUNT];
} GEN;

void LFSR(uint8_t buf[PERIOD]);

void spin(uint8_t bits[]);

void init_g(uint8_t bits[]);

bool chek_period(uint8_t fperiod[], uint8_t speriod[]);

#endif

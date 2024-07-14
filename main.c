/*
  A interperter for the Brainfuck programing laguage.
  Copyright (C) 2023 Imran Mustafa

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "stack.h"

#define SIZE 30000

char mem[SIZE] = {0};
int addr = 0;

bool disable_exec = false;
int balance_counter = 0;

int exec(char instruction);
void reset_tape();

int main(const int argc, char *const argv[]) {
  if (1 == argc) {
    printf("Hopefully a future interactive mode will be here.");
  } else if (strcmp(argv[1], "-c") == 0) {
    for (int ii = 2; ii <= argc; ii++) {
      printf("Hopefully a future release will add the complier.");

      return EXIT_SUCCESS;

      FILE *const fptr = fopen(argv[ii], "r");
      char instruction = EOF;

      char *translation = "char mem[30000] = {0};int addr = 0;";

      if (NULL == fptr) {
        perror(argv[ii]);
        exit(EXIT_FAILURE);
      }
      
      while (EOF != (instruction = fgetc(fptr))) {
        switch (instruction) {
        case '>':
          strcat(translation, "addr++;");
          break;
        case '<':
          strcat(translation, "addr--;");
          break;
        case '+':
          strcat(translation, "mem[addr]++;");
          break;
        case '-':
          strcat(translation, "mem[addr]--;");
          break;
        case '.':
          strcat(translation, "printf(\"%c\", mem[addr])");
          break;
        case ',':
          strcat(translation, "");
          break;
        case '[':
          strcat(translation, "while(addr){");
          break;
        case ']':
          strcat(translation, "}");
          break;
        }
      }

      printf("%s", translation);
    }
  } else {
    for (int ii = 1; ii < argc; ii++) {
      FILE *const fptr = fopen(argv[ii], "r");
      char instruction = EOF;
      int code;

      if (NULL == fptr) {
        perror(argv[ii]);
        exit(EXIT_FAILURE);
      }

      while (EOF != (instruction = fgetc(fptr))) {
        code = exec(instruction);

        switch (code) {
        case 1:
          push(ftell(fptr));
          break;
        case 2:
          fseek(fptr, top(), SEEK_SET);
          break;
        case 3:
          pop();
          break;
        }
      }
      if (-1 != top()) {
        exit(EXIT_FAILURE);
      }

      reset_tape();

      fclose(fptr);
    }
  }

  return EXIT_SUCCESS;
}

int exec(char instruction) {
  int code = 0;

  if (disable_exec) {
    switch (instruction) {
    case '[':
      balance_counter++;
      break;
    case ']':
      if (0 < balance_counter) {
        balance_counter--;
      } else {
        disable_exec = false;
      }
      break;
    }
    return code;
  }

  switch (instruction) {
  case '>':
    if (SIZE - 1 == addr) {
      exit(EXIT_FAILURE);
    }
    addr++;
    break;
  case '<':
    if (0 == addr) {
      exit(EXIT_FAILURE);
    }
    addr--;
    break;
  case '+':
    mem[addr]++;
    break;
  case '-':
    mem[addr]--;
    break;
  case '.':
    printf("%c", mem[addr]);
    break;
  case ',': {
    char input;

    printf("\ninput: ");
    scanf("%c", &input);
    mem[addr] = input;
    break;
  }
  case '[':
    if (0 == mem[addr]) {
      disable_exec = true;
    } else {
      code = 1;
    }
    break;
  case ']':
    if (-1 == top()) {
      exit(EXIT_FAILURE);
    }
    code = 0 != mem[addr] ? 2 : 3;
    break;
  }
  return code;
}

void reset_tape() {
  memset(mem, 0, sizeof mem);
  addr = 0;
}

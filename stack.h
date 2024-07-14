#pragma once

#include <stdbool.h>

typedef struct NODE {
  struct NODE *next;
  long data;
} node;

void push(long point);
void pop();

long top();

bool is_empty();

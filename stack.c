#include "stack.h"

#include <stdbool.h>
#include <stdlib.h>

node *head = NULL;

void push(long point) {
  node *new = (node *)calloc(1, sizeof(node));

  new->next = head;
  new->data = point;

  head = new;
}

void pop() {
  if (!is_empty()) {
    node *temp = head;
    head = head->next;
    free(temp);
  }
}

long top() { return is_empty() ? -1 : head->data; }

bool is_empty() { return NULL == head ? true : false; }

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  FILE *fp = fopen("input.txt", "r");
  int i = 0;
  char line[1000];
  char ch;
  int left = -1, right = -1, sum = 0;

  if (fp == NULL) {
    printf("Error opening file.");
    exit(EXIT_FAILURE);
  }

  while (fgets(line, sizeof(line), fp)) {
    for (int i = 0; i < 100; i++) {
      if (line[i] >= '0' && line[i] <= '9' && left == -1) {
        left = line[i] - '0';
      }
      if (line[i] >= '0' && line[i] <= '9') {
          right = line[i] - '0';
      }
    }

    sum += (left * 10) + right;

    memset(line, 0, sizeof(line));
    left = -1;
    right = -1;
  }

  printf("%d\n", sum);

  fclose(fp);

  return 0;
}

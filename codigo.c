#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

int main() {
    int soma = 0;
    int i = 1;
    while((i <= 1000000)) {
            soma += i;
            i += 1;
    }
    printf("%d\n", soma);
    return 0;
}

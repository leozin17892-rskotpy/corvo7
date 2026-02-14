#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

int add(int a, int b) {
    return (a + b);
}
int main() {
    int i = 0;
    int money = 0;
    bool trabalhou = true;
    while((trabalhou)) {
        printf("%d\n", i);
        printf("%d\n", money);
            i += 1;
            money += 20;
        if (i > 6){
            trabalhou = false;
        }
    }
    return 0;
}

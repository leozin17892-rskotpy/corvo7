#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

int add(int a, int b) {
    return (a + b);
}
int main() {
    int i = 0;
    while(true){
        printf("%d", i);
        i++;
        if(i > 6){
            break;
        }
    }
    return 0;
}

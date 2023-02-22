## Rust Mini Project #5: Find a happy number

In number theory, a happy number is a number which eventually reaches 1 when replaced by the sum of the square of each digit.  
For instance, 13 is a happy number because $1^2+3^2=10$, and $1^{2}+0^{2}=1$.  
On the other hand, 4 is not a happy number because the sequence starting with $(4^{2}=16)$ and 
$1^{2}+6^{2}=37$ eventually reaches $2^{2}+0^{2}=4$, the number that started the sequence, and so the process continues in an infinite cycle without ever reaching 1. A number which is not happy is called sad or unhappy.

[Wikipedia](https://en.wikipedia.org/wiki/Happy_number)

### Usage

1. cargo run -- happy
2. Enter the number you want to know if it is a happy number.
3. Enter 0 if you want to exit.

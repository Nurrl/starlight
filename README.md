## Starlight

Light em' up, but efficiently !

### Some explanations on what I did:

- Usage of the crate `derive_more`:
I decided to use this crate for error management without reimplementing the **Display** trait on my errors manually, it provides graceful ways to define errors.

- The project was formated with `cargo fmt` before commit.

- The project features a number of **0** unsafe `unwrap()` (the one in `alg::starify` is safe because of the character filtration done on the input). 

### How to test:

- **<-** Represents user inputs to `stdin`
- **->** Represents program output to `stdout`

```shell
$ cargo run
<- <start condition>
<- <target condition>
-> <answer>
```


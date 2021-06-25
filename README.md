# arrow-exper1

## Experiment 1

### Setup

* CPU: Ryzen 5 2500U
* OS: Windows 10
* Python version: 3.8.10
* Rust version: 1.53.0

### How to run
* How to run Python version in Powershell

```
cd pyex
pip3 install -r requirements.txt
Measure-Command { python3 .\ex2.py }
```

* How to run Rust version in Powershell

```
cd rsex
cargo build --release
Measure-Command { cargo run --release }
```

### Result 
* Rust version took 2692.8923ms
* Python+pyarrow version took 354189.0579ms (131.5 times)
* Python+Pandas version took 541926.1722ms (201.2 times)

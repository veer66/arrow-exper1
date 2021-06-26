# arrow-exper1

## Experiment 1

### Setup

* CPU: Ryzen 5 2500U
* OS: Windows 10
* Python version: 3.8.10
* Rust version: 1.53.0

### How to run
* How to run Python+pyarrow version in Powershell

```
cd pyex
pip3 install -r requirements.txt
Measure-Command { python3 .\ex1.py }
```

* How to run Python+Pandas version in Powershell

```
cd pyex_pandas
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

## Experiment 2

### Setup 

* The setup is almost the same as Experiment 1.
* Add a cached version of Python+pyarrow (Thanks Sorrawut Kittikeereechaikun)
* Fix output member bug

### Result
* Rust version took 2510.838ms
* Python+pyarrow version took 362449.9403ms (144.4 times)
* Cached Python+pyarrow version took 10742.1672 (4.3 times)

## Experiment 3

### Setup 

Similar to Experiment 2

* Convert numpy array to Python list of float

### Result

* Python + Pandas + Native Python list + float version (ex4.py) took 13265.0073ms
* Python + Pandas version (ex2.py) took 474084.4263ms


## Experiment 4

### Setup

* Similar to experiment 2
* Add parallel version of Rust

### Result

* Rust with Arrow version (rsex) took 2682.4199ms
* Rust with Arrow and Rayon version (rsex-par) took 2319.2673ms (0.86 times)

This project currently isn't on pip, but in the meantime you can install it manually if you have a Rust compiler installed.

I think this should work via `pip install .`, but since I use WSL, I have to do the following:

```bash
pip install maturin
maturin develop --release --target x86_64-unknown-linux-gnu
```

#### Usage

```python
In [1]: s = streaming.CountMinSketch(0.01, 0.01)
[PYFLYBY] import streaming

In [2]: s.add(25, 100)

In [3]: s.add(25, 100)

In [4]: s.get(25)
Out[4]: 200

In [5]: s.get(10)
Out[5]: 0
```

#### Features
Right now, this is just a wrapper around `streaming_algorithms` that uses u64 keys and values. No other values are *currently* supported, though it would not be difficult to extend for other types.
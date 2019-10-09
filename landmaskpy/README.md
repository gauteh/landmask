# landmaskpy

## Build

```shell
maturin develop --release
```

make sure you use `--release`, otherwise things will go slowly. To install as package do:

```shell
maturin build --release
pip install target/wheels/landmask-....whl
```

## Usage

```python
from landmask import Landmask

mask = Landmask("path/to/file.wkb")

x = np.array([5, 15], dtype = 'float64')
y = np.array([64, 65], dtype = 'float64')
l = mask.contains_many (x, y)
```

## Benchmarks and tests

Install the depedencies:

```shell
pip install -r requirements-dev.txt
```

There is a benchmark in `tests/test_landmask.py`:

```shell
pytest -v tests
```


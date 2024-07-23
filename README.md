# pyo3-tutorial

based on PyCon US 2024 tutorial by Cheuk Ting Ho

GitHub: https://github.com/Cheukting/py03_101
YouTube: https://youtu.be/3lGkvKVTt5Y?si=AoD8uI_eNGE4cGpo

## step by step

set up env

```
mkdir pyo3-tutorial && cd pyo3-tutorial
python3 -m venv pyo3venv
source pyo3venv/bin/activate
pip install --upgrade pip
pip install maturin
mkdir pyo3_101 && cd pyo3_101
```

initialize the project

```
maturin init
maturin help
```

build crate and install it as a module in current venv

```
maturin develop
```

see it installed:

```
$ pip list                                                                                                                                  [9:58:27]
Package  Version Editable project location
-------- ------- ------------------------------------------
maturin  1.7.0
pip      24.1.2
pyo3_101 0.1.0   /Users/itamaro/work/pyo3-tutorial/pyo3_101
(pyo3venv)
```

test:

```py
>>> import pyo3_101 as p1
>>> p1
<module 'pyo3_101' from '/Users/itamaro/work/pyo3-tutorial/pyo3venv/lib/python3.12/site-packages/pyo3_101/__init__.py'>
>>> p1.sum_as_string(1,2)
'3'
>>> dir(p1)
['__all__', '__builtins__', '__cached__', '__doc__', '__file__', '__loader__', '__name__', '__package__', '__path__', '__spec__', 'pyo3_101', 'sum_as_string']
>>> p1.pyo3_101
<module 'pyo3_101.pyo3_101' from '/Users/itamaro/work/pyo3-tutorial/pyo3venv/lib/python3.12/site-packages/pyo3_101/pyo3_101.cpython-312-darwin.so'>
```

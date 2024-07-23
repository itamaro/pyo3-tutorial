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

add `say_hello` function and test it:

```
$ maturin develop                                                                                                                           [9:59:56]
📦 Including license file "/Users/itamaro/work/pyo3-tutorial/pyo3_101/LICENSE"
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /Users/itamaro/work/pyo3-tutorial/pyo3venv/bin/python
📡 Using build options features from pyproject.toml
    Blocking waiting for file lock on build directory
   Compiling pyo3_101 v0.1.0 (/Users/itamaro/work/pyo3-tutorial/pyo3_101)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
📦 Built wheel for CPython 3.12 to /var/folders/8f/h1vmzw5s5jv8wl9_msv5xq0c0000gn/T/.tmp1khH3z/pyo3_101-0.1.0-cp312-cp312-macosx_11_0_arm64.whl
✏️  Setting installed package as editable
🛠 Installed pyo3_101-0.1.0
(pyo3venv)

$ python -c 'import pyo3_101 as p1; print(p1.say_hello("Oogi", "PyCon"))'                                                                  [11:37:31]
Hello Oogi, welcome to PyCon
(pyo3venv)
```

Add default value for `conference` and test:

```
$ maturin develop                                                                                                                          [11:43:34]
📦 Including license file "/Users/itamaro/work/pyo3-tutorial/pyo3_101/LICENSE"
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /Users/itamaro/work/pyo3-tutorial/pyo3venv/bin/python
📡 Using build options features from pyproject.toml
   Compiling pyo3_101 v0.1.0 (/Users/itamaro/work/pyo3-tutorial/pyo3_101)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
📦 Built wheel for CPython 3.12 to /var/folders/8f/h1vmzw5s5jv8wl9_msv5xq0c0000gn/T/.tmpFnbfYz/pyo3_101-0.1.0-cp312-cp312-macosx_11_0_arm64.whl
✏️  Setting installed package as editable
🛠 Installed pyo3_101-0.1.0
(pyo3venv)

$ python -c 'from pyo3_101 import say_hello; print(say_hello(name="Oogi"))'                                                                [11:43:41]
Hello Oogi, welcome to the conference
(pyo3venv)

$ python -c 'from pyo3_101 import say_hello; print(say_hello(name="Oogi", conference="PyCon"))'                                            [11:43:43]
Hello Oogi, welcome to PyCon
(pyo3venv)
```
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

Add new function `check_reg` to work with files, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import check_reg; print(check_reg("reg.txt", "Oogi"))'                                                          [11:59:19]
thread '<unnamed>' panicked at src/lib.rs:19:41:
File not found: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Traceback (most recent call last):
  File "<string>", line 1, in <module>
pyo3_runtime.PanicException: File not found: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Raise proper Python exception for file not found, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import check_reg; print(check_reg("reg.txt", "Oogi"))'                                                          [12:04:07]
Traceback (most recent call last):
  File "<string>", line 1, in <module>
FileNotFoundError: File doesn't exist
```

Test also with existing file:

```
$ touch reg.txt                                                                                                                            [12:06:45]

$ python -c 'from pyo3_101 import check_reg; print(check_reg("reg.txt", "Oogi"))'                                                          [12:06:53]
Sorry, you are not in the list

$ echo "1. Oogizaur is registered" >> reg.txt                                                                                              [12:06:56]
$ python -c 'from pyo3_101 import check_reg; print(check_reg("reg.txt", "Oogi"))'                                                          [12:07:52]
You are registered
```

Wrap original error instead:

```
$ python -c 'from pyo3_101 import check_reg; print(check_reg("otherreg.txt", "Oogi"))'                                                     [12:10:27]
Traceback (most recent call last):
  File "<string>", line 1, in <module>
FileNotFoundError: No such file or directory (os error 2)
```

Add a function for counting attendees in list, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import count_attendees; print(count_attendees(["otherreg.txt", "Oogi"]))'                                       [12:15:28]
2
```

Add a function for calculating the average travel budget over a dictionary, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import travel_avg; print(travel_avg({"otherreg.txt": 5, "Oogi": 8}))'                                           [12:25:02]
6.5
```

Add an `Attendee` class, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import Attendee; me = Attendee("Oogi", False); print(me.name, me.is_speaker)'                                    [9:54:07]
Oogi False
```

Add empty name validation, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import Attendee; me = Attendee("", False); print(me.name, me.is_speaker)'                                        [9:58:23]
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ValueError: Must provide name

$ python -c 'from pyo3_101 import Attendee; me = Attendee("Oogi", False); print(me.name, me.is_speaker)'                                    [9:58:26]
Oogi False
```

Add custom name getter method that uppercases the name, test:

```
$ maturin develop
...

$ python -c 'from pyo3_101 import Attendee; me = Attendee("Oogi", False); print(me.name, me.is_speaker)'                                   [10:01:26]
OOGI False
```

Test what happens if trying to write to `name` with no setter:

```
$ python -c 'from pyo3_101 import Attendee; me = Attendee("Oogi", False); me.name = "Bert"; print(me.name, me.is_speaker)'                 [10:02:15]
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AttributeError: attribute 'name' of 'builtins.Attendee' objects is not writable
```

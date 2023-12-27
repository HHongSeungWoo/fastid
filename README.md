# fastid
[![PyPI - Version](https://img.shields.io/pypi/v/fastid.svg)](https://pypi.org/project/fastid)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/fastid.svg)](https://pypi.org/project/fastid)

Created for the easy and fast generation of various IDs

## Installation

```console
pip install fastid
```


## Example

```python
import fastid

fastid.snowflake_int()
fastid.snowflake_str()
fastid.ulid()
fastid.uuid_v7()
```


## Supported IDs
- ulid
- snowflake
- uuid v7 (If you intend to use uuid in your database, this can be a great choice)

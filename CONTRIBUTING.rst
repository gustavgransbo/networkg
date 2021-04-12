=================
Development Guide
=================

Project Structure
-----------------
`networkg` consists of three main layers:

1. The Rust core, which implements all graph logic.
2. `PyO3`_ Python bindings.
3. The Python layer, which provides documentation and type hints to the Python bindings.

.. _PyO3: https://github.com/PyO3/pyo3/


How to set up your development environemt
-----------------------------------------
.. _cargo: https://doc.rust-lang.org/cargo/

Developing `networkg` requires Python 3.7+, and `cargo`_.

Install development dependencies using:

.. code-block::

   pip install -r requirements-dev.txt

Nox
^^^
.. _nox: https://nox.thea.codes/

Actions such as testing, linting and building are managed by `nox`_.
To build and install `networkg` in your current virtual envirionemt, run:

.. code-block::

   nox --session install

To list all available sessions run:

.. code-block::

   nox --list

Running `nox` before making a pull request can save you some time by detecting problems
before they are caught by the GitHub Action workflows.

pre-commit
^^^^^^^^^^
.. _pre-commit: https://pre-commit.com/

Another way to avoid submitting code that will fail CI is to enable `pre-commit`_, which
will run automated checks against any code you try to commit:

.. code-block::

   pre-commit install


Publishing new releases
-----------------------
.. _PyPi: https://pypi.org/project/networkg/
.. _GitHub: https://github.com/gustavgransbo/networkg/

New releases of `networkg` are published to `PyPi`_ using GitHub actions.
The release process has two steps:

1. Bump the version number of the package in `Cargo.toml`.
2. Create a new release on `GitHub`_.

When a new release is created,
a GitHub Action workflow will automatically
build wheels for multiple distributions,
and publish them to `PyPi`_.

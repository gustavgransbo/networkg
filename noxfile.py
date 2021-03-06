"""Nox sessions."""
import sys
from typing import NamedTuple

import nox
from nox.sessions import Session

nox.options.reuse_existing_virtualenvs = True
nox.options.sessions = ["mypy", "lint", "xdoctest-3.9", "test-3.9"]


def _get_path_to_built_wheel(build_output: str) -> str:
    """Get the path to the wheel built by `maturin build`."""
    # The wheel is the last entry on the last line of the command output.
    return build_output.strip().splitlines()[-1].split(" ")[-1]


class _PythonVersion(NamedTuple):
    major: int
    minor: int


def _determine_python_version(session: Session) -> _PythonVersion:
    """Determine the Python version used in a Sessions."""
    if session.python is not None:
        major, minor = session.python.split(".")
        return _PythonVersion(major, minor)
    return _PythonVersion(sys.version_info.major, sys.version_info.minor)


def install_networkg(session: Session) -> None:
    """Build and install networkg using maturin."""
    session.install("maturin", "-c", "requirements-dev.txt")

    # A custom target directory is created for each python version to circumvent
    # maturin problems related to sharing the same build area among interpreters.
    major, minor = _determine_python_version(session)
    target_directory = f"target/{major}-{minor}"
    wheel_directory = f"{target_directory}/wheels"
    wheel_path = _get_path_to_built_wheel(
        session.run(
            "maturin",
            "build",
            "--no-sdist",
            "--interpreter",
            "python",
            f'--cargo-extra-args="--target-dir={target_directory}"',
            "--out",
            wheel_directory,
            silent=True,  # Command output is only returned when silenced=True
        )
    )
    session.install(wheel_path, "--force-reinstall")


@nox.session(python=["3.7", "3.8", "3.9"])
def test(session: Session) -> None:
    """Run Python test-suite."""
    args = session.posargs or ["--import-mode=append"]
    session.install(
        "pytest",
        "hypothesis",
        "-c",
        "requirements-dev.txt",
    )
    install_networkg(session)
    session.run("pytest", *args)


@nox.session(python="3.9")
def lint(session: Session) -> None:
    """Lint Python code using flake8."""
    args = session.posargs or ["networkg"]
    session.install(
        "black",
        "isort",
        "flake8",
        "flake8-black",
        "flake8-isort",
        "flake8-docstrings",
        "darglint",
        "-c",
        "requirements-dev.txt",
    )
    session.run("flake8", *args)


@nox.session(python="3.9")
def mypy(session: Session) -> None:
    """Type-check using mypy."""
    args = session.posargs or ["networkg"]
    session.install("mypy", "-c", "requirements-dev.txt")
    install_networkg(session)
    session.run("mypy", *args)


@nox.session(python=["3.7", "3.8", "3.9"])
def xdoctest(session: Session) -> None:
    """Run Python examples with xdoctest."""
    args = session.posargs or ["all"]
    session.install("xdoctest", "maturin", "-c", "requirements-dev.txt")
    install_networkg(session)
    session.run("xdoctest", "networkg", *args)


@nox.session(python="3.9")
def typeguard(session: Session) -> None:
    """Runtime type-check with typeguard."""
    args = session.posargs or ["--import-mode=append"]
    session.install("pytest", "hypothesis", "typeguard", "-c", "requirements-dev.txt")
    install_networkg(session)
    session.run("pytest", "--typeguard-packages=networkg", *args)


@nox.session(python="3.9")
def docs(session: Session) -> None:
    """Build documentation with Sphinx."""
    session.install("sphinx", "sphinx-autodoc-typehints", "-c", "requirements-dev.txt")
    session.run("sphinx-build", "docs", "docs/_build")


@nox.session(venv_backend="none")
def install(session: Session) -> None:
    """Build and install networkg in the current venv."""
    install_networkg(session)


@nox.session(venv_backend="none")
def rust_test(session: Session) -> None:
    """Run Rust test-suite. Requires cargo."""
    # --features "test" disables compilation of the bindings module
    # See https://github.com/gustavgransbo/networkg/commit/041d239f4f8c5d674943a5215c0d885f73492c0e # noqa
    session.run("cargo", "test", "--features", "test")


@nox.session(venv_backend="none")
def rust_lint(session: Session) -> None:
    """Lint the Rust code. Requires cargo, rustfmt and clippy."""
    session.run("cargo", "fmt", "--all", "--", "--check")
    session.run(
        "cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"
    )

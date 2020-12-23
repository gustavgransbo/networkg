"""Nox sessions."""
import nox
from nox.sessions import Session

nox.options.reuse_existing_virtualenvs = True
nox.options.sessions = ["mypy", "lint", "xdoctest-3.8"]


@nox.session(python="3.8")
def lint(session: Session):
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


@nox.session(python="3.8")
def mypy(session: Session) -> None:
    """Type-check using mypy."""
    args = session.posargs or ["networkg"]
    session.install("mypy", "maturin", "-c", "requirements-dev.txt")
    session.install(".")
    session.run("mypy", *args)


@nox.session(python=["3.7", "3.8", "3.9"])
def xdoctest(session: Session) -> None:
    """Run Python examples with xdoctest."""
    args = session.posargs or ["all"]
    session.install("xdoctest", "maturin", "-c", "requirements-dev.txt")
    session.install(".")
    session.run("xdoctest", "networkg", *args)


@nox.session(python="3.8")
def docs(session: Session) -> None:
    """Build documentation with Sphinx."""
    session.install("sphinx", "sphinx-autodoc-typehints", "-c", "requirements-dev.txt")
    session.run("sphinx-build", "docs", "docs/_build")

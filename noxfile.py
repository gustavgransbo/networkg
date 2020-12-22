"""Nox sessions."""
import nox
from nox.sessions import Session

nox.options.reuse_existing_virtualenvs = True


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
        "dev-requirements.txt",
    )
    session.run("flake8", *args)

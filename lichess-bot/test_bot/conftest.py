"""Remove files created when testing lichess-bot."""
import shutil
import os
from _pytest.config import ExitCode
from _pytest.main import Session
from typing import Union


def pytest_sessionfinish(session: Session, exitstatus: Union[int, ExitCode]) -> None:
    """Remove files created when testing lichess-bot."""
    if os.path.exists("TEMP") and not os.getenv("GITHUB_ACTIONS"):
        shutil.rmtree("TEMP")

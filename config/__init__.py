import os
import sys
from pathlib import Path

AppName = "Xpic"
DataPath = Path(os.getenv("LocalAppData")) / AppName
CachePath = DataPath / "Cache"


def is_bundled() -> bool:
    # https://pyinstaller.org/en/stable/runtime-information.html
    return getattr(sys, "frozen", False) and hasattr(sys, "_MEIPASS")


def get_xpic_dir() -> Path:
    return Path(getattr(sys, "_MEIPASS")) if is_bundled() else Path(__file__).resolve().parent.parent


def init() -> None:
    for path in (DataPath, CachePath):
        if not path.exists():
            path.mkdir(exist_ok=True)


init()

XpicDir = get_xpic_dir()
IconPath = XpicDir / "assets" / "xpic.png"
QSSPath = XpicDir / "assets" / "xpic.qss"

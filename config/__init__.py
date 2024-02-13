import os
from pathlib import Path

AppName = "Xpic"

Path = Path(os.getenv("LocalAppData")) / AppName
CachePath = Path / "Cache"

for path in (Path, CachePath):
    if not path.exists():
        path.mkdir(exist_ok=True)

ContextMenuStyleSheet = """
QMenu {
    background-color: rgba(7, 15, 43, 90%);
    border-radius: 4px;
    font-size: 14px;
    padding: 5px 0px;
}

QMenu::item {
    background-color: transparent;
    padding: 6px 50px 6px 20px;
    margin: 0px 5px;
    color: white;
}

QMenu::item:selected {
    background-color: rgba(255, 255, 255, 20%);
    border-radius: 4px;
}
"""

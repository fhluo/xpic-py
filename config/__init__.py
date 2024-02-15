import os
from pathlib import Path

AppName = "Xpic"

Path = Path(os.getenv("LocalAppData")) / AppName
CachePath = Path / "Cache"
IconPath = "./assets/xpic.png"
QSSPath = "./assets/xpic.qss"

for path in (Path, CachePath):
    if not path.exists():
        path.mkdir(exist_ok=True)

from pathlib import Path

from PIL import Image

if __name__ == "__main__":
    path = Path(__file__).resolve()

    img = Image.open(path.with_name("xpic.png"))
    img.save(path.with_name("xpic.ico"))

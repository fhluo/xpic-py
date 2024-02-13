import ctypes
import os
from typing import NamedTuple

from PIL import Image


class ImageInfo(NamedTuple):
    size: tuple[int, int]
    format: str

    @property
    def width(self) -> int:
        return self.size[0]

    @property
    def height(self) -> int:
        return self.size[1]


def get_image_info(path: str | os.PathLike) -> ImageInfo | None:
    try:
        with Image.open(path) as img:
            return ImageInfo(img.size, img.format)
    except OSError:
        return None


def set_desktop_wallpaper(path: str | os.PathLike) -> None:
    # https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
    ctypes.windll.user32.SystemParametersInfoW(0x14, 0, str(path), 0)

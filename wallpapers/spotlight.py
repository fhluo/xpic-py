import os
from functools import cached_property
from pathlib import Path
from shutil import copyfile
from typing import Generator

from .image import get_image_info, ImageInfo


def assets() -> Generator[Path, None, None]:
    return Path(os.environ.get("LocalAppData")).glob("Packages/*ContentDeliveryManager*/LocalState/Assets/*")


class Image:
    def __init__(self, path: str | os.PathLike):
        self.path = Path(path)

    def __str__(self) -> str:
        return str(self.path)

    @cached_property
    def info(self) -> ImageInfo:
        return get_image_info(self.path)

    def save(self, dst: str | os.PathLike) -> None:
        # add suffix
        dst = (Path(dst) / self.path.name).with_suffix(f".{self.info.format.lower()}")

        # copy image
        if not dst.exists():
            copyfile(self.path, dst)


def images() -> Generator[Image, None, None]:
    for path in assets():
        if not path.is_file():
            continue

        img = Image(path)
        if img.info is None:
            continue

        if img.info.size >= (1920, 1080):
            yield img


def save_images(dst: str | os.PathLike) -> None:
    for img in images():
        img.save(dst)

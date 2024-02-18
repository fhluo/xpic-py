import asyncio
from pathlib import Path
from typing import Generator

import config
from . import bing, spotlight
from .image import Size, ImageInfo, get_image_info, set_desktop_wallpaper

__all__ = [
    "bing",
    "spotlight",
    "Size",
    "ImageInfo",
    "get_image_info",
    "set_desktop_wallpaper",
    "cache_images",
    "get_cached_images",
]


def cache_images() -> None:
    spotlight.save_images(config.CachePath)
    bing.save_images(config.CachePath)


def get_cached_images() -> Generator[Path, None, None]:
    return Path(config.CachePath).glob("*.*")

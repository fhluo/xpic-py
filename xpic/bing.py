import os
from functools import cached_property
from pathlib import Path
from urllib.parse import urljoin, parse_qs, urlparse

import requests


def query_raw(idx: int = 0, n: int = 1) -> dict:
    """Query Bing wallpapers.

    :param idx: The index of the wallpapers. 0: today, 1: yesterday ...
    :param n: The number of returned wallpapers. Maximum value is 8.
    """
    url = "https://cn.bing.com/HPImageArchive.aspx"
    payload = {"format": "js", "idx": idx, "n": n}
    r = requests.get(url, params=payload, timeout=10)

    return r.json()


class Image:
    base_url: str = "https://cn.bing.com/"

    def __init__(self, path: str):
        self._path = path

    @cached_property
    def url(self) -> str:
        return urljoin(self.base_url, self._path)

    def __str__(self) -> str:
        return self.url

    @cached_property
    def name(self) -> str:
        return parse_qs(urlparse(self.url).query)["id"][0]

    def save(self, dst_dir: str | os.PathLike) -> None:
        path = Path(dst_dir) / self.name

        if path.exists():
            return

        r = requests.get(self.url, stream=True)

        with path.open("wb") as f:
            r.raise_for_status()
            for chunk in r.iter_content(chunk_size=4096):
                f.write(chunk)


def query(idx: int = 0, n: int = 1) -> list[Image]:
    """Query Bing wallpapers.

    :param idx: The index of the wallpapers. 0: today, 1: yesterday ...
    :param n: The number of returned wallpapers. Maximum value is 8.
    """
    return [Image(image["url"]) for image in query_raw(idx, n)["images"]]


def images() -> list[Image]:
    return query(0, 8)


def save_images(dst_dir: str | os.PathLike) -> None:
    for img in images():
        img.save(dst_dir)

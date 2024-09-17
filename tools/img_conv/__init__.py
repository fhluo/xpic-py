from pathlib import Path

import typer
from PIL import Image
from typing_extensions import Annotated

__all__ = ["main"]


def convert(
    src: Annotated[Path, typer.Option("--input", "-i")], dst: Annotated[Path, typer.Option("--output", "-o")]
) -> None:
    img = Image.open(src)
    img.save(dst)


def main() -> None:
    typer.run(convert)

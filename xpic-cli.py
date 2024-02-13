from pathlib import Path

import typer
from typing_extensions import Annotated

from wallpapers import spotlight, bing

cli = typer.Typer(add_completion=False)


@cli.command(name="list")
def list_command() -> None:
    """List wallpapers."""
    for img in spotlight.images():
        print(img)

    for img in bing.images():
        print(img.url)


@cli.command()
def save(dst: Annotated[Path, typer.Option(help="Path to save wallpapers.")] = ".") -> None:
    """save wallpapers."""
    spotlight.save_images(dst)
    bing.save_images(dst)


if __name__ == "__main__":
    cli()

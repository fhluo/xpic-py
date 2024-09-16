from .app import App
import asyncio

__all__ = ["main"]

async def main():
    app_ = App()
    task = asyncio.create_task(app_.window.images_widget.cahche_images_async())

    app_.run()
    await task

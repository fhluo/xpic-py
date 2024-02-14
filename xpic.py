import os
import sys
from functools import cached_property
from pathlib import Path
from shutil import copyfile

import win32mica
from PySide6.QtCore import QSize, Qt
from PySide6.QtGui import QIcon, QPixmap, QCursor, QAction, QMouseEvent, QContextMenuEvent, QPainter, QBrush, QColor
from PySide6.QtWidgets import (
    QVBoxLayout,
    QApplication,
    QGridLayout,
    QLabel,
    QFileDialog,
    QMenu,
    QWidget,
)
from win32mica import MicaTheme, MicaStyle

import config
import wallpapers
from wallpapers import cache_images, get_cached_images


class ContextMenu(QMenu):
    def __init__(self, parent: QWidget) -> None:
        super().__init__(parent)

        self.setWindowFlag(Qt.WindowType.FramelessWindowHint)
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setStyleSheet(config.ContextMenuStyleSheet)

        # open wallpaper
        self.action_open = QAction("打开", self)
        self.addAction(self.action_open)

        # save wallpaper
        self.action_save = QAction("保存", self)
        self.addAction(self.action_save)

        # set as desktop wallpaper
        self.action_set_as_desktop_wallpaper = QAction("设为桌面壁纸", self)
        self.addAction(self.action_set_as_desktop_wallpaper)

        self.popup(QCursor.pos())


def get_rounded_pixmap(path: str | os.PathLike, radius: int) -> QPixmap:
    original = QPixmap(path)

    rounded = QPixmap(original.size())
    rounded.fill(QColor(0, 0, 0, 0))

    painter = QPainter(rounded)
    painter.setRenderHint(QPainter.RenderHint.Antialiasing)
    painter.setBrush(QBrush(original))
    painter.setPen(Qt.PenStyle.NoPen)
    painter.drawRoundedRect(original.rect(), radius, radius)

    return rounded


class ImageLabel(QLabel):
    def __init__(self, path: str | os.PathLike) -> None:
        super().__init__()

        self.path = Path(path)
        self.setScaledContents(True)

        self.setPixmap(self.rounded_pixmap)

    @cached_property
    def rounded_pixmap(self) -> QPixmap:
        return get_rounded_pixmap(self.path, 50)

    def open(self) -> None:
        os.startfile(self.path)

    def save(self) -> None:
        filename, _ = QFileDialog.getSaveFileName(
            parent=self,
            caption="保存",
            dir=str(Path().home() / "Pictures"),
            filter="*.jpg",
        )

        if filename != "":
            copyfile(self.path, filename)

    def set_as_desktop_wallpaper(self) -> None:
        wallpapers.set_desktop_wallpaper(self.path)

    def mouseDoubleClickEvent(self, event: QMouseEvent) -> None:
        super().mouseDoubleClickEvent(event)

        self.open()

    def contextMenuEvent(self, event: QContextMenuEvent) -> None:
        super().contextMenuEvent(event)

        menu = ContextMenu(self)
        menu.action_open.triggered.connect(self.open)
        menu.action_save.triggered.connect(self.save)
        menu.action_set_as_desktop_wallpaper.triggered.connect(self.set_as_desktop_wallpaper)


class ImagesWidget(QWidget):
    def __init__(self, size=QSize(int(192 * 1.25), int(108 * 1.25))) -> None:
        super().__init__()

        layout = QGridLayout(self)
        layout.setSpacing(30)
        layout.setContentsMargins(50, 40, 50, 40)

        cache_images()
        images = list(get_cached_images())

        count = self.calc_count(len(images))
        for i, x in enumerate(images):
            layout.addWidget(ImageLabel(x), i // count, i % count)

        width = (
            size.width() * layout.columnCount()
            + layout.spacing() * (layout.columnCount() - 1)
            + layout.contentsMargins().left()
            + layout.contentsMargins().right()
        )
        height = (
            size.height() * layout.rowCount()
            + layout.contentsMargins().top()
            + layout.contentsMargins().bottom()
            + layout.spacing() * (layout.rowCount() - 1)
        )

        if layout.count() == 0:
            label = QLabel("Nothing")
            label.setObjectName("Warning")
            label.setFixedSize(label.sizeHint())
            layout.addWidget(label, 0, 0)

        self.setFixedSize(width, height)

    @staticmethod
    def calc_count(target) -> int:
        """Calculate the widgets every row"""
        number = 1
        count = 2

        # sequence: [1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, ...]
        for i in range(1, target + 1):
            if i == count + 1:
                number += 1
                count += 2 * number

        return number


class MainWindow(QWidget):
    def __init__(self) -> None:
        super().__init__()

        self.setWindowTitle(config.AppName)

        self._layout = QVBoxLayout(self)
        self._layout.setSpacing(0)
        self._layout.setContentsMargins(0, 0, 0, 0)

        self.images_widget = ImagesWidget()
        self._layout.addWidget(self.images_widget)

        self.setFixedSize(self.images_widget.size())

    def apply_mica(self) -> None:
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)

        win32mica.ApplyMica(HWND=self.winId(), Theme=MicaTheme.DARK, Style=MicaStyle.DEFAULT)


class App:
    icon_path = "./assets/xpic.ico"

    def __init__(self) -> None:
        self._app = QApplication(sys.argv)
        self._app.setWindowIcon(QIcon(self.icon_path))

        self._window = MainWindow()

    def run(self) -> None:
        self._window.apply_mica()
        self._window.show()
        self._app.exec()


if __name__ == "__main__":
    App().run()

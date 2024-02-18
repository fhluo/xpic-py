import asyncio
import math
import os
import sys
from functools import cached_property
from pathlib import Path
from shutil import copyfile

import cv2
import win32mica
from PySide6.QtCore import Qt, QMimeData, QUrl, QPoint
from PySide6.QtGui import (
    QIcon,
    QPixmap,
    QCursor,
    QAction,
    QPainter,
    QBrush,
    QColor,
    QDrag,
    QImage,
)
from PySide6.QtWidgets import (
    QVBoxLayout,
    QApplication,
    QGridLayout,
    QLabel,
    QFileDialog,
    QMenu,
    QWidget,
    QScrollArea,
)
from win32mica import MicaTheme, MicaStyle

import config
import wallpapers
from wallpapers import cache_images, get_cached_images, Size


class ContextMenu(QMenu):
    def __init__(self, parent: QWidget) -> None:
        super().__init__(parent)

        self.setWindowFlag(Qt.WindowType.FramelessWindowHint)
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setStyleSheet(
            "background-color: rgba(7, 15, 43, 90%);" "border-radius: 4px;" "font-size: 14px;" "padding: 5px 0px;"
        )

        # open wallpaper
        self.action_open = QAction("Open", self)
        self.addAction(self.action_open)

        # save wallpaper
        self.action_save = QAction("Save", self)
        self.addAction(self.action_save)

        # set as desktop wallpaper
        self.action_set_as_desktop_wallpaper = QAction("Set as desktop wallpaper", self)
        self.addAction(self.action_set_as_desktop_wallpaper)

        self.popup(QCursor.pos())


def get_rounded_pixmap(img: str | os.PathLike | QPixmap, radius: int) -> QPixmap:
    original = img if isinstance(img, QPixmap) else QPixmap(img)

    rounded = QPixmap(original.size())
    rounded.fill(QColor(0, 0, 0, 0))

    painter = QPainter(rounded)
    painter.setRenderHint(QPainter.RenderHint.Antialiasing)
    painter.setBrush(QBrush(original))
    painter.setPen(Qt.PenStyle.NoPen)
    painter.drawRoundedRect(original.rect(), radius, radius)

    return rounded


def get_brighter_image(path: str | os.PathLike, value: int) -> QPixmap:
    img = cv2.imread(str(path))

    hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
    h, s, v = cv2.split(hsv)

    limit = 255 - value
    v[v > limit] = 255
    v[v <= limit] += value

    hsv = cv2.merge((h, s, v))
    img = cv2.cvtColor(hsv, cv2.COLOR_HSV2BGR)

    height, width, channel = img.shape
    return QPixmap.fromImage(QImage(img.data, width, height, 3 * width, QImage.Format.Format_BGR888))


class ImageLabel(QLabel):
    def __init__(self, path: str | os.PathLike) -> None:
        super().__init__()

        self.path = Path(path)
        self.setScaledContents(True)
        self.setPixmap(self.rounded_pixmap)
        self.setStyleSheet("background-color: transparent;")

        self.drag_start_pos: QPoint | None = None

    @cached_property
    def rounded_pixmap(self) -> QPixmap:
        return get_rounded_pixmap(self.path, 50)

    @cached_property
    def brighter_pixmap(self) -> QPixmap:
        return get_rounded_pixmap(get_brighter_image(self.path, 15), 50)

    @cached_property
    def grabbed_pixmap(self) -> QPixmap:
        self.setPixmap(self.rounded_pixmap)
        return self.grab()

    def open(self) -> None:
        os.startfile(self.path)

    def save(self) -> None:
        filename, _ = QFileDialog.getSaveFileName(
            parent=self,
            caption="Save Wallpaper",
            dir=str(Path().home() / "Pictures"),
            filter="Image Files (*.png *.jpg)",
        )

        if filename == "":
            return

        copyfile(self.path, Path(filename).with_suffix(self.path.suffix))

    def set_as_desktop_wallpaper(self) -> None:
        wallpapers.set_desktop_wallpaper(self.path)

    def enterEvent(self, event):
        self.setPixmap(self.brighter_pixmap)

    def leaveEvent(self, event):
        self.setPixmap(self.rounded_pixmap)

    def mouseDoubleClickEvent(self, event) -> None:
        self.open()

    def contextMenuEvent(self, event) -> None:
        menu = ContextMenu(self)
        menu.action_open.triggered.connect(self.open)
        menu.action_save.triggered.connect(self.save)
        menu.action_set_as_desktop_wallpaper.triggered.connect(self.set_as_desktop_wallpaper)

    def mousePressEvent(self, event) -> None:
        if event.button() == Qt.MouseButton.LeftButton:
            self.drag_start_pos = event.position().toPoint()

    def mouseMoveEvent(self, event):
        if event.buttons() != Qt.MouseButton.LeftButton:
            return

        if self.drag_start_pos is None:
            return

        if (event.position().toPoint() - self.drag_start_pos).manhattanLength() < QApplication.startDragDistance():
            return

        self.drag()

    def drag(self):
        drag = QDrag(self)

        data = QMimeData()
        data.setImageData(QImage(self.path))
        data.setUrls([QUrl(self.path.as_uri())])
        drag.setMimeData(data)

        drag.setPixmap(self.grabbed_pixmap)
        drag.exec(Qt.DropAction.CopyAction)

        self.drag_start_pos = None


class ImagesWidget(QWidget):
    def __init__(self, columns: int = 4, rows: int = 4, img_size: Size = Size(240, 135)) -> None:
        super().__init__()

        self._columns = columns
        self._rows = rows
        self.img_size = img_size

        layout = QGridLayout(self)
        layout.setSpacing(30)
        layout.setContentsMargins(50, 30, 50, 30)
        self._layout = layout

        self.images = list(get_cached_images())
        self.image_labels = [ImageLabel(img) for img in self.images]

        self.layout_images()

    @property
    def columns(self) -> int:
        return self._columns if self._columns >= 1 else 1

    @property
    def rows(self) -> int:
        return math.ceil(len(self.image_labels) / self.columns)

    @property
    def full_width(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.left()
            + margin.right()
            + self.columns * self.img_size.width
            + (self.columns - 1) * self._layout.spacing()
        )

    @property
    def full_height(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.top() + margin.bottom() + self.rows * self.img_size.height + (self.rows - 1) * self._layout.spacing()
        )

    @property
    def proper_width(self) -> int:
        return self.full_width

    @property
    def proper_height(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.top()
            + margin.bottom()
            + self._rows * self.img_size.height
            + (self._rows - 1) * self._layout.spacing()
        )

    @property
    def min_width(self) -> int:
        margin = self._layout.contentsMargins()
        return margin.left() + margin.right() + self.img_size.width

    @property
    def min_height(self) -> int:
        margin = self._layout.contentsMargins()
        return margin.top() + margin.bottom() + self.img_size.height

    def remove_image_labels(self) -> None:
        for label in self.image_labels:
            self._layout.removeWidget(label)

    def layout_images(self) -> None:
        self.remove_image_labels()

        for i, label in enumerate(self.image_labels):
            self._layout.addWidget(label, i // self.columns, i % self.columns)

        self.setFixedSize(self.full_width, self.full_height)

    def relayout_images(self, size: Size) -> None:
        margin = self._layout.contentsMargins()
        new_columns = (size.width - margin.left() - margin.right() + self._layout.spacing()) // (
            self.img_size.width + self._layout.spacing()
        )

        if new_columns == self._columns:
            return

        self._columns = new_columns
        self.layout_images()

    async def cahche_images_async(self) -> None:
        try:
            cache_images()
        except (Exception,):
            return

        self.remove_image_labels()
        self.images = list(get_cached_images())
        self.image_labels = [ImageLabel(img) for img in self.images]

        self.layout_images()


class MainWindow(QWidget):
    def __init__(self) -> None:
        super().__init__()

        self.setWindowTitle(config.AppName)

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_area.setAlignment(Qt.AlignmentFlag.AlignCenter)
        scroll_area.setStyleSheet("background-color: transparent; border: none;")

        self.images_widget = ImagesWidget()
        scroll_area.setWidget(self.images_widget)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setContentsMargins(0, 0, 0, 0)

        layout = QVBoxLayout(self)
        layout.setSpacing(0)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(scroll_area)

        self.setMinimumWidth(self.images_widget.min_width)
        self.setMinimumHeight(self.images_widget.min_height)
        self.resize(self.images_widget.proper_width, self.images_widget.proper_height)

    def apply_mica(self) -> None:
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)

        win32mica.ApplyMica(HWND=self.winId(), Theme=MicaTheme.DARK, Style=MicaStyle.DEFAULT)

    def resizeEvent(self, event):
        self.images_widget.relayout_images(Size(event.size().width(), event.size().height()))


class App:
    def __init__(self) -> None:
        self._app = QApplication(sys.argv)
        self._app.setWindowIcon(QIcon(str(config.IconPath)))

        with open(config.QSSPath, "r", encoding="utf-8") as f:
            self._app.setStyleSheet(f.read())

        self.window = MainWindow()

    def run(self) -> None:
        self.window.apply_mica()
        self.window.show()
        self._app.exec()


async def main():
    app = App()
    task = asyncio.create_task(app.window.images_widget.cahche_images_async())

    app.run()
    await task


if __name__ == "__main__":
    asyncio.run(main())

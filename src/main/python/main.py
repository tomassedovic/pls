from fbs_runtime.application_context.PyQt5 import ApplicationContext
from PyQt5.QtCore import Qt, QTimer
from PyQt5.QtWidgets import QWidget, QLabel, QPushButton, QComboBox
from PyQt5.QtWidgets import QVBoxLayout, QHBoxLayout, QGroupBox, QMessageBox

import sys

import pls


ABOUT_TEXT = """\
<p>
You can find the source code at this location:<br />
<a href="https://gitlab.com/Sedovic/pls">https://gitlab.com/Sedovic/pls</a>
</p>
<p>
Copyright (C) 2019 Tomas Sedovic <tomas@sedovic.cz>
</p>
<p>
<b>Program license:</b>
</p>
<p>
This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.
</p>
<p>
This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.
</p>
<p>
You should have received a copy of the GNU General Public License
along with this program. If not, see<br />
<a href="https://www.gnu.org/licenses/">https://www.gnu.org/licenses/</a>.
</p>
<p>
<b>Icons:</b><br />
The "TV Show" application icon comes from Icons8:
</p>
<p>
<a href="https://icons8.com/icon/46904/cute-color">https://icons8.com/icon/46904/cute-color</a>
</p>
<p>
It is provided free of charge under the condition of showing the link above in
the About dialog of the app that uses it.
</p>
"""


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.pls = pls.Pls()

        self.shows = QComboBox()
        for (index, show) in enumerate(self.pls.shows(self.pls.config())):
            self.shows.insertItem(index, show.name, show.id)
        self.shows.activated.connect(lambda: self.refresh_labels())

        self.text = QLabel()
        self.text.setWordWrap(True)

        self.play_last = QPushButton("Play Last")
        self.play_last.clicked.connect(lambda: self.play_last_action())
        self.play_last.setObjectName("play_last")

        self.play_next = QPushButton("Play Next")
        self.play_next.clicked.connect(lambda: self.play_next_action())
        self.play_next.setObjectName("play_next")

        self.settings = QPushButton("Settings");
        self.settings.clicked.connect(
            lambda: pls.play_file(pls.config_file_location()))

        about = QMessageBox(QMessageBox.NoIcon, "About pls", "<b>About pls</b>")
        about.setInformativeText(ABOUT_TEXT)

        self.about = QPushButton("About");
        self.about.clicked.connect(lambda: about.exec())

        play_buttons = QGroupBox()
        layout = QVBoxLayout()
        layout.addWidget(self.play_last)
        layout.addWidget(self.play_next)
        play_buttons.setLayout(layout)

        meta_buttons = QGroupBox()
        layout = QHBoxLayout()
        layout.addWidget(self.settings)
        layout.addWidget(self.about)
        meta_buttons.setLayout(layout)

        layout = QVBoxLayout()
        layout.addWidget(self.shows)
        layout.addWidget(self.text)
        layout.addWidget(play_buttons)
        layout.addWidget(meta_buttons)
        self.setLayout(layout)
        self.refresh_labels()

    def play_last_action(self):
        # NOTE(shadower): Briefly disable the button. This is to prevent
        # accidental double clicking.
        self.play_last.setEnabled(False)
        timer = QTimer()
        timer.singleShot(3000, lambda: self.play_last.setEnabled(True))

        config = self.pls.config()
        show_id = self.shows.currentData()
        series = self.pls.series(config, show_id)
        series.replay_last_watched()
        # NOTE(shadower): we're not modifying the state in here, no need
        # to refresh the UI. Leaving this commented out for now.
        #self.refresh_labels(config=config, series=series)

    def play_next_action(self):
        # NOTE(shadower): Briefly disable the button. This is to prevent
        # accidental double clicking.
        self.play_next.setEnabled(False)
        timer = QTimer()
        timer.singleShot(3000, lambda: self.play_next.setEnabled(True))

        config = self.pls.config()
        show_id = self.shows.currentData()
        series = self.pls.series(config, show_id)
        series.play_next()
        self.pls.set_next_and_save(config, series)
        self.refresh_labels(config=config, series=series)

    def refresh_labels(self, config=None, series=None):
        if config is None:
            config = self.pls.config()
        show_id = self.shows.currentData()
        if series is None or show_id != series.id:
            series = self.pls.series(config, show_id)
        self.text.setText(f"Location: {series.location}")

        last_path = series.last_watched_episode_path
        if isinstance(last_path, pls.Error):
            # TODO: make the text red
            self.play_last.setText(str(last_path))
        else:
            self.play_last.setText(f"Replay last watched:\n{last_path}")

        next_path = series.next_episode_path
        if isinstance(next_path, pls.Error):
            # TODO: make the text red
            self.play_next.setText(str(next_path))
        else:
            self.play_next.setText(f"Play next:\n{next_path}")


class ErrorWindow(QWidget):
    def __init__(self, error_text):
        super().__init__()
        self.setWindowTitle("pls Error")
        self.text = QLabel()
        self.text.setWordWrap(True)
        self.text.setText(error_text)
        layout = QVBoxLayout()
        layout.addWidget(self.text)
        self.setLayout(layout)
        self.resize(600, 320)


if __name__ == '__main__':
    if len(sys.argv) <= 1:
        context = ApplicationContext()
        stylesheet = context.get_resource('styles.qss')
        context.app.setStyleSheet(open(stylesheet).read())
        try:
            window = MainWindow()
        except Exception:
            import traceback
            e = traceback.format_exc()
            window = ErrorWindow(e)
            print(e)
        window.setObjectName("main-window")
        window.show()
        exit_code = context.app.exec_()
        sys.exit(exit_code)
    else:
        print("TODO: use the CLI")

from fbs_runtime.application_context.PyQt5 import ApplicationContext
from PyQt5.QtCore import Qt, QTimer
from PyQt5.QtWidgets import QWidget, QLabel, QListWidget, QPushButton, QComboBox
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

        self.shows = QListWidget()
        for (index, show) in enumerate(self.pls.shows(self.pls.config())):
            self.shows.addItem(show.name)
        self.shows.setCurrentRow(0)
        self.shows.currentItemChanged.connect(lambda: self.refresh_labels())

        self.location = QLabel()
        self.location.setWordWrap(True)
        self.location.setObjectName("location")

        self.play_last = QPushButton("Play Last")
        self.play_last.clicked.connect(lambda: self.play_last_action())
        self.play_last.setObjectName("play_last")

        self.play_next = QPushButton("Play Next")
        self.play_next.clicked.connect(lambda: self.play_next_action())
        self.play_next.setObjectName("play_next")

        self.settings = QPushButton("Settings");
        self.settings.clicked.connect(
            lambda: self.open_settings())

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
        layout.addWidget(self.location)
        layout.addWidget(play_buttons)
        layout.addWidget(meta_buttons)
        self.setLayout(layout)
        self.refresh_labels()

    def open_settings(self):
        conf_file = pls.config_file_location()
        if conf_file.exists():
            pls.play_file(conf_file)
        else:
            mbox = QMessageBox(QMessageBox.Critical, "Error: Config file not found", "<b>Error: Config file not found.</b>")
            mbox.setInformativeText(f"The configuration file could not be found.\nExpected location:\n'{conf_file}'")
            mbox.exec()

    def current_show_id(self):
        shows = list(self.pls.shows(self.pls.config()))
        if shows and self.shows.currentRow() >= 0:
            current_show = shows[self.shows.currentRow()]
            assert current_show.name == self.shows.currentItem().text()
            show_id = current_show.id
        else:
            show_id = None
        return show_id

    def play_last_action(self):
        # NOTE(shadower): Briefly disable the button. This is to prevent
        # accidental double clicking.
        self.play_last.setEnabled(False)
        timer = QTimer()
        timer.singleShot(3000, lambda: self.play_last.setEnabled(True))

        config = self.pls.config()
        show_id = self.current_show_id()
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
        show_id = self.current_show_id()
        series = self.pls.series(config, show_id)
        series.play_next()
        self.pls.set_next_and_save(config, series)
        self.refresh_labels(config=config, series=series)

    def refresh_labels(self, config=None, series=None):
        if config is None:
            config = self.pls.config()

        show_id = self.current_show_id()
        if show_id is None:
            # There's no config or it has no shows. Nothing else to do here.
            mbox = QMessageBox(QMessageBox.Critical, "Error: No shows loaded", "<b>Error: No shows loaded.</b>")
            mbox.setInformativeText(f"Could not load any shows from the configuration file. Either it doesn't exist or it's empty.\n\nTry clicking on the Settings button.")
            mbox.exec()
            return

        if series is None or show_id != series.id:
            series = self.pls.series(config, show_id)
        self.location.setText(f"Location: {series.location}")

        last_path = series.last_watched_episode_path
        if isinstance(last_path, pls.Error):
            # TODO: make the text red
            self.play_last.setText(str(last_path))
        else:
            self.play_last.setText(f"Replay last watched:\n{last_path.relative_to(series.location)}")

        next_path = series.next_episode_path
        if isinstance(next_path, pls.Error):
            # TODO: make the text red
            self.play_next.setText(str(next_path))
        else:
            self.play_next.setText(f"Play next:\n{next_path.relative_to(series.location)}")


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

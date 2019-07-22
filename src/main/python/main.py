from fbs_runtime.application_context.PyQt5 import ApplicationContext
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QWidget, QLabel, QPushButton, QVBoxLayout, QComboBox

import sys

import pls


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

        self.previous_file_label = QLabel()
        self.previous_file_label.setText("Last played:")
        self.next_file_label = QLabel()
        self.next_file_label.setText("Next:")

        self.play_last = QPushButton("Play Last")
        self.play_last.clicked.connect(self.play_last_action)
        self.play_last.setObjectName("play_last")

        self.play_next = QPushButton("Play Next\nLOL")
        self.play_next.clicked.connect(self.play_next_action)
        self.play_next.setObjectName("play_next")

        layout = QVBoxLayout()
        layout.addWidget(self.shows)
        layout.addWidget(self.text)
        #layout.addWidget(self.previous_file_label)
        #layout.addWidget(self.next_file_label)
        layout.addWidget(self.play_last)
        layout.addWidget(self.play_next)
        #layout.setAlignment(self.play_last, Qt.AlignHCenter)
        #layout.setAlignment(self.play_next, Qt.AlignHCenter)
        self.setLayout(layout)
        self.refresh_labels()

    def play_last_action(self):
        config = self.pls.config()
        show_id = self.shows.currentData()
        series = self.pls.series(config, show_id)
        series.replay_last_watched()
        # NOTE(shadower): we're not modifying the state in here, no need
        # to refresh the UI. Leaving this commented out for now.
        #self.refresh_labels(config=config, series=series)

    def play_next_action(self):
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
        self.text.setText(
            f"Series: {series.name}\nLocation: {series.location}")
        self.play_last.setText(
            f"Replay last watched:\n{series.last_watched_episode_path.name}")
        self.play_next.setText(f"Play next:\n{series.next_episode_path.name}")


if __name__ == '__main__':
    if len(sys.argv) <= 1:
        context = ApplicationContext()       # 1. Instantiate ApplicationContext
        stylesheet = context.get_resource('styles.qss')
        context.app.setStyleSheet(open(stylesheet).read())
        window = MainWindow()
        window.setObjectName("main-window")
        #window.resize(640, 480)
        window.show()
        exit_code = context.app.exec_()      # 2. Invoke appctxt.app.exec_()
        sys.exit(exit_code)
    else:
        print("TODO: use the CLI")

from fbs_runtime.application_context.PyQt5 import ApplicationContext
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QWidget, QLabel, QPushButton, QVBoxLayout
import requests

import sys

import pls


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.text = QLabel()
        self.text.setWordWrap(True)

        self.previous_file_label = QLabel()
        self.previous_file_label.setText("Last played:")
        self.next_file_label = QLabel()
        self.next_file_label.setText("Next:")

        self.play_last = QPushButton("Play Last")
        self.play_last.clicked.connect(lambda: self.text.setText("TODO Play Last"))
        self.play_last.setObjectName("play_last")

        self.play_next = QPushButton("Play Next")
        self.play_next.clicked.connect(lambda: self.text.setText("TODO Play Next"))
        self.play_next.setObjectName("play_next")

        layout = QVBoxLayout()
        layout.addWidget(self.text)
        layout.addWidget(self.previous_file_label)
        layout.addWidget(self.next_file_label)
        layout.addWidget(self.play_last, Qt.AlignHCenter)
        layout.addWidget(self.play_next, Qt.AlignHCenter)
        layout.setAlignment(self.play_last, Qt.AlignHCenter)
        layout.setAlignment(self.play_next, Qt.AlignHCenter)
        self.setLayout(layout)
        self.refresh_labels()

    def refresh_labels(self):
        config_path = pls.config_file_location()
        pls.ensure_config_directory_exists(config_path)
        # TODO: create the config file as well, not just the dir

        assert config_path.parent.exists()
        assert config_path.parent.is_dir()
        config = pls.load_config_file(config_path)

        # TODO: make the series configurable from CLI
        series = 'Bleach'

        self.text.setText("Series: {}".format(series))

        prev_path = pls.last_played_file(config, series)
        self.previous_file_label.setText("Last played: {}".format(prev_path.name))

        next_path = pls.file_to_play(config, series)
        self.next_file_label.setText("Next: {}".format(next_path.name))


if __name__ == '__main__':
    if len(sys.argv) <= 1:
        context = ApplicationContext()       # 1. Instantiate ApplicationContext
        stylesheet = context.get_resource('styles.qss')
        context.app.setStyleSheet(open(stylesheet).read())
        window = MainWindow()
        window.setObjectName("main-window")
        window.show()
        exit_code = context.app.exec_()      # 2. Invoke appctxt.app.exec_()
        sys.exit(exit_code)
    else:
        print("TODO: use the CLI")

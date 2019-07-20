from fbs_runtime.application_context.PyQt5 import ApplicationContext
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QWidget, QLabel, QPushButton, QVBoxLayout
import requests

import sys


def fetch_quote():
    return requests.get('https://build-system.fman.io/quote').text


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        text = QLabel()
        text.setWordWrap(True)

        button = QPushButton('Next quote >')
        button.clicked.connect(lambda: text.setText(fetch_quote()))

        layout = QVBoxLayout()
        layout.addWidget(text)
        layout.addWidget(button)
        layout.setAlignment(button, Qt.AlignHCenter)
        self.setLayout(layout)

if __name__ == '__main__':
    context = ApplicationContext()       # 1. Instantiate ApplicationContext
    stylesheet = context.get_resource('styles.qss')
    context.app.setStyleSheet(open(stylesheet).read())
    window = MainWindow()
    window.show()
    exit_code = context.app.exec_()      # 2. Invoke appctxt.app.exec_()
    sys.exit(exit_code)

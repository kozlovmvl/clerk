from PyQt6.QtWidgets import *

import sys

from login import LoginWidget


class MainWindow(QMainWindow):

    def __init__(self):
        super().__init__()
        self.set_up()

    def set_up(self):
        self.setWindowTitle('Main')
        self.resize(300, 300)

        container = QWidget()
        
        layout = QVBoxLayout()
        container.setLayout(layout)
        login_widget = LoginWidget()
        layout.addWidget(login_widget)
        
        self.setCentralWidget(container)


if __name__ == '__main__':
    app = QApplication(sys.argv)
    win = MainWindow()
    win.show()
    app.exec()


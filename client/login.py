from PyQt6.QtCore import *
from PyQt6.QtWidgets import *

from network import request


class LoginWidget(QWidget):

    def __init__(self, parent=None):
        super().__init__(parent)
        self.set_up()

    def set_up(self):
        self.setFixedSize(QSize(300, 100))
        grid = QGridLayout()
        self.setLayout(grid)
        grid.addWidget(QLabel('Login'), 0, 0)
        self.line_login = QLineEdit() 
        grid.addWidget(self.line_login, 0, 1)
        grid.addWidget(QLabel('Password'), 1, 0)
        self.line_password = QLineEdit()
        grid.addWidget(self.line_password, 1, 1)
        button = QPushButton('OK')
        button.clicked.connect(self.click_ok)
        grid.addWidget(button)

    def click_ok(self):
        response = request.post(
            url='http://127.0.0.1:8080/user/login', 
            headers={'content-type': 'application/json'},
            body={'username': self.line_login.text(), 'password': self.line_password.text()}
        )
        if response.status == 200:
            print('success')
        else:
            print(response.status)


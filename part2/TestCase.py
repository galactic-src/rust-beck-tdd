from TestResult import TestResult
from TestSuite import TestSuite


class TestCase:
    def __init__(self, name):
        self.name = name

    def set_up(self):
        pass

    def tear_down(self):
        pass

    def run(self, result):
        result.test_started()
        try:
            self.set_up()
            method = getattr(self, self.name)
            method()
        except AssertionError:
            raise
        except Exception as e:
            print(e)
            result.test_failed()
        self.tear_down()
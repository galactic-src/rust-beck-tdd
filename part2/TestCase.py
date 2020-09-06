from TestResult import TestResult


class TestCase:
    def __init__(self, name):
        self.name = name

    def set_up(self):
        pass

    def tear_down(self):
        pass

    def run(self):
        result = TestResult()
        result.test_started()
        try:
            self.set_up()
            method = getattr(self, self.name)
            method()
        except AssertionError:
            raise
        except Exception:
            result.test_failed()
        self.tear_down()
        return result

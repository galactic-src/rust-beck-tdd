from TestResult import TestResult
from TestCase import TestCase


class WasRun(TestCase):
    def __init__(self, name):
        self.log = None
        TestCase.__init__(self, name)

    def set_up(self):
        self.log = "set_up "

    def test_method(self):
        self.log += "test_method "

    def tear_down(self):
        self.log += "tear_down "

    def test_broken_method(self):
        raise Exception


class SetupFails(TestCase):
    def __init__(self, name):
        TestCase.__init__(self, name)

    def set_up(self):
        raise Exception

    def test_method(self):
        pass


class TestCaseTest(TestCase):
    def set_up(self):
        self.test = WasRun("test_method")

    def test_template_method(self):
        test = WasRun("test_method")
        test.run()
        assert "set_up test_method tear_down " == test.log

    def test_result(self):
        test = WasRun("test_method")
        result = test.run()
        assert "1 run, 0 failed" == result.summary()

    def test_failed_result(self):
        test = WasRun("test_broken_method")
        result = test.run()
        assert "1 run, 1 failed" == result.summary()

    def test_failed_result_formatting(self):
        result = TestResult()
        result.test_started()
        result.test_failed()
        assert "1 run, 1 failed" == result.summary()

    def test_failed_setup_result(self):
        test = SetupFails("test_method")
        result = test.run()
        assert "1 run, 1 failed" == result.summary()

    def run_without_failure(self):
        result = self.run()
        assert "1 run, 0 failed" == result.summary()


TestCaseTest("test_template_method").run_without_failure()
TestCaseTest("test_result").run_without_failure()
TestCaseTest("test_failed_result_formatting").run_without_failure()
TestCaseTest("test_failed_result").run_without_failure()
TestCaseTest("test_failed_setup_result").run_without_failure()

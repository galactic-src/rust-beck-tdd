from TestResult import TestResult
from TestCase import TestCase
from TestSuite import TestSuite


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
        self.result = TestResult()

    def test_template_method(self):
        test = WasRun("test_method")
        test.run(self.result)
        assert "set_up test_method tear_down " == test.log

    def test_result(self):
        test = WasRun("test_method")
        test.run(self.result)
        assert "1 run, 0 failed" == self.result.summary()

    def test_failed_result(self):
        test = WasRun("test_broken_method")
        test.run(self.result)
        assert "1 run, 1 failed" == self.result.summary()

    def test_failed_result_formatting(self):
        self.result.test_started()
        self.result.test_failed()
        assert "1 run, 1 failed" == self.result.summary()

    def test_failed_setup_result(self):
        test = SetupFails("test_method")
        test.run(self.result)
        assert "1 run, 1 failed" == self.result.summary()

    def test_suite(self):
        test_suite = TestSuite()
        test_suite.add(WasRun("test_method"))
        test_suite.add(WasRun("test_broken_method"))

        test_suite.run(self.result)
        assert "2 run, 1 failed" == self.result.summary()


suite = TestSuite()
suite.add(TestCaseTest("test_template_method"))
suite.add(TestCaseTest("test_result"))
suite.add(TestCaseTest("test_failed_result_formatting"))
suite.add(TestCaseTest("test_failed_result"))
suite.add(TestCaseTest("test_failed_setup_result"))

suiteResult = TestResult()
suite.run(suiteResult)
print(suiteResult.summary())
